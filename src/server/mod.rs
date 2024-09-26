use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use colored::Colorize;
use tokio::net::TcpListener;

mod poller;

pub async fn start_server(port: String) -> anyhow::Result<String> {
    let (listener, app) = create_server(port).await?;
    axum::serve(listener, app).await.unwrap();
    Ok("Shutdown server with no errors".to_string())
}

async fn create_server(port: String) -> anyhow::Result<(TcpListener, Router)> {
    let binding = format!("0.0.0.0:{}", port);
    println!("listeng at: {}", binding.green());

    // build our application with a single route
    let app = Router::new().nest("/", make_api());

    let listener = tokio::net::TcpListener::bind(binding).await.unwrap();

    Ok((listener, app))
}

fn make_api() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route( "/", get(view))
}

async fn healthz() -> Response {
    StatusCode::OK.into_response()
}

async fn view() -> Response {
    StatusCode::OK.into_response()
}
