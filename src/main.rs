use actix_web::{App, HttpServer};
use sea_orm_migration::prelude::*;

mod config;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use config::database::DatabaseConfig;
use config::database_manager::Database;
use routes::main_routes;
use crate::routes::file_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 서버를 시작합니다: http://localhost:8080");

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

    // Database 매니저에 연결 초기화
    Database::initialize(db.clone());

    // 마이그레이션 실행 전 상태 확인
    println!("📋 마이그레이션을 실행합니다...");
    match migration::Migrator::up(&db, None).await {
        Ok(_) => {
            println!("✅ 마이그레이션 완료!");
            // 마이그레이션 후 파일 존재 확인
            if std::path::Path::new("./data/app.db").exists() {
                println!("✅ app.db 파일이 정상적으로 생성되었습니다!");
                let metadata = std::fs::metadata("./data/app.db").unwrap();
                println!("📊 파일 크기: {} bytes", metadata.len());
            } else {
                println!("⚠️ app.db 파일이 생성되지 않았습니다.");
            }
        }
        Err(e) => {
            eprintln!("❌ 마이그레이션 실행 실패: {:?}", e);
            eprintln!("마이그레이션 오류 세부사항: {}", e);
            std::process::exit(1);
        }
    }

    println!("🎯 데이터베이스 연결 및 마이그레이션 완료");

    HttpServer::new(move || {
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
