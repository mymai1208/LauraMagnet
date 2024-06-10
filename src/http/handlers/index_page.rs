use std::net::SocketAddr;

use askama::Template;
use axum::{
    extract::{ConnectInfo, Request},
    response::IntoResponse,
    routing::get,
};

use crate::{
    structs::{HtmlTemplate, IndexPage},
    traits::HandlerTrait,
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexPageTemplate {}

#[async_trait::async_trait]
impl HandlerTrait for IndexPage {
    async fn setup(&self, router_ptr: &mut axum::Router, router: axum::Router) {
        *router_ptr = router.route("/", get(index_handler));
    }
}

impl IndexPage {
    pub fn new() -> Self {
        Self { }
    }
}

pub async fn index_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
) -> impl IntoResponse {
    let template = IndexPageTemplate {};

    HtmlTemplate(template)
}
