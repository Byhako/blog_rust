use crate::schema::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreatePostForm {
    pub title: String,
    pub body: String,
}

// Struct para INSERTAR registros
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub body: &'a str,
}

// Struct para ACTUALIZAR registros
#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = posts)]
pub struct PostForm<'a> {
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub slug: Option<&'a str>,
}
