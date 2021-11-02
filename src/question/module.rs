use crate::question::repositories::QuestionRepository;
use crate::question::service::QuestionService;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::user::module::UserModule;
use crate::config::AppConfig;

pub struct QuestionModuleImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
    pub user_module: Box<dyn UserModule>,
    pub config: Box<AppConfig>
}

pub trait QuestionModule {
    fn question_repository(&self) -> Box<dyn QuestionRepository>;
    fn question_service(&self) -> Box<dyn QuestionService>;
}

impl QuestionModule for QuestionModuleImpl {

    fn question_repository(&self) -> Box<dyn QuestionRepository> {
        use crate::question::dao::PgQuestionDAO;
        return Box::new(
            PgQuestionDAO {
                pool: self.pool.clone()
            }
        );
    }
    fn question_service(&self) -> Box<dyn QuestionService> {
        use crate::question::service::QuestionServiceImpl;
        use crate::user::module::UserModuleImpl;
        return Box::new(
            QuestionServiceImpl {
                pool: self.pool.clone(),
                user_module: Box::new(UserModuleImpl::new(self.pool.clone())),
                question_module: Box::new(
                    QuestionModuleImpl {
                        pool: self.pool.clone(),
                        user_module: Box::new(UserModuleImpl::new(self.pool.clone())),
                        config: self.config.clone()
                    }
                ),
                config: self.config.clone()
            }
        );
    }
}