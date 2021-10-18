#[macro_use]
extern crate diesel;
pub mod schema;
mod user;
mod question;
mod post;
mod diesel_util;
//use diesel::Connection;
use diesel::prelude::*;
use diesel::Connection;
use crate::user::entities::{PublicUser, User};
use crate::question::entities::Question;
//use crate::question::entities::Question;
use diesel::dsl::sql;
use crate::schema::questions;
use crate::schema::users;
use crate::schema::posts;
use crate::post::entities;
use diesel::pg::PgConnection;
use crate::post::entities::Post;
use crate::diesel_util::selectable::Selectable;
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
    let users = users::dsl::users.load::<User>(&connection).expect("failed");
    for u in users {
        println!("{}", u.username);
    }
    let result = users::dsl::users.select(PublicUser::columns()).load::<PublicUser>(&connection).expect("failed");

    for u in result {
        println!("{}", u.username);
    }
    //question::dsl::question::load::<Question>(&connection).expect("取得失敗");

}