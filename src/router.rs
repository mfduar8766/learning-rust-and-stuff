use crate::{db, handlers, renderers, state, CONFIG};
use axum::{
    routing::{delete, get, post},
    Router,
};
use mongodb::Database;
use std::sync::{Arc, Mutex};
use tower_http::{services::ServeDir, trace::TraceLayer};

pub fn create_router(db_instance: Option<Database>) -> Router {
    let conf = CONFIG.lock().unwrap();
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
        // .route(
        //     &format!("{}/add/todos", conf.api_version_url_prefix),
        //     post(handlers::add_todos),
        // )
        .route(
            &format!("{}/change-state", conf.api_version_url_prefix),
            post(handlers::change_state),
        )
        .route(
            &format!("{}/delete/todos/:id", conf.api_version_url_prefix),
            delete(handlers::delete_todo).patch(handlers::update_todo),
        )
        .with_state(Arc::new(Mutex::new(state::ApplicationState::new(
            db::Db::new(db_instance),
        ))))
        .layer(conf.create_cors())
        .layer(TraceLayer::new_for_http())
        .nest_service("/assets", ServeDir::new("../dist"));
    return app;
}
