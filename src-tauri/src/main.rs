#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod domain;
mod services;

#[tokio::main]
async fn main() {
    services::core::init().await;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::login::login,
            command::login::unlogin,
            command::login::get_username,
            command::login::is_logined,
            command::login::get_user_head_render_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
