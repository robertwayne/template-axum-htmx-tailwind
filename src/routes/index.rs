use axum::{extract::State, response::IntoResponse};
use axum_htmx::HxBoosted;

use crate::state::SharedState;

pub async fn index(boosted: HxBoosted, state: State<SharedState>) -> impl IntoResponse {
    state.render(boosted, "index.html")
}

pub async fn about(boosted: HxBoosted, state: State<SharedState>) -> impl IntoResponse {
    state.render(boosted, "about.html")
}
