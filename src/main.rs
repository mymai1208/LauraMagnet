use structs::{AdminPage, IndexPage, Server};
use traits::{ServerTrait};

mod handlers;
mod response;
mod server;
mod structs;
mod traits;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut server = Server::new(false);

    server.add_page(Box::new(AdminPage::new()));
    server.add_page(Box::new(IndexPage::new()));

    server.init_server().await?;

    Ok(())
}
