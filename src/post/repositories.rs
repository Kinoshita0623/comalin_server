extern crate diesel;
//use diesel::query_dsl::methods;
use crate::post::entities::*;
use std::vec::Vec;

pub trait PostRepository {
    fn create(&self, new_post: NewPost) -> usize;

    fn find_all(&self) -> Vec<Post>;

    fn find_one(&self, id: i32) -> Post;

    fn delete(&self, id: i32) -> Result<(), diesel::result::Error>;
}