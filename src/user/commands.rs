use bcrypt::{BcryptError, DEFAULT_COST};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use uuid::Uuid;
use validator::ValidationError;
use crate::user::repositories::UserRepository;
use crate::schema::user_tokens;
use crate::schema::users;
use validator::Validate;

#[derive(Insertable, Validate)]
#[table_name="users"]
pub struct NewUser {
    pub id: Uuid,

    #[validate(custom(function="is_unique_username", arg="&'v_a UserRepository"))]
    pub username: String,
    pub avatar_icon: Option<String>,
    pub encrypted_password: String
}

pub fn is_unique_username(value: &str, arg: &dyn UserRepository) -> Result<(), ValidationError>{

    if let Ok(_) = arg.find_by_name(value.to_string()) {
        return Err(ValidationError::new("duplicate_username"));
    }
    return Ok(())
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



mod test {
    use chrono::Utc;
    use uuid::Uuid;
    use crate::user::entities::{User, UserToken};
    use crate::user::commands::NewUserToken;

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
