use metadata_data_layer_utils::PoolState;
use metadata_http::{init_router, AppState};
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer().map_event_format(|format| format.json()))
        .init();

    let state = AppState::new(PoolState::from_env().finalize());
    let app = init_router(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
