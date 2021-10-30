use validator::ValidationErrors;
use actix_web::HttpResponse;
use serde::Serialize;
use actix_web::http::StatusCode;
use actix_web::error::BlockingError;

#[derive(Debug, PartialEq)]
pub enum ServiceError {
    AuthenticationError,
    ValidationError {
        body: ValidationErrors
    },
    InternalError {
        body: Option<String>
    },
    NotFoundError

}

#[derive(Serialize)]
struct ErrorBody<T: Serialize> {
    message: String,
    body: T 
}

impl ServiceError {

    pub fn response(&self) -> HttpResponse {
        return match &self {
            ServiceError::AuthenticationError => {
                HttpResponse::build(StatusCode::UNAUTHORIZED).json(
                    ErrorBody {
                        message: "Unauthorized".to_string(),
                        body: "未認証状態又はTokenが無効です。"
                    }
                )
            }
            ServiceError::ValidationError { body } => {
                HttpResponse::build(StatusCode::BAD_REQUEST).json(
                    ErrorBody {
                        message: "Validation Failed".to_string(),
                        body: body
                    }
                )
            }
            ServiceError::InternalError { body: _ } => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(
                    ErrorBody {
                        message: "Server Error".to_string(),
                        body: "サーバで問題が発生しました。管理者に連絡してください".to_string()
                    }
                )
            }
            ServiceError::NotFoundError => {
                HttpResponse::build(StatusCode::NOT_FOUND).json(
                    ErrorBody {
                        message: "Not Found".to_string(),
                        body: "コンテンツは存在しません".to_string()
                    }
                )
            }
        };
    }
}

impl Into<ServiceError> for BlockingError<ServiceError> {

    fn into(self) -> ServiceError {
        match self {
            BlockingError::Canceled => {
                let res = ServiceError::InternalError { body: None };
                return res;
            },
            BlockingError::Error(e) => {
                return e;
            }
        }
    }
}