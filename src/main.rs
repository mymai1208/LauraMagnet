use std::path;

use structs::{AdminPage, IndexPage, Server};
use tracing_subscriber::layer::SubscriberExt;
use traits::ServerTrait;

mod analyzer;
mod handlers;
mod response;
mod server;
mod structs;
mod traits;

const IS_USE_CLOUDFLARE: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new("./logs").exists() {
        std::fs::create_dir("./logs")?;
    }

    let file_appender = tracing_appender::rolling::daily("./logs", "access.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_writer(std::io::stdout)
        .init();

    let mut server = Server::new(IS_USE_CLOUDFLARE);

    server.add_page(Box::new(AdminPage::new()));
    server.add_page(Box::new(IndexPage::new()));

    server.init_server().await?;

    Ok(())
}
