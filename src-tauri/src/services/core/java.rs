use std::path::PathBuf;

use tauri::async_runtime::Mutex;

use crate::domain::core::{java::{JvmLocationsInfo, JvmLocation}, config::Configurable};

pub async fn get_available_jvms() -> JvmLocationsInfo {
    JvmLocationsInfo::load("java/jvms.json".into(), true).await.expect("error loading jvms list")
}

pub fn get_javaw_executable(location: JvmLocation) -> PathBuf {
    location.path.join("bin/java/javaw")
}

pub fn check_valid_jvm(location: JvmLocation) -> bool {
    if !location.path.join("release").exists() {
        return false;
    }

    if !location.path.join("bin").exists() {
        return false;
    }
    if !location.path.join("bin/java").exists() {
        return false;
    }
    if !location.path.join("bin/javaw").exists() {
        return false;
    }

    true
}