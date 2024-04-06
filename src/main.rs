use axum::{
    http::{
        // header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Error,
        HeaderValue,
        Method,
    },
    routing::{delete, get, post},
    Router,
};
use std::sync::{Arc, Mutex};
use tokio;
use tower_http::services::ServeDir;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
mod db;
mod handlers;
mod json_payload;
mod logger;
mod renderers;
mod state;
mod todos;
mod types;
mod utils;
mod views;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let log = logger::Logger::new("rust-app");
    // log.log_infof(logger::LogMessage::new("FOO", "BAR"))
    //     .unwrap();
    // log.log_infof(logger::LogMessage::new("", "BAR")).unwrap();
    // log.log_infof(&logger::LogMessage::new("FOO", "")).unwrap();
    // log.log_info("NO OBJECT JUST MESSAGE");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(false)
        .allow_headers(Any);
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(renderers::renderers::render_index))
        .route("/views/auth", get(renderers::renderers::auth))
        // .route("/api/v1/login", post(handlers::handlers::handle_login))
        .route("/api/v1/todos", get(handlers::handlers::get_todos))
        .route("/api/v1/healthcheck", get(handlers::handlers::health_check))
        .route("/api/v1/add/todos", post(handlers::handlers::add_todos))
        .route(
            "/api/v1/change-state",
            post(handlers::handlers::change_state),
        )
        .route(
            "/api/v1/delete/todos/:id",
            delete(handlers::handlers::delete_todo).patch(handlers::handlers::update_todo),
        )
        .with_state(Arc::new(Mutex::new(state::init_state())))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .nest_service("/assets", ServeDir::new("../dist"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("Listening on port {}...\n", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
