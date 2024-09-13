use anyhow::anyhow;
use remote::{GitHubDep, Github};

use crate::commands::HtmlType;
use crate::config;
use crate::html;

pub mod remote;

pub async fn check_licenses_on(
    token: &str,
    org: Option<String>,
    repo: Option<String>,
) -> anyhow::Result<String> {
    let config = config::Config::from_file();

    let gh_cl = Github::new();

    let Some(org) = org.or(Some(config.clone().org)) else {
        return Err(anyhow!("no org"));
    };

    let Some(repo) = repo.or(config.clone().repo) else {
        return Err(anyhow!("no repo"));
    };

    let res = gh_cl.get_graph(&org, &repo, token).await?;

    let source: remote::Source = remote::Source(res);

    let not_allowed_licenses: Vec<GitHubDep> = source
        .0
        .clone()
        .iter()
        .filter(|&dep| config.clone().is_forbidden_license(dep.clone().license))
        .cloned()
        .collect();

    let not_found: Vec<GitHubDep> = source
        .0
        .clone()
        .iter()
        .filter(|dep| dep.license == "not found")
        .cloned()
        .collect();

    if !not_allowed_licenses.is_empty() {
        let pretty = serde_json::to_string_pretty(&not_allowed_licenses)?;
        return Err(anyhow!(pretty));
    }
    if config.forbidden_unknown() && (!not_found.is_empty()) {
        let pretty = serde_json::to_string_pretty(&not_found)?;
        return Err(anyhow!(pretty));
    }

    Ok("All good, no license violation found".to_string())
}

pub async fn get_deps(
    token: &str,
    org: Option<String>,
    repo: Option<String>,
) -> anyhow::Result<String> {
    let config = config::Config::from_file();

    let gh_cl = Github::new();

    let Some(org) = org.or(Some(config.clone().org)) else {
        return Err(anyhow!("no org"));
    };

    let Some(repo) = repo.or(config.clone().repo) else {
        return Err(anyhow!("no repo"));
    };

    let res = gh_cl.get_graph(&org, &repo, token).await?;

    let source: remote::Source = remote::Source(res);

    let pretty = serde_json::to_string_pretty(&source.0)?;

    Ok(pretty)
}

async fn global_deps(
    token: &str,
    org: Option<String>,
    repos_path: Option<String>,
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

    let repos = config::repos_from_file(repos_path)?;

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
) -> anyhow::Result<String> {
    let source = global_deps(token, org, repos_path).await?;
    let config = config::Config::from_file();
    let non_accepted_licenses = config.forbidden_licenses();

    let (filtered, non_accepted_licenses): (Vec<GitHubDep>, Vec<GitHubDep>) = source
        .0
        .into_iter()
        .filter(|d| {
            // if they are the same it means it is the repo itself
            d.name != d.tipe
        })
        .partition(|d| non_accepted_licenses.contains(&d.license));

    let res = if html {
        let pretty = serde_json::to_string_pretty(&non_accepted_licenses)?;
        eprintln!("{}", &pretty);
        html::render_html(filtered, html_type)
    } else {
        let both = vec![filtered, non_accepted_licenses];
        serde_json::to_string_pretty(&both)?
    };

    Ok(res)
}
