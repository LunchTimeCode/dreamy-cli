use crate::{commands::HtmlType, gh};

use super::auth_state::AuthState;

#[derive(Clone, Debug)]
pub struct AppState {
    html: String,
    repos: Vec<String>,
    auth_state: AuthState,
}

impl AppState {
    pub fn from_keys(header_key: String, env_key: String) -> anyhow::Result<Self> {
        Ok(AppState {
            html: "No App State".to_string(),
            repos: Vec::new(),
            auth_state: AuthState::from_keys(header_key, env_key)?,
        })
    }

    pub fn get_auth(&self) -> AuthState {
        self.auth_state.clone()
    }

    pub fn replace_repos(&mut self, repos: Vec<String>) {
        self.repos = repos
    }

    pub async fn set_deps(
        &mut self,
        token: &str,
        org: Option<String>,
        repos_path: Option<String>,
        _html: bool,
        html_type: HtmlType,
    ) -> anyhow::Result<String> {
        self.html =
            gh::get_deps_global(token, org, repos_path, true, html_type, self.repos.clone())
                .await?;
        Ok("Updated Appstate".to_string())
    }

    pub fn get_html(&self) -> String {
        self.html.clone()
    }
}
