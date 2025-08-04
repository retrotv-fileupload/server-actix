use sea_orm::*;
use crate::models::entity::file::{self, Entity as FileEntity, Model as FileModel, ActiveModel as FileActiveModel};
use crate::config::database_manager::Database;

pub struct FileRepository;

impl FileRepository {
    // Create - 새로운 파일 레코드 생성
    pub async fn create(file_data: FileActiveModel) -> Result<FileModel, DbErr> {
        let db = Database::get();
        file_data.insert(db).await
    }

    // Read - ID로 파일 조회
    pub async fn find_by_id(id: &str) -> Result<Option<FileModel>, DbErr> {
        let db = Database::get();
        FileEntity::find_by_id(id).one(db).await
    }

    // Read - 모든 활성 파일 조회 (is_use = true)
    pub async fn find_all_active() -> Result<Vec<FileModel>, DbErr> {
        let db = Database::get();
        FileEntity::find()
            .filter(file::Column::IsUse.eq(true))
            .all(db)
            .await
    }

    // Read - 페이지네이션으로 활성 파일 조회
    pub async fn find_active_with_pagination(
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<FileModel>, u64), DbErr> {
        let db = Database::get();
        let paginator = FileEntity::find()
            .filter(file::Column::IsUse.eq(true))
            .order_by_desc(file::Column::CreatedAt)
            .paginate(db, per_page);
        
        let total_pages = paginator.num_pages().await?;
        let files = paginator.fetch_page(page - 1).await?;
        
        Ok((files, total_pages))
    }

    // Read - 카테고리별 파일 조회
    pub async fn find_by_category(category: &str) -> Result<Vec<FileModel>, DbErr> {
        let db = Database::get();
        FileEntity::find()
            .filter(file::Column::Category.eq(category))
            .filter(file::Column::IsUse.eq(true))
            .all(db)
            .await
    }

    // Read - 업로드한 사용자별 파일 조회
    pub async fn find_by_uploaded_by(uploaded_by: &str) -> Result<Vec<FileModel>, DbErr> {
        let db = Database::get();
        FileEntity::find()
            .filter(file::Column::UploadedBy.eq(uploaded_by))
            .filter(file::Column::IsUse.eq(true))
            .all(db)
            .await
    }

    // Read - 파일 해시로 조회 (중복 파일 체크용)
    pub async fn find_by_hash(hash: &str) -> Result<Option<FileModel>, DbErr> {
        let db = Database::get();
        FileEntity::find()
            .filter(file::Column::Hash.eq(hash))
            .filter(file::Column::IsUse.eq(true))
            .one(db)
            .await
    }

    // Update - 파일 정보 업데이트
    pub async fn update(id: &str, mut file_data: FileActiveModel) -> Result<FileModel, DbErr> {
        let db = Database::get();
        // updated_at을 현재 시간으로 설정
        file_data.updated_at = Set(chrono::Utc::now());
        
        let file = FileEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("File not found".to_string()))?;

        let mut active_model: FileActiveModel = file.into();
        
        // 변경된 필드만 업데이트
        if file_data.original_file_name.is_set() {
            active_model.original_file_name = file_data.original_file_name;
        }
        if file_data.file_path.is_set() {
            active_model.file_path = file_data.file_path;
        }
        if file_data.mime_type.is_set() {
            active_model.mime_type = file_data.mime_type;
        }
        if file_data.size.is_set() {
            active_model.size = file_data.size;
        }
        if file_data.hash.is_set() {
            active_model.hash = file_data.hash;
        }
        if file_data.is_use.is_set() {
            active_model.is_use = file_data.is_use;
        }
        if file_data.description.is_set() {
            active_model.description = file_data.description;
        }
        if file_data.uploaded_by.is_set() {
            active_model.uploaded_by = file_data.uploaded_by;
        }
        if file_data.category.is_set() {
            active_model.category = file_data.category;
        }
        if file_data.metadata.is_set() {
            active_model.metadata = file_data.metadata;
        }
        active_model.updated_at = Set(chrono::Utc::now());

        active_model.update(db).await
    }

    // Delete - 논리적 삭제 (is_use를 false로 설정)
    pub async fn soft_delete(id: &str) -> Result<FileModel, DbErr> {
        let db = Database::get();
        let file = FileEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("File not found".to_string()))?;

        let mut active_model: FileActiveModel = file.into();
        active_model.is_use = Set(false);
        active_model.updated_at = Set(chrono::Utc::now());

        active_model.update(db).await
    }

    // Delete - 물리적 삭제 (실제 레코드 삭제)
    pub async fn hard_delete(id: &str) -> Result<DeleteResult, DbErr> {
        let db = Database::get();
        FileEntity::delete_by_id(id).exec(db).await
    }

    // Utility - 파일 존재 여부 확인
    pub async fn exists(id: &str) -> Result<bool, DbErr> {
        let db = Database::get();
        let count = FileEntity::find_by_id(id).count(db).await?;
        Ok(count > 0)
    }

    // Utility - 활성 파일 개수 조회
    pub async fn count_active() -> Result<u64, DbErr> {
        let db = Database::get();
        FileEntity::find()
            .filter(file::Column::IsUse.eq(true))
            .count(db)
            .await
    }

    // Utility - 전체 파일 크기 조회
    pub async fn total_size() -> Result<Option<i64>, DbErr> {
        let db = Database::get();
        use sea_orm::sea_query::Expr;
        
        let result = FileEntity::find()
            .filter(file::Column::IsUse.eq(true))
            .select_only()
            .column_as(Expr::col(file::Column::Size).sum(), "total_size")
            .into_tuple::<Option<i64>>()
            .one(db)
            .await?;

        Ok(result.flatten())
    }
}
