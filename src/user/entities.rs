use diesel::sql_types::Uuid;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    //pub encrypted_password: String,
    //pub avatar_icon: String,
}


pub fn hoge() {
    println!("hoge");
}