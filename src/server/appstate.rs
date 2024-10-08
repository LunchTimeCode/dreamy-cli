use crate::{commands::HtmlType, gh};

#[derive(Clone, Debug)]
pub struct AppState {
    html: String,
    repos: Vec<String>
}

impl AppState {
    pub fn empty() -> Self {
        AppState {
            html: "No App State".to_string(),
            repos: Vec::new()
        }
    }
    
    pub fn replace_repos(&mut self, repos: Vec<String>){
        self.repos = repos
    }
    
    pub fn get_repos(&self) ->  Vec<String>{
            self.repos.clone()
    }

    pub async fn set_deps(
        &mut self,
        token: &str,
        org: Option<String>,
        repos_path: Option<String>,
        _html: bool,
        html_type: HtmlType
    ) -> anyhow::Result<String> {
        self.html = gh::get_deps_global(token, org, repos_path, true, html_type, self.repos.clone()).await?;
        Ok("Updated Appstate".to_string())
    }

    pub fn get_html(&self) -> String {
        self.html.clone()
    }
}
