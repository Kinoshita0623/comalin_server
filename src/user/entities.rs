use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::diesel_util::selectable::Selectable;
use crate::schema::users;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crate::user::commands::RawToken;

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

#[derive(Queryable, Eq,PartialEq)]
pub struct UserToken {
    pub id: Uuid,
    pub hashed_token: String,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime
}



impl User {
    pub fn make_token(&self) -> RawToken{
        let mut sha256 = Sha256::new();
        sha256.input_str(&Uuid::new_v4().to_string());
        return RawToken {
            user_id: self.id,
            token: sha256.result_str()
        }
    }
}


impl UserToken {
    pub fn check_token(&self, token: &String) -> bool {
        let mut sha256 = Sha256::new();
        sha256.input_str(token);
        return sha256.result_str() == self.hashed_token;
    }
}
