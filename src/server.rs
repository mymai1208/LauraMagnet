use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Request},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::
    trace::TraceLayer
;

use crate::structs::{HtmlTemplate, IndexPageTemplate};

pub async fn init_server() {
    let router = Router::new()
        .route("/", get(index_handler));

    let app = router.layer(TraceLayer::new_for_http());

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
