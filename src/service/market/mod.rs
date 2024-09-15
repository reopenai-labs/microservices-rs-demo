use crate::appctx::Context;
use crate::errors::AppError;
use axum::{
    extract::{Query, State},
    Json,
};
use entity::symbol;
use sea_orm::*;
pub mod entity;
mod request;

pub async fn list_all(
    State(state): State<&'static Context>,
) -> Result<Json<Vec<symbol::Model>>, AppError> {
    let db = state.get_datasource().get_conn();
    let v = symbol::Entity::find().all(db).await?;
    return Ok(Json(v));
}

pub async fn get_by_id(
    Query(request): Query<request::IdParam>,
    State(state): State<&'static Context>,
) -> Result<Json<Option<symbol::Model>>, AppError> {
    let db = state.get_datasource().get_conn();
    let v = symbol::Entity::find_by_id(request.id).one(db).await?;
    return Ok(Json(v));
}
