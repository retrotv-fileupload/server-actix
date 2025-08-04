use sea_orm::{Database, DatabaseConnection, DbErr};
use std::fs;
use std::path::Path;

pub struct DatabaseConfig;

impl DatabaseConfig {
    pub async fn connect() -> Result<DatabaseConnection, DbErr> {
        // 현재 작업 디렉토리 확인
        let current_dir = std::env::current_dir()
            .map_err(|e| DbErr::Custom(format!("현재 디렉토리 확인 실패: {}", e)))?;
        println!("현재 작업 디렉토리: {:?}", current_dir);

        // 프로젝트 루트의 data 폴더 경로 설정
        let data_dir = "./data";
        if !Path::new(data_dir).exists() {
            println!("data 디���토리 생성: {}", data_dir);
            fs::create_dir_all(data_dir)
                .map_err(|e| DbErr::Custom(format!("폴더 생성 실패: {}", e)))?;
        }

        let database_url = "sqlite:./data/app.db?mode=rwc";
        println!("데이터베이스 연결 시도: {}", database_url);

        match Database::connect(database_url).await {
            Ok(connection) => {
                println!("✅ 데이터베이스 연결 성공!");
                // 데이터베이스 파일 존재 확인
                if Path::new("./data/app.db").exists() {
                    println!("✅ app.db 파일이 생성되었습니다!");
                } else {
                    println!("⚠️ app.db 파일이 아직 생성되지 않았습니다.");
                }
                Ok(connection)
            }
            Err(e) => {
                eprintln!("❌ 데이터베이스 연결 실패: {:?}", e);
                Err(e)
            }
        }
    }
}
