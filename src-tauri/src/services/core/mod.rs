use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub mod java;

lazy_static::lazy_static! {
    static ref APP_DIR: PathBuf = {
        let current_dir = env::current_dir().expect("error getting current working dir");

        let app_dir = current_dir.join("app");
        if !app_dir.exists() {
            fs::create_dir(&app_dir).expect("err creating app dir");
        }
        app_dir
    };
}

pub fn app_dir() -> &'static Path {
    APP_DIR.as_path()
}
