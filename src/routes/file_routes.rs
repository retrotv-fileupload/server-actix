use actix_web::{web, Scope};
use crate::handlers::file_handlers;

pub fn file_routes() -> Scope {
    web::scope("/files")
        .route("/download/{sessionId}", web::get().to(file_handlers::download))
        .route("/upload/init", web::post().to(file_handlers::init))
        .route("/upload/chunk", web::post().to(file_handlers::chunk))
        .route("/upload/complete", web::post().to(file_handlers::complete))
        .route("/upload/cancel", web::post().to(file_handlers::cancel))
        .route("/upload/status", web::get().to(file_handlers::status))
}