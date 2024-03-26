use clap::{Arg, Command};
use http::Method;
use metadata_data_layer_utils::PoolState;
use metadata_http::{init_router, AppState};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{self, CorsLayer},
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn cli() -> Command {
    Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("postgres_host")
                .long("postgres-host")
                .env("POSTGRES_HOST")
                .help("The hostname of the PostgreSQL database to use")
                .default_value("localhost")
        )
        .arg(
            Arg::new("postgres_port")
                .long("postgres-port")
                .env("POSTGRES_PORT")
                .value_parser(clap::value_parser!(u16))
                .help("The port to use to connect with the PostgreSQL database")
                .default_value("5432")
        )
        .arg(
            Arg::new("postgres_user")
                .long("postgres-user")
                .env("POSTGRES_USER")
                .required(true)
                .help("The username to use for the authentication to the PostgreSQL database")
        )
        .arg(
            Arg::new("postgres_password")
                .long("postgres-password")
                .env("POSTGRES_PASSWORD")
                .required(true)
                .help("The password to use for the authentication to the PostgreSQL database")
        )
        .arg(
            Arg::new("postgres_database")
                .long("postgres-database")
                .env("POSTGRES_DATABASE")
                .required(true)
                .help("The database to use once the connection is established with the PostgreSQL database"),
        )
}

#[tokio::main]
pub(crate) async fn main() {
    let args = cli().get_matches();

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

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
