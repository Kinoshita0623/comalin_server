use crate::schema::questions;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::NaiveDateTime;
use crate::diesel_util::geography::GeographyPoint;
use uuid::Uuid;
use crate::schema::question_files;
use crate::user::entities::User;
use crate::files::entities::AppFile;

#[derive(PartialEq)]
pub struct Question {
    pub id: Uuid,
    pub title: String,
    pub text: Option<String>,
    pub longitude: BigDecimal,
    pub latitude: BigDecimal,
    pub address_id: Option<Uuid>,
    pub user_id: Uuid,
    pub file_ids: Vec<Uuid>,
    pub answers_count: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key="user_id")]
#[table_name="questions"]
#[primary_key(id)]
pub struct QuestionDTO {
    pub id: Uuid,
    pub title: String,
    pub text: Option<String>,
    pub longitude: BigDecimal,
    pub latitude: BigDecimal,
    pub location_point: GeographyPoint,
    pub address_id: Option<Uuid>,
    pub user_id: Uuid,
    pub answers_count: i32,
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

#[derive(Insertable)]
#[table_name="question_files"]
pub struct NewQuestionFile {
    pub file_id: Uuid,
    pub question_id: Uuid
}


#[derive(Queryable, Identifiable, PartialEq, Associations, Clone)]
#[belongs_to(AppFile, foreign_key="file_id")]
#[belongs_to(QuestionDTO, foreign_key="question_id")]
#[table_name="question_files"]
pub struct QuestionFile {
    pub id: i64,
    pub file_id: Uuid,
    pub question_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
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

impl Into<Question> for (QuestionDTO, Vec<QuestionFile>) {
    fn into(self) -> Question {
        let q = self.0;
        let files = self.1;
        return Question {
            id: q.id,
            title: q.title,
            text: q.text,
            longitude: q.longitude,
            latitude: q.latitude,
            address_id: q.address_id,
            user_id: q.user_id,
            file_ids: files.iter().map(|qf| qf.file_id).collect(),
            answers_count: q.answers_count,
            created_at: Some(q.created_at),
            updated_at: Some(q.updated_at)
        };
    }
}

impl From<&Question> for QuestionDTO {
    fn from(from: &Question) -> QuestionDTO{
        return QuestionDTO {
            id: from.id,
            title: from.title.clone(),
            text: from.text.clone(),
            latitude: from.latitude.clone(),
            longitude: from.longitude.clone(),
            location_point: GeographyPoint {
                srid: Some(4326),
                x: from.longitude.to_f64().unwrap(),
                y: from.latitude.to_f64().unwrap()
            },
            address_id: from.address_id,
            answers_count: from.answers_count,
            user_id: from.user_id,
            created_at: from.created_at.unwrap(),
            updated_at: from.updated_at.unwrap()
        }
    }
}