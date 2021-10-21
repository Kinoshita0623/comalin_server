use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::user::repositories::UserRepository;
use crate::user::dao::*;

pub struct UserModuleImpl {
    pool: Box<Pool<ConnectionManager<PgConnection>>>
}

pub trait UserModule {
    fn user_repository(&self) -> Box<dyn UserRepository>;

}

impl UserModuleImpl {
    pub fn new(pool: Box<Pool<ConnectionManager<PgConnection>>>) -> Self{
        return Self {
            pool
        }
    }
}


impl UserModule for UserModuleImpl {
    fn user_repository(&self) -> Box<dyn UserRepository> {
        let pool = self.pool.clone();
        return Box::new(PgUserDAO::new(pool));
    }
}