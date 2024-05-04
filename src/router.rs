use crate::{db, handlers, state, CONFIG};
use axum::{
    routing::{get, post},
    Router,
};
use mongodb::Database;
use std::sync::{Arc, Mutex};
use tower_http::{services::ServeDir, trace::TraceLayer};

pub fn create_router(db_instance: Option<Database>) -> Router {
    let conf = CONFIG.lock().unwrap();
    let app = Router::new()
        .route("/", get(handlers::index))
        .route(
            &format!("{}/login", conf.api_version_url_prefix),
            post(handlers::handle_login),
        )
        .route(
            &format!("{}/logout", conf.api_version_url_prefix),
            post(handlers::handle_logout),
        )
        .route(
            &format!("{}/healthcheck", conf.api_version_url_prefix),
            get(handlers::health_check),
        )
        .route(
            &format!("{}/change-state", conf.api_version_url_prefix),
            post(handlers::change_state),
        )
        .route(
            &format!("{}/image/:resource_id", conf.api_version_url_prefix),
            get(handlers::get_image),
        )
        .with_state(Arc::new(Mutex::new(state::ApplicationState::new(
            db::Db::new(db_instance),
        ))))
        .layer(conf.create_cors())
        .layer(TraceLayer::new_for_http())
        .nest_service("/assets", ServeDir::new("../dist"));
    return app;
}
