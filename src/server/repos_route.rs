use axum::{
    extract::{self, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde_derive::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::appstate::AppState;

#[derive(Deserialize, Debug)]
pub struct Repos {
    repos: Vec<String>,
}

pub async fn set_repos(
    State(app_state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
    extract::Json(payload): extract::Json<Repos>,
) -> Response {
    let mut state = app_state.lock().await;

    if !state.get_auth().is_valid(headers) {
        return StatusCode::FORBIDDEN.into_response();
    };

    state.replace_repos(payload.repos);
    StatusCode::OK.into_response()
}
