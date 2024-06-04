use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{
    structs::Server,
    traits::{HandlerTrait, ServerTrait},
};

#[async_trait::async_trait]
impl ServerTrait for Server {
    async fn init_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        let router = {
            let mut router = Router::new();

            for handler in &self.pages {
                let cloned = router.clone();

                handler.setup(&mut router, cloned).await;
            }

            router
        };

        let app = router.layer(TraceLayer::new_for_http());

        let tcp_listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 80))).await?;

        axum::serve(
            tcp_listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

        Ok(())
    }

    fn add_page(&mut self, page: Box<dyn HandlerTrait + Send + Sync>) {
        self.pages.push(page);
    }
}

impl Server {
    pub fn new(is_use_cloudflare: bool) -> Self {
        Self {
            is_use_cloudflare,
            pages: Vec::new(),
        }
    }
}