use crate::DbPool;
use crate::actions::{create_post, delete_post, update_post};
use crate::models::{CreatePostForm, Post};
use crate::schema::posts::dsl::*;
use actix_web::{HttpResponse, Responder, get, post, web};
use diesel::prelude::*;

#[get("/")]
pub async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Error obteniendo conexión");

    match web::block(move || posts.select(Post::as_select()).load::<Post>(&mut conn)).await {
        // Si el hilo terminó bien Y la consulta SQL fue exitosa, extrae el vector
        Ok(Ok(posts_list)) => HttpResponse::Ok().json(posts_list),
        // El hilo terminó bien, pero la base de datos arrojó un error
        Ok(Err(db_err)) => HttpResponse::InternalServerError().body(db_err.to_string()),
        // El hilo de web::block falló
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/new-post")]
pub async fn create_post_handler(
    pool: web::Data<DbPool>,
    item: web::Json<CreatePostForm>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error obteniendo conexión");

    match web::block(move || {
        create_post(
            &mut conn,
            &item.title,
            &item.title.to_string().replace(" ", "-").to_lowercase(),
            &item.body,
        )
    })
    .await
    {
        Ok(Ok(new_post)) => HttpResponse::Created().json(new_post),
        Ok(Err(db_err)) => HttpResponse::InternalServerError().body(db_err.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Función para registrar todos los endpoints de este módulo
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(create_post_handler);
    // cfg.service(get_post_by_id);
}
