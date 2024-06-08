use std::net::SocketAddr;

use askama::Template;
use axum::{
    extract::{ConnectInfo, Request},
    response::IntoResponse,
    routing::get
};
use tracing::info;

use crate::{
    server::get_ip, structs::{AdminPage, HtmlTemplate}, traits::HandlerTrait
};

#[async_trait::async_trait]
impl HandlerTrait for AdminPage {
    async fn setup(&self, router_ptr: &mut axum::Router, router: axum::Router) {
        *router_ptr = router.route("/admin/login", get(admin_handler));
    }
}

impl AdminPage {
    pub fn new() -> Self {
        Self { }
    }
}

async fn admin_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
) -> impl IntoResponse {
    let template = AdminPageTemplate {};

    let ip = get_ip(Some(&request), Some(&addr));

    info!("Request from: {}", ip.unwrap_or("aa".to_string()));

    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "admin1-login.html")]
pub struct AdminPageTemplate {}
