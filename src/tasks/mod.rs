mod turnstile;
pub use turnstile::TurnstileTask;
mod recaptcha_v2;
pub use recaptcha_v2::RecaptchaV2Task;

use crate::errors::Error;
use serde::de::DeserializeOwned;
use serde_json::Value;



pub trait Task {
    type TaskResult: DeserializeOwned;

    fn get_task_id(&self) -> Option<i64>;
    fn set_task_id(&mut self, task_id: i64);
    fn description(&self) -> String;
    fn as_value(&self) -> Result<Value, Error>;
}

pub fn merge(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                } else {
                    merge(a.entry(k).or_insert(Value::Null), v);
                }
            }
            return;
        }
    }
    *a = b;
}
