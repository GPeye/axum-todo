use dotenvy::*;
use dotenvy_macro::dotenv;
use sea_orm::Database;
use todo_app::{app_state::AppState, run, utilities::token_wrapper::TokenWrapper};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL").to_owned();
    let jwt_secret = dotenv!("JWT_SECRET").to_owned();
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Failed to connect to database: {:?}", error);
            panic!("Failed to connect to database");
        }
    };
    let app_state = AppState {
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };
    run(app_state).await;
}
