use chrono::{DateTime, FixedOffset};

pub mod create_task;
pub mod create_task_extractor;
pub mod get_tasks;
pub mod update_tasks;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestTask {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTime<FixedOffset>>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseDataTask {
    pub data: ResponseTask,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseDataTasks {
    pub data: Vec<ResponseTask>,
}
