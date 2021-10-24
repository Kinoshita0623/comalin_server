use crypto::digest::Digest;
use crypto::sha2::Sha256;
use diesel::{PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use crate::user::repositories::UserRepository;
use crate::schema::users;
use uuid::Uuid;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::errors::service_error::ServiceError;
use log::{error, debug};
use validator::{Validate, ValidationError, ValidateArgs};
use crate::schema::user_tokens;
use crate::user::commands::{NewUser, NewUserToken};
use crate::user::entities::{User, UserToken};

pub struct PgUserDAO {
    pool: Box<Pool<ConnectionManager<PgConnection>>>
}

impl UserRepository for PgUserDAO {
    fn create(&self, user: &NewUser) -> Result<User, ServiceError> {

        let c = self.get_connection()?;
        if let Err(err) = user.validate_args(self) {
            return Err(
                ServiceError::ValidationError {
                    body: err
                }
            );
        }

        let e = match diesel::insert_into(users::dsl::users).values(user).get_result::<User>(&c) {
            Ok(user) => {
                debug!("Success create user userId:{}, username:{}", user.id.to_string(), user.username);
                return Ok(user);
            },
            Err(e) => e
        };

        error!("Failed create user {}", e.to_string());

        return Err(
            ServiceError::InternalError {
                body: None
            }
        );

    }

    fn find_by_name(&self, username: String) -> Result<User, ServiceError> {
        let c = self.get_connection()?;
        let e: ServiceError = match users::dsl::users.filter(users::username.eq(username)).first::<User>(&c) {
            Ok(u) => {
                return Ok(u);
            }
            Err(e) => e.into()
        };
        return Err(e);

    }

    fn find(&self, id: Uuid) -> Result<User, ServiceError> {
        let c = self.get_connection()?;
        let e: ServiceError = match users::dsl::users.filter(users::id.eq(id)).first::<User>(&c) {
            Ok(user) => return Ok(user),
            Err(e) => e.into()
        };
        return Err(e);
    }

    fn delete(&self, id: Uuid) -> Result<bool, ServiceError> {
        let c = self.get_connection()?;
        let count = match diesel::delete(users::dsl::users.filter(users::id.eq(id))).execute(&c) {
            Ok(count) => count,
            Err(e) => return Err(e.into())
        };
        return Ok(count == 1);
    }

    fn find_by_token(&self, token: &str) -> Result<User, ServiceError> {
        let c = self.get_connection()?;
        let mut sha256 = Sha256::new();
        sha256.input_str(token);
        let res = user_tokens::dsl::user_tokens
            .filter(user_tokens::hashed_token.eq(sha256.result_str()))
            .first::<UserToken>(&c);
        let token: UserToken = match res {
            Ok(token) => token,
            Err(e) => {
                return Err(e.into());
            }
        };
        return self.find(token.user_id);
    }

    fn save_token(&self, token: NewUserToken) -> Result<(), ServiceError> {
        let c = self.get_connection()?;
        return match diesel::insert_into(user_tokens::dsl::user_tokens).values(token).execute(&c) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        };
    }

    fn delete_token(&self, token: &str) -> Result<(), ServiceError> {
        let mut sha256 = Sha256::new();
        sha256.input_str(token);
        let c = self.get_connection()?;
        let filter = user_tokens::dsl::user_tokens.filter(user_tokens::hashed_token.eq(sha256.result_str()));
        return match diesel::delete(filter).execute(&c) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }

}


impl PgUserDAO {
    pub fn new(pool: Box<Pool<ConnectionManager<PgConnection>>>) -> Self{
        return Self {
            pool
        }
    }

    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, ServiceError> {
        return match self.pool.get() {
            Err(e) => {
                error!("Failed to get connection {}", e.to_string());
                return Err(ServiceError::InternalError{
                    body: Some(e.to_string())
                });
            },
            Ok(c) => Ok(c)
        };
    }
}



impl Into<ServiceError> for DieselError {
    fn into(self) -> ServiceError {
        if DieselError::NotFound == self {
            return ServiceError::NotFoundError;
        }
        return ServiceError::InternalError {
            body: Some(self.to_string())
        }
    }
}