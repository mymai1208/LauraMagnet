use axum::Router;

#[async_trait::async_trait]
pub trait HandlerTrait {
    async fn setup(&self, router: &mut axum::Router);
}

#[async_trait::async_trait]
pub trait ServerTrait {
    async fn init_server(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn add_page(&mut self, page: Box<dyn HandlerTrait + Send + Sync>);
}