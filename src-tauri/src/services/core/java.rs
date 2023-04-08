use serde::Serialize;

#[derive(Serialize)]
pub struct JvmInfo {
  java_version: u8,
  name: String,
}
