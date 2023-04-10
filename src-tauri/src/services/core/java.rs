use std::path::PathBuf;

use anyhow::Context;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use tauri::PathResolver;

use crate::domain::core::{
    config::Configurable,
    java::{JvmLocation, JvmLocationsInfo},
};

pub fn get_available_jvms(resolver: PathResolver) -> anyhow::Result<JvmLocationsInfo> {
    JvmLocationsInfo::load(resolver, "java/jvms.json", true).context("error loading jvms list")
}

pub fn get_javaw_executable(location: JvmLocation) -> PathBuf {
    location.path.join("bin/java/javaw")
}

pub fn check_valid_jvm(location: &JvmLocation) -> bool {
    ["release", "bin", "bin/java", "bin/javaw"]
        .into_par_iter()
        .all(|path| location.path.join(path).exists())
}
