use crate::schema::questions;
use bigdecimal::{BigDecimal, ToPrimitive};
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
    pub location_point: GeographyPoint,
    pub address_id: Option<Uuid>,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="questions"]
pub struct NewQuestion {
    pub id: Uuid,
    pub title: String,
    pub text: Option<String>,
    pub longitude: BigDecimal,
    pub latitude: BigDecimal,
    pub location_point: GeographyPoint,
    pub address_id: Option<Uuid>,
    pub user_id: Uuid
}

pub struct NewQuestionAttr<'a> {
    pub title: &'a str,
    pub text: Option<&'a str>,
    pub longitude: &'a BigDecimal,
    pub latitude: &'a BigDecimal,
    pub user_id: &'a Uuid
}


impl NewQuestion {
    pub fn new(new_question_attr: NewQuestionAttr) -> Result<NewQuestion, ()> {
        let n =  NewQuestion {
            id: Uuid::new_v4(),
            title: new_question_attr.title.to_string(),
            text: match new_question_attr.text {
                Some(t) => Some(t.to_string()),
                None => None
            },
            longitude: new_question_attr.longitude.clone(),
            latitude: new_question_attr.latitude.clone(),
            address_id: Option::None,
            user_id: new_question_attr.user_id.clone(),
            location_point: GeographyPoint {
                x: new_question_attr.latitude.to_f64().ok_or(())?,
                y: new_question_attr.longitude.to_f64().ok_or(())?,
                srid: Some(4326)
            }
        };
        return Ok(n);
    }
}
