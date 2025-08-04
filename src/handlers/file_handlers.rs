use actix_web::{web, HttpResponse, Result};
use crate::services::file_service::FileService;
use crate::models::InitUploadRequest;

pub async fn download(path: web::Path<String>) -> Result<HttpResponse> {
    let session_id = path.into_inner();
    let message = FileService::download(session_id).await;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": message
    })))
}

pub async fn init(req: web::Json<InitUploadRequest>) -> Result<HttpResponse> {
    let upload_request = req.into_inner();
    let message = FileService::init(upload_request).await;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": message
    })))
}

pub async fn chunk() -> Result<HttpResponse> {
    let message = FileService::chunk().await;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": message
    })))
}

pub async fn complete() -> Result<HttpResponse> {
    let message = FileService::complete().await;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": message
    })))
}

pub async fn cancel() -> Result<HttpResponse> {
    let message = FileService::cancel().await;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": message
    })))
}

pub async fn status() -> Result<HttpResponse> {
    let message = FileService::status().await;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": message
    })))
}
