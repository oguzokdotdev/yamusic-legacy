use keyring::Entry;
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const KEYRING_SERVICE: &str = "yamusic-legacy";
const KEYRING_USER: &str = "yandex_token";

#[tauri::command]
fn save_token(token: String) -> Result<(), String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER).map_err(|e| e.to_string())?;
    entry.set_password(&token).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_token() -> Result<Option<String>, String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER).map_err(|e| e.to_string())?;
    match entry.get_password() {
        Ok(token) => Ok(Some(token)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn delete_token() -> Result<(), String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER).map_err(|e| e.to_string())?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
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
        .user_agent("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Mobile Safari/537.36")
        .on_navigation(move |url| {
            let url_str = url.as_str();

            if url_str.contains("access_token=") {
                let token = url_str.split("access_token=")
                    .nth(1)
                    .and_then(|s| s.split('&').next())
                    .unwrap_or("")
                    .to_string();

                if !token.is_empty() {
                    println!("ТОКЕН ПОЙМАН: {}", token);

                    // Сохраняем в keyring
                    if let Ok(entry) = Entry::new(KEYRING_SERVICE, KEYRING_USER) {
                        let _ = entry.set_password(&token);
                    }

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
        .invoke_handler(tauri::generate_handler![
            open_auth_window,
            save_token,
            get_token,
            delete_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}