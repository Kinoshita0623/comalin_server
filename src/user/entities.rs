use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::diesel_util::selectable::Selectable;
use crate::users;
use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub avatar_icon: Option<String>,
    pub encrypted_password: String,
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
#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub id: Uuid,
    pub username: String,
    pub avatar_icon: Option<String>,
    pub encrypted_password: String
}

pub struct NewUserAttr<'a> {
    pub username: &'a str,
    pub avatar_icon: Option<&'a str>,
    pub password: &'a str
}

impl NewUser {
    pub fn new(new_user: NewUserAttr) -> Result<NewUser, BcryptError> {
        let user = NewUser {
            id: Uuid::new_v4(),
            username: new_user.username.to_string(),
            avatar_icon: match new_user.avatar_icon {
                Some(t) => Some(t.to_string()),
                None => None
            },
            encrypted_password: bcrypt::hash(new_user.password, DEFAULT_COST)?
        };
        return Ok(user);
    }
}