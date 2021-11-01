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

pub struct NewUserAttr {
    pub username: String,
    pub avatar_icon: Option<String>,
    pub password: String
}

impl NewUser {
    pub fn new(new_user: NewUserAttr) -> Result<NewUser, BcryptError> {
        //NOTE: bcryptはそこそこ遅いので頻繁に呼び出さないこと
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



