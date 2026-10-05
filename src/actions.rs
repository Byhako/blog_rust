use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::{NewPost, Post, PostForm};
use crate::schema::posts::dsl::*;

// Insert
pub fn create_post(
    conn: &mut PgConnection,
    new_title: &str,
    new_slug: &str,
    new_body: &str,
) -> QueryResult<Post> {
    let new_post = NewPost {
        title: new_title,
        slug: new_slug,
        body: new_body,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
}

// Update
pub fn update_post(
    conn: &mut PgConnection,
    post_id: i32,
    form: &PostForm,
) -> QueryResult<Post> {
    diesel::update(posts.find(post_id))
        .set(form)
        .returning(Post::as_returning())
        .get_result(conn)
}

// Delete
pub fn delete_post(conn: &mut PgConnection, post_id: i32) -> QueryResult<usize> {
    diesel::delete(posts.find(post_id))
        .execute(conn)
}
