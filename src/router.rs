use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::middleware::require_auth::require_auth;

use crate::routes::tasks::create_task::create_task;
use crate::routes::tasks::get_tasks::{get_all_tasks, get_one_task};
use crate::routes::tasks::update_tasks::{
    mark_completed, mark_deleted, mark_uncompleted, mark_undeleted, update_task,
};
use crate::routes::users::{create_user::create_user, login::login, logout::logout};
use crate::{app_state::AppState, routes::hello_world::hello_world};

#[derive(OpenApi)]
#[openapi(
    paths(crate::routes::users::create_user::create_user),
    components(schemas(
        crate::routes::users::ResponseDataUser,
        crate::routes::users::ResponseUser,
        crate::routes::users::RequestCreateUser
    )),
    tags((name = "Todo Project", description = "Todo Project API"))
)]
pub struct ApiDoc;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/users/logout", post(logout))
        .route("/api/v1/tasks", post(create_task))
        .route("/api/v1/tasks", get(get_all_tasks))
        .route("/api/v1/tasks/:task_id", get(get_one_task))
        .route("/api/v1/tasks/:task_id/completed", put(mark_completed))
        .route("/api/v1/tasks/:task_id/uncompleted", put(mark_uncompleted))
        .route("/api/v1/tasks/:task_id", patch(update_task))
        .route("/api/v1/tasks/:task_id", delete(mark_deleted))
        .route("/api/v1/tasks/:task_id/undelete", put(mark_undeleted))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_auth,
        ))
        .merge(SwaggerUi::new("/api/v1/").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(hello_world))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}
