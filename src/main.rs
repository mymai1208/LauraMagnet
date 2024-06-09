use std::{fmt::Debug, path};

use structs::{AdminPage, IndexPage, Server};
use tracing::{field::Field, Level};
use tracing_subscriber::{
    fmt::{format, writer::MakeWriterExt, FormatFields, FormattedFields},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
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

    let mut server = Server::new(IS_USE_CLOUDFLARE);

    server.add_page(Box::new(AdminPage::new()));
    server.add_page(Box::new(IndexPage::new()));

    server.init_server().await?;

    Ok(())
}
