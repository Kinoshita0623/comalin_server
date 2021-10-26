#[macro_use]
pub extern crate diesel;
extern crate crypto;

#[macro_use]
extern crate log;

pub mod schema;
pub mod app_module;
mod user;
mod question;
mod post;
mod diesel_util;
mod auth;
mod db;
mod errors;
mod router;

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
use env_logger::{Builder, Target};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
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
            .configure(router::route)
    })
    .bind(host)?
    .run()
    .await
}

