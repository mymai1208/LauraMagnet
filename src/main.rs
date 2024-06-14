use structs::{AdminPage, IndexPage, Server};
use tracing::Level;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use traits::ServerTrait;

mod http;
mod structs;
mod traits;
mod utils;

const IS_USE_CLOUDFLARE: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new("./logs").exists() {
        std::fs::create_dir("./logs")?;
    }

    let file_appender = tracing_appender::rolling::daily("./logs", "access.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout.with_max_level(Level::INFO))
                .pretty()
                .compact(),
        )
        .init();

    let mut http_server = Server::new(IS_USE_CLOUDFLARE);

    http_server.add_page(Box::new(AdminPage::new()));
    http_server.add_page(Box::new(IndexPage::new()));

    http_server.init_server().await?;

    Ok(())
}
