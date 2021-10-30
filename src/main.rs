#[macro_use]
pub extern crate diesel;
extern crate crypto;

#[macro_use]
extern crate log;

pub mod schema;
pub mod app_module;
pub mod config;
mod user;
mod question;
mod post;
mod diesel_util;
mod auth;
mod db;
mod errors;
mod router;
mod files;

use actix_web::{HttpServer};

use actix_web::App;
use crate::app_module::AppModule;
use std::env;
use crate::db::DbConfig;
use env_logger::{Builder, Target};
use crate::config::AppConfig;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let mut builder = Builder::from_default_env();

    let app_config = AppConfig {
        app_url: String::new(),
        max_file_capacity: 54000000
    };
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
                    pool: Box::new(pool.clone()),
                    config: Box::new(app_config.clone())
                }
            )
            .configure(router::route)
    })
    .bind(host)?
    .run()
    .await
}

