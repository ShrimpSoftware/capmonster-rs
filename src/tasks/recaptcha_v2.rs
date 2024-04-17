use crate::errors::Error;
use crate::proxies::{Proxiable, Proxy};
use crate::response::RecaptchaV2Response;
use crate::tasks::{merge, Task};

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Default)]
pub struct RecaptchaV2Task {
    #[serde(skip_serializing)]
    pub id: Option<i64>,
    #[serde(rename = "type")]
    pub task_type: String,
    #[serde(rename = "websiteURL")]
    pub website_url: String,
    #[serde(rename = "websiteKey")]
    pub website_key: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "recaptchaDataSValue")]
    pub data_s_value: Option<String>,
    #[serde(skip_serializing)]
    pub proxy: Option<Proxy>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "userAgent")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(rename = "isInvisible", skip_serializing_if = "Option::is_none", )]
    pub is_invisible: Option<bool>,
  
}

impl RecaptchaV2Task {
    pub fn new(url: impl Into<String>, site_key: impl Into<String>) -> Self {
        RecaptchaV2Task {
            id: None,
            task_type: String::from("RecaptchaV2TaskProxyless"),
            website_url: url.into(),
            website_key: site_key.into(),
            ..Default::default()
        }
    }

    pub fn set_sdata(mut self, sdata: impl Into<String>) -> Self {
        self.data_s_value = Some(sdata.into());
        self
    }
    
    pub fn set_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
    
    pub fn set_cookies(mut self, cookies: impl Into<String>) -> Self {
        self.cookies = Some(cookies.into());
        self
    }

    pub fn set_invisible(mut self, is_invisible: bool) -> Self {
        self.is_invisible = Some(is_invisible);
        self
    }
}

impl Task for RecaptchaV2Task {
    type TaskResult = RecaptchaV2Response;

    fn get_task_id(&self) -> Option<i64> {
        self.id
    }

    fn set_task_id(&mut self, task_id: i64) {
        self.id = Some(task_id)
    }

    fn description(&self) -> String {
        String::from("Solve Google Recaptcha automatically")
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

impl Proxiable for RecaptchaV2Task {
    fn set_proxy(&mut self, proxy: Proxy) {
        self.proxy = Some(proxy);
        self.task_type = String::from("RecaptchaV2Task");
    }
}
