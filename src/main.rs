mod database;
mod models;
mod handlers;

#[tokio::main]
async fn main() {

    let maybe_connection = std::env::var("DATABASE_URL");
    match maybe_connection {
        Ok(database_url) => {
            let _pool = database::create_pool(&database_url).await.expect("Failed to create pool");
        }
        Err(_) => {}
    }
    

    let application =
        axum::Router::new()
            .route("/health", axum::routing::get(handlers::health_check))
            .route("/init", axum::routing::get(handlers::init))
            .fallback_service(
                tower_http::services::ServeDir::new("assets"));

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();

    axum::serve(listener, application).await.unwrap();
}