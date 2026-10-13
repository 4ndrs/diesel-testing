use crate::schema::{manekinas, posts};
use chrono::NaiveDateTime;

use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(HasQuery, Debug)]
#[diesel(table_name = manekinas)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Manekina {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
