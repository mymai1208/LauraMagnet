use askama::Template;
use axum::{
    response::{Html, IntoResponse, Response},
};

use crate::structs::HtmlTemplate;

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        let body = self.0.render().unwrap();
        
        Html(body).into_response()
    }
}
