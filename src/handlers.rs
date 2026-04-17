use crate::models::{
    AppState,
    InitializationData,
    Bot,
    CreateBotRequest
};

pub async fn health_ok() -> &'static str {
    "OK\n"
}

pub async fn health_no_db() -> &'static str {
    "No database\n"
}

pub async fn minimal_init() -> axum::response::Json<InitializationData> {
    let data = InitializationData {text: "Шаг 1: нужно описание".to_string()};
    axum::response::Json(data)
}

pub async fn init() -> axum::response::Json<InitializationData> {
    let data = InitializationData {text: "Шаг 1: описание".to_string()};
    axum::response::Json(data)
}

pub async fn create_bot(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Json(payload): axum::extract::Json<CreateBotRequest>,
) -> (axum::http::StatusCode, axum::response::Json<Bot>) {
    let result = sqlx::query_as::<_, Bot>(
        "INSERT INTO bots (name, token) VALUES ($1, $2) RETURNING id, name, token"
    )
    .bind(&payload.name)
    .bind(&payload.token)
    .fetch_one(&state.database)
    .await
    .expect("Ошибка при создании новой записи.");

    (axum::http::StatusCode::CREATED, axum::response::Json(result))
}

pub async fn get_bots(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::response::Json<Vec<Bot>> {
    let result = sqlx::query_as::<_, Bot>("SELECT id, name, email, created_at FROM users")
        .fetch_all(&state.database)
        .await
        .expect("Ошибка при получении данных");

    axum::response::Json(result)
}