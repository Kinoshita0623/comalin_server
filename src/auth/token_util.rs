use actix_web::HttpRequest;
use crate::errors::service_error::ServiceError;

pub fn get_bearer_token(req: HttpRequest) -> Result<String, ServiceError>{
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                let token = auth_str[6..auth_str.len()].trim();
                /*if let Ok(b) = app_module.user_module().user_service().check_token(token) {
                    if b {
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;
                            return Ok(res);
                        });
                    }

                }*/
                return Ok(token.to_string());
            }
        }
    }
    return Err(ServiceError::AuthenticationError);
}

