use actix_web::{web, Scope};
use crate::handlers::main_handlers;

pub fn main_routes() -> Scope {
    web::scope("/api")
        .route("/check", web::get().to(main_handlers::check))
}
