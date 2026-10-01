use diesel::prelude::*;
use crate::schema::posts;

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

// Struct para INSERTAR registros
#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub body: &'a str,
}

// Struct para ACTUALIZAR registros
#[derive(AsChangeset)]
#[diesel(table_name = posts)]
pub struct PostForm<'a> {
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub slug: Option<&'a str>,
}
