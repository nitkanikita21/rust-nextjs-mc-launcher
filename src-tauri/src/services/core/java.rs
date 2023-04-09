use std::path::PathBuf;

use crate::domain::core::{
    config::Configurable,
    java::{JvmLocation, JvmLocationsInfo},
};

pub async fn get_available_jvms() -> JvmLocationsInfo {
    JvmLocationsInfo::load("java/jvms.json", true).expect("error loading jvms list")
}

pub fn get_javaw_executable(location: JvmLocation) -> PathBuf {
    location.path.join("bin/java/javaw")
}

pub fn check_valid_jvm(location: &JvmLocation) -> bool {
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
