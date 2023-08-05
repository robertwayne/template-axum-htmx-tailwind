use axum::{body::Body, extract::State, http::Request, response::IntoResponse};
use axum_htmx::HxBoosted;
use minijinja::context;
use serde::Serialize;

use crate::state::SharedState;

#[derive(Serialize)]
struct NotFoundTemplate {
    pub message: String,
}

pub async fn not_found(
    HxBoosted(boosted): HxBoosted,
    state: State<SharedState>,
    req: Request<Body>,
) -> impl IntoResponse {
    let message = format!("{:?} not found", req.uri().path());

    state.render_with_context(
        boosted,
        "not_found.html",
        context! {
            message
        },
    )
}
