use crate::{health, state::AppState};
use axum::Router;

pub fn build(state: AppState) -> Router {
    health::router().with_state(state)
}
