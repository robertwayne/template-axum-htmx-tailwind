use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::state::SharedState;

pub async fn robots(state: State<SharedState>) -> impl IntoResponse {
    let Some(asset) = state.assets.get("robots.txt") else {
        return StatusCode::NOT_FOUND.into_response();
    };

    if let Ok(string) = String::from_utf8(asset.contents.clone().to_vec()) {
        return string.into_response();
    }

    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
