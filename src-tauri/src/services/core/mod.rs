
use std::{path::{Path, PathBuf}, env, fs::create_dir};

use tauri::async_runtime::Mutex;


pub mod java;


lazy_static::lazy_static! {
    static ref APP_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
}

pub async fn initialize_app_dir() -> PathBuf {
    let current_dir = env::current_dir().expect("error getting current working dir");

    let app_dir = current_dir.join("app");
    if !app_dir.exists() {
        create_dir(&app_dir).expect("err creating app dir");
    }

    (*APP_DIR.lock().await) = Some(app_dir.clone());
    println!("app dir inited at {:?}", app_dir.clone().as_os_str());
    app_dir
}

pub async fn get_app_dir() -> PathBuf {
    let app_dir_option = APP_DIR.lock().await;
    if app_dir_option.is_none() {
        initialize_app_dir().await
    } else {
        app_dir_option.clone().unwrap()
    }
}


pub async fn init() {
    initialize_app_dir().await;
}