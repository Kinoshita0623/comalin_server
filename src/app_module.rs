use diesel::r2d2::Pool;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use crate::user::module::{UserModule, UserModuleImpl};

pub struct AppModule {
    pool: Box<Pool<ConnectionManager<PgConnection>>>
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
}

mod test {
    use actix_web::App;
    use diesel::r2d2::{ConnectionManager, Pool};
    use crate::app_module::AppModule;


    fn test_get_user_repo() {
        use crate::db::DbConfig;
        let pool = DbConfig {
            url: "hoge".to_string(),
            connection_count: 30
        }.create_pool();
        let app = AppModule {
            pool: Box::new(pool)
        };
        let user_repository = app.user_module().user_repository();

        assert_eq!(type_of(user_repository), "UserRepository")


    }

    fn type_of<T>(_: T) -> String{
        let a = std::any::type_name::<T>();
        return a.to_string();
    }
}