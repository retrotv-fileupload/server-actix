use actix_web::{App, HttpServer};

mod handlers;
mod routes;
mod services;

use routes::main_routes;
use crate::routes::file_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("서버를 시작합니다: http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(
                main_routes::main_routes()
                    .service(file_routes::file_routes())
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
