use core::fmt;
use std::fs;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub report_title: String,
    pub allowed_licenses: Option<Vec<String>>,
    pub forbid_unknown: bool,
    pub forbidden_licenses: Vec<String>,
    pub org: String,
    pub repo: String,
}

impl Config {
    pub fn example() -> Self {
        Self {
            report_title: "very good title".to_string(),
            allowed_licenses: None,
            forbidden_licenses: vec![],
            forbid_unknown: false,
            org: "acme".to_string(),
            repo: "some_repo".to_string(),
        }
    }

    #[allow(unused)]
    pub fn is_allowed_license(&self, license: String) -> bool {
        let Some(allowed) = self.allowed_licenses.clone() else {
            return true;
        };
        allowed.contains(&license)
    }

    pub fn is_forbidden_license(&self, license: String) -> bool {
        self.forbidden_licenses.contains(&license)
    }

    pub fn from_file() -> Self {
        let raw = fs::read_to_string("dy.toml").expect("no file found called dy.toml");
        toml::from_str(&raw).expect("could not read toml file")
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&toml::to_string_pretty(self).expect("not possible"), f)
    }
}
