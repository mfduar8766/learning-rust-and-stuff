use axum::http::{
    header::{ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE},
    HeaderName, HeaderValue, Method,
};
use std::env;
use tower_http::cors::{AllowHeaders, CorsLayer};

#[derive(Debug)]
pub struct Envs {
    pub api_version: String,
    pub api_port: String,
    pub api_host: String,
    pub db_collection: String,
    pub db_name: String,
    pub mongo_url: String,
    pub max_db_connection_retries: i32,
    pub use_db: bool,
}

#[allow(dead_code)]
pub struct Config {
    pub addr: String,
    /* /api/v1 */
    pub api_version_url_prefix: String,
    pub api_url: String,
    pub url: String,
    pub service_name: String,
    pub db_url: String,
    api_version: String,
    host: String,
    port: String,
    envs: Envs,
}

impl Envs {
    fn new() -> Self {
        let max_db_connection_env = env::var("MAX_DB_CONNECT_RETRIES").unwrap_or(5.to_string());
        let max_db_connection_retries = max_db_connection_env.parse::<i32>().unwrap();
        let use_db_env = env::var("USE_DB").unwrap_or("false".to_string());
        let use_db = use_db_env.parse::<bool>().unwrap();
        return Self {
            api_version: env::var("API_VERSION").unwrap_or("v1".to_string()),
            api_port: env::var("API_PORT").unwrap_or("3000".to_string()),
            api_host: env::var("API_HOST").unwrap_or("127.0.0.1".to_string()),
            db_collection: env::var("DB_COLLECTION").unwrap_or("users".to_string()),
            db_name: env::var("DB_NAME").unwrap_or("travel".to_string()),
            mongo_url: env::var("MONGODB_URL").unwrap_or("mongodb://localhost:27017".to_string()),
            max_db_connection_retries,
            use_db,
        };
    }
    fn get_envs(&self) -> &Envs {
        return self;
    }
}

impl Config {
    pub fn new() -> Self {
        let env = Envs::new();
        let host = &env.api_host;
        let port = &env.api_port;
        let api_version = &env.api_version;
        let api_version_url_prefix = &format!("/api/{}", api_version);
        let addr = &format!("{}:{}", host, port);
        let api_url = &format!("http://{}{}/", addr, api_version_url_prefix);
        let url = &format!("http://{}", addr);
        let db_url = &env.mongo_url;
        return Self {
            host: host.to_string(),
            port: port.to_string(),
            api_version: api_version.to_string(),
            api_version_url_prefix: api_version_url_prefix.to_string(),
            api_url: api_url.to_string(),
            addr: addr.to_string(),
            db_url: db_url.to_string(),
            service_name: String::from("rust-app"),
            url: url.to_string(),
            envs: env,
        };
    }
    pub fn get_envs(&self) -> &Envs {
        return self.envs.get_envs();
    }
    pub fn create_cors(&self) -> CorsLayer {
        let allowed_headers = AllowHeaders::list([
            HeaderName::from_static("hx-get"),
            HeaderName::from_static("hx-post"),
            HeaderName::from_static("hx-put"),
            HeaderName::from_static("hx-delete"),
            HeaderName::from_static("hx-target"),
            HeaderName::from_static("hx-current-url"),
            HeaderName::from_static("hx-request"),
            HeaderName::from_static("hx-redirect"),
            HeaderName::from_static("hx-location"),
            HeaderName::from_static("hx-trigger"),
            ACCESS_CONTROL_ALLOW_ORIGIN,
            CONTENT_TYPE,
            ACCEPT,
        ]);
        let cors = CorsLayer::new()
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
            .allow_credentials(false)
            .allow_headers(allowed_headers);
        return cors;
    }
}
