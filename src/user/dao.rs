use diesel::{PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use crate::user::entities::{NewUser, User};
use crate::user::module::UserModule;
use crate::user::repositories::UserRepository;
use crate::schema::users;
use anyhow::Result;
use uuid::Uuid;
use diesel::prelude::*;


pub struct PgUserDAO {
    pool: Box<Pool<ConnectionManager<PgConnection>>>
}

impl UserRepository for PgUserDAO {
    fn create(&self, user: &NewUser) -> Result<User> {
        let c = self.pool.get()?;
        let user = diesel::insert_into(users::dsl::users).values(user).get_result::<User>(&c)?;
        return Ok(user);
    }

    fn find_by_name(&self, username: String) -> Result<User> {
        let c = self.pool.get()?;
        return users::dsl::users.filter(users::username.eq(username)).first::<User>(&c);
    }

    fn find(&self, id: Uuid) -> Result<User> {
        let c = self.pool.get()?;
        return users::dsl::users.filter(users::id.eq(id)).first::<User>(&c);
    }

    fn delete(&self, id: Uuid) -> Result<bool> {
        let c = self.pool.get()?;
        let count = diesel::delete(users::id.eq(id)).execute(&c)?;
        return Ok(count == 1);
    }


}

impl PgUserDAO {
    pub fn new(pool: Box<Pool<ConnectionManager<PgConnection>>>) -> Self{
        return Self {
            pool
        }
    }
}