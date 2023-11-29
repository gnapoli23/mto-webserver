use config::ConfigError;
use serde::Deserialize;
use tokio::sync::OnceCell;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub database_min_conns: u32,
    pub database_max_conns: u32,
    pub database_conn_timeout: u64,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    fn from_env() -> Result<Self, ConfigError> {
        ::config::Config::builder()
            .add_source(::config::Environment::default())
            .build()?
            .try_deserialize::<Config>()
    }
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

pub async fn get() -> Result<&'static Config, ConfigError> {
    CONFIG
        .get_or_try_init(|| async { Config::from_env() })
        .await
}

#[cfg(test)]
mod config_tests {

    #[tokio::test]
    async fn test_load_conf() {
        dotenv::dotenv().unwrap();
        let conf = super::get().await.unwrap();
        assert_eq!(conf.database_min_conns, 1);
        assert_eq!(conf.database_max_conns, 100);
    }
}
