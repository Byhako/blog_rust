pub mod models;
pub mod schema;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::{NewPost, Post, PostForm};
use crate::schema::posts;
use crate::schema::posts::dsl::*;

// Función para insertar un post
pub fn create_post(conn: &mut PgConnection, new_title: &str, new_slug: &str, new_body: &str) -> Post {
    let new_post = NewPost { title: new_title, slug: new_slug, body: new_body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error guardando el nuevo post")
}

// Función para actualizar un post
pub fn update_post(conn: &mut PgConnection, post_id: i32, form: &PostForm) -> Post {
    diesel::update(posts.find(post_id))
        .set(form)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error al actualizar el post")
}

// Función para eliminar un post
pub fn delete_post(conn: &mut PgConnection, post_id: i32) -> usize {
    diesel::delete(posts.find(post_id))
        .execute(conn)
        .expect("Error al eliminar el post")
}

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut conn = PgConnection::establish(&database_url)
        .expect("Error conectando a la base de datos");

    // 1. Insertamos un nuevo post de prueba
    let nuevo = create_post(
        &mut conn,
        "Mi primer post en Rust",
        "mi-primer-post",
        "¡Hola desde Diesel y Supabase!",
    );

    println!(" Post creado con ID: {}", nuevo.id);

    // 2. Actualizamos el post
    let datos_actualizados = PostForm {
        title: None,
        slug: Some("chat-toto"),
        body: None,
    };
    let post_modificado = update_post(&mut conn, 7341, &datos_actualizados);
    println!(" Post modificado con ID: {}", post_modificado.id);

    // 3. Consultamos todos los posts para verificar
    let posts_results = posts
        .select(Post::as_select())
        .load::<Post>(&mut conn)
        .expect("Error cargando los posts");

    println!("\nLista de posts en la base de datos:");
    for post in posts_results {
        println!("- [{}] {} ({})", post.id, post.title, post.slug);
    }
}
