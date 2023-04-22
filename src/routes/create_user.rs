use axum::extract::State;
use sea_orm::DatabaseConnection;

pub async fn create_user(State(_database):State<DatabaseConnection>)->String{
    "user creates".to_owned()
}