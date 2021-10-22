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

fn main() {
    println!("hello");

    let connection = PgConnection::establish("postgres://dbuser:secret@postgis:5432/database").expect("error");

    //let posts = posts::dsl::posts.load::<Post>(&connection).expect("load error");
    /*for p in posts {
        println!("{}", p.title);
    }*/
    /*let questions = questions::dsl::questions.load::<Question>(&connection).expect("failed");
    for q in questions {
        println!("{}", q.title);
    }*/
    /*let users = users::dsl::users.load::<User>(&connection).expect("failed");
    for u in users {
        println!("{}", u.username);
    }*/


    let a = NewUser::new(NewUserAttr {
        username: uuid::Uuid::new_v4().to_string(),
        avatar_icon: None,
        password: "hogehoge".to_string()
    }).expect("error");
    let user = diesel::insert_into(users::dsl::users).values(a).get_result::<User>(&connection).expect("error");
    println!("inserted user {}", user.username);
    let question = NewQuestion::new(
        NewQuestionAttr {
            user_id: &user.id,
            title: "hogehoge",
            text: None,
            latitude: &BigDecimal::from(34.7073686),
            longitude: &BigDecimal::from(135.4969857)
        }
    ).expect("create failed");
    let question = diesel::insert_into(questions::dsl::questions).values(question)
        .get_result::<Question>(&connection)
        .expect("error");

    println!("question id:{}", question.title);


    let result = users::dsl::users.select(PublicUser::columns()).load::<PublicUser>(&connection).expect("failed");


    for u in result {
        println!("username:{}", u.username);
    }

    let questions = questions::dsl::questions
        .select((
            questions::id,
            questions::title,
            questions::text,
            questions::longitude,
            questions::latitude,
            questions::location_point,
            questions::address_id,
            questions::user_id,
            questions::created_at,
            questions::updated_at
            ))
        .load::<Question>(&connection)
        .expect("load error");
    for q in questions {
        println!("{}", q.title);
    }
    //question::dsl::question::load::<Question>(&connection).expect("取得失敗");

}