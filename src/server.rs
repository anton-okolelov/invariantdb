use crate::config::Config;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use log::{info, error};

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Server {
        Server { config: config }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        info!("Listening port {}", self.config.port);
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.config.port)).await?;

        loop {
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            error!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        error!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    }
}
