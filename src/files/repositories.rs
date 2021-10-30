use crate::files::entities::*;
use crate::errors::service_error::ServiceError;

pub trait AppFileRepository {
    fn create(&self, file: &NewAppFile) -> Result<AppFile, ServiceError>;
    fn find_by_hash(&self, hash: &str) -> Result<AppFile, ServiceError>;
}