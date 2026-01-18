mod hello_world;
mod project_version;

use axum::{Router, routing::get};
use crate::routes::hello_world::hello_world;
use crate::routes::project_version::project_version;

pub fn create_routes() -> Router {
    Router::new()
    .route("/", get(hello_world))
    .route("/version", get(project_version))
}