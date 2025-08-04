use actix_web::{App, HttpServer};
use sea_orm_migration::prelude::*;

mod config;
mod handlers;
mod models;
mod routes;
mod services;

use config::database::DatabaseConfig;
use routes::main_routes;
use crate::routes::file_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("서버를 시작합니다: http://localhost:8080");

    // 데이터베이스 연결
    let db = match DatabaseConfig::connect().await {
        Ok(connection) => {
            println!("✅ 데이터베이스 연결 성공!");
            connection
        }
        Err(e) => {
            eprintln!("❌ 데이터베이스 연결 실패: {:?}", e);
            eprintln!("현재 작업 디렉토리: {:?}", std::env::current_dir().unwrap());
            std::process::exit(1);
        }
    };

    // 마이그레이션 실행 - MigratorTrait을 사용
    match migration::Migrator::up(&db, None).await {
        Ok(_) => println!("✅ 마이그레이션 완료!"),
        Err(e) => {
            eprintln!("❌ 마이그레이션 실행 실패: {:?}", e);
            std::process::exit(1);
        }
    }

    println!("🚀 데이터베이스 연결 및 마이그레이션 완료");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db.clone()))
            .service(
                main_routes::main_routes()
                    .service(file_routes::file_routes())
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
