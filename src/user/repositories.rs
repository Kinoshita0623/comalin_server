use uuid::Uuid;
use crate::errors::service_error::ServiceError;
use crate::user::commands::{NewUser, NewUserToken};
use crate::user::entities::User;


pub trait UserRepository {
    fn create(&self, user: &NewUser) -> Result<User, ServiceError>;
    fn find_by_name(&self, username: String) -> Result<User, ServiceError>;
    fn find(&self, id: Uuid) -> Result<User, ServiceError>;
    fn delete(&self, id: Uuid) -> Result<bool, ServiceError>;
    fn find_by_token(&self, token: &str) -> Result<User, ServiceError>;
    fn save_token(&self, token: NewUserToken) -> Result<(), ServiceError>;
    fn delete_token(&self, token: &str) -> Result<(), ServiceError>;
}