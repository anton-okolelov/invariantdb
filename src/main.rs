mod config;
mod server;

extern crate tokio;

use crate::config::Config;
use crate::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    let server = Server::new(config);
    server.run().await
}
