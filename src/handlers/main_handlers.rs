use actix_web::{HttpResponse, Result};
use crate::services::main_service::MainService;

pub async fn check() -> Result<HttpResponse> {
    let message = MainService::health_check().await;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": message
    })))
}
