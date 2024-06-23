use axum::http::Error;
use config::Config;
use dotenv::dotenv;
use std::process;
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
// use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use tracing_subscriber;

// static CONFIG: Lazy<F> = Lazy::new(|| {
//     return F(Mutex::new(Config::new()));
// });

static CONFIG: OnceCell<Config> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    CONFIG.set(Config::new()).unwrap();
    let postgres_pool = match db::connect_to_db().await {
        Ok(db) => db,
        Err(e) => {
            error!("main()::error connection to DB exitig program: {}", e);
            process::exit(1);
        }
    };
    let c = CONFIG.get().unwrap();
    let app = router::create_router(postgres_pool);
    let listener = match tokio::net::TcpListener::bind(c.addr.as_str()).await {
        Ok(v) => v,
        Err(err) => {
            error!(
                "Main::main()::error creating TCT connection exiting program.... {}",
                err
            );
            process::exit(1);
        }
    };
    info!("Listening on {}...\n", listener.local_addr().unwrap());
    match axum::serve(listener, app.into_make_service()).await {
        Ok(()) => {
            info!("Listening on: {:?}...\n", "url");
            return Ok(());
        }
        Err(err) => {
            error!("error axum::serve():{}", err);
            std::process::exit(1);
        }
    }
}
