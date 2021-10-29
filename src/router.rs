use actix_web::web;
use crate::auth::auth_controller;
use crate::files::file_controller;
pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth_controller::login))
                    .route("/register", web::post().to(auth_controller::register))
            )
            .service(
                web::scope("/files")
                    .route("", web::post().to(file_controller::upload))
            )
    );
}