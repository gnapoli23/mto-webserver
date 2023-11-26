use std::error::Error;

use serde::Deserialize;
use tokio::sync::OnceCell;

#[derive(Deserialize)]
pub struct Config {
    #[serde(alias = "database")]
    pub db: DbConfig,
}

#[derive(Deserialize)]
pub struct DbConfig {
    pub protocol: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub schema: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub conn_timeout: u64,
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

pub async fn get() -> &'static Config {
    CONFIG
        .get_or_try_init(|| async {
            let data = tokio::fs::read("./config.yaml").await.expect("Unable to read config file");
            Ok::<Config, Box<dyn Error>>(serde_yaml::from_slice::<Config>(&data).expect("Unable to deserialize config file"))
        })
        .await
        .unwrap()
}

#[cfg(test)]
mod config_tests {
    use super::get;

    #[tokio::test]
    async fn test_load_conf() {
        let conf = get().await;
        assert_eq!(conf.db.protocol, "mysql");
    }
}
