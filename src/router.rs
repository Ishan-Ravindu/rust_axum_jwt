use axum::{Router, routing::{get, post}};

use crate::{routes::{hello_world::hello_world, create_user::create_user}, app_state::AppState};

pub fn create_router(app_state:AppState)->Router{
    Router::new()
    .route("/", get(hello_world))
    .route("/user", post(create_user))
    .with_state(app_state)
}