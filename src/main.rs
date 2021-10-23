#[macro_use]
extern crate diesel;
extern crate crypto;

pub mod schema;
pub mod app_module;
mod user;
mod question;
mod post;
mod diesel_util;
mod auth;
mod db;
mod errors;

use actix_web::{HttpResponse, HttpServer, Responder, web};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::Connection;
use diesel::dsl::sql;
use crate::schema::questions;
use crate::schema::users;
use crate::schema::posts;
use crate::post::entities;
use diesel::pg::PgConnection;
use crate::post::entities::Post;
use crate::diesel_util::selectable::Selectable;
use diesel::sql_types;
use diesel_util::geography::*;
use crate::question::entities::{NewQuestion, NewQuestionAttr, Question};
use crate::user::entities::{PublicUser, User};
use crate::user::commands::{NewUser, NewUserAttr};
use actix_web::App;
use crate::auth::sample_middleware;
use crate::auth::sample_middleware::SayHi;
use crate::auth::auth_middleware::TokenAuth;
use crate::app_module::AppModule;
use std::env;
use crate::db::DbConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URLが存在しません");
    println!("DATABASE_URL:{}", database_url);
    let config = DbConfig {
        url: database_url,
        connection_count: 2
    };
    let pool = config.create_pool();
    HttpServer::new(move || {
        App::new()
            .data(
                AppModule {
                    pool: Box::new(pool.clone())
                }
            )
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/private")
                    .wrap(SayHi{})
                    .wrap(TokenAuth{})
                    .route("/hoge", web::get().to(private_hello))
            )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn private_hello() -> impl Responder {
    return HttpResponse::Ok().body("Private There!!");
}