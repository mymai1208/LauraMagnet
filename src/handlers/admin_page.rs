use std::net::SocketAddr;

use askama::Template;
use axum::{extract::{ConnectInfo, Request}, response::IntoResponse, routing::get};

use crate::{structs::{AdminPage, HtmlTemplate}, traits::HandlerTrait};

#[async_trait::async_trait]
impl HandlerTrait for AdminPage {
    async fn setup(&self, router: &mut axum::Router) {
        *router = router.clone().route("/admin/login", get(admin_handler));
    }
}

impl AdminPage {
    pub fn new() -> Self {
        Self {}
    }
}

async fn admin_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
) -> impl IntoResponse {
    let template = AdminPageTemplate {};

    println!("Request from: {}", addr.ip());

    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "admin1-login.html")]
pub struct AdminPageTemplate {
    
}