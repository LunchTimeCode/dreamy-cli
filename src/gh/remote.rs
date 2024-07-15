use anyhow::anyhow;
use reqwest::header;
use serde_derive::{Deserialize, Serialize};
use std::env;

pub struct Github {
    client: reqwest::Client,
}

impl Github {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Self { client }
    }

    pub async fn get_graph(
        &self,
        org: &str,
        repo: &str,
        personal_token: &str,
    ) -> anyhow::Result<Vec<GitHubDep>> {
        let token = if personal_token.is_empty() {
            env::var("GITHUB_TOKEN")
        } else {
            Ok(personal_token.to_string())
        };

        let Ok(token) = token else {
            return Err(anyhow!("no github token set"));
        };

        let mut headers = header::HeaderMap::new();
        headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
        headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
        headers.insert("User-Agent", "Dreamy-App".parse().unwrap());

        let bearer = format!("Bearer {}", token);
        headers.insert("Authorization", bearer.parse().unwrap());

        let url = format!(
            "https://api.github.com/repos/{}/{}/dependency-graph/sbom",
            org, repo
        );

        eprintln!("getting sbom for {:?}", repo);

        let res = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|f| f.to_string());

        let res = res
            .map_err(|err| anyhow!(err))?
            .error_for_status()
            .map_err(|err| anyhow!(err))?;

        let bom = match res.json::<RepoBom>().await {
            Ok(r) => r,
            Err(err) => {
                eprintln!("could not parse {err}");
                return Err(anyhow!(err));
            }
        };

        let deps = bom.to_github_deps(repo);
        Ok(deps)
    }
}

impl core::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({self:?})")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source(pub Vec<GitHubDep>);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubRepo {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubDep {
    pub repo: String,
    pub name: String,
    pub _type: String,
    pub version: String,
    pub license: String,
}

#[derive(Serialize, Deserialize)]
pub struct RepoBom {
    #[serde(rename = "sbom")]
    sbom: Sbom,
}

impl RepoBom {
    fn to_github_deps(&self, repo: &str) -> Vec<GitHubDep> {
        let mut deps: Vec<GitHubDep> = vec![];

        for package in self.sbom.packages.clone() {
            let name = package.clone().name;

            let license = match package.clone().license_declared {
                None => package
                    .license_concluded
                    .clone()
                    .unwrap_or_else(|| "not found".to_string()),
                Some(l) => l,
            };

            let package_name = name
                .split_once(':')
                .unwrap_or_else(|| (&*package.name, &*package.name));
            let _type = package_name.0;
            let dep_name = package_name.1;

            let dep = GitHubDep {
                repo: repo.to_string(),
                _type: _type.to_string(),
                name: dep_name.to_string(),
                version: package.version_info.to_string(),
                license,
            };
            deps.push(dep)
        }

        deps
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sbom {
    name: String,
    packages: Vec<Package>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    name: String,
    version_info: String,
    license_concluded: Option<String>,
    license_declared: Option<String>,
}
