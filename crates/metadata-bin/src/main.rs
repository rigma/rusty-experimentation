use axum::{routing::get, Router};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder().with_default_directive(LevelFilter::TRACE.into()).from_env_lossy()
        )
        .with(
            tracing_subscriber::fmt::layer()
            .map_event_format(|format| format.json())
        )
        .init();

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
