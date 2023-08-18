use axum::{extract::State, response::IntoResponse};
use axum_htmx::HxBoosted;
use serde::Serialize;

use super::BaseTemplateData;
use crate::state::SharedState;

#[derive(Serialize)]
struct IndexTemplate {
    base: Option<BaseTemplateData>,
}

pub async fn index(boosted: HxBoosted, state: State<SharedState>) -> impl IntoResponse {
    state.render(boosted, "index.html")
}

pub async fn about(boosted: HxBoosted, state: State<SharedState>) -> impl IntoResponse {
    state.render(boosted, "about.html")
}
