// Спасибо github.com/MarshalX за актуальный client_id <3
// https://github.com/MarshalX/yandex-music-api/blob/0fa54f2d32084a9e461bce41890d1c9ab70d91aa/yandex_music/_client/device_auth.py#L2

use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use serde::{Serialize, Deserialize}; // Добавили
use std::fs;                           // Добавили

#[derive(Serialize, Deserialize)]
struct Session {
    access_token: String,
}

#[tauri::command]
async fn open_auth_window(app: tauri::AppHandle) {
    let client_id = "23cabbbdc6cd418abb4b39c32c41195d";
    let auth_url = format!(
        "https://oauth.yandex.ru/authorize?response_type=token&client_id={}",
        client_id
    );

    let handle = app.clone();

    let _auth_window = WebviewWindowBuilder::new(&app, "auth", WebviewUrl::External(auth_url.parse().unwrap()))
        .title("Yandex Auth")
        .inner_size(600.0, 700.0)
        .incognito(false)
        .user_agent("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Mobile Safari/537.36")
        .on_navigation(move |url| {
            let url_str = url.as_str();
            
            if url_str.contains("access_token=") {
                let token = url_str.split("access_token=")
                    .nth(1)
                    .and_then(|s| s.split('&').next())
                    .unwrap_or("");

                if !token.is_empty() {
                    println!("ТОКЕН ПОЙМАН: {}", token);

                    // --- Сохранение сессии ---
                    let token_string = token.to_string();
                    let app_handle = handle.clone();
                    
                    // Сохраняем в папку данных приложения (AppData/Roaming/... или ~/.local/share/...)
                    if let Ok(app_dir) = app_handle.path().app_data_dir() {
                        let _ = fs::create_dir_all(&app_dir);
                        let path = app_dir.join("session.json");
                        let session = Session { access_token: token_string };
                        if let Ok(json) = serde_json::to_string_pretty(&session) {
                            let _ = fs::write(path, json);
                        }
                    }
                    // -------------------------

                    let _ = handle.emit("auth-success", token);
                    
                    let h = handle.clone();
                    tauri::async_runtime::spawn(async move {
                        if let Some(window) = h.get_webview_window("auth") {
                            let _ = window.close();
                        }
                    });
                    return false;
                }
            }
            true
        })
        .build()
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_auth_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}