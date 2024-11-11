use anyhow::Result;
use axum::{routing::get, Router};
use opentelemetry::{trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    runtime,
    trace::{self, RandomIdGenerator, TracerProvider},
    Resource,
};
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

    // opentelemetry tracing layer for tracing-subscriber
    let provider = init_tracer()?;
    let tracer = provider.tracer("ecosystem");
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(console)
        .with(file)
        .with(opentelemetry)
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

fn init_tracer() -> Result<TracerProvider> {
    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            trace::Config::default()
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(32)
                .with_max_attributes_per_span(64)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "axum-tracing",
                )])),
        )
        .install_batch(runtime::Tokio)?;

    Ok(tracer_provider)
}
