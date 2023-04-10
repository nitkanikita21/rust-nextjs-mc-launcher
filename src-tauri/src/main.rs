#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod domain;
mod services;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let resolver = app.path_resolver();
            let jvms = services::core::java::get_available_jvms(resolver);
            println!("{:?}", jvms);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::login::login,
            command::login::logout,
            command::login::get_username,
            command::login::is_logged_in,
            command::login::get_user_head_render_url,

            command::core::java::get_available_jvms
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
