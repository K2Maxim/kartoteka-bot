mod database;
mod models;
mod handlers;

use crate::models::AppState;

async fn get_application_with_database_support() -> Option<axum::Router> {
    dotenvy::dotenv().ok()?;
    let database_url = std::env::var("DATABASE_URL").ok()?;
    let pool = database::create_pool(&database_url).await.ok()?;
    sqlx::migrate!().run(&pool).await.ok()?;
    let state = AppState {database: pool};
    Some(axum::Router::new()
        .route("/health", axum::routing::get(handlers::health_ok))
        .route("/init", axum::routing::get(handlers::init))
        .route("/bot", axum::routing::post(handlers::create_bot))
        .route("/bots", axum::routing::get(handlers::get_bots))
        .fallback_service(tower_http::services::ServeDir::new("assets"))
        .with_state(state))
}

#[tokio::main]
async fn main() {

    let application =
        get_application_with_database_support().await.unwrap_or_else(||
        axum::Router::new()
        .route("/health", axum::routing::get(handlers::health_no_db))
        .route("/init", axum::routing::get(handlers::minimal_init))
        .fallback_service(tower_http::services::ServeDir::new("assets")));

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();

    axum::serve(listener, application).await.unwrap();
}