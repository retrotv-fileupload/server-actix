use crate::sea_orm::Set;
use sea_orm::entity::prelude::*;
use sea_orm::NotSet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "file")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub saved_file_name: String,  // UUID v7을 문자열로 저장
    pub original_file_name: String,
    pub file_path: String,
    pub mime_type: String,
    pub size: i64,
    pub hash: String,
    pub is_use: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub description: Option<String>,
    pub uploaded_by: Option<String>,
    pub category: Option<String>,
    pub metadata: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new_with_uuid(
        original_file_name: String,
        file_path: String,
        size: i64,
        hash: String,
        mime_type: Option<String>,
    ) -> ActiveModel {
        let now = chrono::Utc::now();
        ActiveModel {
            saved_file_name: Set(Uuid::now_v7().to_string()),
            original_file_name: Set(original_file_name),
            file_path: Set(file_path),
            mime_type: Set(mime_type.unwrap_or_else(|| "application/octet-stream".to_string())),
            size: Set(size),
            hash: Set(hash),
            is_use: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            description: NotSet,
            uploaded_by: NotSet,
            category: NotSet,
            metadata: NotSet,
        }
    }
}
