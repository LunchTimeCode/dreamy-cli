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
    let filtered: Vec<GitHubDep> = raw
        .into_iter()
        .filter(|d| {
            // if they are the same it means it is the repo it self
            d.name != d.tipe
        })
        .collect();

    let hello = DependencyTemplate {
        items: filtered,
        title: "Dependencies".to_string(),
    };
    hello.render().unwrap()
}
