use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod create_user;
pub mod login;
pub mod logout;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ResponseDataUser {
    data: ResponseUser,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RequestCreateUser {
    username: String,
    password: String,
}
