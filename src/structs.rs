use crate::traits::HandlerTrait;

pub struct AdminPage {
    
}

pub struct IndexPage {
    
}

pub struct Server {
    pub is_use_cloudflare: bool,
    pub pages: Vec<Box<dyn HandlerTrait + Send + Sync>>,
}

pub struct Analyzer {

}

pub struct HtmlTemplate<T>(pub T);
