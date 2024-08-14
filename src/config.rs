use core::fmt;
use std::fs;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub report_title: Option<String>,
    pub allowed_licenses: Option<Vec<String>>,
    pub forbid_unknown: Option<bool>,
    forbidden_licenses: Option<Vec<String>>,
    pub org: String,
    pub repo: Option<String>,
}

impl Config {
    pub fn example() -> Self {
        Self {
            report_title: Some("very good title".to_string()),
            allowed_licenses: None,
            forbidden_licenses: Some(vec![]),
            forbid_unknown: Some(false),
            org: "acme".to_string(),
            repo: Some("some_repo".to_string()),
        }
    }

    pub fn global_example() -> Self {
        Self {
            org: "acme".to_string(),
            report_title: None,
            allowed_licenses: None,
            forbidden_licenses: None,
            forbid_unknown: None,
            repo: None,
        }
    }

    pub fn forbidden_unknown(&self) -> bool {
        self.forbid_unknown.unwrap_or(false)
    }

    pub fn forbidden_licenses(&self) -> Vec<String> {
        self.forbidden_licenses.clone().unwrap_or_default()
    }

    pub fn allowed_licenses(&self) -> Vec<String> {
        self.allowed_licenses.clone().unwrap_or_default()
    }

    #[allow(unused)]
    pub fn is_allowed_license(&self, license: String) -> bool {
        let allowed = self.allowed_licenses();
        if allowed.is_empty() {
            return true;
        }
        allowed.contains(&license)
    }

    pub fn is_forbidden_license(&self, license: String) -> bool {
        self.forbidden_licenses().contains(&license)
    }

    pub fn from_file() -> Self {
        let raw = fs::read_to_string("dy.toml").expect("no file found called dy.toml");
        toml::from_str(&raw).expect("could not read toml file")
    }

    pub fn try_from_file() -> anyhow::Result<Self> {
        let raw = fs::read_to_string("dy.toml")?;
        let file: Config = toml::from_str(&raw)?;
        Ok(file)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&toml::to_string_pretty(self).expect("not possible"), f)
    }
}

pub fn repos_from_file(path: Option<String>) -> Vec<String> {
    let path: String = path.unwrap_or_else(|| "repos.json".to_string());

    let raw = fs::read_to_string(path).expect("no repo json file found");

    serde_json::from_str(&raw).expect("could not read repo json file")
}
