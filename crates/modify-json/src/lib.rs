pub mod json_path;
pub mod ensure;
pub mod serialize;

pub(crate) type JsonObject = serde_json::Map<String, serde_json::Value>;