mod router;
mod routes;
mod database;
pub mod app_state;
use app_state::AppState;
use router::create_router;

pub async fn run(app_state:AppState){
    let app = create_router(app_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}