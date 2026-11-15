use sqlx::mysql::{MySqlConnection, MySqlPool, MySqlPoolOptions};
use sqlx::{AssertSqlSafe, Connection, MySql, Transaction};
use std::path::Path;
use std::time::{Duration, Instant};
use tokio::task::JoinHandle;

use crate::storage::config::{AppConfig, DatabaseConfig};
use crate::storage::error::StorageResult;

/// Relatorio de saúde da conexao MySQL
#[derive(Debug, Clone)]
pub struct DatabaseHealth {
    pub is_healthy: bool,
    pub latency_ms: u128,
    pub server_version: String,
    pub active_connections: u32,
    pub idle_connections: usize,
}

/// Motor principal da conexao com o banco de dados MySQL
#[derive(Debug, Clone)]
pub struct DatabaseEngine {
    pool: MySqlPool,
    config: DatabaseConfig,
}

impl DatabaseEngine {
    /// Inicializa o motor
    pub async fn from_config_file<P: AsRef<Path>>(path: P) -> StorageResult<Self> {
        let app_config = AppConfig::load(path)?;
        Self::connect(app_config.database).await
    }

    /// Conecta ao servidor MySQL e cria o banco de dados (se habilitado)
    pub async fn connect(config: DatabaseConfig) -> StorageResult<Self> {
        config.validate()?;

        if config.auto_create_database {
            Self::ensure_database_exists(&config).await?;
        }

        tracing::info!(
            "Inicializando pool de conexao MySQL em {}:{}/{} (min={}, max={})...",
            config.host,
            config.port,
            config.database,
            config.min_connections,
            config.max_connections
        );

        let pool = MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.connect_timeout_secs))
            .idle_timeout(Some(Duration::from_secs(config.idle_timeout_secs)))
            .max_lifetime(Some(Duration::from_secs(config.max_lifetime_secs)))
            .test_before_acquire(true)
            .connect_with(config.database_connect_options())
            .await?;

        let engine = Self { pool, config };

        // Testa conexao
        engine.ping().await?;

        tracing::info!("Motor de banco de dados MySQL pronto e operacional.");
        Ok(engine)
    }

    ///executa (CREATE DATABASE IF NOT EXISTS)
    async fn ensure_database_exists(config: &DatabaseConfig) -> StorageResult<()> {
        tracing::info!(
            "Verificando existencia do banco de dados '{}' em {}:{}...",
            config.database,
            config.host,
            config.port
        );

        let mut conn = MySqlConnection::connect_with(&config.server_connect_options()).await?;

        let create_query = format!(
            "CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;",
            config.database
        );

        sqlx::query(AssertSqlSafe(create_query))
            .execute(&mut conn)
            .await?;
        conn.close().await?;

        tracing::info!(
            "Banco de dados '{}' garantido com sucesso.",
            config.database
        );
        Ok(())
    }

    // Executa um ping para validar conectividade
    pub async fn ping(&self) -> StorageResult<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    pub async fn health_check(&self) -> StorageResult<DatabaseHealth> {
        let start = Instant::now();
        let row: (String,) = sqlx::query_as("SELECT VERSION()")
            .fetch_one(&self.pool)
            .await?;
        let latency_ms = start.elapsed().as_millis();

        Ok(DatabaseHealth {
            is_healthy: true,
            latency_ms,
            server_version: row.0,
            active_connections: self.pool.size(),
            idle_connections: self.pool.num_idle(),
        })
    }

    pub fn spawn_health_monitor(&self, interval: Duration) -> JoinHandle<()> {
        let engine = self.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            loop {
                ticker.tick().await;
                if engine.pool.is_closed() {
                    tracing::info!("Pool de conexões encerrado; finalizando monitor de saúde.");
                    break;
                }
                match engine.ping().await {
                    Ok(_) => {
                        tracing::debug!(
                            "Health-check MySQL OK (conexões={}/idle={})",
                            engine.pool.size(),
                            engine.pool.num_idle()
                        );
                    }
                    Err(err) => {
                        tracing::warn!(
                            "Falha no health-check MySQL (o pool tentará reconectar automaticamente): {}",
                            err
                        );
                    }
                }
            }
        })
    }

    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }

    /// Retorna referencia as configuracoes.
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// Inicia uma nova transacao.
    pub async fn begin_tx(&self) -> StorageResult<Transaction<'_, MySql>> {
        let tx = self.pool.begin().await?;
        Ok(tx)
    }

    /// Encerra todas as conexoes.
    pub async fn close(&self) {
        tracing::info!("Encerrando pool de conexões MySQL...");
        self.pool.close().await;
    }
}
