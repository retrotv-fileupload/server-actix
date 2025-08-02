pub struct MainService;

impl MainService {
    pub async fn health_check() -> &'static str {
        "This is working!"
    }
}

