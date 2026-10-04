pub mod actions;
pub mod handlers;
pub mod models;
pub mod schema;

use actix_web::{App, HttpServer, web};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;
use tera::Tera;

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

    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Error cargando templates");

    let tera_data = web::Data::new(tera);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .app_data(tera_data.clone())
            .configure(handlers::config) // <- Monta todas las rutas limpiamente
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
