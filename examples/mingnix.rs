use std::sync::Arc;

use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer};

struct Config {
    listener_addr: String,
    upstream_addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let layer = fmt::layer().pretty().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = resolve_config();
    let config = Arc::new(config);
    let listener = TcpListener::bind(&config.listener_addr).await?;

    loop {
        let (client, addr) = listener.accept().await?;
        info!("Accept connection from {}", addr);
        let config_cloned = Arc::clone(&config);
        tokio::spawn(async move {
            let upstream = TcpStream::connect(&config_cloned.upstream_addr).await?;
            proxy(client, upstream).await?;
            Ok::<(), anyhow::Error>(())
        });
    }
}

async fn proxy(mut client: TcpStream, mut upstream: TcpStream) -> Result<()> {
    let (mut client_reader, mut client_writer) = client.split();
    let (mut upstream_reader, mut upstream_writer) = upstream.split();
    let client_to_upstream = tokio::io::copy(&mut client_reader, &mut upstream_writer);
    let upstream_to_client = tokio::io::copy(&mut upstream_reader, &mut client_writer);

    if let Err(e) = tokio::try_join!(client_to_upstream, upstream_to_client) {
        warn!("Error during proxy: {}", e);
    }

    Ok(())
}

fn resolve_config() -> Config {
    Config {
        listener_addr: "0.0.0.0:8081".to_string(),
        upstream_addr: "0.0.0.0:8080".to_string(),
    }
}
