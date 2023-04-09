use std::{
    env,
    fs::{create_dir, File},
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use anyhow::Ok;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use tokio::fs::create_dir_all;

use crate::services::core::get_app_dir;

pub async fn load_config<T: Serialize + Default + DeserializeOwned>(
    path: PathBuf,
    save: bool,
) -> anyhow::Result<T> {
    let add_dir = get_app_dir().await;
    let cfg_file_path = add_dir.join(path);

    if !cfg_file_path.parent().unwrap().exists() {
        create_dir_all(cfg_file_path.parent().unwrap()).await?;
    }
    if !cfg_file_path.exists() {
        let mut __cfg_file = File::create(&cfg_file_path)?;
        let default = T::default();

        __cfg_file.write_all(serde_json::to_string(&default)?.as_str().as_bytes())?;

        Ok(default)
    } else {
        let mut cfg_file = &File::open(cfg_file_path)?;
        let buf = BufReader::new(cfg_file);

        let a: T = serde_json::from_reader(buf)?;
        if save {
            cfg_file.write_all(serde_json::to_string(&a)?.as_str().as_bytes())?;
        }

        Ok(a)
    }
}

#[async_trait]
pub trait Configurable<T: Serialize + Default + DeserializeOwned> {
    async fn load(path: PathBuf, save: bool) -> anyhow::Result<T>;
}
