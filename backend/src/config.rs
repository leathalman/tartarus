use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub prowlarr_url: String,
    pub prowlarr_api_key: String,
    pub qbit_url: String,
    pub qbit_username: String,
    pub qbit_password: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            prowlarr_url: require_env("PROWLARR_URL")?,
            prowlarr_api_key: require_env("PROWLARR_API_KEY")?,
            qbit_url: require_env("QBIT_URL")?,
            qbit_username: require_env("QBIT_USERNAME")?,
            qbit_password: require_env("QBIT_PASSWORD")?,
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .map_err(|_| "PORT must be a valid u16".to_string())?,
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn require_env(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("Missing required environment variable: {key}"))
}
