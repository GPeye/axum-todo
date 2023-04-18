use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::DatabaseConnection;

use super::{create_task_extractor::ValidateCreateTask, ResponseDataTask, ResponseTask};
use crate::{database::users::Model as UserModel, queries, utilities::app_error::AppError};

pub async fn create_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    let task = queries::task_queries::create_task(task, &user, &db).await?;

    let response = ResponseDataTask {
        data: ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
            completed_at: task.completed_at.map(|time| time.to_string()),
        },
    };
    Ok((StatusCode::CREATED, Json(response)))
}
