use crate::files::entities::*;
use crate::errors::service_error::ServiceError;
use std::vec::Vec;
use uuid::Uuid;

pub trait AppFileRepository {
    fn create(&self, file: &NewAppFile) -> Result<AppFile, ServiceError>;
    fn find_by_hash(&self, hash: &str) -> Result<AppFile, ServiceError>;
    fn find_in(&self, vec: &Vec<Uuid>) -> Result<Vec<AppFile>, ServiceError>;
}