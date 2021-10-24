use actix_web::web;
use crate::auth;
pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::auth_controller::login))
                    .route("/register", web::post().to(auth::auth_controller::register))
            )
    );
}