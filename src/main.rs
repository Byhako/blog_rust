pub mod actions;
pub mod models;
pub mod schema;

use actix_web::guard::Connect;
use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

use crate::actions::{create_post, delete_post, update_post};
use crate::models::{NewPost, Post, PostForm};
use crate::schema::posts;
use crate::schema::posts::dsl::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conection = ConnectionManager::<PgConnection>::new(&database_url);

    let pool = Pool::builder()
        .build(conection)
        .expect("Error creando el pool");

    let pool_data = web::Data::new(pool);

    HttpServer::new(move || App::new().service(hello).app_data(pool_data.clone()))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await

    // let mut conn = PgConnection::establish(&database_url)
    //     .expect("Error conectando a la base de datos");

    // // 1. Insertamos un nuevo post de prueba
    // let nuevo = create_post(
    //     &mut conn,
    //     "Mi primer post en Rust",
    //     "mi-primer-post",
    //     "¡Hola desde Diesel y Supabase!",
    // );

    // println!(" Post creado con ID: {}", nuevo.id);

    // // 2. Actualizamos el post
    // let datos_actualizados = PostForm {
    //     title: None,
    //     slug: Some("chat-toto"),
    //     body: None,
    // };
    // let post_modificado = update_post(&mut conn, 7341, &datos_actualizados);
    // println!(" Post modificado con ID: {}", post_modificado.id);

    // // 3. Consultamos todos los posts para verificar
    // let posts_results = posts
    //     .select(Post::as_select())
    //     .load::<Post>(&mut conn)
    //     .expect("Error cargando los posts");

    // println!("\nLista de posts en la base de datos:");
    // for post in posts_results {
    //     println!("- [{}] {} ({})", post.id, post.title, post.slug);
    // }
}
