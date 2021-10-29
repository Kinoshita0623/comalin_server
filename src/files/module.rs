use crate::files::repositories::AppFileRepository;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::files::dao::AppFileDAO;

pub trait AppFileModule {
    fn app_file_reposiitory(&self) -> Box<dyn AppFileRepository>;
}

pub struct AppFileModuleImpl {
    pool: Box<Pool<ConnectionManager<PgConnection>>>
}

impl AppFileModuleImpl {
    pub fn new(pool: Box<Pool<ConnectionManager<PgConnection>>>) -> Self {
        return AppFileModuleImpl {
            pool: pool
        };
    }
}

impl AppFileModule for AppFileModuleImpl  {
    fn app_file_reposiitory(&self) -> Box<dyn AppFileRepository> {
        return Box::new(
            AppFileDAO {
                pool: self.pool.clone()
            }
        );
    }
}