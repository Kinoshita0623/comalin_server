use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::diesel_util::selectable::Selectable;
use crate::schema::users;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crate::user::commands::RawToken;
use serde::Serialize;
use bcrypt::{BcryptError, DEFAULT_COST};

#[derive(Queryable, Identifiable, Clone)]
#[table_name="users"]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub avatar_icon: Option<String>,
    pub questions_count: i32,
    pub answers_count: i32,
    pub thanks_count: i32,
    pub encrypted_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Queryable, Serialize, Clone)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub avatar_icon: Option<String>,
    pub questions_count: i32,
    pub answers_count: i32,
    pub thanks_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Selectable for PublicUser {
    type Columns = (users::id, users::username, users::avatar_icon, users::questions_count, users::answers_count, users::thanks_count, users::created_at, users::updated_at);
    fn columns() -> Self::Columns {
        return (users::id, users::username, users::avatar_icon, users::questions_count, users::answers_count, users::thanks_count, users::created_at, users::updated_at);
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

    pub fn check_password(&self, password: &str) -> bool {
        return match bcrypt::verify(password, &self.encrypted_password.clone()) {
            Ok(r) =>  r,
            Err(_) => false
        };
    }

    pub fn change_password(&mut self, password: &str) -> Result<(), BcryptError>{
        self.encrypted_password = bcrypt::hash(password, DEFAULT_COST)?;
        
        return Ok(());
    }

    pub fn set_avatar_icon(&mut self, avatar_icon: Option<String>) {
        self.avatar_icon = avatar_icon;
    }
}


impl UserToken {
    pub fn check_token(&self, token: &String) -> bool {
        let mut sha256 = Sha256::new();
        sha256.input_str(token);
        return sha256.result_str() == self.hashed_token;
    }
}


impl Into<PublicUser> for User {
    fn into(self) -> PublicUser {
        return PublicUser {
            id: self.id,
            username: self.username,
            avatar_icon: self.avatar_icon,
            questions_count: self.questions_count,
            answers_count: self.answers_count,
            thanks_count: self.thanks_count,
            created_at: self.created_at,
            updated_at: self.updated_at
        };
    }
}