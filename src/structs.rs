use std::sync::Arc;

use askama::Template;
use axum::Router;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexPageTemplate {
    
}

pub struct Handler {
    pub router: Arc<Router>
}

pub struct HtmlTemplate<T>(pub T);
