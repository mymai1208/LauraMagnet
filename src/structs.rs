use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexPageTemplate {
    
}

pub struct AdminPage {

}

pub struct HtmlTemplate<T>(pub T);
