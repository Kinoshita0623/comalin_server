use crate::schema::questions;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use crate::diesel_util::geography::GeographyPoint;
use uuid::Uuid;
use crate::diesel_util::sql_types::Geography;


#[derive(Queryable)]
pub struct Question {
    pub id: Uuid,
    pub title: String,
    pub text: Option<String>,
    pub longitude: BigDecimal,
    pub latitude: BigDecimal,
    pub address_id: Option<Uuid>,
    pub user_id: Uuid,
    pub location_point: GeographyPoint,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}