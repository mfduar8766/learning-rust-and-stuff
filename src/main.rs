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

// #[macro_use]
// extern crate lazy_static;

// lazy_static! {
//     static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
// }

// #[macro_use]
// extern crate lazy_static;

// lazy_static! {
//     pub static ref APP_CONFIG: Mutex<config::Config> = Mutex::new(config::Config::new());
// }

// pub fn get_app_config<'a>() -> MutexGuard<'a, config::Config> {
//     return APP_CONFIG.lock().unwrap();
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let log = logger::Logger::new("rust-app");
    // log.log_infof(logger::LogMessage::new("FOO", "BAR"))
    //     .unwrap();
    // log.log_infof(logger::LogMessage::new("", "BAR")).unwrap();
    // log.log_infof(&logger::LogMessage::new("FOO", "")).unwrap();
    // log.log_info("NO OBJECT JUST MESSAGE");
    let conf = config::Config::new();

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/views/auth", get(renderers::auth))
        .route(
            &format!("{}/login", conf.api_version_url_prefix),
            post(handlers::handle_login),
        )
        .route(
            &format!("{}/todos", conf.api_version_url_prefix),
            get(handlers::get_todos),
        )
        .route(
            &format!("{}/healthcheck", conf.api_version_url_prefix),
            get(handlers::health_check),
        )
        .route(
            &format!("{}/add/todos", conf.api_version_url_prefix),
            post(handlers::add_todos),
        )
        .route(
            &format!("{}/change-state", conf.api_version_url_prefix),
            post(handlers::change_state),
        )
        .route(
            &format!("{}/delete/todos/:id", conf.api_version_url_prefix),
            delete(handlers::delete_todo).patch(handlers::update_todo),
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
