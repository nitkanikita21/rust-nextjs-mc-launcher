use std::borrow::Cow;

use tauri::{AppHandle, Manager, Runtime};

use crate::{
    domain::{self, login::LoginStatus},
    services,
};

#[tauri::command]
pub async fn is_logged_in() -> bool {
    let login_info = domain::login::LOGIN_INFO.lock().await;
    login_info.is_some()
}

#[tauri::command]
pub async fn get_username() -> Cow<'static, str> {
    let login_info = domain::login::LOGIN_INFO.lock().await;
    login_info
        .as_ref()
        .map(|info| info.profile.name.clone().into())
        .unwrap_or(Cow::Borrowed("NONE"))
}

#[tauri::command]
pub async fn get_user_head_render_url() -> String {
    let login_info = domain::login::LOGIN_INFO.lock().await;
    login_info
        .as_ref()
        .map(|info| {
            format!(
                "https://crafatar.com/avatars/{}?size=64&default=MHF_Steve&overlay",
                info.profile.id
            )
        })
        .unwrap_or_default()
}

#[tauri::command]
pub async fn login<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    let locked = &mut domain::login::LOGIN_INFO.lock().await;
    services::auth::login_in_ms(locked, &app_handle)
        .await
        .map_err(|e| format!("Error logging in: {e}"))?;
    app_handle
        .emit_all("login_status", LoginStatus::LoggedIn)
        .map_err(|e| format!("error emitting: {e}"))
}

#[tauri::command]
pub async fn logout<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    let mut info = domain::login::LOGIN_INFO.lock().await;
    *info = None;
    app_handle
        .emit_all("login_status", LoginStatus::LoggedOut)
        .map_err(|e| format!("error emitting: {e}"))
}
