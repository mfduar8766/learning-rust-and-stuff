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
use tracing_subscriber;

static CONFIG: Lazy<Mutex<config::Config>> = Lazy::new(|| {
    return Mutex::new(config::Config::new());
});

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let conf = CONFIG.lock().unwrap();
    let db_instane = match db::connect_to_db().await {
        Ok(db) => db,
        Err(e) => {
            error!("error connection to DB exitig program: {}", e);
            process::exit(1);
        }
    };
    let app = router::create_router(db_instane);
    let listener = tokio::net::TcpListener::bind(&conf.addr).await.unwrap();
    info!("Listening on port {}...\n", listener.local_addr().unwrap());
    match axum::serve(listener, app.into_make_service()).await {
        Ok(()) => {
            info!("Listening on: {}...\n", CONFIG.lock().unwrap().url);
            return Ok(());
        }
        Err(err) => {
            error!("error axum::serve():{}", err);
            std::process::exit(1);
        }
    };
}
