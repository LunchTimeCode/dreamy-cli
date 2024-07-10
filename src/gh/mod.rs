use crate::config;

pub fn check_licenses_on(_token: &str, _org: Option<String>) -> anyhow::Result<String> {
    let config = config::Config::from_file();
    let ms = format!(
        "nothing checked not implemented yet but there is a config file:

dy.toml
-------------
{config}
-------------
"
    );
    Ok(ms.to_string())
}
