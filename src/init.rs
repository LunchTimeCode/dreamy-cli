use std::fs;

use crate::config;

pub fn example() -> anyhow::Result<String> {
    let c = config::Config::example();
    let as_toml = toml::to_string_pretty(&c)?;
    fs::write("dy.toml", as_toml).expect("Unable to write file");
    Ok("config file created, please adjust to your needs".to_string())
}

pub fn global_example() -> anyhow::Result<String> {
    let c = config::Config::global_example();
    let as_toml = toml::to_string_pretty(&c)?;
    fs::write("dy.toml", as_toml).expect("Unable to write file");
    Ok("config file created, please adjust to your needs".to_string())
}
