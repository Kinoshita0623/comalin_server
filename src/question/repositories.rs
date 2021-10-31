use crate::question::entities::Question;
use uuid::Uuid;
use crate::errors::service_error::ServiceError;

pub trait QuestionRepository {
    fn create(&self, question: &Question) -> Result<Question, ServiceError>;
    fn save(&self, questin: &Question) -> Result<Question, ServiceError>;
    fn delete(&self, question_id: &Uuid) -> Result<(), ServiceError>;
    fn find(&self, question_id: &Uuid) -> Result<Question, ServiceError>;
}