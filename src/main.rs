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

    let host = env::var("HOST").expect("HOSTが存在しません");
    let connection_count = env::var("CONNECTION_COUNT").expect("CONNECTION_POOL数を読み込めませんでした");


    println!("DATABASE_URL:{}", database_url);
    let config = DbConfig {
        url: database_url.clone(),
        connection_count: connection_count.parse::<u32>().expect("コネクション数は整数型です")
    };
    let pool = config.create_pool();
    println!("DATABASE_URL:{}", database_url.clone());
    HttpServer::new(move || {
        App::new()
            .data(
                AppModule {
                    pool: Box::new(pool.clone())
                }
            )
            .route("/login", web::post().to(auth::auth_controller::login))
            .route("/register", web::post().to(auth::auth_controller::register))
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/private")
                    .wrap(SayHi{})
                    .wrap(TokenAuth{})
                    .route("/hoge", web::get().to(private_hello))
            )
    })
    .bind(host)?
    .run()
    .await
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn private_hello() -> impl Responder {
    return HttpResponse::Ok().body("Private There!!");
}