use std::borrow::Cow;

use tauri::{AppHandle, Manager};

use crate::{domain, services};

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
pub async fn login(app_handle: AppHandle) {
    let locked = &mut domain::login::LOGIN_INFO.lock().await;
    services::auth::login_in_ms(locked, &app_handle)
        .await
        .expect("error logging in");
    app_handle
        .emit_all("login_status", {})
        .expect("error emitting");
}

#[tauri::command]
pub async fn logout(app_handle: AppHandle) {
    let mut info = domain::login::LOGIN_INFO.lock().await;
    *info = None;
    app_handle
        .emit_all("login_status", ())
        .expect("error emitting");
}
