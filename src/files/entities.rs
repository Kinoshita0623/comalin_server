use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::files;
use serde::Serialize;


#[derive(Queryable, Serialize)]
pub struct AppFile {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="files"]
pub struct NewAppFile {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub hash: String,
}