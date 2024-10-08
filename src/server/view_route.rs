use axum::extract::State;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::appstate::AppState;

pub async fn view(State(app_state): State<Arc<Mutex<AppState>>>) -> axum::response::Html<String> {
    let state = app_state.lock().await;
    axum::response::Html(state.get_html().to_string())
}
