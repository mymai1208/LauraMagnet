use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::Request,
    Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{warn, Span};

use crate::{
    structs::{Analyzer, Server},
    traits::{HandlerTrait, ServerTrait},
    IS_USE_CLOUDFLARE,
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

        let app = router.layer(
            ServiceBuilder::new().layer(TraceLayer::new_for_http().on_request(
                |request: &Request<Body>, _span: &Span| {
                    on_request(request);
                },
            )),
        );

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

fn on_request(request: &Request<Body>) {
    let result = Analyzer::global().analyze(request.uri().clone());

    if result.is_err() {
        warn!("Failed to analyze request: {:?}", result.err().unwrap());
    }
}

pub fn get_ip(
    request: Option<Request>,
    address: Option<SocketAddr>,
) -> Result<String, Box<dyn std::error::Error>> {
    return if IS_USE_CLOUDFLARE {
        if request.is_none() {
            return Err("Request is None".into());
        }

        let request = request.unwrap();
        let header = request.headers().get("CF-Connecting-IP");

        if header.is_none() {
            return Err("CF-Connecting-IP is None".into());
        }

        Ok(header.unwrap().to_str().unwrap().to_string())
    } else {
        if address.is_none() {
            return Err("Address is None".into());
        }

        Ok(address.unwrap().ip().to_string())
    };
}
