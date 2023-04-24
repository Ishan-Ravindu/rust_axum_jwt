use axum::{http::StatusCode, extract::State, Extension};
use sea_orm::{Set, DatabaseConnection, IntoActiveModel, ActiveModelTrait};

use crate::database::users::Model;

pub async fn logout(
    State(database): State<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();

    user.token = Set(None);

    user.save(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}