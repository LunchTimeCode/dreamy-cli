use anyhow::anyhow;
use remote::{GitHubDep, Github};

use crate::config;

mod remote;

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

    let Some(repo) = repo.or(Some(config.clone().repo)) else {
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
    if config.forbid_unknown && (!not_found.is_empty()) {
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

    let Some(repo) = repo.or(Some(config.clone().repo)) else {
        return Err(anyhow!("no repo"));
    };

    let res = gh_cl.get_graph(&org, &repo, token).await?;

    let source: remote::Source = remote::Source(res);

    let pretty = serde_json::to_string_pretty(&source.0)?;

    Ok(pretty)
}
