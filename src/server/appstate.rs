use crate::{commands::HtmlType, gh};

#[derive(Clone, Debug)]
pub struct AppState {
    html: String,
}

impl AppState {
    pub fn empty() -> Self {
        AppState {
            html: "No App State".to_string(),
        }
    }

    pub async fn set_deps(
        &mut self,
        token: &str,
        org: Option<String>,
        repos_path: Option<String>,
        _html: bool,
        html_type: HtmlType,
    ) -> anyhow::Result<String> {
        self.html = gh::get_deps_global(token, org, repos_path, true, html_type).await?;
        Ok("Updated Appstate".to_string())
    }

    pub fn get_html(&self) -> String {
        self.html.clone()
    }
}
