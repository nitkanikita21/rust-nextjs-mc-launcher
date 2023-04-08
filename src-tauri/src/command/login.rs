use ::reqwest::{Method, Request};
use tauri::{async_runtime::Mutex, AppHandle, Manager, State};

use crate::{
    domain::{self, login::LoginInfo},
    services,
};

#[tauri::command]
pub async fn is_logined() -> bool {
    let login_info = domain::login::LOGIN_INFO.lock().await;
    login_info.is_some()
}

#[tauri::command]
pub async fn get_username() -> String {
    let login_info = domain::login::LOGIN_INFO.lock().await;
    match login_info.as_ref() {
        Some(info) => info.profile.name.clone(),
        None => "NONE".to_string()
    }
}

#[tauri::command]
pub async fn get_user_head_render_url() -> String {
    let login_info = domain::login::LOGIN_INFO.lock().await;
    match login_info.as_ref() {
        Some(info) => format!("https://crafatar.com/avatars/{}?size=64&default=MHF_Steve&overlay", info.profile.id),
        None => "".to_string()
    }
}

#[tauri::command]
pub async fn login(app_handle: AppHandle) {
    let locked = &mut domain::login::LOGIN_INFO.lock().await;
    services::auth::login_in_ms(locked, &app_handle).await.expect("error logining");
    app_handle.emit_all("login_status", {}).expect("error emiting");
}

#[tauri::command]
pub async fn unlogin(app_handle: AppHandle) {
    let mut info = &mut domain::login::LOGIN_INFO.lock().await;
    **info = None;
    app_handle.emit_all("login_status", {}).expect("error emiting");
}
