#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub age: &'a i32,
}
