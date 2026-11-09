use AsyncTTRPG96::storage::DatabaseConfig;

fn sample_config() -> DatabaseConfig {
    DatabaseConfig {
        host: "127.0.0.1".to_string(),
        port: 3306,
        username: "root".to_string(),
        password: "".to_string(),
        database: "asyncttrpg96".to_string(),
        max_connections: 10,
        min_connections: 2,
        connect_timeout_secs: 10,
        idle_timeout_secs: 300,
        max_lifetime_secs: 1800,
        auto_create_database: true,
        auto_migrate: true,
    }
}

#[test]
fn test_valid_config() {
    let cfg = sample_config();
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_invalid_database_name_rejected() {
    let mut cfg = sample_config();
    cfg.database = "invalid-db-name; DROP TABLE".to_string();
    assert!(cfg.validate().is_err());
}

#[test]
fn test_invalid_pool_limits_rejected() {
    let mut cfg = sample_config();
    cfg.min_connections = 20;
    cfg.max_connections = 10;
    assert!(cfg.validate().is_err());
}

