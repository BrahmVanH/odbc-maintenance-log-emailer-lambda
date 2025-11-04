use std::env;

use axum::{ extract::Extension, http::{ HeaderValue, Method }, Router };
use dotenvy::dotenv;
use opendal::Operator;
use tower::ServiceBuilder;
use tower_http::{ compression::CompressionLayer, cors::{ Any, CorsLayer } };
use tracing::{ info, error };

use maintenance_sheet_emailer::{
    config::Config,
    dropbox::create_dbx_operator,
    job_scheduler::init_job_scheduler,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let dsn = env::var("SENTRY_DSN").unwrap_or_else(|_| "".to_string());
    let _guard = sentry::init((
        dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            send_default_pii: true,
            ..Default::default()
        },
    ));

    tracing_subscriber
        ::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .init();

    info!("starting up...");

    let config = Config::from_env().unwrap_or_else(|e| {
        error!("Failed to load configuration, using defaults: {}", e);
        Config::default()
    });

    info!("Configuration loaded: {:?}", config);

    // Production mode - restrict origins (you'd configure this based on your needs)
    let allowed_origins: Vec<String> = config
        .clone()
        .allow_origins.split(",")
        .map(|s| s.to_string())
        .collect();

    let mut cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    for origin in &allowed_origins {
        let header_value = origin
            .parse::<HeaderValue>()
            .map_err(|e| {
                error!("Could not configure cors layer: {:?}", e);
                std::process::exit(1);
            })
            .unwrap();
        cors_layer = cors_layer.allow_origin(header_value);
    }

    let router = Router::new();

    // Add middleware layers
    let app = router.layer(
        ServiceBuilder::new()
            .layer(CompressionLayer::new().gzip(true).deflate(true).br(true))
            .layer(Extension(config.clone()))
            .layer(cors_layer)
    );

    let port = env
        ::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let bind_address = format!("0.0.0.0:{}", port);

    let listener = match tokio::net::TcpListener::bind(&bind_address).await {
        Ok(l) => l,
        Err(e) => {
            error!("Fatal error binding to {}: {}", bind_address, e);
            std::process::exit(1);
        }
    };

    info!("Server running on http://localhost:{}", port);

    // Create Dropbox operator before starting server
    let dbx: Operator = create_dbx_operator(&config.dropbox).await
        .map_err(|e| {
            error!("error in creating dbx operator: {}", e);
            e
        })
        .unwrap_or_else(|e| {
            error!("could not create dbx operator: {:?}", e);
            std::process::exit(1);
        });

    if let Ok(sched) = init_job_scheduler(&dbx, &config.lettre).await {
        if let Err(e) = sched.start().await {
            error!("Failed to start job scheduler: {:?}", e);
            std::process::exit(1);
        }
    } else {
        error!("could not initiate job scheduler");
        std::process::exit(1);
    }

    if let Err(e) = axum::serve(listener, app).await {
        error!("Fatal error running server: {}", e);
        std::process::exit(1);
    }
}
