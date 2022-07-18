#[macro_use]
extern crate diesel;

pub mod users;
pub mod schema;

use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Finds user by UID.
#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        let conn = pool.get()?;
        users::actions::find_user(user_uid, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {user_uid}"));
        Ok(res)
    }
}

/// Inserts new user with name defined in form.
#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<users::models::User>,
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        let conn = pool.get()?;
        users::actions::create_user(form.id, &form.lang, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

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

    log::info!("starting HTTP server at http://localhost:8080");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(get_user)
            .service(add_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}