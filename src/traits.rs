#[async_trait::async_trait]
pub trait HandlerTrait {
    async fn setup(&self, router: &mut axum::Router);
}