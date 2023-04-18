use crate::database::users::{self, Entity as Users};
use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::utilities::{app_error::AppError, jwt::validate_token, token_wrapper::TokenWrapper};

pub async fn require_auth<B>(
    State(db): State<DatabaseConnection>,
    State(token_secret): State<TokenWrapper>,
    headers: HeaderMap,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let header_token = if let Some(token) = headers.get("x-auth-token") {
        token.to_str().map_err(|error| {
            eprintln!("Error converting token to string: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        })?
    } else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "No token provided"));
    };

    validate_token(&token_secret.0, header_token)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(Some(header_token.to_owned())))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error finding user: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a problem getting your account",
            )
        })?;

    if let Some(user) = user {
        request.extensions_mut().insert(user);
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You Are Not Authorized To Access This Resource",
        ));
    }
    Ok(next.run(request).await)
}
