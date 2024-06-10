use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{connect_info, ConnectInfo, Request},
    Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{info_span, warn, Span};

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
            ServiceBuilder::new().layer(TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let connect_info: Option<&ConnectInfo<SocketAddr>> = request.extensions().get::<ConnectInfo<SocketAddr>>();

                let ip = if let Some(connect_info) = connect_info {
                    get_ip(Some(request), Some(&connect_info.0))
                } else {
                    get_ip(Some(request), None)
                }.unwrap_or("unknown".to_string());

                info_span!("Request", uri = %request.uri(), method = %request.method(), version = ?request.version(), ip = %ip)
            }).on_request(
                |request: &Request<Body>, _span: &Span| {
                    on_request(request);
                },
            )),
        );

        let tcp_listener = TcpListener::bind("0.0.0.0:80").await?;

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
    let result = Analyzer::global().analyze(request);

    if result.is_err() {
        warn!("Failed to analyze request: {:?}", result.err().unwrap());
    }
}

pub fn get_ip(
    request: Option<&Request>,
    address: Option<&SocketAddr>,
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
