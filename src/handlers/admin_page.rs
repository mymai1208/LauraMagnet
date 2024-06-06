use std::net::SocketAddr;

use askama::Template;
use axum::{
    extract::{ConnectInfo, Request},
    response::IntoResponse,
    routing::get
};

use crate::{
    server::get_ip, structs::{AdminPage, HtmlTemplate}, traits::HandlerTrait, IS_USE_CLOUDFLARE
};

#[async_trait::async_trait]
impl HandlerTrait for AdminPage {
    async fn setup(&self, router_addr: &mut axum::Router, router: axum::Router) {
        *router_addr = router.route("/admin/login", get(admin_handler));
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

    let ip = get_ip(Some(request), Some(addr));

    println!("Request from: {}", ip.unwrap_or("aa".to_string()));

    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "admin1-login.html")]
pub struct AdminPageTemplate {}
