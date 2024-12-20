use crate::handlers::auth_token;
use crate::handlers::callback_handler;
use crate::handlers::get_user_info;
use crate::handlers::healthcheck;
use crate::jwt::jwt_auth;
use crate::jwt::jwt_handler;
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
            .route("/api/v1/userinfo", get(handler))
            .route_layer(axum::middleware::from_fn(auth_test))
            .layer(ValidateRequestHeaderLayer::custom(auth));

        let app = Router::new()
            .route("/", get(serve_index))
            .route("/api/v1/ping", get(|| async { "pong" }))
            .route("/api/v1/userinfo2", get(get_user_info))
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

use tokio::task_local;

task_local! {
    pub static USER: jwt_handler::Claims;
}

async fn auth_test(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    println!("---erer_--{:?}", auth_header);

    let jwtt = jwt_handler::JwtHandler {
        secret: "my_secret_key".to_string(),
    };

    if let Ok(current_user) = jwtt.decode_token(
        auth_header
            .to_string()
            .strip_prefix("Bearer ")
            .unwrap_or(&auth_header)
            .to_string(),
    ) {
        // State is setup here in the middleware
        Ok(USER.scope(current_user, next.run(req)).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

struct UserResponse;

impl IntoResponse for UserResponse {
    fn into_response(self) -> axum::response::Response {
        // State is accessed here in the IntoResponse implementation
        let current_user = USER.with(|u| u.clone());
        (
            StatusCode::OK,
            axum::Json(serde_json::json!({"name": current_user.name,"email": current_user.email})),
        )
            .into_response()
    }
}

async fn handler() -> UserResponse {
    UserResponse
}
