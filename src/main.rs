use axum::{
    http::Error,
    routing::{delete, get, post},
    Router,
};
use std::sync::{Arc, Mutex};
use tokio;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{error, info};
mod config;
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
    let conf = config::Config::default();

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(renderers::renderers::render_index))
        .route("/views/auth", get(renderers::renderers::auth))
        .route(
            &format!("{}/login", conf.api_version_url_prefix),
            post(handlers::handlers::handle_login),
        )
        .route(
            &format!("{}/todos", conf.api_version_url_prefix),
            get(handlers::handlers::get_todos),
        )
        .route(
            &format!("{}/healthcheck", conf.api_version_url_prefix),
            get(handlers::handlers::health_check),
        )
        .route(
            &format!("{}/add/todos", conf.api_version_url_prefix),
            post(handlers::handlers::add_todos),
        )
        .route(
            &format!("{}/change-state", conf.api_version_url_prefix),
            post(handlers::handlers::change_state),
        )
        .route(
            &format!("{}/delete/todos/:id", conf.api_version_url_prefix),
            delete(handlers::handlers::delete_todo).patch(handlers::handlers::update_todo),
        )
        .with_state(Arc::new(Mutex::new(state::init_state())))
        .layer(conf.cors)
        .layer(TraceLayer::new_for_http())
        .nest_service("/assets", ServeDir::new("../dist"));

    let listener = tokio::net::TcpListener::bind(conf.addr).await.unwrap();
    info!("Listening on port {}...\n", listener.local_addr().unwrap());
    let result = axum::serve(listener, app.into_make_service()).await;
    match result {
        Ok(()) => {
            info!("Listening on: {}...\n", conf.url);
            return Ok(());
        }
        Err(err) => {
            error!("{}", err);
            std::process::exit(-1);
        }
    }
}
