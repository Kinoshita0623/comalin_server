use sqlx::types::Uuid;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub encrypted_password: String,
    pub avatar_icon: String,
}

