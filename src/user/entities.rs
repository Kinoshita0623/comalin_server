use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::diesel_util::selectable::Selectable;
use crate::users;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub encrypted_password: String,
    pub avatar_icon: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Queryable)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub avatar_icon: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Selectable for PublicUser {
    type Columns = (users::id, users::username, users::avatar_icon, users::created_at, users::updated_at);
    fn columns() -> Self::Columns {
        return (users::id, users::username, users::avatar_icon, users::created_at, users::updated_at);
    }
}