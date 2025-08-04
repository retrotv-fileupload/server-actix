use crate::models::body::request::InitUploadRequest;
use crate::repositories::file_repository::FileRepository;
use crate::models::entity::file::{self, Model as FileModel};
use sea_orm::{DbErr};

pub struct FileService;

impl FileService {
    pub async fn download(session_id: String) -> String {
        format!("Downloading file for session: {}", session_id)
    }

    pub async fn init(request: InitUploadRequest) -> String {
        let mime_type = request.mime_type
            .unwrap_or_else(|| "application/octet-stream".to_string());

        format!(
            "파일 업로드 초기화 - 파일명: {}, 크기: {} bytes, 청크수: {}, 타입: {}",
            request.file_name,
            request.file_size,
            request.total_chunks,
            mime_type
        )
    }

    pub async fn chunk() -> String {
        "this is file service chunk".to_string()
    }

    pub async fn complete() -> String {
        let new_file = file::Model::new_with_uuid(
            "example_file_id".to_string(),
            "root".to_string(),
            1234,
            "saldkmqowep1i23".to_string(),
            Some("application/octet-stream".to_string()),
        );

        FileRepository::create(new_file).await.expect("TODO: panic message");
        
        "file save success!".to_string()
    }

    pub async fn cancel() -> String {
        "this is file service cancel".to_string()
    }

    pub async fn status() -> String {
        "this is file service status".to_string()
    }
}