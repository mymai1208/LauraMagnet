use crate::server::init_server;

mod server;
mod structs;
mod response;
mod traits;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    init_server().await;
    
    println!("Hello, world!");
}
