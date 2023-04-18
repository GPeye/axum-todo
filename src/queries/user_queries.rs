use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TryIntoModel,
};

use crate::database::users::{Entity as Users, Model as UserModel};
use crate::{database::users, utilities::app_error::AppError};

pub async fn save_active_user(
    db: &DatabaseConnection,
    user: users::ActiveModel,
) -> Result<UserModel, AppError> {
    let user = user.save(db).await.map_err(|error| {
        let error_message = error.to_string();
        if error_message.contains("duplicate key value violates unique constraint") {
            return AppError::new(
                StatusCode::BAD_REQUEST,
                "Username already taken, try again with a different user name",
            );
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        }
    })?;

    convert_active_to_model(user)
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: String,
) -> Result<UserModel, AppError> {
    Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error finding user by username: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error logging in, please try again later",
            )
        })?
        .ok_or_else(|| {
            AppError::new(
                StatusCode::NOT_FOUND,
                "User not found, please try again with a different username",
            )
        })
}

fn convert_active_to_model(active_user: users::ActiveModel) -> Result<UserModel, AppError> {
    active_user.try_into_model().map_err(|error| {
        eprint!("Error converting task to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    })
}
