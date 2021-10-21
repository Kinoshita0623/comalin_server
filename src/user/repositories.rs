use uuid::Uuid;
use crate::user::entities::{NewUser, User};
use anyhow::Result;


pub trait UserRepository {
    fn create(&self, user: &NewUser) -> Result<User>;
    fn find_by_name(&self, username: String) -> Result<User>;
    fn find(&self, id: Uuid) -> Result<User>;
    fn delete(&self, id: Uuid) -> Result<bool>;
}