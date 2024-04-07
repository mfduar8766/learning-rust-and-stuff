use axum::http::{
    header::{ACCEPT, CONTENT_TYPE},
    HeaderName, HeaderValue, Method,
};
use tower_http::cors::{AllowHeaders, CorsLayer};

#[allow(dead_code)]
pub struct Config<'a> {
    pub cors: CorsLayer,
    pub addr: String,
    /* /api/v1 */
    pub api_version_url_prefix: String,
    pub url: String,
    api_version: &'a str,
    allowed_headers: AllowHeaders,
    host: String,
    port: String,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        let host = &String::from("localhost");
        let port = &String::from("3000");
        let api_version = "v1";
        let api_version_url_prefix = format!("/api/{}", api_version);
        let url = &format!("http://{}:{}", host, port);
        let allowed_headers = &AllowHeaders::list([
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
        return Self {
            allowed_headers: allowed_headers.clone(),
            host: host.to_string(),
            port: port.to_string(),
            api_version,
            api_version_url_prefix,
            url: url.to_string(),
            cors: set_cors(&url, allowed_headers.clone()),
            addr: format!("127.0.0.1:{}", port),
        };
    }
}

fn set_cors(url: &str, allowed_headers: AllowHeaders) -> CorsLayer {
    return CorsLayer::new()
        .allow_origin(url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(false)
        .allow_headers(allowed_headers);
}
