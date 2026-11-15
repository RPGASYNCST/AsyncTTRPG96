pub mod config;
pub mod database;
pub mod error;

pub use config::{AppConfig, DatabaseConfig, SmtpConfig};
pub use database::{DatabaseEngine, DatabaseHealth};
pub use error::{StorageError, StorageResult};

/// Caminho padrao do arquivo de configuraçao
pub const DEFAULT_CONFIG_PATH: &str = "config.json";
