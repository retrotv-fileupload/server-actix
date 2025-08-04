use sea_orm::DatabaseConnection;
use std::sync::OnceLock;

static DB_CONNECTION: OnceLock<DatabaseConnection> = OnceLock::new();

pub struct Database;

impl Database {
    // 데이터베이스 연결 초기화
    pub fn initialize(connection: DatabaseConnection) {
        DB_CONNECTION.set(connection).expect("Database connection already initialized");
    }

    // 데이터베이스 연결 가져오기
    pub fn get() -> &'static DatabaseConnection {
        DB_CONNECTION.get().expect("Database connection not initialized")
    }
}
