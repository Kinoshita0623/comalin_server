use crate::user::entities::PublicUser;
use validator::Validate;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use std::vec::Vec;
use chrono::NaiveDateTime;
use crate::question::module::QuestionModule;
use crate::errors::service_error::ServiceError;
use crate::question::entities::{Question, QuestionDTO, QuestionFile};
use crate::user::entities::User;
use crate::user::module::UserModule;
use bigdecimal::{FromPrimitive, BigDecimal};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use crate::files::entities::AppFile;
use std::collections::HashMap;
use std::hash::Hash;
use bigdecimal::ToPrimitive;
use crate::schema::questions;

#[derive(Validate, Deserialize, Debug)]
pub struct CreateQuestion {
    #[validate(required)]
    pub title: Option<String>,
    pub text: Option<String>,

    #[validate(required)]
    pub latitude: Option<f64>,
    #[validate(required)]
    pub longitude: Option<f64>,

    #[validate(required)]
    pub file_ids: Option<Vec<Uuid>>
}

#[derive(Serialize, Clone)]
pub struct QuestionView {
    pub id: Uuid,
    pub title: String,
    pub text: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<String>,
    pub user_id: Uuid,
    pub user: PublicUser,
    pub files: Vec<QuestionFileView>,
    pub answers_count: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Serialize, Clone)]
pub struct QuestionFileView {
    pub url: String,
    pub name: String,
}

pub trait QuestionService {
    fn create(&self, token: &str, question: &CreateQuestion) -> Result<QuestionView, ServiceError>;
    fn find_all(&self) -> Result<Vec<QuestionView>, ServiceError>;
}

pub struct QuestionServiceImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
    pub question_module: Box<dyn QuestionModule>,
    pub user_module: Box<dyn UserModule>,
}

impl QuestionService for QuestionServiceImpl {
    fn create(&self, token: &str, question: &CreateQuestion) -> Result<QuestionView, ServiceError> {
        let user = self.user_module.user_repository().find_by_token(&token)?;
        let q: Question = (&user, question.clone()).into();
        let q: Question = self.question_module.question_repository().create(&q)?;

        let dto = QuestionDTO::from(&q);
        let questions = self.load_question_views_from_questions(vec![dto])?;
        return match questions.get(0) {
            Some(q) => Ok(q.clone()),
            None => return Err(ServiceError::InternalError{ body: None })
        }
    }

    fn find_all(&self) -> Result<Vec<QuestionView>, ServiceError> {
        let c = self.get_connection()?;
        let questions = match questions::dsl::questions.load::<QuestionDTO>(&c) {
            Ok(list) => list,
            Err(e) => return Err(e.into())
        };
        return self.load_question_views_from_questions(questions);
    }

}

impl QuestionServiceImpl {

    fn load_question_views_from_questions(&self, questions: Vec<QuestionDTO>) -> Result<Vec<QuestionView>, ServiceError> {
        use crate::schema::question_files;
        use crate::schema::files;
        use crate::schema::users;
        let c = self.get_connection()?;
        

        let question_ids: Vec<Uuid> = questions.iter().map(|q| q.id).collect();


        let qf_and_af: Vec<(QuestionFile, AppFile)> = match question_files::dsl::question_files
            .filter(question_files::question_id.eq_any(question_ids))
            .inner_join(files::dsl::files)
            .order_by(question_files::id.asc())
            .load::<(QuestionFile, AppFile)>(&c) {
            Ok(list) => list,
            Err(e) => return Err(e.into())
        };

        let qf_and_af_map: HashMap<Uuid, Vec<(QuestionFile, AppFile)>> = group_by(qf_and_af, |v| v.0.question_id);


        let user_ids: Vec<Uuid> = questions.iter().map(|q| q.user_id).collect();
        let users: HashMap<Uuid, User> = match users::dsl::users.filter(users::id.eq_any(user_ids)).load::<User>(&c) {
            Ok(users) => users.into_iter().map(|u| (u.id, u)).collect(),
            Err(e) => return Err(e.into())
        };

        let views: Vec<QuestionView> = questions.iter().map(|q: &QuestionDTO| -> QuestionView {
            let files = match qf_and_af_map.get(&q.id) {
                Some(files) => files.iter().map(|f| QuestionFileView {
                    url: "".to_string(),
                    name: f.1.filename.clone()
                }).collect(),
                None => Vec::new()
            };
            return QuestionView {
                id: q.id,
                title: q.title.clone(),
                text: q.text.clone(),
                latitude: q.latitude.to_f64().unwrap(),
                longitude: q.longitude.to_f64().unwrap(),
                answers_count: q.answers_count,
                address: None,
                files: files.clone(),
                created_at: Some(q.created_at.clone()),
                updated_at: Some(q.updated_at.clone()),
                user: (users.get(&q.user_id).unwrap().clone()).into(),
                user_id: q.user_id
            };
        })
        .collect();
    
        return Ok(views);

    }

    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, ServiceError> {
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

impl Into<Question> for (&User, &CreateQuestion) {

    fn into(self) -> Question {
        return Question {
            id: Uuid::new_v4(),
            title: self.1.title.clone().unwrap(),
            text: self.1.text.clone(),
            latitude: BigDecimal::from_f64(self.1.latitude.unwrap()).unwrap(),
            longitude: BigDecimal::from_f64(self.1.longitude.unwrap()).unwrap(),
            address_id: None,
            answers_count: 0,
            file_ids: self.1.file_ids.clone().unwrap(),
            user_id: self.0.id,
            created_at: None,
            updated_at: None
        };
    }
}

fn group_by<K, V, F>(list: Vec<V>, mut f: F) -> HashMap<K, Vec<V>> where F: FnMut(V) -> K, K: Eq + Hash, V: Clone {
    let mut map: HashMap<K, Vec<V>> = HashMap::new();

    for el in list {
        let k: K = f(el.clone());
        let mut list: Vec<V> = match map.get_mut(&k) {
            Some(list) => list.clone(),
            None => Vec::new()
        };
        list.push(el.clone());
        map.insert(k, list.clone());
    }
    return map;
}

