pub mod actions;
pub mod handlers;
pub mod models;
pub mod schema;

use actix_web::{App, HttpServer, web};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("Error creando el pool");

    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .configure(handlers::config) // <- Monta todas las rutas limpiamente
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await

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
}
