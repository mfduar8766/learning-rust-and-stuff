use axum::http::{
    header::{ACCEPT, CONTENT_TYPE},
    HeaderName, HeaderValue, Method,
};
use tower_http::cors::{AllowHeaders, CorsLayer};

#[allow(dead_code)]
pub struct Config {
    pub cors: CorsLayer,
    pub addr: String,
    /* /api/v1 */
    pub api_version_url_prefix: String,
    pub url: String,
    pub service_name: String,
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
            .allow_origin(url.parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
            .allow_credentials(false)
            .allow_headers(allowed_headers);

        return Self {
            host: host.to_string(),
            port: port.to_string(),
            api_version: api_version.to_string(),
            api_version_url_prefix,
            url: url.to_string(),
            addr: format!("127.0.0.1:{}", port),
            cors,
            service_name: String::from("rust-app"),
        };
    }

    pub fn service_name(&self) -> &str {
        return &self.service_name;
    }
}
