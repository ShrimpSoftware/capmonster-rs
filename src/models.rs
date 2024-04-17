use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct APIRequest {
    #[serde(rename = "clientKey")]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Value>,
    #[serde(rename = "callbackUrl", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<i64>,
}
