#[tokio::main]
async fn main() {

    let application =
        axum::Router::new()
            .fallback_service(
                tower_http::services::ServeDir::new("assets"));

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();

    axum::serve(listener, application).await.unwrap();
}