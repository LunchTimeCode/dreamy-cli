use appstate::AppState;

use axum::{
    extract::{self, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use colored::Colorize;
use poll_schedule::PollSchedule;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::spawn;
use tokio::sync::Mutex;
use tokio_schedule::{every, Job};

use crate::commands::HtmlType;

mod appstate;
mod auth_state;
mod poll_schedule;
mod repos_route;
mod view_route;

#[allow(clippy::too_many_arguments)]
pub async fn start_server(
    port: String,
    token: &str,
    org: Option<String>,
    repos_path: Option<String>,
    html: bool,
    html_type: HtmlType,
    poll_schedule: String,
    header_key: String,
    auth_env_key: String,
) -> anyhow::Result<String> {
    let (listener, app) = create_server(
        port,
        token.to_owned(),
        org,
        repos_path,
        html,
        html_type,
        poll_schedule,
        header_key,
        auth_env_key,
    )
    .await?;
    axum::serve(listener, app).await.unwrap();
    Ok("Shutdown server with no errors".to_string())
}

#[allow(clippy::too_many_arguments)]
async fn create_server(
    port: String,
    token: String,
    org: Option<String>,
    repos_path: Option<String>,
    html: bool,
    html_type: HtmlType,
    poll_schedule: String,
    header_key: String,
    auth_env_key: String,
) -> anyhow::Result<(TcpListener, Router)> {
    let app_state = Arc::new(Mutex::new(AppState::from_keys(header_key, auth_env_key)?));

    let app_state_clone = Arc::clone(&app_state);

    let j = move || {
        let token_clone = token.clone();
        let org_clone = org.clone();
        let repos_path_clone = repos_path.clone();
        let html_clone = html;
        let html_type_clone = html_type;

        let app_state_clone = Arc::clone(&app_state_clone);

        async move {
            let mut state = app_state_clone.lock().await;
            let result = state
                .set_deps(
                    &token_clone,
                    org_clone,
                    repos_path_clone,
                    html_clone,
                    html_type_clone,
                )
                .await;
            match result {
                Ok(o) => println!("{}", o.green()),
                Err(e) => eprintln!("{}", e.to_string().red()),
            }
        }
    };

    let poll_schedule: PollSchedule = poll_schedule
        .try_into()
        .map_err(|e: String| anyhow::anyhow!(e))?;

    match poll_schedule {
        PollSchedule::Seconds(s) => {
            let schedule = every(s).seconds().in_timezone(&chrono::Utc);
            spawn(schedule.perform(j.clone()));
        }
        PollSchedule::Minutes(m) => {
            let schedule = every(m).minutes().in_timezone(&chrono::Utc);
            spawn(schedule.perform(j.clone()));
        }
        PollSchedule::Hours(h) => {
            let schedule = every(h).hours().in_timezone(&chrono::Utc);
            spawn(schedule.perform(j.clone()));
        }
        PollSchedule::Days(d) => {
            let schedule = every(d).days().in_timezone(&chrono::Utc);
            spawn(schedule.perform(j.clone()));
        }
        PollSchedule::Weeks(w) => {
            let schedule = every(w).weeks().in_timezone(&chrono::Utc);
            spawn(schedule.perform(j.clone()));
        }
        PollSchedule::Months(m) => {
            let schedule = every(m).weeks().in_timezone(&chrono::Utc);
            spawn(schedule.perform(j.clone()));
        }
    }

    tokio::spawn(async move { j().await });

    let binding = format!("0.0.0.0:{}", port);
    println!("listeng at: {}", binding.green());

    // build our application with a single route
    let app = Router::new().nest("/", make_api()).with_state(app_state);

    let listener = tokio::net::TcpListener::bind(binding).await.unwrap();

    Ok((listener, app))
}

fn make_api() -> Router<Arc<Mutex<AppState>>> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(view_route::view))
        .route("/repos", post(repos_route::set_repos))
}

async fn healthz() -> Response {
    StatusCode::OK.into_response()
}
