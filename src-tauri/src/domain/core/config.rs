use std::{
    borrow::Cow,
    fs::{self, File},
    io::{BufReader, Write},
    path::Path,
};

use anyhow::{anyhow, Ok};
use serde::{de::DeserializeOwned, Serialize};
use tauri::PathResolver;

pub fn load_config<T>(
    resolver: PathResolver,
    path: impl AsRef<Path>,
    save: bool,
) -> anyhow::Result<T>
where
    T: Serialize + Default + DeserializeOwned,
{
    let cfg_file_path = resolver
        .resolve_resource(&path)
        .ok_or_else(|| anyhow!("Failed to resolve resource: {}", path.as_ref().display()))?;

    let cfg_file_parent = cfg_file_path
        .parent()
        .map(Cow::Borrowed)
        .unwrap_or_default();
    if !cfg_file_parent.exists() {
        fs::create_dir_all(cfg_file_parent)?;
    }
    if !cfg_file_path.exists() {
        let mut cfg_file = File::create(&cfg_file_path)?;
        let default_cfg = T::default();

        cfg_file.write_all(serde_json::to_string(&default_cfg)?.as_str().as_bytes())?;

        return Ok(default_cfg);
    }

    let cfg_file = File::open(&cfg_file_path)?;
    let buf = BufReader::new(cfg_file);

    let cfg: T = serde_json::from_reader(buf)?;
    if save {
        let mut cfg_file = File::create(cfg_file_path)?; // otherwise it will append to the file, as the cursor is already moved
        cfg_file.write_all(serde_json::to_string(&cfg)?.as_str().as_bytes())?;
    }

    Ok(cfg)
}

pub trait Configurable: Serialize + Default + DeserializeOwned {
    fn load(resolver: PathResolver, path: impl AsRef<Path>, save: bool) -> anyhow::Result<Self>;
}
