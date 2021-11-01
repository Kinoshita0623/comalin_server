use actix_web::web;
use actix_web::HttpRequest;
use actix_web::Responder;
use crate::auth::token_util::get_bearer_token;
use crate::app_module::AppModule;
use crate::question::service::CreateQuestion;
use actix_web::HttpResponse;


pub async fn create(data: web::Data<AppModule>, req: HttpRequest, json: web::Json<CreateQuestion>) -> impl Responder {

    let token = get_bearer_token(req);
    let token = match token {
        Ok(token) => token,
        Err(e) => return e.response()
    };

    return match data.question_module().question_service().create(&token, &json.0) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => e.response()
    };

}


pub async fn all(data: web::Data<AppModule>) -> impl Responder {
    return match data.question_module().question_service().find_all() {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => e.response()
    };
}