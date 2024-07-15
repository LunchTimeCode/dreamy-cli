use anyhow::anyhow;
use remote::Github;

use crate::config;

mod remote;

pub async fn check_licenses_on(
    token: &str,
    org: Option<String>,
    repo: Option<String>,
    base: Option<String>,
    head: Option<String>,
) -> anyhow::Result<String> {
    let config = config::Config::from_file();
    let ms = format!(
        "nothing checked not implemented yet but there is a config file:

dy.toml
-------------
{config}
-------------
"
    );
    let gh_cl = Github::new();

    let Some(org) = org else {
        return Err(anyhow!("no org"));
    };

    let Some(repo) = repo else {
        return Err(anyhow!("no repo"));
    };

    let Some(base) = base else {
        return Err(anyhow!("no base"));
    };

    let Some(head) = head else {
        return Err(anyhow!("no head"));
    };

    let _ = gh_cl
        .get_per_commit(token.to_owned(), org, repo, base, head)
        .await;

    Ok(ms.to_string())
}
