#[derive(sqlx::FromRow)]
pub struct Bot {
    pub id: i32,
    pub name: String,
    pub token: String
}

#[derive(serde::Serialize)]
pub struct InitializationData {
    pub text: String,
}