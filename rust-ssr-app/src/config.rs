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
    pub db_users: String,
    pub db_itineary: String,
    pub db_name: String,
    pub db_url: String,
    pub max_db_connection_retries: i32,
    pub max_connections: u32,
    pub assets_location: String,
}

impl Default for Envs {
    fn default() -> Self {
        let max_db_connection_env = env::var("MAX_DB_CONNECT_RETRIES")
            .unwrap_or(5.to_string())
            .parse::<i32>()
            .unwrap();
        return Self {
            api_version: env::var("API_VERSION").unwrap_or("v1".to_string()),
            api_port: env::var("API_PORT").unwrap_or("3000".to_string()),
            api_host: env::var("API_HOST").unwrap_or("127.0.0.1".to_string()),
            db_users: env::var("DB_USERS").unwrap_or("users".to_string()),
            db_itineary: env::var("DB_ININEARY").unwrap_or("itineary".to_string()),
            db_name: env::var("DB_NAME").unwrap_or("travel".to_string()),
            db_url: env::var("DB_URL").unwrap_or("mongodb://localhost:27017".to_string()),
            max_db_connection_retries: max_db_connection_env,
            max_connections: env::var("MAX_CONNECTIONS")
                .unwrap_or(1.to_string())
                .parse::<u32>()
                .unwrap(),
            assets_location: env::var("ASSETS_LOCATION").unwrap_or("assets".to_string()),
        };
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    pub addr: String,
    /* /api/v1 */
    pub api_version_url_prefix: String,
    pub url: String,
    pub service_name: String,
    pub api_url: String,
    api_version: String,
    host: String,
    port: String,
    envs: Envs,
}

impl Envs {
    // fn new() -> Self {
    //     let max_db_connection_env = env::var("MAX_DB_CONNECT_RETRIES")
    //         .unwrap_or(5.to_string())
    //         .parse::<i32>()
    //         .unwrap();
    //     return Self {
    //         api_version: env::var("API_VERSION").unwrap_or("v1".to_string()),
    //         api_port: env::var("API_PORT").unwrap_or("3000".to_string()),
    //         api_host: env::var("API_HOST").unwrap_or("127.0.0.1".to_string()),
    //         db_users: env::var("DB_USERS").unwrap_or("users".to_string()),
    //         db_itineary: env::var("DB_ININEARY").unwrap_or("itineary".to_string()),
    //         db_name: env::var("DB_NAME").unwrap_or("travel".to_string()),
    //         db_url: env::var("DB_URL").unwrap_or("mongodb://localhost:27017".to_string()),
    //         max_db_connection_retries: max_db_connection_env,
    //         max_connections: env::var("MAX_CONNECTIONS")
    //             .unwrap_or(1.to_string())
    //             .parse::<u32>()
    //             .unwrap(),
    //     };
    // }
    fn get_envs(&self) -> &Self {
        return self;
    }
}

impl Default for Config {
    fn default() -> Self {
        let envs = Envs::default();
        let host = &envs.api_host;
        let port = &envs.api_port;
        let api_version = &envs.api_version;
        let api_version_url_prefix = format!("/api/{}", api_version);
        let api_url = format!("http://{}:{}{}/", host, port, api_version_url_prefix).to_string();
        return Self {
            host: host.to_string(),
            port: port.to_string(),
            api_version: api_version.to_string(),
            api_version_url_prefix,
            api_url,
            url: format!("http://{}:{}", "localhost", port).to_string(),
            addr: format!("{}:{}", host, port),
            service_name: String::from("rust-app"),
            envs,
        };
    }
}

impl Config {
    pub fn new() -> Self {
        let envs = Envs::default();
        let host = &envs.api_host;
        let port = &envs.api_port;
        let api_version = &envs.api_version;
        let api_version_url_prefix = format!("/api/{}", api_version);
        let api_url = format!("http://{}:{}{}/", host, port, api_version_url_prefix).to_string();
        return Self {
            host: host.to_string(),
            port: port.to_string(),
            api_version: api_version.to_string(),
            api_version_url_prefix,
            api_url,
            url: format!("http://{}:{}", "localhost", port).to_string(),
            addr: format!("{}:{}", host, port),
            service_name: String::from("rust-app"),
            envs,
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
            HeaderName::from_static("hx-retarget"),
            HeaderName::from_static("hx-reswap"),
            ACCESS_CONTROL_ALLOW_ORIGIN,
            CONTENT_TYPE,
            ACCEPT,
        ]);
        let cors = CorsLayer::new()
            .allow_origin(self.url.parse::<HeaderValue>().unwrap())
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::PATCH,
                Method::DELETE,
            ])
            .allow_credentials(false)
            .allow_headers(allowed_headers);
        return cors;
    }
}
