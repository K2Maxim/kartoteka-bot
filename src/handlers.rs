use crate::models::InitializationData;

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn init() -> axum::response::Json<InitializationData> {
    let data = InitializationData {text: "Шаг 1: нужно описание".to_string()};
    axum::response::Json(data)
}