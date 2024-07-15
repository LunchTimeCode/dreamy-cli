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

    pub async fn get_per_commit(
        &self,
        personal_token: String,
        org: String,
        repo: String,
        base: String,
        head: String,
    ) -> anyhow::Result<DataSource> {
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
            "https://api.github.com/repos/{}/{}/dependency-graph/compare/{}...{}",
            org, repo, base, head
        );

        let _ = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|f| f.to_string());

        Ok(vec![])
    }
}

pub type DataSource = Vec<DataSourceElement>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSourceElement {
    pub change_type: String,
    pub manifest: String,
    pub ecosystem: String,
    pub name: String,
    pub version: String,
    pub package_url: String,
    pub license: Option<serde_json::Value>,
    pub source_repository_url: String,
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vulnerability {
    pub severity: String,
    pub advisory_ghsa_id: String,
    pub advisory_summary: String,
    pub advisory_url: String,
}
