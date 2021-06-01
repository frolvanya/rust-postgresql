use diesel::{pg::PgConnection, RunQueryDsl};

use crate::models::User;

pub fn display_users<'a>(connection: &PgConnection) -> Vec<User> {
    use super::super::schema::users::dsl::*;

    users
        .load::<super::super::models::User>(connection)
        .expect("Error showing users")
}