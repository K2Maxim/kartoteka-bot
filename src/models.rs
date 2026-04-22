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

#[derive(serde::Serialize)]
pub struct BotCreationResult {
    pub success: bool,
    pub id: i32,
    pub name: String
}

impl BotCreationResult {
    pub fn new(id: i32, name: String) -> Self {
        BotCreationResult {success: true, id, name}
    }
}

#[derive(serde::Serialize)]
pub struct BotCreationError {
    pub success: bool,
    pub name: String,
    pub error: String
}

impl BotCreationError {
    pub fn new(name: String, error: String) -> Self {
        BotCreationError {success: false, name, error}
    }
}

pub enum CreateBotResponse {
    Success(BotCreationResult),
    Failure(BotCreationError)
}

impl axum::response::IntoResponse for CreateBotResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(result) =>
                (
                    axum::http::StatusCode::CREATED,
                    axum::response::Json(result)).into_response(),
            Self::Failure(error) =>
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    axum::response::Json(error)).into_response()
        }
    }
}

#[derive(serde::Serialize)]
pub struct BotListingResult {
    pub success: bool,
    pub bots: Vec<Bot>
}

impl BotListingResult {
    pub fn new(bots: Vec<Bot>) -> Self {
        BotListingResult {success: true, bots}
    }
}

#[derive(serde::Serialize)]
pub struct BotListingError {
    pub success: bool,
    pub error: String
}

impl BotListingError {
    pub fn new(error: String) -> Self {
        BotListingError {success: false, error}
    }
}

pub enum ListBotsResponse {
    Success(BotListingResult),
    Failure(BotListingError)
}

impl axum::response::IntoResponse for ListBotsResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(result) =>
                (
                    axum::http::StatusCode::CREATED,
                    axum::response::Json(result)).into_response(),
            Self::Failure(error) =>
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    axum::response::Json(error)).into_response()
        }
    }
}