#[derive(Clone)]
pub struct AppState {
    pub database: sqlx::PgPool,
}

#[derive(serde::Serialize)]
pub struct InitializationData {
    pub text: String,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Bot {
    pub id: i32,
    pub name: String,
    pub token: String
}

#[derive(serde::Deserialize)]
pub struct CreateBotRequest {
    pub name: String,
    pub token: String,
}
