use diesel::PgConnection;  
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use crate::files::repositories::AppFileRepository;
use crate::errors::service_error::ServiceError;
use crate::files::entities::{NewAppFile, AppFile};
use crate::schema::files;
use log::{error, debug};
use diesel::prelude::*;
use uuid::Uuid;

pub struct AppFileDAO {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>
}

impl AppFileRepository for AppFileDAO {

    fn create(&self, file: &NewAppFile) -> Result<AppFile, ServiceError> {
        let c = self.get_connectoin()?;
        let e = match diesel::insert_into(files::dsl::files).values(file).get_result::<AppFile>(&c) {
            Ok(file) => {
                return Ok(file);
            },
            Err(e) => e
        };

        error!("Failed create file {}", e.to_string());

        return Err(
            ServiceError::InternalError {
                body: None
            }
        );
    }

    fn find_by_hash(&self, hash: &str) -> Result<AppFile, ServiceError> {
        let c = self.get_connectoin()?;
        let e: ServiceError = match files::dsl::files.filter(files::hash.eq(hash)).first::<AppFile>(&c) {
            Ok(f) => {
                return Ok(f);
            }
            Err(e) => e.into()
        };
        return Err(e);
    }

    fn find_in(&self, ids: &Vec<Uuid>) -> Result<Vec<AppFile>, ServiceError> {
        let c = self.get_connectoin()?;
        return match files::dsl::files.filter(files::id.eq_any(ids))
            .load::<AppFile>(&c) {
            Ok(files) => Ok(files),
            Err(e) => Err(e.into())
        };
    }
}

impl AppFileDAO {

    fn get_connectoin(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, ServiceError> {
        return match self.pool.get() {
            Err(e) => {
                error!("Failed to get connection {}", e.to_string());
                return Err(ServiceError::InternalError{
                    body: Some(e.to_string())
                });
            },
            Ok(c) => Ok(c)
        };
    }
}