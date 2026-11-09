-- Migration inicial do AsyncTTRPG96
-- Cria tabela de controle do motor de banco de dados

CREATE TABLE IF NOT EXISTS app_metadata (
    meta_key VARCHAR(64) NOT NULL PRIMARY KEY,
    meta_value TEXT NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

INSERT INTO app_metadata (meta_key, meta_value)
VALUES ('schema_version', '1.0.0'),
       ('app_name', 'AsyncTTRPG96')
ON DUPLICATE KEY UPDATE updated_at = CURRENT_TIMESTAMP;

