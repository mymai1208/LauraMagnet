use std::{net::SocketAddr, vec};

use axum::{
    extract::{ConnectInfo, Request},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{
    structs::{AdminPage, HtmlTemplate, IndexPageTemplate},
    traits::HandlerTrait,
};

pub async fn init_server() {
    let handlers: Vec<Box<dyn HandlerTrait>> = vec![Box::new(AdminPage::new())];

    let router = {
        let mut router = Router::new();

        for handler in handlers {
            handler.setup(&mut router).await;
        }

        router
    };

    let app = router
        .route("/", get(index_handler))
        .layer(TraceLayer::new_for_http());

    let tcp_listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 80)))
        .await
        .unwrap();

    axum::serve(
        tcp_listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn index_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
) -> impl IntoResponse {
    let template = IndexPageTemplate {};

    println!("Request from: {}", addr.ip());

    HtmlTemplate(template)
}
