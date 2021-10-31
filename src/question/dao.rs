use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use crate::question::repositories::QuestionRepository;
use crate::question::entities::*;
use crate::errors::service_error::ServiceError;
use uuid::Uuid;
use crate::schema::users;
use crate::schema::questions;
use crate::schema::question_files;
use crate::diesel_util::geography::GeographyPoint;
use bigdecimal::ToPrimitive;
use diesel::prelude::*;
use crate::question::entities::{QuestionDTO, QuestionFile, NewQuestionFile};
use diesel::expression::dsl::now;

pub struct PgQuestionDAO {
    pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl QuestionRepository for PgQuestionDAO {

    fn create(&self, question: &Question) -> Result<Question, ServiceError> {
        let c = self.get_connection()?;
        let dto: QuestionDTO = match diesel::insert_into(questions::dsl::questions).values(
            &NewQuestion {
                id: question.id,
                title: question.title.clone(),
                text: question.text.clone(),
                longitude: question.longitude.clone(),
                latitude: question.latitude.clone(),
                location_point: GeographyPoint {
                    x: match question.latitude.to_f64() {
                        Some(f) => f,
                        None => 0.0
                    },
                    y: match question.longitude.to_f64() {
                        Some(f) => f,
                        None => 0.0
                    },
                    srid: Some(4326)
                },
                address_id: question.address_id,
                user_id: question.user_id,
            }
        ).get_result::<QuestionDTO>(&c) {
            Ok(dto) => dto,
            Err(e) => return Err(e.into())
        };

        let files: Vec<QuestionFile> = question.file_ids.iter().map(|file_id: &Uuid| -> Result<QuestionFile, ServiceError> {
            let result = diesel::insert_into(question_files::dsl::question_files)
            .values(
                NewQuestionFile {
                    file_id: file_id.clone(),
                    question_id: dto.id
                }
            )
            .get_result::<QuestionFile>(&c);
            match result {
                Ok(qf) => Ok(qf),
                Err(e) => Err(e.into())
            }
        })
        .filter_map(|result| match result {
            Ok(f) => Some(f),
            Err(_) => None
        })
        .collect();

        // NOTE: 集計列を更新する

        if let Err(e) = diesel::update(users::dsl::users.filter(users::id.eq(question.user_id)))
            .set(users::questions_count.eq(users::dsl::questions_count + 1))
            .execute(&c) {
            return Err(e.into());
        }

        return Ok((dto, files).into());
    }

    /**
     * タイトル、本文、添付ファイルを変更することができる。
     */
    fn save(&self, question: &Question) -> Result<Question, ServiceError> {
        let ex: Question = self.find(&question.id)?;

        if &ex == question {
            return Ok(ex);
        }

        let c = self.get_connection()?;
        if let Err(e) = diesel::update(questions::dsl::questions.filter(questions::id.eq(question.id)))
            .set((
                questions::title.eq(question.title.clone()),
                questions::text.eq(question.text.clone()),
                questions::updated_at.eq(now)
            )).execute(&c) {
            return Err(e.into());
        }

        if ex.file_ids != question.file_ids {

            if let Err(e) = diesel::delete(
                question_files::dsl::question_files
                    .filter(question_files::question_id.eq(question.id))
            ).execute(&c) {
                return Err(e.into());
            }

            let files: Vec<QuestionFile> = question.file_ids.iter().map(move |file_id: &Uuid| -> Result<QuestionFile, ServiceError> {
                let result = diesel::insert_into(question_files::dsl::question_files)
                .values(
                    NewQuestionFile {
                        file_id: file_id.clone(),
                        question_id: question.id
                    }
                )
                .get_result::<QuestionFile>(&c);
                match result {
                    Ok(qf) => Ok(qf),
                    Err(e) => Err(e.into())
                }
            })
            .filter_map(|result| match result {
                Ok(f) => Some(f),
                Err(_) => None
            }).collect();

            if files.len() != question.file_ids.len() {
                return Err(
                    ServiceError::InternalError {
                        body: None
                    }
                )
            }

        }
        
        return self.find(&question.id);
    }
    fn delete(&self, question_id: &Uuid) -> Result<(), ServiceError> {
        let c = self.get_connection()?;
        if let Err(e) = diesel::delete(
            question_files::dsl::question_files
                .filter(question_files::question_id.eq(question_id))
        ).execute(&c) {
            return Err(e.into());
        }
        if let Err(e) = diesel::delete(
            questions::dsl::questions
                .filter(questions::id.eq(question_id))
        ).execute(&c) {
            return Err(e.into());
        }
        return Ok(());
    }

    fn find(&self, question_id: &Uuid) -> Result<Question, ServiceError> {

        let c = self.get_connection()?;
        let dto = match questions::dsl::questions.filter(questions::id.eq(question_id)).first::<QuestionDTO>(&c) {
            Ok(dto) => dto,
            Err(e) => return Err(e.into())
        };

        let files = match question_files::dsl::question_files
            .filter(question_files::question_id.eq(&dto.id))
            .load::<QuestionFile>(&c) {
            Ok(files) => files,
            Err(e) => return Err(e.into())
        };
        return Ok((dto, files).into());
    }
    
}

impl PgQuestionDAO {

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