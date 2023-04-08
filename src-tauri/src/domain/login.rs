
use minecraft_msa_auth::{self, MinecraftAccessToken};
use serde::Deserialize;
use tauri::async_runtime::Mutex;
use uuid::Uuid;

pub struct LoginInfo {
    pub access_token: MinecraftAccessToken,
    pub username: String,
    pub profile: ProfileInfoResponce
}

lazy_static::lazy_static! {
    pub static ref LOGIN_INFO: Mutex<Option<LoginInfo>> = Mutex::new(None);
}


#[derive(Deserialize, Debug)]
pub struct ProfileInfoResponce {
    pub id: Uuid,
    pub name: String
}