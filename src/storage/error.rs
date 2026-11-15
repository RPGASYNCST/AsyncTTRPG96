use thiserror::Error;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Erro ao ler ou gravar o arquivo de configuração: {0}")]
    ConfigIo(#[from] std::io::Error),

    #[error("Erro ao interpretar o JSON de configuração: {0}")]
    ConfigParse(#[from] serde_json::Error),

    #[error("Configuração de banco de dados inválida: {0}")]
    InvalidConfig(String),

    #[error("Erro no banco de dados MySQL: {0}")]
    Database(#[from] sqlx::Error),
}
