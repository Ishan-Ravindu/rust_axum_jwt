use axum::{extract::State, Json};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use serde:: Deserialize;

use crate::database::users;

#[derive(Deserialize)]
pub struct RequestUser{
    username:String,
    password:String
}

pub async fn create_user(
    State(database):State<DatabaseConnection>,
    Json(user):Json<RequestUser>
)->String{
    
    let user = users::ActiveModel{
        username:Set(user.username) ,
        password:Set(user.password) ,
        token:Set(Some("1n2nkk4nkn5nk5nk4nk5kn".to_owned())),
        ..Default::default()
    };

    let _result =
         users::Entity::insert(user).exec(&database).await.unwrap();
    
    "user created".to_owned()
}