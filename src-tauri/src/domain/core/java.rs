use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::services::{self, core::java};

use super::config::{load_config, Configurable};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct JvmInfo {
    java_version: u8,
    name: String,
    fullname: String,
}
impl Display for JvmInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.name, self.java_version)?;
        if self.fullname != self.name {
            write!(f, "({})", self.fullname)?;
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

#[async_trait]
impl Configurable<JvmLocationsInfo> for JvmLocationsInfo {
    async fn load(path: PathBuf, save: bool) -> anyhow::Result<JvmLocationsInfo> {
        println!("test");
        let list: JvmLocationsInfo = load_config(path, save).await?;

        let filtered = list.0
            .iter()
            .filter(|&x| java::check_valid_jvm(x.clone()))
            .cloned()
            .collect();

        Ok(JvmLocationsInfo(filtered))
    }
}

pub mod verified {
    use std::path::PathBuf;

    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};

    use crate::domain::core::config::{load_config, Configurable};

    use super::JvmInfo;

    #[derive(Serialize, Deserialize, Clone, Default, Debug)]
    pub struct JvmDownloadSource {
        url: String,
        info: JvmInfo,
    }

    #[derive(Serialize, Deserialize, Clone, Default, Debug)]
    pub struct JvmRepo(pub Vec<JvmDownloadSource>);

    #[async_trait]
    impl Configurable<JvmRepo> for JvmRepo {
        async fn load(path: PathBuf, save: bool) -> anyhow::Result<JvmRepo> {
            load_config(path, save).await
        }
    }
}
