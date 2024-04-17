use crate::errors::Error;
use crate::proxies::{Proxiable, Proxy};
use crate::response::TurnstileResponse;
use crate::tasks::{merge, Task};

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Default)]
pub struct TurnstileTask {
    #[serde(skip_serializing)]
    pub id: Option<i64>,
    #[serde(rename = "type")]
    pub task_type: String,
    #[serde(rename = "websiteURL")]
    pub website_url: String,
    #[serde(rename = "websiteKey")]
    pub website_key: String,
    #[serde(skip_serializing)]
    pub proxy: Option<Proxy>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudflareTaskType")]
    pub cloudflare_task_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "htmlPageBase64")]
    pub html_page_base64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "userAgent")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "pageAction")]
    pub page_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "pageData")]
    pub page_data: Option<String>,
}

impl TurnstileTask {
    pub fn new<I>(url: I, site_key: I) -> Self
    where
        I: Into<String>,
    {
        TurnstileTask {
            id: None,
            task_type: String::from("TurnstileTaskProxyless"),
            website_url: url.into(),
            website_key: site_key.into(),
            ..Default::default()
        }
    }

    pub fn set_clearance(mut self, user_agent: String, html_page_base64: String) -> Self {
        self.cloudflare_task_type = Some(String::from("cf_clearance"));
        self.html_page_base64 = Some(html_page_base64);
        self.user_agent = Some(user_agent);
        self
    }

    pub fn set_token(mut self, user_agent: String, page_action: String, data: String, page_data: String) -> Self {
        self.cloudflare_task_type = Some(String::from("token"));
        self.user_agent = Some(user_agent);
        self.page_action = Some(page_action);
        self.data = Some(data);
        self.page_data = Some(page_data);
        self
    }
}

impl Task for TurnstileTask {
    type TaskResult = TurnstileResponse;

    fn get_task_id(&self) -> Option<i64> {
        self.id
    }

    fn set_task_id(&mut self, task_id: i64) {
        self.id = Some(task_id)
    }

    fn description(&self) -> String {
        String::from("Bypass Turnstile captcha")
    }

    fn as_value(&self) -> Result<Value, Error> {
        if let Some(proxy) = &self.proxy {
            let proxy_value = serde_json::to_value(proxy)?;
            let mut task_value = serde_json::to_value(self)?;
            merge(&mut task_value, proxy_value);
            return Ok(task_value);
        }
        Ok(serde_json::to_value(self)?)
    }
}

impl Proxiable for TurnstileTask {
    fn set_proxy(&mut self, proxy: Proxy) {
        self.proxy = Some(proxy);
        self.task_type = String::from("TurnstileTask");
    }
}
