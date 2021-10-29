use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::files;

#[derive(Queryable)]
pub struct AppFile {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub raw_name: Option<String>,
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
    pub raw_name: String,
    pub hash: String,
}