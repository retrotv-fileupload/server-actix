use sea_orm::{Database, DatabaseConnection, DbErr};
use std::fs;
use std::path::Path;

pub struct DatabaseConfig;

impl DatabaseConfig {
    pub async fn connect() -> Result<DatabaseConnection, DbErr> {
        // data 폴더가 없으면 생성
        let data_dir = "./data";
        if !Path::new(data_dir).exists() {
            fs::create_dir_all(data_dir)
                .map_err(|e| DbErr::Custom(format!("폴더 생성 실패: {}", e)))?;
        }
        
        let database_url = "sqlite:./data/app.db?mode=rwc";
        println!("데이터베이스 연결 시도: {}", database_url);
        
        Database::connect(database_url).await
    }
}
