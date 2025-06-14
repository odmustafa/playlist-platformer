use crate::spotify::config::SpotifyConfig;
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
            "playlist-read-private user-read-private streaming user-read-email user-read-playback-state user-modify-playback-state",
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
