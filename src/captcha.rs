use crate::errors::Error;
use crate::models::APIRequest;
use crate::response::{BalanceResponse, ErrorResponse, TaskResponse, TaskStatus};
use crate::tasks::Task;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct CapMonster {
    pub client: reqwest::Client,
    pub key: String,
}

impl CapMonster {
    pub fn new(key: String) -> Self {
        let client = reqwest::Client::new();
        CapMonster { client, key }
    }

    pub async fn balance(&self) -> Result<BalanceResponse, Error> {
        let data = APIRequest {
            key: self.key.clone(),
            task: None,
            callback_url: None,
        };

        let response: BalanceResponse =
            request(&self.client, String::from("/getBalance"), &data).await?;
        Ok(response)
    }

    pub async fn create_task<T: Task>(&self, task: &mut T) -> Result<(), Error> {
        let data = APIRequest {
            key: self.key.clone(),
            task: Some(task.as_value()?),
            callback_url: None,
        };

        let response: TaskResponse =
            request(&self.client, String::from("/createTask"), &data).await?;

        task.set_task_id(response.task_id);
        Ok(())
    }

    pub async fn wait_for<T: Task>(&self, task: &T) -> Result<TaskStatus<T::TaskResult>, Error> {
        let data = APIRequest {
            key: self.key.clone(),
            task: None,
            callback_url: None,
        };

        let mut response: TaskStatus<T::TaskResult> =
            request(&self.client, String::from("/getTaskResult"), &data).await?;
        while response.solution.is_none() {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            response = request(&self.client, String::from("/getTaskResult"), &data).await?;
        }

        Ok(response)
    }
}

pub async fn request<T: Serialize, R: DeserializeOwned>(
    client: &reqwest::Client,
    path: String,
    data: T,
) -> Result<R, Error> {
    let response = client
        .post(format!("https://api.capmonster.cloud{}", path))
        .json(&data)
        .send()
        .await?
        .text()
        .await?;

    if response.contains("errorCode") {
        let result: ErrorResponse = serde_json::from_str(&response)?;
        return Err(Error::ApiError(result));
    }

    let result: R = serde_json::from_str(&response)?;
    Ok(result)
}
