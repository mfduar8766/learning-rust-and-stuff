use crate::{db, handlers, state, CONFIG};
use axum::{
    routing::{get, post},
    Router,
};
use futures::lock::Mutex;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tower_http::{services::ServeDir, trace::TraceLayer};

pub fn create_router(postgres_pool: Pool<Postgres>) -> Router {
    let c = CONFIG.get().unwrap();
    let mut db = db::Db::new();
    db.set_db(postgres_pool);
    let app = Router::new()
        .route("/", get(handlers::index))
        .route(
            &format!("{}/login", c.api_version_url_prefix),
            post(handlers::handle_login),
        )
        .route(
            &format!("{}/logout", c.api_version_url_prefix),
            post(handlers::handle_logout),
        )
        .route(
            &format!("{}/healthcheck", c.api_version_url_prefix),
            get(handlers::health_check),
        )
        .route(
            &format!("{}/change-state", c.api_version_url_prefix),
            post(handlers::change_state),
        )
        .route(
            &format!("{}/image/:resource_id", c.api_version_url_prefix),
            get(handlers::get_image),
        )
        .with_state(Arc::new(Mutex::new(state::ApplicationState::new(db))))
        .layer(c.create_cors())
        .layer(TraceLayer::new_for_http())
        .nest_service("/assets", ServeDir::new("../dist"));
    return app;
}
