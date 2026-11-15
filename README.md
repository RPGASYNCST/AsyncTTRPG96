# AsyncTTRPG96 🎲💻

> **Um RPG de Mesa (TTRPG) Assíncrono com o consagrado design escultural e minimalista do NeXTSTEP (OPENSTEP), construído em Rust moderno com Iced, Tokio e MySQL (SQLx).**

---

## 📖 Visão Geral

O **AsyncTTRPG96** é uma aplicação desktop desenvolvida para permitir sessões assíncronas de RPG de mesa (Tabletop RPG). Sua proposta une:

1. **Estética NeXTSTEP / OPENSTEP**: Interface inspirada no design escultural e refinado do lendário sistema operacional da NeXT. Conta com barra de título preta sólida com texto branco nítido, paleta em cinza neutro NeXT (`#B3B3B3`), moldura externa preta sólida de 2px, botões 3D volumosos chanfrados com contorno preto de alta precisão, campos de entrada brancos em baixo-relevo (*sunken*) e painéis de grupo sulcados (*grooved/etched*).
2. **Alta Performance & Robustez**: Escrito em **Rust (edição 2024)**, aproveitando concorrência segura com **Tokio** e renderização reativa com a biblioteca de GUI **Iced 0.14**.
3. **Persistência Relacional**: Camada de armazenamento assíncrona utilizando **MySQL** via **SQLx**, com suporte automático a criação de banco de dados, pool de conexões resiliente e monitor de integridade (*health-check*).

---

## 🛠️ Tecnologias e Dependências

