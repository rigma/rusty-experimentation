use http::Method;
use metadata_data_layer_utils::PoolState;
use metadata_http::{init_router, AppState};
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{self, CorsLayer},
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
pub(super) async fn entrypoint(args: clap::ArgMatches) {
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new(
        PoolState::builder()
            .application_name(clap::crate_name!())
            .host(args.get_one::<String>("postgres_host").unwrap())
            .port(*args.get_one("postgres_port").unwrap())
            .user(args.get_one::<String>("postgres_user").unwrap())
            .password(args.get_one::<String>("postgres_password").unwrap())
            .dbname(args.get_one::<String>("postgres_database").unwrap())
            .finalize(),
    );
    let app = init_router(state)
        .layer(
            // TODO(rigma): CORS parameters should be configurable
            CorsLayer::new().allow_origin(cors::Any).allow_methods([
                Method::GET,
                Method::HEAD,
                Method::OPTIONS,
            ]),
        )
        .layer(
            ServiceBuilder::new().layer(CompressionLayer::new()).layer(
                TraceLayer::new_for_http()
                    .on_request(DefaultOnRequest::new().level(Level::TRACE))
                    .on_response(DefaultOnResponse::new().level(Level::TRACE)),
            ),
        );

    let listener = TcpListener::bind(SocketAddr::new(
        *args.get_one::<IpAddr>("host").unwrap(),
        *args.get_one("port").unwrap(),
    ))
    .await
    .unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
