use crate::config::DbConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub async fn connect(db_config: &DbConfig) -> Result<DatabaseConnection, DbErr> {
    let conn_url = format!(
        "{}://{}:{}@{}/{}",
        db_config.protocol,
        db_config.username,
        db_config.password,
        db_config.host,
        db_config.schema
    );
    let mut conn_opts = ConnectOptions::new(&conn_url);
    conn_opts
        .min_connections(db_config.min_connections)
        .max_connections(db_config.max_connections)
        .connect_timeout(Duration::from_secs(db_config.conn_timeout))
        .sqlx_logging(true);
    //.sqlx_logging_level(log::LevelFilter::Info);

    Database::connect(conn_opts).await
}

#[cfg(test)]
mod connect_tests {

    use super::connect;
    use crate::config;

    #[tokio::test]
    async fn check_connection() {
        let config = config::get().await;
        let conn = connect(&config.db).await.unwrap();
        conn.ping().await.unwrap()
    }
}
