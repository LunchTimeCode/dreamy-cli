use crate::{commands::HtmlType, gh::remote::GitHubDep};
use askama::Template;

#[derive(Debug, Template)]
#[template(path = "dependencies.html")]
struct DependencyTemplate {
    title: String,
    items: Vec<GitHubDep>,
    licenses: Vec<String>,
}

#[derive(Debug, Template)]
#[template(path = "licenses.html")]
struct LicenseTemplate {
    title: String,
    items: Vec<GitHubDep>,
    licenses: Vec<String>,
}

pub fn render_html(raw: Vec<GitHubDep>, unique_licenses: Vec<String>, tipe: HtmlType) -> String {
    match tipe {
        HtmlType::Licenses => render_lic_html(raw, unique_licenses),
        HtmlType::Dependencies => render_dep_html(raw, unique_licenses),
    }
}

pub fn render_lic_html(raw: Vec<GitHubDep>, unique_licenses: Vec<String>) -> String {
    let hello = LicenseTemplate {
        items: raw,
        title: "Licenses".to_string(),
        licenses: unique_licenses,
    };
    hello.render().unwrap()
}

pub fn render_dep_html(raw: Vec<GitHubDep>, unique_licenses: Vec<String>) -> String {
    let hello = DependencyTemplate {
        items: raw,
        title: "Dependencies".to_string(),
        licenses: unique_licenses,
    };
    hello.render().unwrap()
}
