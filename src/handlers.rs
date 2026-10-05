use crate::DbPool;
use crate::actions::{create_post, delete_post, update_post};
use crate::models::{CreatePostForm, Post, PostForm};
use crate::schema::posts::dsl::*;

use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use diesel::prelude::*;
use tera::Context;

#[get("/")]
pub async fn index(pool: web::Data<DbPool>, tera: web::Data<tera::Tera>) -> impl Responder {
    let mut conn = pool.get().expect("Error obteniendo conexión");

    match web::block(move || posts.select(Post::as_select()).load::<Post>(&mut conn)).await {
        // Si el hilo terminó bien Y la consulta SQL fue exitosa, extrae el vector
        Ok(Ok(posts_list)) => {
            if posts_list.is_empty() {
                return HttpResponse::NotFound().body("Posts no encontrados");
            }

            let mut context = Context::new();
            context.insert("posts", &posts_list);

            let html = tera.render("index.html", &context).unwrap();
            HttpResponse::Ok().body(html)
        }
        // El hilo terminó bien, pero la base de datos arrojó un error
        Ok(Err(db_err)) => HttpResponse::InternalServerError().body(db_err.to_string()),
        // El hilo de web::block falló
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/post/{slug}")]
pub async fn get_post(
    pool: web::Data<DbPool>,
    tera: web::Data<tera::Tera>,
    slug_data: web::Path<std::string::String>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error obteniendo conexión");
    let slug_param = slug_data.into_inner();

    match web::block(move || {
        posts
            .filter(slug.eq(slug_param))
            .select(Post::as_select())
            .first::<Post>(&mut conn)
    })
    .await
    {
        // Si el hilo terminó bien Y la consulta SQL fue exitosa, extrae el vector
        Ok(Ok(post)) => {
            let mut context = Context::new();
            context.insert("post", &post);

            let html = tera.render("post.html", &context).unwrap();
            HttpResponse::Ok().body(html)
        }
        // Si Diesel no encontró ninguna fila con ese slug:
        Ok(Err(diesel::result::Error::NotFound)) => {
            HttpResponse::NotFound().body("Post no encontrado")
        }
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

#[patch("/update/{id}")]
pub async fn update_post_handler(
    pool: web::Data<DbPool>,
    post_id: web::Path<i32>,
    item: web::Json<CreatePostForm>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error obteniendo conexión");
    let target_id = post_id.into_inner();
    let new_slug = item.title.replace(' ', "-").to_lowercase();

    match web::block(move || {
        let form = PostForm {
            title: Some(&item.title),
            body: Some(&item.body),
            slug: Some(&new_slug),
        };
        update_post(&mut conn, target_id, &form)
    })
    .await
    {
        Ok(Ok(updated_post)) => HttpResponse::Ok().json(updated_post),
        Ok(Err(db_err)) => HttpResponse::InternalServerError().body(db_err.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/delete/{id}")]
pub async fn delete_post_handler(
    pool: web::Data<DbPool>,
    post_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error obteniendo conexión");
    let target_id = post_id.into_inner();

    match web::block(move || delete_post(&mut conn, target_id)).await {
        Ok(Ok(deleted_count)) => HttpResponse::Ok().json(deleted_count),
        Ok(Err(db_err)) => HttpResponse::InternalServerError().body(db_err.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Función para registrar todos los endpoints de este módulo
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(get_post);
    cfg.service(create_post_handler);
    cfg.service(update_post_handler);
    cfg.service(delete_post_handler);
}
