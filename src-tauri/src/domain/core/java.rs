use std::{
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tauri::PathResolver;

use crate::services::core::java;

use super::config::{load_config, Configurable};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct JvmInfo {
    java_version: u8,
    name: String,
    full_name: String,
}
impl Display for JvmInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.name, self.java_version)?;
        if self.full_name != self.name {
            write!(f, "({})", self.full_name)?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct JvmLocation {
    pub path: PathBuf,
    pub info: JvmInfo,
    verified: bool,
}

impl JvmLocation {
    fn is_verified(&self) -> bool {
        self.verified
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct JvmLocationsInfo(pub Vec<JvmLocation>);

impl Configurable for JvmLocationsInfo {
    fn load(resolver: PathResolver, path: impl AsRef<Path>, save: bool) -> anyhow::Result<Self> {
        println!("test");
        let list: JvmLocationsInfo = load_config(resolver, path, save)?;

        let filtered = list.0.into_iter().filter(java::check_valid_jvm).collect();

        Ok(JvmLocationsInfo(filtered))
    }
}

pub mod verified {
    use std::path::Path;

    use serde::{Deserialize, Serialize};
    use tauri::PathResolver;

    use crate::domain::core::config::{load_config, Configurable};

    use super::JvmInfo;

    #[derive(Serialize, Deserialize, Clone, Default, Debug)]
    pub struct JvmDownloadSource {
        url: String,
        info: JvmInfo,
    }

    #[derive(Serialize, Deserialize, Clone, Default, Debug)]
    pub struct JvmRepo(pub Vec<JvmDownloadSource>);

    impl Configurable for JvmRepo {
        fn load(
            resolver: PathResolver,
            path: impl AsRef<Path>,
            save: bool,
        ) -> anyhow::Result<Self> {
            load_config(resolver, path, save)
        }
    }
}
