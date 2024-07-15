use core::fmt;
use std::fs;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Config {
    report_title: String,
    allowed_licenses: Vec<String>,
    forbidden_licenses: Vec<String>,
}

impl Config {
    pub fn example() -> Self {
        Self {
            report_title: "very good title".to_string(),
            allowed_licenses: vec![],
            forbidden_licenses: vec![],
        }
    }

    #[allow(unused)]
    pub fn is_allowed_license(&self, license: String) -> bool {
        self.allowed_licenses.contains(&license)
    }

    #[allow(unused)]
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
