use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

use crate::config::Config;

pub async fn connect(conf: &Config) -> Result<DatabaseConnection, DbErr> {
    let mut conn_opts = ConnectOptions::new(&conf.database_url);
    conn_opts
        .min_connections(conf.database_min_conns)
        .max_connections(conf.database_max_conns)
        .connect_timeout(Duration::from_secs(conf.database_conn_timeout));

    // Under the hood, a sqlx::Pool is created and owned by DatabaseConnection
    Database::connect(conn_opts).await
}

#[cfg(test)]
mod connect_tests {

    use super::connect;
    use crate::config;

    #[tokio::test]
    async fn check_connection() {
        let config = config::get().await.unwrap();
        let conn = connect(config).await.unwrap();
        assert!(conn.ping().await.is_ok())
    }
}
