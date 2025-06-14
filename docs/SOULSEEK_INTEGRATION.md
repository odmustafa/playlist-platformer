# Soulseek Integration Guide

## Overview
This guide details how to integrate Soulseek peer-to-peer functionality into the Playlist Platformer application, based on analysis of the Nicotine+ implementation.

## Soulseek Protocol Basics

### Network Architecture
- **Server Connection**: Central server for user discovery and search coordination
- **Peer-to-Peer**: Direct connections between users for file transfers
- **Protocol**: Custom binary protocol over TCP

### Key Components
1. **Server Messages**: Login, search requests, user info
2. **Peer Messages**: File sharing, downloads, browse requests
3. **File Sharing**: Share local files with other users
4. **Search System**: Query the network for files

## Implementation Strategy

### 1. Core Network Module
Create `src-tauri/src/soulseek/network.rs`:

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;

pub struct SoulseekClient {
    server_connection: Option<TcpStream>,
    peer_connections: HashMap<String, TcpStream>,
    username: String,
    password: String,
    token: Option<u32>,
}

impl SoulseekClient {
    pub fn new(username: String, password: String) -> Self {
        Self {
            server_connection: None,
            peer_connections: HashMap::new(),
            username,
            password,
            token: None,
        }
    }

    pub async fn connect_to_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Default Soulseek server
        let server_addr = "server.slsknet.org:2242";
        let stream = TcpStream::connect(server_addr).await?;
        self.server_connection = Some(stream);
        Ok(())
    }

    pub async fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut stream) = self.server_connection {
            let login_message = self.build_login_message();
            stream.write_all(&login_message).await?;
            
            // Read response
            let mut buffer = vec![0u8; 1024];
            let n = stream.read(&mut buffer).await?;
            self.parse_login_response(&buffer[..n])?;
        }
        Ok(())
    }

    fn build_login_message(&self) -> Vec<u8> {
        // Message format: [length][code][username][password][version][hash]
        let mut message = Vec::new();
        
        // Message code for login (1)
        message.extend_from_slice(&1u32.to_le_bytes());
        
        // Username
        message.extend_from_slice(&(self.username.len() as u32).to_le_bytes());
        message.extend_from_slice(self.username.as_bytes());
        
        // Password
        message.extend_from_slice(&(self.password.len() as u32).to_le_bytes());
        message.extend_from_slice(self.password.as_bytes());
        
        // Version (183 is common)
        message.extend_from_slice(&183u32.to_le_bytes());
        
        // Hash (MD5 of username+password, simplified here)
        let hash = format!("{}{}", self.username, self.password);
        let hash_bytes = md5::compute(hash.as_bytes());
        message.extend_from_slice(&hash_bytes.0);
        
        // Prepend total length
        let total_length = message.len() as u32;
        let mut final_message = total_length.to_le_bytes().to_vec();
        final_message.extend(message);
        
        final_message
    }

    fn parse_login_response(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if data.len() < 8 {
            return Err("Invalid login response".into());
        }
        
        let length = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let code = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        
        match code {
            1 => {
                // Success - extract token if present
                if data.len() >= 12 {
                    self.token = Some(u32::from_le_bytes([data[8], data[9], data[10], data[11]]));
                }
                println!("Login successful");
            }
            _ => {
                return Err("Login failed".into());
            }
        }
        
        Ok(())
    }
}
```

### 2. Search Implementation
Create `src-tauri/src/soulseek/search.rs`:

```rust
use super::network::SoulseekClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub username: String,
    pub filename: String,
    pub size: u64,
    pub bitrate: Option<u32>,
    pub duration: Option<u32>,
    pub quality: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub token: u32,
}

impl SoulseekClient {
    pub async fn search(&mut self, query: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let search_token = self.generate_search_token();
        let search_message = self.build_search_message(query, search_token);
        
        if let Some(ref mut stream) = self.server_connection {
            stream.write_all(&search_message).await?;
        }
        
        // In a real implementation, you'd listen for responses asynchronously
        // For now, we'll return empty results
        Ok(Vec::new())
    }

    fn generate_search_token(&self) -> u32 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32
    }

    fn build_search_message(&self, query: &str, token: u32) -> Vec<u8> {
        let mut message = Vec::new();
        
        // Message code for file search (26)
        message.extend_from_slice(&26u32.to_le_bytes());
        
        // Search token
        message.extend_from_slice(&token.to_le_bytes());
        
        // Query string
        message.extend_from_slice(&(query.len() as u32).to_le_bytes());
        message.extend_from_slice(query.as_bytes());
        
        // Prepend total length
        let total_length = message.len() as u32;
        let mut final_message = total_length.to_le_bytes().to_vec();
        final_message.extend(message);
        
        final_message
    }

    pub fn format_search_query(artist: &str, track: &str) -> String {
        // Format: "artist track" - common Soulseek search format
        format!("{} {}", artist, track)
    }
}
```

### 3. Download Management
Create `src-tauri/src/soulseek/downloads.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadItem {
    pub id: String,
    pub username: String,
    pub filename: String,
    pub size: u64,
    pub downloaded: u64,
    pub status: DownloadStatus,
    pub local_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DownloadStatus {
    Queued,
    Connecting,
    Downloading,
    Completed,
    Failed(String),
    Paused,
}

pub struct DownloadManager {
    downloads: Vec<DownloadItem>,
    download_dir: PathBuf,
}

impl DownloadManager {
    pub fn new(download_dir: PathBuf) -> Self {
        Self {
            downloads: Vec::new(),
            download_dir,
        }
    }

    pub fn add_download(&mut self, username: String, filename: String, size: u64) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let local_path = self.download_dir.join(&filename);
        
        let download = DownloadItem {
            id: id.clone(),
            username,
            filename,
            size,
            downloaded: 0,
            status: DownloadStatus::Queued,
            local_path,
        };
        
        self.downloads.push(download);
        id
    }

    pub fn get_downloads(&self) -> &[DownloadItem] {
        &self.downloads
    }

    pub fn get_download_progress(&self, id: &str) -> Option<f64> {
        self.downloads
            .iter()
            .find(|d| d.id == id)
            .map(|d| if d.size > 0 { d.downloaded as f64 / d.size as f64 } else { 0.0 })
    }

    pub async fn start_download(&mut self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.id == id) {
            download.status = DownloadStatus::Connecting;
            
            // In a real implementation, this would:
            // 1. Connect to the peer
            // 2. Request the file
            // 3. Stream the data to local file
            // 4. Update progress
            
            // Placeholder implementation
            download.status = DownloadStatus::Downloading;
        }
        
        Ok(())
    }

    pub fn pause_download(&mut self, id: &str) {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.id == id) {
            download.status = DownloadStatus::Paused;
        }
    }

    pub fn remove_download(&mut self, id: &str) {
        self.downloads.retain(|d| d.id != id);
    }
}
```

### 4. Tauri Commands for Soulseek
Add to `src-tauri/src/main.rs`:

```rust
mod soulseek;

