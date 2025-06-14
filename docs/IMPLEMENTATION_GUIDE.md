# Implementation Guide - Playlist Platformer

## Prerequisites

### Development Environment
- **Rust**: Latest stable version (1.70+)
- **Node.js**: Version 18+ for frontend tooling
- **Tauri CLI**: `cargo install tauri-cli`
- **Git**: For version control

### Required Accounts & Keys
- **Spotify Developer Account**: Create app at https://developer.spotify.com/dashboard
- **Spotify Client ID & Secret**: Already configured in `spotify.settings.json`

## Step 1: Project Initialization

### 1.1 Create Tauri Project
```bash
# Create new Tauri project
cargo tauri init

# Project name: playlist-platformer
# Window title: Playlist Platformer
# Web assets location: ../dist
# Dev server URL: http://localhost:3000
# Frontend framework: React/Vue/Vanilla (choose based on preference)
```

### 1.2 Configure Tauri
Edit `src-tauri/tauri.conf.json`:
```json
{
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "devPath": "http://localhost:3000",
    "distDir": "../dist"
  },
  "package": {
    "productName": "Playlist Platformer",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": true,
        "scope": ["$DOWNLOAD/*", "$CONFIG/*"]
      },
      "http": {
        "all": true,
        "request": true,
        "scope": ["https://api.spotify.com/*", "https://accounts.spotify.com/*"]
      },
      "shell": {
        "all": false,
        "open": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.playlistplatformer.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 800,
        "resizable": true,
        "title": "Playlist Platformer",
        "width": 1200,
        "minWidth": 800,
        "minHeight": 600
      }
    ]
  }
}
```

## Step 2: Spotify Integration

### 2.1 Backend Dependencies
Add to `src-tauri/Cargo.toml`:
```toml
[dependencies]
tauri = { version = "1.0", features = ["api-all"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.0", features = ["full"] }
base64 = "0.21"
url = "2.4"
uuid = { version = "1.0", features = ["v4"] }
sha2 = "0.10"
```

### 2.2 Spotify API Module
Create `src-tauri/src/spotify/mod.rs`:
```rust
pub mod auth;
pub mod api;
pub mod types;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl SpotifyConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string("spotify.settings.json")?;
        let config: serde_json::Value = serde_json::from_str(&config_str)?;
        
        Ok(SpotifyConfig {
            client_id: config["SPOTIFY_CLIENT_ID"].as_str().unwrap().to_string(),
            client_secret: config["SPOTIFY_CLIENT_SECRET"].as_str().unwrap().to_string(),
            redirect_uri: "http://localhost:8080/callback".to_string(),
        })
    }
}
```

### 2.3 OAuth Implementation
Create `src-tauri/src/spotify/auth.rs`:
```rust
use crate::spotify::SpotifyConfig;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthState {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<u64>,
}

pub struct SpotifyAuth {
    config: SpotifyConfig,
    client: reqwest::Client,
}

impl SpotifyAuth {
    pub fn new(config: SpotifyConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub fn generate_auth_url(&self) -> (String, String, String) {
        let state = uuid::Uuid::new_v4().to_string();
        let code_verifier = generate_code_verifier();
        let code_challenge = generate_code_challenge(&code_verifier);
        
        let auth_url = format!(
            "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope={}&redirect_uri={}&state={}&code_challenge_method=S256&code_challenge={}",
            self.config.client_id,
            "playlist-read-private user-read-private",
            urlencoding::encode(&self.config.redirect_uri),
            state,
            code_challenge
        );
        
        (auth_url, state, code_verifier)
    }

    pub async fn exchange_code(&self, code: &str, code_verifier: &str) -> Result<AuthState, Box<dyn std::error::Error>> {
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &self.config.redirect_uri),
            ("client_id", &self.config.client_id),
            ("code_verifier", code_verifier),
        ];

        let response = self.client
            .post("https://accounts.spotify.com/api/token")
            .form(&params)
            .send()
            .await?;

        let token_response: serde_json::Value = response.json().await?;
        
        Ok(AuthState {
            access_token: token_response["access_token"].as_str().map(String::from),
            refresh_token: token_response["refresh_token"].as_str().map(String::from),
            expires_at: Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs() + token_response["expires_in"].as_u64().unwrap_or(3600)
            ),
        })
    }
}

fn generate_code_verifier() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..128)
        .map(|_| {
            let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
            chars[rng.gen_range(0..chars.len())] as char
        })
        .collect()
}

fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    general_purpose::URL_SAFE_NO_PAD.encode(hash)
}
```

