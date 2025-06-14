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
