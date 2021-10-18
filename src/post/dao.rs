//use diesel::Connection;
/*extern crate diesel;
//use diesel::query_dsl::methods;
use crate::po;
use crate::schema::posts;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::vec::Vec;
use crate::post::repositories::PostRepository;
use crate::post::entities::*;
use diesel::dsl::sql;
//use models::Post;

pub struct PostDAO<'a>{
    pub connection: &'a PgConnection
}


impl<'a> PostRepository for PostDAO<'a>{

    fn create(&self, new_post: NewPost) -> usize {
        return diesel::insert_into(posts::table)
            .values(new_post)
            .execute(self.connection)
            .expect("作成に失敗");
    }

    fn find_all(&self) -> Vec<Post> {

        // where inのサンプル
        //return posts::dsl::posts.filter(posts::id.eq_any([5, 6, 7])).load::<Post>(self.connection).expect("取得失敗");

        // where inのサンプル（サブクエリバージョン)
        //return posts::dsl::posts.filter(posts::id.eq_any(posts::dsl::posts.select(posts::id))).load::<Post>(self.connection).expect("取得失敗");

        // filter部分にSQLを使用する例
        //return posts::dsl::posts.filter(sql("id in(7,8,9,10)")).load::<Post>(self.connection).expect("取得に失敗");
        return posts::dsl::posts.load::<Post>(self.connection).expect("取得に失敗");
    }

    fn find_one(&self, id: i32) -> Post {
        return posts::dsl::posts.filter(posts::id.eq(id)).first::<Post>(self.connection).expect("取得に失敗");
    }

    fn delete(&self, id: i32) -> Result<(), diesel::result::Error> {
        let result = diesel::delete(posts::dsl::posts.filter(posts::id.eq(id))).execute(self.connection);
        if let Err(err) = result {
            return Err(err);
        };
        return Ok(());
    }
}*/