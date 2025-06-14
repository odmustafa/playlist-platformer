mod spotify;

use spotify::{auth::SpotifyAuth, config::SpotifyConfig};

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
    let auth_state = auth
        .exchange_code(&code, &code_verifier)
        .await
        .map_err(|e| e.to_string())?;
    serde_json::to_string(&auth_state).map_err(|e| e.to_string())
}

#[tauri::command]
async fn open_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    use tauri_plugin_shell::ShellExt;
    app.shell().open(url, None).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            spotify_auth_url,
            spotify_exchange_code,
            open_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
