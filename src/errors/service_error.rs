use validator::ValidationErrors;
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