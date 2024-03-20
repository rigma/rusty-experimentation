use metadata_data_layer_utils::PoolState;
use metadata_http::{init_router, AppState};
use http::Method;
use tokio::net::TcpListener;
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer}};

pub(crate) async fn serve() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new(PoolState::from_env().finalize());
    let app: axum::Router = init_router(state)
        .layer(
             CorsLayer::new()
                .allow_origin(["http://localhost:8080".parse().unwrap()])
                .allow_methods([Method::GET, Method::HEAD, Method::OPTIONS])
                .allow_credentials(true)
        )
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(
                    TraceLayer::new_for_http()
                        .on_request(DefaultOnRequest::new().level(Level::TRACE))
                        .on_response(DefaultOnResponse::new().level(Level::TRACE))
                )
        );

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
