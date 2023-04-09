use minecraft_msa_auth::{self, MinecraftAccessToken};
use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;
use uuid::Uuid;

pub struct LoginInfo {
    pub access_token: MinecraftAccessToken,
    pub username: String,
    pub profile: ProfileInfoResponse,
}

lazy_static::lazy_static! {
    pub static ref LOGIN_INFO: Mutex<Option<LoginInfo>> = Mutex::new(None);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoginStatus {
    LoggedIn,
    LoggedOut,
}

#[derive(Deserialize, Debug)]
pub struct ProfileInfoResponse {
    pub id: Uuid,
    pub name: String,
}
