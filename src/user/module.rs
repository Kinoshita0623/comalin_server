use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::user::repositories::UserRepository;
use crate::user::dao::*;
use crate::user::services::{PgUserService, UserService};

pub struct UserModuleImpl {
    pool: Box<Pool<ConnectionManager<PgConnection>>>
}

pub trait UserModule {
    fn user_repository(&self) -> Box<dyn UserRepository>;
    fn user_service(&self) -> Box<dyn UserService>;
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

    fn user_service(&self) -> Box<dyn UserService> {
        let pool = self.pool.clone();
        return Box::new(PgUserService {
            user_module: Box::new(UserModuleImpl::new(pool))
        });
    }
}