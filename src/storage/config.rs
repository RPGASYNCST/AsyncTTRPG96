use std::fs;
use std::path::Path;
use serde::Deserialize;
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};
use crate::storage::error::{StorageError, StorageResult};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
}

impl AppConfig {
    /// Carrega a configuracao do banco (config.json)
    pub fn load<P: AsRef<Path>>(path: P) -> StorageResult<Self> {
        let content = fs::read_to_string(path.as_ref())?;
        let config: Self = serde_json::from_str(&content)?;
        config.database.validate()?;
        Ok(config)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
    pub auto_create_database: bool,
    pub auto_migrate: bool,
}

impl DatabaseConfig {
    /// Valida se os parametros da conexao do banco sao validos.
    pub fn validate(&self) -> StorageResult<()> {
        if self.host.trim().is_empty() {
            return Err(StorageError::InvalidConfig(
                "O campo 'host' nao pode estar vazio.".to_string(),
            ));
        }
        if self.port == 0 {
            return Err(StorageError::InvalidConfig(
                "A porta do MySQL deve ser maior que zero.".to_string(),
            ));
        }
        if self.username.trim().is_empty() {
            return Err(StorageError::InvalidConfig(
                "O campo 'username' nao pode estar vazio.".to_string(),
            ));
        }
        if self.database.trim().is_empty()
            || !self
                .database
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(StorageError::InvalidConfig(
                "O nome do banco de dados deve conter apenas letras, numeros e underscore ('_')."
                    .to_string(),
            ));
        }
        if self.max_connections == 0 {
            return Err(StorageError::InvalidConfig(
                "'max_connections' deve ser pelo menos 1.".to_string(),
            ));
        }
        if self.min_connections > self.max_connections {
            return Err(StorageError::InvalidConfig(
                "'min_connections' nao pode ser maior que 'max_connections'.".to_string(),
            ));
        }
        Ok(())
    }

    pub fn server_connect_options(&self) -> MySqlConnectOptions {
        MySqlConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .ssl_mode(MySqlSslMode::Preferred)
            .charset("utf8mb4")
    }

    pub fn database_connect_options(&self) -> MySqlConnectOptions {
        self.server_connect_options().database(&self.database)
    }
}


