use actix_web::middleware;
use actix_web::middleware::DefaultHeaders;

pub mod error;
pub mod routes;
pub mod helper;

pub fn weapon() -> DefaultHeaders {
    middleware::DefaultHeaders::new().add((
        "Weapon",
        std::env::var("WEAPON").unwrap_or_else(|_| "UNKNOWN".to_owned()),
    ))
}