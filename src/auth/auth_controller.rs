
use actix_web::web;
use actix_web::Responder;
use actix_web::HttpResponse;
use crate::app_module::AppModule;
use crate::user::services::LoginRequest;
use crate::user::services::RegisterRequest;
use crate::errors::service_error::ServiceError;
use crate::user::services::AuthResponse;


pub async fn login(data: web::Data<AppModule>, json: web::Json<LoginRequest>) -> impl Responder {

    let be: ServiceError = match web::block(move || -> Result<AuthResponse, ServiceError>{
        let um = data.as_ref().user_module();
        let user_service = um.user_service();
        return user_service.login(&json.0);
    }).await {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        },
        Err(e) => e.into()
    };
    return be.response();


}

pub async fn register(data: web::Data<AppModule>, json: web::Json<RegisterRequest>) -> impl Responder {

    let be = match web::block(move || -> Result<AuthResponse, ServiceError> { 
        let um = data.as_ref().user_module();
        let user_service = um.user_service();

        user_service.register(&json.0)
    } ).await {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        },
        Err(e) => e
    };
    let e: ServiceError = be.into();
    return e.response();
}
