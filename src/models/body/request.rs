use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InitUploadRequest {
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    #[serde(rename = "totalChunks")]
    pub total_chunks: i32,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
}
