use AsyncTTRPG96::storage::{DatabaseConfig, SmtpConfig};

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

fn sample_smtp_config() -> SmtpConfig {
    SmtpConfig {
        host: "smtp.example.com".to_string(),
        port: 587,
        username: "user@example.com".to_string(),
        password: "secret".to_string(),
        from_email: "noreply@example.com".to_string(),
        from_name: "RPGASYNCST".to_string(),
        use_tls: true,
    }
}

#[test]
fn test_valid_smtp_config() {
    assert!(sample_smtp_config().validate().is_ok());
}

#[test]
fn test_empty_smtp_host_rejected() {
    let mut cfg = sample_smtp_config();
    cfg.host = "   ".to_string();
    assert!(cfg.validate().is_err());
}

#[test]
fn test_zero_smtp_port_rejected() {
    let mut cfg = sample_smtp_config();
    cfg.port = 0;
    assert!(cfg.validate().is_err());
}

#[test]
fn test_invalid_smtp_from_email_rejected() {
    let mut cfg = sample_smtp_config();
    cfg.from_email = "invalido".to_string();
    assert!(cfg.validate().is_err());
}
