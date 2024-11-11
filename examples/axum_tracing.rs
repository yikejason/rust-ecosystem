use anyhow::Result;
use axum::{routing::get, Router};
use std::{net::SocketAddr, time::Duration};
use tokio::time::{sleep, Instant};
use tracing::{info, instrument, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

#[tokio::main]
async fn main() -> Result<()> {
    let file_appender = tracing_appender::rolling::hourly("/tmp/logs", "ecosystem.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let console = fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::INFO);

    let file = fmt::layer()
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::WARN);

    tracing_subscriber::registry()
        .with(console)
        .with(file)
        .init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let app = Router::new().route("/", get(index_handler));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Serving on {:?}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

#[instrument]
async fn index_handler() -> &'static str {
    sleep(Duration::from_millis(10)).await;
    let ret = long_task().await;
    info!(http.status = 200, "index_handler have already completed");
    ret
}

#[instrument]
async fn long_task() -> &'static str {
    let start = Instant::now();
    sleep(Duration::from_millis(112)).await;
    let elapsed = start.elapsed().as_millis() as u64;
    warn!(
        app.task_duration = elapsed,
        "long_task has already completed"
    );
    "Hello world!"
}
