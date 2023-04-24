use axum::{Router, routing::{get, post}, middleware};

use crate::{routes::{hello_world::hello_world, create_user::create_user, login::login, logout::logout, guard::guard}, app_state::AppState};

pub fn create_router(app_state:AppState)->Router{
    Router::new()
    .route("/", get(hello_world))
    .route("/logout", post(logout))
    .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
    .route("/login", post(login))
    .route("/user", post(create_user))
    .with_state(app_state)
}