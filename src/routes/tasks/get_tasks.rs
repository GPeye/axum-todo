use axum::extract::Path;
use axum::{extract::State, Extension, Json};
use sea_orm::DatabaseConnection;

use crate::queries::task_queries;
use crate::routes::tasks::ResponseTask;
use crate::{database::users::Model as UserModel, utilities::app_error::AppError};

use super::{ResponseDataTask, ResponseDataTasks};

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTasks>, AppError> {
    let tasks = task_queries::get_all_tasks(&db, user.id, false)
        .await?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            description: db_task.description,
            priority: db_task.priority,
            completed_at: db_task.completed_at.map(|time| time.to_string()),
        })
        .collect();

    Ok(Json(ResponseDataTasks { data: tasks }))
}

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = task_queries::find_task_by_id(&db, task_id, user.id).await?;

    let response_task = ResponseTask {
        id: task.id,
        title: task.title,
        description: task.description,
        priority: task.priority,
        completed_at: task.completed_at.map(|time| time.to_string()),
    };

    Ok(Json(ResponseDataTask {
        data: response_task,
    }))
}
