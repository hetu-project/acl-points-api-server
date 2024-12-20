use crate::handlers::auth_token;
use crate::handlers::callback_handler;
use crate::handlers::healthcheck;
use crate::jwt::jwt_auth;
use crate::server;
use async_trait::async_trait;
use axum::{
    error_handling::HandleErrorLayer,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    BoxError, Router,
};
use clap::{Arg, ArgMatches, Command};
use cli::CommandHandler;
use std::{borrow::Cow, time::Duration};
use tower::ServiceBuilder;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
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
        //let configs = config::Config::load_config(config_file.clone().into()).unwrap();
        let share_state = server::app::SharedState::new(config_file.clone().into()).await;

        let auth = jwt_auth::Authorization {
            jwt_handler: share_state.0.read().await.jwt_handler.clone(),
        };

        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers(Any);

        let auth_router = Router::new()
            .route("/api/v1/ping", get(|| async { "pong" }))
            .layer(ValidateRequestHeaderLayer::custom(auth));

        let app = Router::new()
            .route("/", get(serve_index))
            .route("/api/v1/userinfo", get(healthcheck))
            .route("/api/v1/healthcheck", get(healthcheck))
            .route("/auth/callback", get(callback_handler))
            .route("/auth/token", post(auth_token))
            .nest("/", auth_router)
            .layer(cors)
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(handle_error))
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

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}

#[axum::debug_handler]
async fn serve_index() -> axum::response::Html<String> {
    let content = std::fs::read_to_string("static/index.html").expect("Failed to read index.html");
    axum::response::Html(content)
}
