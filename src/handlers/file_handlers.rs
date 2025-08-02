use actix_web::{HttpResponse, Result};
use crate::services::file_service::FileService;

pub async fn download() -> Result<HttpResponse> {
    let message = FileService::download().await;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": message
    })))
}

pub async fn init() -> Result<HttpResponse> {
    let message = FileService::init().await;
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
