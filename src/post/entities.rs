use crate::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
