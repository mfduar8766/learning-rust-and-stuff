use axum::http::Error;
use dotenv::dotenv;
use std::{process, sync::Mutex};
use tokio;
use tracing::{error, info};
mod config;
mod db;
mod handlers;
mod json_payload;
mod logger;
mod renderers;
mod router;
mod state;
mod types;
mod utils;
mod views;
use once_cell::sync::Lazy;

static CONFIG: Lazy<Mutex<config::Config>> = Lazy::new(|| {
    return Mutex::new(config::Config::new());
});

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let db_instance = match db::create_db("db.json").await {
        Ok(db) => db,
        Err(e) => {
            error!("error connection to DB exitig program: {}", e);
            process::exit(1);
        }
    };
    let app = router::create_router(db_instance);
    let listener = tokio::net::TcpListener::bind(CONFIG.lock().unwrap().addr.as_str())
        .await
        .unwrap();
    info!("Listening on port {}...\n", listener.local_addr().unwrap());
    let result = axum::serve(listener, app.into_make_service()).await;
    match result {
        Ok(()) => {
            info!("Listening on: {}...\n", CONFIG.lock().unwrap().url);
            return Ok(());
        }
        Err(err) => {
            error!("error axum::serve():{}", err);
            std::process::exit(1);
        }
    }
}
