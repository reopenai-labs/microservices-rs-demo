use crate::{appctx::Context, service};
use axum::{routing::get, Router};
use service::market;

pub fn markets(ctx: &'static Context) -> Router {
    Router::new()
        .route("/markets:list", get(market::list_all))
        .route("/markets", get(market::get_by_id))
        .with_state(ctx)
}
