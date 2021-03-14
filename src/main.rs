mod config;
mod server;

extern crate tokio;

use crate::config::Config;
use crate::server::Server;
use std::error::Error;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    env_logger::init();
    info!("Start Invariant DB");
    let config = Config::from_env()?;

    let server = Server::new(config);
    server.run().await
}
