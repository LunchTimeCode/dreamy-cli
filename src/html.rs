use crate::{commands::HtmlType, gh::remote::GitHubDep};
use askama::Template;

#[derive(Debug, Template)]
#[template(path = "dependencies.html")]
struct DependencyTemplate {
    title: String,
    items: Vec<GitHubDep>,
}

#[derive(Debug, Template)]
#[template(path = "licenses.html")]
struct LicenseTemplate {
    title: String,
    items: Vec<GitHubDep>,
}

pub fn render_html(raw: Vec<GitHubDep>, tipe: HtmlType) -> String {
    match tipe {
        HtmlType::Licenses => render_lic_html(raw),
        HtmlType::Dependencies => render_dep_html(raw),
    }
}

pub fn render_lic_html(raw: Vec<GitHubDep>) -> String {
    let hello = LicenseTemplate {
        items: raw,
        title: "Licenses".to_string(),
    };
    hello.render().unwrap()
}

pub fn render_dep_html(raw: Vec<GitHubDep>) -> String {
    let hello = DependencyTemplate {
        items: raw,
        title: "Dependencies".to_string(),
    };
    hello.render().unwrap()
}
