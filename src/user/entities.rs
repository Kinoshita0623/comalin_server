use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::diesel_util::selectable::Selectable;
use crate::users;
use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

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


#[derive(Queryable, Eq,PartialEq)]
pub struct UserToken {
    pub id: Uuid,
    pub hashed_token: String,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime
}

#[derive(Insertable,Eq,PartialEq)]
#[table_name="user_tokens"]
pub struct NewUserToken {
    pub id: Uuid,
    pub hashed_token: String,
    pub user_id: Uuid
}

#[derive(Eq,PartialEq)]
pub struct RawToken {
    pub token: String,
    pub user_id: Uuid
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

impl From<&RawToken> for NewUserToken {
    fn from(token: &RawToken) -> Self {
        let mut sha256 = Sha256::new();
        sha256.input_str(&token.token);
        return NewUserToken {
            id: Uuid::new_v4(),
            hashed_token: sha256.result_str(),
            user_id: token.user_id
        }
    }
}

impl Into<NewUserToken> for RawToken {
    fn into(self) -> NewUserToken {
        return NewUserToken::from(&self);
    }
}


impl UserToken {
    pub fn check_token(&self, token: &String) -> bool {
        let mut sha256 = Sha256::new();
        sha256.input_str(token);
        return sha256.result_str() == self.hashed_token;
    }
}


mod test {
    use chrono::Utc;
    use uuid::Uuid;
    use crate::user::entities::{NewUserToken, User, UserToken};

    #[test]
    fn test_make_token () {
        let now = Utc::now().naive_utc();
        let user = User {
            id: Uuid::new_v4(),
            username: "hogehoge".to_string(),
            encrypted_password: "hogehoge".to_string(),
            avatar_icon: None,
            created_at: now,
            updated_at: now,
        };
        let raw_token = &user.make_token();
        let new_token = NewUserToken::from(raw_token);
        let token = UserToken {
            id: Uuid::new_v4(),
            user_id: user.id,
            hashed_token: new_token.hashed_token,
            created_at: now
        };
        assert!(token.check_token(&raw_token.token))
    }
}
