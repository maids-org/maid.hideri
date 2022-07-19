#[macro_use]
extern crate diesel;

mod core;
mod schema;
mod users;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    log::info!("Running server on http://localhost:8080");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new().add((
                "Weapon",
                std::env::var("WEAPON").unwrap_or_else(|_| "UNKNOWN".to_owned()),
            )))
            .service(core::routes::index)
            .service(users::routes::get_user)
            .service(users::routes::add_user)
            .default_service(web::route().to(core::error::not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
