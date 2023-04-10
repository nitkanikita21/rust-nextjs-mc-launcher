use tauri::AppHandle;

use crate::{services, domain::core::java::JvmLocationsInfo};

#[tauri::command]
pub fn get_available_jvms(app_handle: AppHandle) -> JvmLocationsInfo {
    services::core::java::get_available_jvms(app_handle.path_resolver()).expect("tauri command - error getting available jvms")
}

#[tauri::command]
pub fn install_jvm(app_handle: AppHandle, id: String) {

}