use soulseek::{SoulseekClient, SearchResult, DownloadManager, DownloadItem};
use std::sync::Mutex;
use tauri::State;

struct AppState {
    soulseek_client: Mutex<Option<SoulseekClient>>,
    download_manager: Mutex<DownloadManager>,
}

#[tauri::command]
async fn soulseek_connect(username: String, password: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut client = SoulseekClient::new(username, password);
    client.connect_to_server().await.map_err(|e| e.to_string())?;
    client.login().await.map_err(|e| e.to_string())?;
    
    *state.soulseek_client.lock().unwrap() = Some(client);
    Ok(())
}

#[tauri::command]
async fn soulseek_search(artist: String, track: String, state: State<'_, AppState>) -> Result<Vec<SearchResult>, String> {
    let query = SoulseekClient::format_search_query(&artist, &track);
    
    if let Some(ref mut client) = *state.soulseek_client.lock().unwrap() {
        client.search(&query).await.map_err(|e| e.to_string())
    } else {
        Err("Not connected to Soulseek".to_string())
    }
}

#[tauri::command]
async fn add_download(username: String, filename: String, size: u64, state: State<'_, AppState>) -> Result<String, String> {
    let id = state.download_manager.lock().unwrap().add_download(username, filename, size);
    Ok(id)
}

#[tauri::command]
async fn get_downloads(state: State<'_, AppState>) -> Result<Vec<DownloadItem>, String> {
    Ok(state.download_manager.lock().unwrap().get_downloads().to_vec())
}

#[tauri::command]
async fn start_download(id: String, state: State<'_, AppState>) -> Result<(), String> {
    state.download_manager.lock().unwrap().start_download(&id).await.map_err(|e| e.to_string())
}

// Update main function to include new commands and state
fn main() {
    let download_dir = dirs::download_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
    
    tauri::Builder::default()
        .manage(AppState {
            soulseek_client: Mutex::new(None),
            download_manager: Mutex::new(DownloadManager::new(download_dir)),
        })
        .invoke_handler(tauri::generate_handler![
            spotify_auth_url,
            spotify_exchange_code,
            soulseek_connect,
            soulseek_search,
            add_download,
            get_downloads,
            start_download
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Frontend Integration

### Search Component
```javascript
// src/components/TrackSearch.jsx
import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';

export function TrackSearch({ artist, track }) {
    const [results, setResults] = useState([]);
    const [loading, setLoading] = useState(false);

    const handleSearch = async () => {
        setLoading(true);
        try {
            const searchResults = await invoke('soulseek_search', { artist, track });
            setResults(searchResults);
        } catch (error) {
            console.error('Search failed:', error);
        } finally {
            setLoading(false);
        }
    };

    const handleDownload = async (result) => {
        try {
            const downloadId = await invoke('add_download', {
                username: result.username,
                filename: result.filename,
                size: result.size
            });
            
            await invoke('start_download', { id: downloadId });
        } catch (error) {
            console.error('Download failed:', error);
        }
    };

    return (
        <div className="track-search">
            <button onClick={handleSearch} disabled={loading}>
                {loading ? 'Searching...' : 'Search Soulseek'}
            </button>
            
            {results.length > 0 && (
                <div className="search-results">
                    {results.map((result, index) => (
                        <div key={index} className="result-item">
                            <span>{result.filename}</span>
                            <span>{(result.size / 1024 / 1024).toFixed(2)} MB</span>
                            <span>{result.quality}</span>
                            <button onClick={() => handleDownload(result)}>
                                Download
                            </button>
                        </div>
                    ))}
                </div>
            )}
        </div>
    );
}
```

## Important Considerations

### Legal and Ethical
- Add clear disclaimers about copyright and legal use
- Implement user responsibility acknowledgments
- Consider adding educational/research use only notices

### Technical Challenges
- **Protocol Complexity**: Soulseek protocol is complex and undocumented
- **Network Reliability**: P2P connections can be unstable
- **File Verification**: Ensure downloaded files match expectations
- **Rate Limiting**: Avoid overwhelming the network

### Security
- Validate all incoming data
- Implement connection timeouts
- Sanitize file paths and names
- Consider sandboxing download operations

## Next Steps
1. Study Nicotine+ source code in detail
2. Implement basic server connection
3. Add search functionality
4. Build download management UI
5. Test with real Soulseek network
6. Add error handling and recovery
