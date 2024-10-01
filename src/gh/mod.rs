use anyhow::anyhow;
use remote::{GitHubDep, Github};

use crate::commands::HtmlType;
use crate::config;
use crate::html;

pub mod remote;

async fn global_deps(
    token: &str,
    org: Option<String>,
    repos_path: Option<String>,
    repos: Vec<String>,
) -> anyhow::Result<remote::Source> {
    let config = config::Config::try_from_file();

    let gh_cl = Github::new();

    let org = match org {
        Some(org) => org,
        None => match config {
            Ok(c) => c.org,
            Err(_) => return Err(anyhow!("no org")),
        },
    };

    let repos = config::repos_from_file(repos_path).unwrap_or(repos);

    let mut graph: Vec<GitHubDep> = vec![];
    for repo in repos {
        if let Ok(res) = gh_cl.get_graph(&org, &repo, token).await {
            for single in res {
                graph.push(single)
            }
        } else {
            eprintln!("Could not find sbom of repo {}, you may need to activate dependency graph in this repo", repo)
        }
    }
    let source: remote::Source = remote::Source(graph);
    Ok(source)
}

pub async fn get_deps_global(
    token: &str,
    org: Option<String>,
    repos_path: Option<String>,
    html: bool,
    html_type: HtmlType,
    repos: Vec<String>,
) -> anyhow::Result<String> {
    let source = global_deps(token, org, repos_path, repos).await?;
    let config = config::Config::from_file();
    let forbidden_licenses = config.forbidden_licenses();

    let (forbidden_licenses, filtered): (Vec<GitHubDep>, Vec<GitHubDep>) = source
        .0
        .into_iter()
        .filter(|d| {
            // if they are the same it means it is the repo itself
            d.name != d.tipe
        })
        .partition(|d| forbidden_licenses.contains(&d.license));

    let licenses = filtered
        .iter()
        .cloned()
        .map(|d| d.license)
        .collect::<std::collections::HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();

    let res = if html {
        let pretty = serde_json::to_string_pretty(&forbidden_licenses)?;
        eprintln!("{}", &pretty);
        html::render_html(filtered, licenses, html_type)
    } else {
        let both = vec![filtered, forbidden_licenses];
        serde_json::to_string_pretty(&both)?
    };

    Ok(res)
}
