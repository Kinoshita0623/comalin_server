use crate::schema::questions;
use sqlx::types::Uuid;
use bigdecimal::BigDecimal;


#[derive(Queryable)]
pub struct Question {
    pub id: Uuid,
    pub title: String,
    pub text: Option<String>,
    pub longitude: BigDecimal,
    pub latitude: BigDecimal,
    pub address_id: Option<Uuid>,
    pub user_id: Uuid,
}