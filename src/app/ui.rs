use axum::Router;
use tower_http::services::ServeDir;

pub fn route() -> Router {
    Router::new().fallback_service(ServeDir::new("static"))
}
