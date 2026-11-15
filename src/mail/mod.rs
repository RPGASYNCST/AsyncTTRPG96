use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::storage::{AppConfig, DEFAULT_CONFIG_PATH, SmtpConfig};

pub fn generate_confirmation_code() -> String {
    let mut rng = rand::rng();
    let code: u32 = rng.random_range(100_000..=999_999);
    code.to_string()
}

#[derive(Debug, Clone)]
pub struct LoadedPortrait {
    pub file_name: String,
    pub png_bytes: Vec<u8>,
    pub base64_png: String,
}

pub async fn pick_portrait_image() -> Result<Option<LoadedPortrait>, String> {
    let ps_script = r#"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

$dlg = New-Object System.Windows.Forms.OpenFileDialog
$dlg.Title = "RPGASYNCST - Escolher Retrato do Personagem"
$dlg.Filter = "Imagens (*.png;*.jpg;*.jpeg;*.bmp;*.gif)|*.png;*.jpg;*.jpeg;*.bmp;*.gif|Todos os arquivos (*.*)|*.*"
$dlg.Multiselect = $false

$owner = New-Object System.Windows.Forms.Form
$owner.TopMost = $true

if ($dlg.ShowDialog($owner) -eq [System.Windows.Forms.DialogResult]::OK) {
    $path = $dlg.FileName
    $name = [System.IO.Path]::GetFileName($path)
    $src = [System.Drawing.Bitmap]::FromFile($path)
    $maxSide = 256
    $w = $src.Width
    $h = $src.Height
    if ($w -gt $maxSide -or $h -gt $maxSide) {
        if ($w -ge $h) {
            $newW = $maxSide
            $newH = [Math]::Max(1, [int][Math]::Round(($h * $maxSide) / $w))
        } else {
            $newH = $maxSide
            $newW = [Math]::Max(1, [int][Math]::Round(($w * $maxSide) / $h))
        }
    } else {
        $newW = $w
        $newH = $h
    }
    $bmp = New-Object System.Drawing.Bitmap($newW, $newH)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::NearestNeighbor
    $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::Half
    $g.DrawImage($src, 0, 0, $newW, $newH)
    $g.Dispose()
    $src.Dispose()

    $ms = New-Object System.IO.MemoryStream
    $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
    $b64 = [Convert]::ToBase64String($ms.ToArray())
    $ms.Dispose()
    Write-Output "$name|$b64"
}
$owner.Dispose()
"#;

    let output = tokio::task::spawn_blocking(move || {
        let mut cmd = std::process::Command::new("powershell");
        cmd.args(["-NoProfile", "-Sta", "-Command", ps_script]);
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        cmd.output()
    })
    .await
    .map_err(|e| format!("Task de seleção de imagem interrompida: {e}"))?
    .map_err(|e| format!("Falha ao abrir seletor de imagem: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "Erro ao carregar imagem selecionada: {}",
            stderr.trim()
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout
        .lines()
        .rev()
        .find(|l| l.contains('|'))
        .map(str::trim)
        .unwrap_or("");

    if line.is_empty() {
        return Ok(None);
    }

    let (file_name, b64) = line
        .split_once('|')
        .ok_or_else(|| "Formato inesperado ao ler imagem selecionada.".to_string())?;

    let png_bytes = decode_base64_simple(b64.trim())
        .ok_or_else(|| "Falha ao decodificar bytes PNG da imagem.".to_string())?;

    Ok(Some(LoadedPortrait {
        file_name: file_name.trim().to_string(),
        png_bytes,
        base64_png: b64.trim().to_string(),
    }))
}

fn decode_base64_simple(input: &str) -> Option<Vec<u8>> {
    fn val(b: u8) -> Option<u8> {
        match b {
            b'A'..=b'Z' => Some(b - b'A'),
            b'a'..=b'z' => Some(b - b'a' + 26),
            b'0'..=b'9' => Some(b - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }

    let clean: Vec<u8> = input.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    if clean.is_empty() || clean.len() % 4 != 0 {
        return None;
    }

    let mut out = Vec::with_capacity((clean.len() / 4) * 3);
    for chunk in clean.chunks_exact(4) {
        let a = val(chunk[0])?;
        let b = val(chunk[1])?;
        out.push((a << 2) | (b >> 4));

        if chunk[2] != b'=' {
            let c = val(chunk[2])?;
            out.push(((b & 0x0f) << 4) | (c >> 2));
            if chunk[3] != b'=' {
                let d = val(chunk[3])?;
                out.push(((c & 0x03) << 6) | d);
            }
        }
    }
    Some(out)
}

pub fn roll_4d6_drop_lowest() -> u8 {
    let mut rng = rand::rng();
    let mut dice = [
        rng.random_range(1..=6u8),
        rng.random_range(1..=6u8),
        rng.random_range(1..=6u8),
        rng.random_range(1..=6u8),
    ];
    dice.sort_unstable();
    dice[1] + dice[2] + dice[3]
}

#[derive(Debug, Clone, Serialize)]
pub struct SolicitarVerificacaoPayload {
    pub login: String,
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub papel: String,
    pub sessao_nome: String,
    pub sessao_sistema: String,
    pub sessao_descricao: String,
    pub codigo_convite: String,
    pub servidor: String,
    pub codigo: String,
    pub enviar_email: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SolicitarVerificacaoResposta {
    pub token: String,
    pub codigo: String,
    pub verify_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CadastroJogadorPayload {
    pub login: String,
    pub nome: String,
    pub senha: String,
    pub codigo_convite: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfirmarVerificacaoPayload {
    pub codigo: Option<String>,
    pub token: Option<String>,
    pub login: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfirmarVerificacaoResposta {
    pub confirmado: bool,
    pub usuario_id: u64,
    pub login: String,
    pub handle: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginPayload {
    pub login: String,
    pub senha: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UsuarioLogado {
    pub id: u64,
    pub login: String,
    pub nome: String,
    pub admin: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginResposta {
    pub token: String,
    pub usuario: UsuarioLogado,
    #[serde(default = "default_sessao_id")]
    pub sessao_id: u64,
    #[serde(default = "default_sessao_nome")]
    pub sessao_nome: String,
    #[serde(default = "default_sessao_sistema")]
    pub sessao_sistema: String,
    #[serde(default = "default_sessao_id")]
    pub players_id: u64,
    #[serde(default)]
    pub tem_ficha: bool,
}

fn default_sessao_id() -> u64 {
    1
}
fn default_sessao_nome() -> String {
    "Crônicas de Kharos".to_string()
}
fn default_sessao_sistema() -> String {
    "D&D".to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateSheetApiPayload {
    pub sessao_id: u64,
    pub players_id: u64,
    pub system: String,
    pub sheet: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SavedSheetResponse {
    pub id: u64,
    pub sessao_id: u64,
    pub players_id: u64,
    pub system: String,
}

#[derive(Debug, Deserialize)]
struct ApiErrorBody {
    error: Option<String>,
}

async fn http_json_request(
    host_port: &str,
    method: &str,
    path: &str,
    json_body: Option<&str>,
) -> Result<(u16, String), String> {
    http_json_request_with_auth(host_port, method, path, None, json_body).await
}

async fn http_json_request_with_auth(
    host_port: &str,
    method: &str,
    path: &str,
    bearer_token: Option<&str>,
    json_body: Option<&str>,
) -> Result<(u16, String), String> {
    let addr = host_port
        .trim()
        .strip_prefix("http://")
        .or_else(|| host_port.trim().strip_prefix("https://"))
        .unwrap_or(host_port.trim())
        .trim_end_matches('/');

    let mut stream = TcpStream::connect(addr)
        .await
        .map_err(|e| format!("Não foi possível conectar a {addr}: {e}"))?;

    let auth_header = match bearer_token {
        Some(t) if !t.trim().is_empty() => format!("Authorization: Bearer {}\r\n", t.trim()),
        _ => String::new(),
    };

    let body_bytes = json_body.unwrap_or("").as_bytes();
    let req = if json_body.is_some() {
        format!(
            "{method} {path} HTTP/1.1\r\n\
             Host: {addr}\r\n\
             {auth_header}\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\r\n",
            body_bytes.len()
        )
    } else {
        format!(
            "{method} {path} HTTP/1.1\r\n\
             Host: {addr}\r\n\
             {auth_header}\
             Connection: close\r\n\r\n"
        )
    };

    stream
        .write_all(req.as_bytes())
        .await
        .map_err(|e| format!("Erro ao enviar cabeçalho HTTP: {e}"))?;
    if !body_bytes.is_empty() {
        stream
            .write_all(body_bytes)
            .await
            .map_err(|e| format!("Erro ao enviar corpo HTTP: {e}"))?;
    }
    stream
        .flush()
        .await
        .map_err(|e| format!("Erro no flush HTTP: {e}"))?;

    let mut raw = Vec::new();
    stream
        .read_to_end(&mut raw)
        .await
        .map_err(|e| format!("Erro ao ler resposta HTTP: {e}"))?;

    let texto = String::from_utf8_lossy(&raw);
    let (headers, body) = texto
        .split_once("\r\n\r\n")
        .ok_or_else(|| "Resposta HTTP malformada.".to_string())?;

    let status_line = headers.lines().next().unwrap_or("");
    let status_code = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(500);

    Ok((status_code, body.trim().to_string()))
}

fn main_server_addrs() -> (String, String) {
    let host = AppConfig::load(DEFAULT_CONFIG_PATH)
        .map(|cfg| cfg.database.host)
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    let clean_host = host.trim();
    (format!("{clean_host}:8080"), format!("{clean_host}:8081"))
}

pub async fn register_and_send_confirmation_email(
    to_email: String,
    username: String,
    password: String,
    session_name: String,
    session_system: String,
    session_description: String,
    confirmation_code: String,
) -> Result<SolicitarVerificacaoResposta, String> {
    let (api_addr, verifier_addr) = main_server_addrs();
    let payload = SolicitarVerificacaoPayload {
        login: username.trim().to_lowercase(),
        nome: username.trim().to_string(),
        email: to_email.trim().to_string(),
        senha: password,
        papel: "Mestre (Host)".to_string(),
        sessao_nome: session_name.trim().to_string(),
        sessao_sistema: session_system.trim().to_string(),
        sessao_descricao: session_description.trim().to_string(),
        codigo_convite: session_name.trim().to_string(),
        servidor: api_addr.clone(),
        codigo: confirmation_code.clone(),
        enviar_email: false,
    };

    let body_json =
        serde_json::to_string(&payload).map_err(|e| format!("Erro ao serializar JSON: {e}"))?;

    let (status, resp_body) = http_json_request(
        &verifier_addr,
        "POST",
        "/verify/solicitar",
        Some(&body_json),
    )
    .await?;

    if !(200..300).contains(&status) {
        let msg = serde_json::from_str::<ApiErrorBody>(&resp_body)
            .ok()
            .and_then(|b| b.error)
            .unwrap_or_else(|| format!("Erro HTTP {status} no verificador"));
        return Err(msg);
    }

    let verif: SolicitarVerificacaoResposta = serde_json::from_str(&resp_body)
        .map_err(|e| format!("Resposta inválida do RpgEmailVerifier: {e}"))?;

    let app_config = AppConfig::load(DEFAULT_CONFIG_PATH)
        .map_err(|e| format!("Falha ao ler '{}': {}", DEFAULT_CONFIG_PATH, e))?;

    let smtp = app_config
        .smtp
        .ok_or_else(|| "Seção 'smtp' não configurada no config.json.".to_string())?;

    send_with_config(
        &smtp,
        &to_email,
        &username,
        &session_name,
        &api_addr,
        &verif.codigo,
        &verif.token,
        &verif.verify_url,
    )
    .await?;

    Ok(verif)
}

pub async fn register_player_direct(
    username: String,
    password: String,
    invite_code: String,
) -> Result<UsuarioLogado, String> {
    let (api_addr, _) = main_server_addrs();
    let payload = CadastroJogadorPayload {
        login: username.trim().to_lowercase(),
        nome: username.trim().to_string(),
        senha: password,
        codigo_convite: Some(invite_code.trim().to_string()),
    };

    let body_json =
        serde_json::to_string(&payload).map_err(|e| format!("Erro ao serializar cadastro: {e}"))?;

    let (status, resp_body) =
        http_json_request(&api_addr, "POST", "/auth/cadastro", Some(&body_json)).await?;

    if !(200..300).contains(&status) {
        let msg = serde_json::from_str::<ApiErrorBody>(&resp_body)
            .ok()
            .and_then(|b| b.error)
            .unwrap_or_else(|| format!("Erro ao cadastrar jogador (HTTP {status})"));
        return Err(msg);
    }

    serde_json::from_str::<UsuarioLogado>(&resp_body)
        .map_err(|e| format!("Resposta inválida do servidor no cadastro: {e}"))
}

pub async fn confirm_player_on_verifier(
    username: String,
    code_or_token: String,
) -> Result<ConfirmarVerificacaoResposta, String> {
    let (_, verifier_addr) = main_server_addrs();
    let trimmed = code_or_token.trim().to_string();
    let is_6_digit = trimmed.len() == 6 && trimmed.chars().all(|c| c.is_ascii_digit());

    let payload = ConfirmarVerificacaoPayload {
        codigo: if is_6_digit {
            Some(trimmed.clone())
        } else {
            None
        },
        token: if !is_6_digit { Some(trimmed) } else { None },
        login: username.trim().to_lowercase(),
    };

    let body_json =
        serde_json::to_string(&payload).map_err(|e| format!("Erro ao serializar JSON: {e}"))?;

    let (status, resp_body) = http_json_request(
        &verifier_addr,
        "POST",
        "/verify/confirmar",
        Some(&body_json),
    )
    .await?;

    if !(200..300).contains(&status) {
        let msg = serde_json::from_str::<ApiErrorBody>(&resp_body)
            .ok()
            .and_then(|b| b.error)
            .unwrap_or_else(|| format!("Código inválido ou expirado (HTTP {status})"));
        return Err(msg);
    }

    serde_json::from_str::<ConfirmarVerificacaoResposta>(&resp_body)
        .map_err(|e| format!("Erro ao ler confirmação do servidor: {e}"))
}

pub async fn check_verification_status(
    token: String,
) -> Result<ConfirmarVerificacaoResposta, String> {
    let (_, verifier_addr) = main_server_addrs();
    let path = format!("/verify/status?token={}", token.trim());
    let (status, resp_body) = http_json_request(&verifier_addr, "GET", &path, None).await?;

    if !(200..300).contains(&status) {
        return Err("Ainda aguardando confirmação.".to_string());
    }

    serde_json::from_str::<ConfirmarVerificacaoResposta>(&resp_body)
        .map_err(|e| format!("Erro ao consultar status: {e}"))
}

pub async fn authenticate_player(
    username: String,
    password: String,
) -> Result<LoginResposta, String> {
    let (api_addr, _) = main_server_addrs();
    let payload = LoginPayload {
        login: username.trim().to_lowercase(),
        senha: password,
    };

    let body_json =
        serde_json::to_string(&payload).map_err(|e| format!("Erro ao serializar login: {e}"))?;

    let (status, resp_body) =
        http_json_request(&api_addr, "POST", "/auth/login", Some(&body_json)).await?;

    if !(200..300).contains(&status) {
        let msg = serde_json::from_str::<ApiErrorBody>(&resp_body)
            .ok()
            .and_then(|b| b.error)
            .unwrap_or_else(|| format!("Falha na autenticação (HTTP {status})"));
        return Err(msg);
    }

    serde_json::from_str::<LoginResposta>(&resp_body)
        .map_err(|e| format!("Resposta inválida do servidor de login: {e}"))
}

pub async fn save_character_sheet(
    auth_token: String,
    sessao_id: u64,
    players_id: u64,
    system: String,
    character_name: String,
    race: String,
    class_profile: String,
    alignment: String,
    sheet_json: serde_json::Value,
) -> Result<SavedSheetResponse, String> {
    let (api_addr, _) = main_server_addrs();
    let payload = CreateSheetApiPayload {
        sessao_id,
        players_id,
        system,
        sheet: sheet_json,
    };

    let body_json =
        serde_json::to_string(&payload).map_err(|e| format!("Erro ao serializar ficha: {e}"))?;

    let (status, resp_body) = http_json_request_with_auth(
        &api_addr,
        "POST",
        "/api/players_sheets",
        Some(&auth_token),
        Some(&body_json),
    )
    .await?;

    if !(200..300).contains(&status) {
        let msg = serde_json::from_str::<ApiErrorBody>(&resp_body)
            .ok()
            .and_then(|b| b.error)
            .unwrap_or_else(|| format!("Erro ao salvar ficha (HTTP {status})"));
        return Err(msg);
    }

    let saved: SavedSheetResponse = serde_json::from_str(&resp_body)
        .map_err(|e| format!("Resposta inválida ao salvar ficha: {e}"))?;

    let update_player = serde_json::json!({
        "name": character_name.trim(),
        "resume": format!("{} — {} ({})", race, class_profile, alignment),
    });
    if let Ok(update_json) = serde_json::to_string(&update_player) {
        let _ = http_json_request_with_auth(
            &api_addr,
            "PUT",
            &format!("/api/sessao_players/{players_id}"),
            Some(&auth_token),
            Some(&update_json),
        )
        .await;
    }

    Ok(saved)
}

#[allow(clippy::too_many_arguments)]
async fn send_with_config(
    smtp: &SmtpConfig,
    to_email: &str,
    username: &str,
    session_name: &str,
    server: &str,
    confirmation_code: &str,
    token: &str,
    verify_url: &str,
) -> Result<(), String> {
    let from_mailbox: Mailbox = format!("{} <{}>", smtp.from_name, smtp.from_email)
        .parse()
        .map_err(|e| format!("Endereço remetente inválido: {}", e))?;

    let to_mailbox: Mailbox = format!("{} <{}>", username, to_email)
        .parse()
        .or_else(|_| to_email.parse())
        .map_err(|e| format!("Endereço de destino inválido: {}", e))?;

    let body = format!(
        "Saudações, Mestre {username}!\n\n\
         A criação da sessão '{session_name}' e do seu usuário de Mestre (Host) no RPGASYNCST foi iniciada para o servidor {server}.\n\n\
         ========================================================\n\
         1. CONFIRMAR PELO NAVEGADOR (WEBNAVIGATOR NeXTSTEP):\n\
            Clique no link abaixo para validar sua conta de Mestre e ativar a sessão:\n\
            {verify_url}\n\
            (Token de Ativação: {token})\n\n\
         2. OU CONFIRMAR PELO APLICATIVO DESKTOP:\n\
            Código de confirmação (6 dígitos): {confirmation_code}\n\
         ========================================================\n\n\
         Se você não solicitou este cadastro, ignore este e-mail.\n\n\
         -- Equipe AsyncTTRPG96"
    );

    let email = Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject(format!(
            "[RPGASYNCST] Código de Confirmação: {} ({})",
            confirmation_code, token
        ))
        .body(body)
        .map_err(|e| format!("Erro ao montar e-mail: {}", e))?;

    let mut builder = if smtp.use_tls {
        if smtp.port == 465 {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp.host)
                .map_err(|e| format!("Erro ao configurar TLS SMTP: {}", e))?
                .port(smtp.port)
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp.host)
                .map_err(|e| format!("Erro ao configurar STARTTLS SMTP: {}", e))?
                .port(smtp.port)
        }
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp.host).port(smtp.port)
    };

    if !smtp.username.trim().is_empty() {
        builder = builder.credentials(Credentials::new(
            smtp.username.clone(),
            smtp.password.clone(),
        ));
    }

    let mailer = builder.build();

    mailer
        .send(email)
        .await
        .map_err(|e| format!("Falha ao enviar e-mail via SMTP: {}", e))?;

    tracing::info!(
        "E-mail de confirmação enviado para '{}' (usuário '{}', token '{}').",
        to_email,
        username,
        token
    );

    Ok(())
}
