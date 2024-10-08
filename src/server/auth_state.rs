use axum::http::HeaderMap;

#[derive(Clone, Debug)]
pub struct AuthState {
    auth_header_key: String,
    secret_from_env: String,
}

impl AuthState {
    pub fn from_keys(header_key: String, env_key: String) -> anyhow::Result<Self> {
        Ok(Self {
            auth_header_key: header_key,
            secret_from_env: std::env::var(env_key)?,
        })
    }

    pub fn is_valid(&self, header_map: HeaderMap) -> bool {
        if let Some(Ok(secret_from_header)) = header_map
            .get(self.auth_header_key.clone())
            .map(|o| o.to_str())
        {
            return secret_from_header == self.secret_from_env;
        }

        false
    }
}
