use serde::Deserialize;

#[derive(Queryable, PartialEq, Debug, Insertable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct NewRegistratedUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub confirmed_password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}
