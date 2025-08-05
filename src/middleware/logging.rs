use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform, Payload},
    Error, HttpMessage, web::Bytes,
};
use futures_util::future::LocalBoxFuture;
use slog::{info, warn, Logger};
use std::{
    future::{ready, Ready},
    rc::Rc,
    time::Instant,
};
use futures_util::StreamExt;

pub struct SlogLoggingMiddleware {
    logger: Logger,
    log_request_body: bool,
    max_body_size: usize,
}

impl SlogLoggingMiddleware {
    pub fn new(logger: Logger) -> Self {
        Self {
            logger,
            log_request_body: false,
            max_body_size: 1024, // 1KB default limit
        }
    }

    pub fn with_request_body_logging(mut self, enabled: bool) -> Self {
        self.log_request_body = enabled;
        self
    }

    pub fn with_max_body_size(mut self, size: usize) -> Self {
        self.max_body_size = size;
        self
    }
}

impl<S, B> Transform<S, ServiceRequest> for SlogLoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SlogLoggingMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SlogLoggingMiddlewareService {
            service: Rc::new(service),
            logger: self.logger.clone(),
            log_request_body: self.log_request_body,
            max_body_size: self.max_body_size,
        }))
    }
}

pub struct SlogLoggingMiddlewareService<S> {
    service: Rc<S>,
    logger: Logger,
    log_request_body: bool,
    max_body_size: usize,
}

impl<S, B> Service<ServiceRequest> for SlogLoggingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let logger = self.logger.clone();
        let start_time = Instant::now();
        let log_request_body = self.log_request_body;
        let max_body_size = self.max_body_size;

        // 요청 정보 수집
        let method = req.method().to_string();
        let uri = req.uri().to_string();
        let user_agent = req
            .headers()
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();
        let remote_addr = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_string();
        let content_length = req
            .headers()
            .get("content-length")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        Box::pin(async move {
            // 요청 Body 읽기 (필요한 경우)
            let request_body = if log_request_body && content_length > 0 && content_length <= max_body_size as u64 {
                let mut payload = req.take_payload();
                let mut body_bytes = Vec::new();

                while let Some(chunk) = payload.next().await {
                    match chunk {
                        Ok(bytes) => {
                            body_bytes.extend_from_slice(&bytes);
                            if body_bytes.len() > max_body_size {
                                body_bytes.truncate(max_body_size);
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }

                // 읽은 body를 다시 payload로 설정
                let body_clone = body_bytes.clone();
                req.set_payload(Payload::from(Bytes::from(body_clone)));

                Some(String::from_utf8_lossy(&body_bytes).to_string())
            } else {
                None
            };

            // 요청 시작 로깅 (비동기)
            tokio::spawn({
                let logger = logger.clone();
                let method = method.clone();
                let uri = uri.clone();
                let remote_addr = remote_addr.clone();
                let user_agent = user_agent.clone();
                let request_body = request_body.clone();

                async move {
                    if let Some(body) = request_body {
                        info!(logger, "Request started with body";
                            "method" => method,
                            "uri" => uri,
                            "remote_addr" => remote_addr,
                            "user_agent" => user_agent,
                            "content_length" => content_length,
                            "request_body" => body,
                            "timestamp" => chrono::Utc::now().to_rfc3339()
                        );
                    } else {
                        info!(logger, "Request started";
                            "method" => method,
                            "uri" => uri,
                            "remote_addr" => remote_addr,
                            "user_agent" => user_agent,
                            "content_length" => content_length,
                            "timestamp" => chrono::Utc::now().to_rfc3339()
                        );
                    }
                }
            });

            let res = service.call(req).await?;

            let duration = start_time.elapsed();
            let status = res.status().as_u16();
            let response_size = res
                .headers()
                .get("content-length")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);

            // 응답 완료 로깅 (비동기)
            tokio::spawn({
                let logger = logger.clone();
                let method = method.clone();
                let uri = uri.clone();

                async move {
                    info!(logger, "Request completed";
                        "method" => &method,
                        "uri" => &uri,
                        "status" => status,
                        "duration_ms" => duration.as_millis() as u64,
                        "response_size" => response_size,
                        "timestamp" => chrono::Utc::now().to_rfc3339()
                    );

                    // 에러나 느린 요청에 대한 추가 로깅
                    if status >= 400 {
                        warn!(logger, "Error response detected";
                            "method" => &method,
                            "uri" => &uri,
                            "status" => status,
                            "duration_ms" => duration.as_millis() as u64
                        );
                    } else if duration.as_millis() > 1000 {
                        warn!(logger, "Slow request detected";
                            "method" => &method,
                            "uri" => &uri,
                            "status" => status,
                            "duration_ms" => duration.as_millis() as u64
                        );
                    }
                }
            });

            Ok(res)
        })
    }
}