### 2.4 Tauri Commands
Add to `src-tauri/src/main.rs`:
```rust
mod spotify;

use spotify::{SpotifyConfig, auth::SpotifyAuth};

#[tauri::command]
async fn spotify_auth_url() -> Result<(String, String, String), String> {
    let config = SpotifyConfig::load().map_err(|e| e.to_string())?;
    let auth = SpotifyAuth::new(config);
    Ok(auth.generate_auth_url())
}

#[tauri::command]
async fn spotify_exchange_code(code: String, code_verifier: String) -> Result<String, String> {
    let config = SpotifyConfig::load().map_err(|e| e.to_string())?;
    let auth = SpotifyAuth::new(config);
    let auth_state = auth.exchange_code(&code, &code_verifier).await.map_err(|e| e.to_string())?;
    serde_json::to_string(&auth_state).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            spotify_auth_url,
            spotify_exchange_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Step 3: Frontend Setup

### 3.1 Install Dependencies
```bash
npm install
npm install @spotify/web-playback-sdk
npm install axios
npm install react-router-dom  # if using React
```

### 3.2 Spotify Service
Create `src/services/spotify.js`:
```javascript
import { invoke } from '@tauri-apps/api/tauri';

class SpotifyService {
    constructor() {
        this.accessToken = localStorage.getItem('spotify_access_token');
        this.refreshToken = localStorage.getItem('spotify_refresh_token');
    }

    async authenticate() {
        try {
            const [authUrl, state, codeVerifier] = await invoke('spotify_auth_url');
            
            // Store code verifier for later use
            localStorage.setItem('spotify_code_verifier', codeVerifier);
            localStorage.setItem('spotify_state', state);
            
            // Open auth URL in external browser
            await invoke('open_url', { url: authUrl });
            
            return { authUrl, state, codeVerifier };
        } catch (error) {
            console.error('Authentication error:', error);
            throw error;
        }
    }

    async handleCallback(code, state) {
        const storedState = localStorage.getItem('spotify_state');
        const codeVerifier = localStorage.getItem('spotify_code_verifier');
        
        if (state !== storedState) {
            throw new Error('State mismatch');
        }
        
        try {
            const authStateJson = await invoke('spotify_exchange_code', { 
                code, 
                codeVerifier 
            });
            const authState = JSON.parse(authStateJson);
            
            this.accessToken = authState.access_token;
            this.refreshToken = authState.refresh_token;
            
            localStorage.setItem('spotify_access_token', this.accessToken);
            localStorage.setItem('spotify_refresh_token', this.refreshToken);
            
            return authState;
        } catch (error) {
            console.error('Token exchange error:', error);
            throw error;
        }
    }

    async getPlaylists() {
        if (!this.accessToken) {
            throw new Error('Not authenticated');
        }

        const response = await fetch('https://api.spotify.com/v1/me/playlists', {
            headers: {
                'Authorization': `Bearer ${this.accessToken}`
            }
        });

        if (!response.ok) {
            throw new Error('Failed to fetch playlists');
        }

        return response.json();
    }

    async getPlaylistTracks(playlistId) {
        if (!this.accessToken) {
            throw new Error('Not authenticated');
        }

        const response = await fetch(`https://api.spotify.com/v1/playlists/${playlistId}/tracks`, {
            headers: {
                'Authorization': `Bearer ${this.accessToken}`
            }
        });

        if (!response.ok) {
            throw new Error('Failed to fetch playlist tracks');
        }

        return response.json();
    }
}

export default new SpotifyService();
```

## Step 4: Soulseek Integration Planning

### 4.1 Protocol Research
Study the Nicotine+ implementation:
- Connection management: `pynicotine/core/network.py`
- Search functionality: `pynicotine/core/search.py`
- Download handling: `pynicotine/core/downloads.py`

### 4.2 Rust Implementation Strategy
1. **Network Layer**: TCP connections to Soulseek server and peers
2. **Protocol Messages**: Binary message parsing and construction
3. **Search System**: Query formatting and result parsing
4. **Download Manager**: File transfer coordination

### 4.3 Key Components to Implement
- Server connection and login
- Peer discovery and connection
- Search request/response handling
- File transfer protocol
- Queue management

## Next Steps
1. Complete Spotify integration and test authentication flow
2. Build basic UI for playlist browsing
3. Research and implement Soulseek protocol basics
4. Integrate Web Playback SDK for audio previews
5. Add download management functionality

## Important Notes
- Always handle errors gracefully
- Implement proper logging for debugging
- Consider rate limiting for API calls
- Add configuration options for user preferences
- Test cross-platform compatibility regularly
