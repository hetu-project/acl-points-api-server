use crate::handlers::{auth_token, callback_handler, get_user_info, healthcheck, serve_index};
use crate::middlewares;
use crate::server;
use async_trait::async_trait;
use axum::{
    error_handling::HandleErrorLayer,
    extract::Extension,
    http::Method,
    middleware,
    routing::{get, post},
    Router,
};
use clap::{Arg, ArgMatches, Command};
use cli::CommandHandler;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub struct RunCommand;

#[async_trait]
impl CommandHandler for RunCommand {
    fn name(&self) -> String {
        "run".to_string()
    }

    fn define(&self) -> Command {
        Command::new("run").about("run app").arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_parser(clap::value_parser!(String))
                .help("config file path"),
        )
    }

    async fn run(&self, matches: &ArgMatches) {
        let config_file = matches.get_one::<String>("config").unwrap();
        let share_state = server::app::SharedState::new(config_file.clone().into()).await;

        let auth_router = Router::new()
            .route("/api/v1/userinfo", get(get_user_info))
            .layer(middleware::from_fn(middlewares::auth_middleware))
            .layer(Extension(share_state.clone()));

        let app = Router::new()
            .route("/", get(serve_index))
            .route("/api/v1/ping", get(|| async { "pong" }))
            .route("/api/v1/healthcheck", get(healthcheck))
            .route("/auth/callback", get(callback_handler))
            .route("/auth/token", post(auth_token))
            .nest("/", auth_router)
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::GET, Method::POST])
                    .allow_headers(Any),
            )
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(middlewares::handle_error))
                    .timeout(Duration::from_secs(600))
                    .layer(
                        TraceLayer::new_for_http()
                            .on_request(
                                |request: &axum::http::Request<axum::body::Body>,
                                 _: &tracing::span::Span| {
                                    let method = request.method();
                                    let uri = request.uri();
                                    tracing::info!("Received request: {} {}", method, uri);
                                },
                            )
                            .on_response(
                                |response: &axum::http::Response<axum::body::Body>,
                                 latency: Duration,
                                 _: &tracing::span::Span| {
                                    let status = response.status();
                                    tracing::info!(
                                        "Sending response: {} with status: {}",
                                        latency.as_secs_f64(),
                                        status
                                    );
                                },
                            ),
                    ),
            )
            .with_state(share_state.clone());

        let addr = format!(
            "{}:{}",
            share_state.0.read().await.config.server.host,
            share_state.0.read().await.config.server.port
        );
        tracing::info!("Server is running on {}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
