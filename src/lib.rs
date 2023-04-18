use std::net::SocketAddr;

use app_state::AppState;
use router::create_router;

pub mod app_state;
mod database;
mod middleware;
pub mod queries;
mod router;
mod routes;
pub mod utilities;

pub async fn run(app_state: AppState) {
    let app = create_router(app_state);
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
