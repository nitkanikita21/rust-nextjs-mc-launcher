#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod domain;
mod services;

#[tokio::main]
async fn main() {
    _ = services::core::app_dir(); // init app dir

    let jvms = services::core::java::get_available_jvms().await;

    println!("{:?}", jvms);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::login::login,
            command::login::logout,
            command::login::get_username,
            command::login::is_logged_in,
            command::login::get_user_head_render_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
