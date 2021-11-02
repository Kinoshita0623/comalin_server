use crate::errors::service_error::ServiceError;
use crate::user::entities::PublicUser;
use crate::user::module::UserModule;
use validator::ValidationErrors;
use validator::ValidationError;
use crate::user::commands::{NewUser, NewUserAttr};
use validator::Validate;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(required)]
    pub username: Option<String>,
    #[validate(required, length(min = 6, max = 40))]
    pub password: Option<String>
}

#[derive(Validate, Deserialize)]
pub struct RegisterRequest {
    #[validate(required)]
    pub username: Option<String>,
    #[validate(required)]
    pub password: Option<String>
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: PublicUser
}

pub trait UserService {
    fn login(&self, req: &LoginRequest) -> Result<AuthResponse, ServiceError>;
    fn logout(&self, token: &str) -> Result<(), ServiceError>;
    fn register(&self, req: &RegisterRequest) -> Result<AuthResponse, ServiceError>;
    fn check_token(&self, token: &str) -> Result<bool, ServiceError>;
}

pub struct PgUserService {
    pub user_module: Box<dyn UserModule>
}
impl UserService for PgUserService {
    fn login(&self, req: &LoginRequest) -> Result<AuthResponse, ServiceError> {
        /*if let Err(e) = req.validate() {
            return Err(ServiceError::ValidationError { body: e })
        }*/
        let ur = self.user_module.user_repository();
        let res = ur.find_by_name(req.username.clone().unwrap());
        if let Err(e) = res {
            return match e {
                ServiceError::NotFoundError => {
                    let mut ve = ValidationErrors::new();
                    ve.add("username", ValidationError::new("unknown user"));
                    Err(
                        ServiceError::ValidationError {
                            body: ve
                        }
                    )
                }
                _ => {
                    Err(e)
                }
            }
        }
        let u = match res {
            Ok(u) => u,
            Err(e) => {
                return match e {
                    ServiceError::NotFoundError => {
                        let mut ve = ValidationErrors::new();
                        ve.add("username", ValidationError::new("unknown user"));
                        Err(
                            ServiceError::ValidationError {
                                body: ve
                            }
                        )
                    }
                    _ => {
                        Err(e)
                    }
                }
            }
        };

        if !u.check_password(&req.password.clone().unwrap()) {
            let mut ve = ValidationErrors::new();
            ve.add("password", ValidationError::new("unknown user"));
            return Err(
                ServiceError::ValidationError {
                    body: ve
                }
            )
        }

        let token = &u.make_token();
         return match ur.save_token(token.into()) {
             Ok(_) => Ok(
                 AuthResponse {
                     token: token.token.clone(),
                     user: u.into()
                 }
             ),
             Err(e) => Err(e)
         }
    }

    fn logout(&self, token: &str) -> Result<(), ServiceError> {
        let ur = self.user_module.user_repository();
        return ur.delete_token(token);

    }

    fn register(&self, req: &RegisterRequest) -> Result<AuthResponse, ServiceError> {
        if let Err(e) = req.validate() {
            return Err(ServiceError::ValidationError { body: e })
        }
        let ur = self.user_module.user_repository();

        let new_user = NewUser::new(NewUserAttr {
            username: req.username.clone().unwrap(),
            password: req.password.clone().unwrap(),
            avatar_id: None,
            avatar_url: None
        });
        let new_user = match new_user {
            Ok(u) => u,
            Err(e) => return Err(
                ServiceError::InternalError {
                    body: Some(e.to_string())
                }
            )
        };
        let res = ur.create(&new_user);
        let u = match res {
            Ok(u) => u,
            Err(e) => return Err(e)
        };
        let token = &u.make_token();
        ur.save_token(token.into())?;
        return Ok(
            AuthResponse {
                token: token.token.clone(),
                user: u.into()
            }
        );
    }

    fn check_token(&self, token: &str) -> Result<bool, ServiceError> {
        let ur = self.user_module.user_repository();
        return match ur.find_by_token(token) {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        };
    }


}