| Componente | Tecnologia | Versão / Detalhes |
| :--- | :--- | :--- |
| **Linguagem** | [Rust](https://www.rust-lang.org/) | Rust 2024 Edition (requer Rust 1.85+) |
| **Interface Gráfica (GUI)** | [Iced](https://iced.rs/) | `0.14.0` (Tokio runtime, image support, advanced widgets) |
| **Runtime Assíncrono** | [Tokio](https://tokio.rs/) | `1.54` (multi-thread, sync, fs, time) |
| **Banco de Dados & ORM** | [SQLx](https://github.com/launchbadge/sqlx) | `0.9.0` (MySQL, runtime Tokio, rustls) |
| **Serialização** | [Serde](https://serde.rs/) / `serde_json` | `1.0` |
| **Data e Hora** | [Chrono](https://github.com/chronotope/chrono) | `0.4` |
| **Tratamento de Erros** | [thiserror](https://github.com/dtolnay/thiserror) | `2.0` |
| **Logs & Diagnóstico** | [tracing](https://tokio.rs/tokio/topics/tracing) / `tracing-subscriber` | `0.1` / `0.3` |

---

## 📂 Estrutura de Diretórios

```text
AsyncTTRPG96/
├── assets/                       # Recursos visuais e tipográficos
│   ├── fonts/                    # Fontes retrô embutidas
│   │   ├── Silkscreen-Bold.ttf   # Títulos e destaques retrô
│   │   ├── Silkscreen-Regular.ttf
│   │   └── VT323-Regular.ttf     # Fonte de terminal/UI retrô
│   └── icons/                    # Ícones do projeto
│       ├── d20.png               # Ícone do dado d20 clássico
│       ├── d20_transparent.png
│       └── logo.png              # Ícone da janela da aplicação
├── src/                          # Código-fonte principal
│   ├── main.rs                   # Ponto de entrada (tracing + inicialização da GUI)
│   ├── lib.rs                    # Exposição dos módulos `storage` e `ui`
│   ├── storage/                  # Camada de banco de dados e configuração
│   │   ├── mod.rs                # Reexportações do módulo storage
│   │   ├── config.rs             # Deserialização e validação do config.json
│   │   ├── database.rs           # Gerenciador de pool de conexões e health-check
│   │   └── error.rs              # Definições de erros específicos (StorageError)
│   └── ui/                       # Interface gráfica NeXTSTEP com Iced
│       ├── mod.rs                # Exporta run_gui e temas
│       ├── app.rs                # Aplicação Iced (RetroApp: Model, View, Update)
│       └── theme.rs              # Cores NeXTSTEP, botões esculturais e painéis sulcados
├── tests/                        # Testes automatizados
│   └── config_test.rs            # Validações de limites e segurança da configuração
├── Cargo.toml                    # Manifesto do projeto e dependências Rust
├── Cargo.lock                    # Versões fixadas das dependências
├── config.json                   # Arquivo de configuração local (banco, pool)
└── README.md                     # Documentação do projeto
```

---

## ⚙️ Pré-Requisitos

1. **Rust Toolchain**:
   - Recomenda-se a versão estável mais recente do Rust (suportando a edição 2024).
   - Instale via [rustup](https://rustup.rs/):
     ```bash
     rustup update stable
     ```
2. **MySQL ou MariaDB**:
   - Um servidor MySQL rodando localmente ou remotamente (porta padrão: `3306`).
   - Usuário com privilégios para criar banco de dados caso `auto_create_database` esteja ativo.
3. **Dependências do Sistema Operacional (Windows / Linux / macOS)**:
   - **Windows**: [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
   - **Linux**: Bibliotecas de desenvolvimento para X11/Wayland (ex: `libx11-dev`, `libasound2-dev`, `libudev-dev`).

---

## 🚀 Configuração e Execução

### 1. Clonar o Repositório
```bash
git clone https://github.com/seu-usuario/AsyncTTRPG96.git
cd AsyncTTRPG96
```

### 2. Configurar o Banco de Dados (`config.json`)
O projeto utiliza um arquivo `config.json` na raiz para carregar as opções de conexão com o MySQL. Ajuste com suas credenciais:

```json
{
  "database": {
    "host": "127.0.0.1",
    "port": 3306,
    "username": "root",
    "password": "",
    "database": "asyncttrpg96",
    "max_connections": 10,
    "min_connections": 2,
    "connect_timeout_secs": 10,
    "idle_timeout_secs": 300,
    "max_lifetime_secs": 1800,
    "auto_create_database": true
  }
}
```

> **Nota de Segurança**: O validador em `src/storage/config.rs` sanitiza o nome do banco de dados (permitindo apenas caracteres alfanuméricos e `_`) e impede limites inconsistentes no pool de conexões (`min_connections > max_connections`).

### 3. Executar Testes
Para garantir que o ambiente e as validações de configuração estejam operantes:

```bash
cargo test
```

### 4. Rodar a Aplicação
Para compilar e abrir a interface gráfica do jogo:

```bash
cargo run
```

Para executar com logs detalhados de depuração da engine de banco e interface:

```bash
# PowerShell (Windows)
$env:RUST_LOG="info"; cargo run

# Bash (Linux/macOS)
RUST_LOG=info cargo run
```

---

## 🧩 Arquitetura do Sistema

```
+-----------------------------------------------------------+
|                     AsyncTTRPG96                          |
|                                                           |
|   +-----------------------+     +---------------------+   |
|   |        UI (Iced)      |     |   Storage (SQLx)    |   |
|   |  - RetroApp (MVU)     |     |  - DatabaseEngine   |   |
|   |  - Theme (NeXTSTEP)   |<--->|  - Connection Pool  |   |
|   |  - Custom Frameless   |     |  - Health Monitor   |   |
|   |  - Sculpted / Bevels  |     |  - Safe Validation  |   |
|   +-----------------------+     +---------------------+   |
+--------------------------------------------|--------------+
                                             v
                                  +--------------------+
                                  |   MySQL Server     |
                                  |   (Database / DB)  |
                                  +--------------------+
```

### Arquitetura da Interface (`src/ui/`)
- **Padrão Elm / MVU (Model-View-Update)**: O `RetroApp` gerencia o estado através de `Message`s claras (`UsernameChanged`, `AccessKeyChanged`, `Connect`, `CreateSheet`, `Options`, `Minimize`, `Close`, etc.).
- **Window Management Customizado**: A janela é iniciada sem decorações nativas do sistema (`decorations: false`), com barra de título preta sólida NeXTSTEP com suporte a clique e arrasto (`iced::window::drag`).
- **Widgets e Painéis NeXTSTEP**: `sunken_3d` (inputs escavados com fundo branco puro), `etched_panel` (painéis com sulco/groove do Interface Builder) e botões esculturais com destaque chanfrado e contorno preto de alta precisão.

### Motor de Banco de Dados (`src/storage/`)
- **`DatabaseEngine`**: Gerencia o pool assíncrono via `sqlx::MySqlPool`.
- **Criação Automática**: Caso a base de dados configurada ainda não exista, o método `ensure_database_exists` conecta ao servidor MySQL e executa `CREATE DATABASE IF NOT EXISTS`.
- **Monitor de Saúde em Background**: `spawn_health_monitor` permite agendar verificações periódicas de latência e integridade sem bloquear a UI.

---

## 🗺️ Roadmap de Desenvolvimento

- [x] Base do projeto em Rust 2024.
- [x] Tela de Sessão de Acesso no estilo NeXTSTEP com Iced.
- [x] Tela de Criar Sessão com layout NeXTSTEP, GroupBoxes e parâmetros de mesa.
- [x] Módulo de configuração e validação com `config.json`.
- [x] Motor MySQL com pool de conexões e health-check configurável.
- [ ] Conectar o botão **"Conectar"** à autenticação real de usuários no banco.
- [ ] Persistir e iniciar a sessão criada no banco de dados.
- [ ] Interface de Ficha de Personagem retrô (atributos, inventário, perícias).
- [ ] Sistema de rolagem de dados (D20, D6, D100, etc.) com registro no histórico da sessão.
- [ ] Troca de mensagens e ações assíncronas entre Mestre e Jogadores.

---

## 🤝 Contribuição

1. Crie uma branch para sua funcionalidade ou correção:
   ```bash
   git checkout -b feature/minha-nova-feature
   ```
2. Realize seus commits seguindo boas práticas:
   ```bash
   git commit -m "feat: adiciona componente de rolagem de dados"
   ```
3. Garanta que todos os testes passem:
   ```bash
   cargo test
   ```
4. Abra um Pull Request detalhando suas alterações.

---

## 📜 Licença

Este projeto está sob a licença definida no repositório. Consulte os arquivos do projeto para mais informações.

