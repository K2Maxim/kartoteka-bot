use crate::models::{
    AppState, Bot, BotCreationError, BotCreationResult, BotListingError, BotListingResult, CreateBotRequest, CreateBotResponse, InitializationData, ListBotsResponse
};
use crate::database::create_pool;

pub async fn health_ok() -> &'static str {
    "OK\n"
}

pub async fn health_no_db() -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push("Нет БД".to_string());
    let database_url: String =
        match std::env::var("DATABASE_URL") {
            Ok(value) => {
                lines.push("- переменная окружения DATABASE_URL определена".to_string());
                lines.push(format!{"- значение переменной окружения: {value}"});
                value
            }
            Err(e) => {
                lines.push(format!{"- не удалось получить значение переменной окружения: {e}"});
                return lines.join("\n").to_string()
            }
        };
    let pool: sqlx::Pool<sqlx::Postgres> =
        match create_pool(&database_url).await {
            Ok(value) => {
                lines.push("- пул подключений создается".to_string());
                value
            }
            Err(e) => {
                lines.push(format!{"- не удалось создать пул подключений: {e}"});
                return lines.join("\n").to_string()
            }
        };
    match sqlx::migrate!().run(&pool).await {
        Ok(()) => lines.push("- миграция выполнена".to_string()),
        Err(e) => {
            lines.push(format!{"- не удалось провести миграцию: {e}"});
            return lines.join("\n").to_string()
        }
    }
    lines.join("\n").to_string()
}

pub async fn minimal_init() -> axum::response::Json<InitializationData> {
    let data: InitializationData = InitializationData {text: "Шаг 1: нужно описание".to_string()};
    axum::response::Json(data)
}

pub async fn init() -> axum::response::Json<InitializationData> {
    let data: InitializationData = InitializationData {text: "Шаг 1: описание".to_string()};
    axum::response::Json(data)
}

pub async fn create_bot(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Json(payload): axum::extract::Json<CreateBotRequest>,
) -> CreateBotResponse {
    let result: Result<Bot, sqlx::Error> =
        sqlx::query_as::<_, Bot>(
        "INSERT INTO bots (name, token) VALUES ($1, $2) RETURNING id, name, token")
        .bind(&payload.name)
        .bind(&payload.token)
        .fetch_one(&state.database)
        .await;
    match result {
        Ok(value) =>
            CreateBotResponse::Success(BotCreationResult::new(value.id, value.name)),
        Err(error) =>
            CreateBotResponse::Failure(BotCreationError::new(payload.name, error.to_string()))
    }
}

pub async fn get_bots(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ListBotsResponse {
    let result: Result<Vec<Bot>, sqlx::Error> = sqlx::query_as::<_, Bot>(
        "SELECT id, name, token FROM bots")
        .fetch_all(&state.database)
        .await;
    match result {
        Ok(value) =>
            ListBotsResponse::Success(BotListingResult::new(value)),
        Err(error) =>
            ListBotsResponse::Failure(BotListingError::new(error.to_string()))
    }
}

pub async fn debug_environment_variables() -> axum::response::Json<std::collections::HashMap<String, String>> {
    let environment_variables: std::collections::HashMap<String, String> = std::env::vars().collect();
    axum::response::Json(environment_variables)
}