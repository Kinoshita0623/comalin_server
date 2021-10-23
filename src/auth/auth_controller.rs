
use actix_web::web;
use actix_web::Responder;
use actix_web::HttpResponse;
use crate::app_module::AppModule;
use crate::user::services::LoginRequest;
use crate::user::services::RegisterRequest;

pub async fn login(data: web::Data<AppModule>, json: web::Json<LoginRequest>) -> impl Responder {

    let um = data.as_ref().user_module();
    let user_service = um.user_service();
    match user_service.login(&json.0) {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        }
        Err(e) => {
            return e.response();
        }
    }   



}

pub async fn register(data: web::Data<AppModule>, json: web::Json<RegisterRequest>) -> impl Responder {

    let um = data.as_ref().user_module();
    let user_service = um.user_service();

    match user_service.register(&json.0) {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        }
        Err(e) => {
            return e.response();
        }
    }


}