use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::files;
use serde::Serialize;
use crate::config::AppConfig;


#[derive(Identifiable, Queryable, PartialEq, Serialize, Clone)]
#[table_name="files"]
pub struct AppFile {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl AppFile {
    pub fn get_url(&self, config: &AppConfig) -> String {
        return format!("{}/files/{}", config.app_url, self.filename)
    }
}

#[derive(Insertable)]
#[table_name="files"]
pub struct NewAppFile {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub hash: String,
}