use crate::{db, handlers, renderers, state};
use axum::{
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        HeaderName, HeaderValue, Method,
    },
    routing::{delete, get, post},
    Router,
};
use mongodb::Database;
use std::sync::{Arc, Mutex};
use tower_http::{
    cors::{AllowHeaders, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};

#[allow(dead_code)]
pub struct Config {
    pub addr: String,
    /* /api/v1 */
    pub api_version_url_prefix: String,
    pub url: String,
    pub service_name: String,
    pub db_url: String,
    api_version: String,
    host: String,
    port: String,
}

impl Config {
    pub fn new() -> Self {
        let host = &String::from("localhost");
        let port = &String::from("3000");
        let api_version = "v1";
        let api_version_url_prefix = format!("/api/{}", api_version);
        let url = &format!("http://{}:{}", host, port);
        return Self {
            host: host.to_string(),
            port: port.to_string(),
            api_version: api_version.to_string(),
            api_version_url_prefix,
            url: url.to_string(),
            addr: format!("127.0.0.1:{}", port),
            db_url: String::from("mongodb://localhost:27017"),
            service_name: String::from("rust-app"),
        };
    }
    pub fn create_router(&self, db_instance: Database) -> axum::Router {
        let app = Router::new()
            .route("/", get(handlers::index))
            .route("/views/auth", get(renderers::auth))
            .route(
                &format!("{}/login", self.api_version_url_prefix),
                post(handlers::handle_login),
            )
            .route(
                &format!("{}/todos", self.api_version_url_prefix),
                get(handlers::get_todos),
            )
            .route(
                &format!("{}/healthcheck", self.api_version_url_prefix),
                get(handlers::health_check),
            )
            .route(
                &format!("{}/add/todos", self.api_version_url_prefix),
                post(handlers::add_todos),
            )
            .route(
                &format!("{}/change-state", self.api_version_url_prefix),
                post(handlers::change_state),
            )
            .route(
                &format!("{}/delete/todos/:id", self.api_version_url_prefix),
                delete(handlers::delete_todo).patch(handlers::update_todo),
            )
            .with_state(Arc::new(Mutex::new(state::ApplicationState::new(
                db::Db::new(db_instance),
            ))))
            .layer(self.create_cors())
            .layer(TraceLayer::new_for_http())
            .nest_service("/assets", ServeDir::new("../dist"));
        return app;
    }

    fn create_cors(&self) -> CorsLayer {
        let allowed_headers = AllowHeaders::list([
            HeaderName::from_static("hx-get"),
            HeaderName::from_static("hx-post"),
            HeaderName::from_static("hx-put"),
            HeaderName::from_static("hx-delete"),
            HeaderName::from_static("hx-target"),
            HeaderName::from_static("hx-current-url"),
            HeaderName::from_static("hx-request"),
            CONTENT_TYPE,
            ACCEPT,
        ]);
        let cors = CorsLayer::new()
            .allow_origin(self.url.parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
            .allow_credentials(false)
            .allow_headers(allowed_headers);
        return cors;
    }
}
