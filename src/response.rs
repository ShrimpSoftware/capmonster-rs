use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "errorId")]
    pub error_id: i8,
    #[serde(rename = "errorCode")]
    pub error_code: String,
    #[serde(rename = "errorDescription")]
    pub error_description: String,
}

#[derive(Debug, Deserialize)]
pub struct TaskStatus<T> {
    pub error_code: Option<String>,
    pub error_description: Option<String>,
    pub error_id: i64,
    pub status: String,
    pub solution: Option<T>,
}

#[derive(Debug, Deserialize)]
pub struct TaskResponse {
    #[serde(rename = "taskId")]
    pub task_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct BalanceResponse {
    pub balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct RecaptchaV2Response {
    #[serde(rename = "gRecaptchaResponse")]
    pub grecaptcha: String,
}


#[derive(Serialize, Deserialize)]
struct HCaptchaResponse {
    #[serde(rename = "gRecaptchaResponse")]
    pub g_recaptcha_response: String,
    #[serde(rename = "respKey")]
    pub resp_key: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
}


#[derive(Serialize, Deserialize)]
struct GeeTestResponse {
    pub challenge: String,
    pub validate: String,
    pub seccode: String,
}

#[derive(Debug, Deserialize)]
pub struct TurnstileResponse {
    #[serde(rename = "cf_clearance")]
    pub cf_clearance: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
struct ImageToTextResponse {
    pub text: String,
}