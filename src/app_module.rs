use diesel::r2d2::Pool;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use crate::user::module::{UserModule, UserModuleImpl};
use crate::files::module::{AppFileModuleImpl, AppFileModule};

pub struct AppModule {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>
}

impl Clone for AppModule{
    fn clone(&self) -> Self{
        return Self {
            pool: self.pool.clone()
        }
    }
}

impl AppModule {
    pub fn user_module(&self) -> Box<dyn UserModule>{
        let pool = self.pool.clone();
        return Box::new(
            UserModuleImpl::new(pool)
        );
    }

    pub fn file_module(&self) -> Box<dyn AppFileModule> {
        let pool = self.pool.clone();
        return Box::new(
            AppFileModuleImpl::new(pool)
        );
    }
}

