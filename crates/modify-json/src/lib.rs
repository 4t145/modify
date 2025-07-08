pub mod json_path;
pub mod ensure;

pub(crate) type JsonObject = serde_json::Map<String, serde_json::Value>;