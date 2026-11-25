use iced::Task;

use crate::ui::app::RetroApp;
use crate::ui::models::{Message, SessionVisibility};

impl RetroApp {
    pub(crate) fn update_session(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SessionNameChanged(val) => {
                self.session_name = val;
            }
            Message::SessionSystemChanged(val) => {
                self.session_system = val;
            }
            Message::SessionHostChanged(val) => {
                self.session_host = val;
            }
            Message::SessionEmailChanged(val) => {
                self.session_email = val;
            }
            Message::SessionPasswordChanged(val) => {
                self.session_password = val;
            }
            Message::SessionMaxPlayersChanged(val) => {
                self.session_max_players = val;
            }
            Message::SessionVisibilityChanged(vis) => {
                self.session_visibility = vis;
            }
            Message::SessionHelp => {
                self.session_status = (
                    "Defina a sessão, o Mestre (Host), e-mail e chave para validar sua conta."
                        .to_string(),
                    None,
                );
            }
            Message::SubmitCreateSession => {
                if self.session_is_sending {
                    return Task::none();
                }

                let name = self.session_name.trim();
                let system = self.session_system.as_str();
                let host = self.session_host.trim();
                let email = self.session_email.trim();
                let max_players = self.session_max_players.trim().parse::<u32>();

                if name.is_empty() {
                    self.session_status = ("Informe o Nome da Sessão.".to_string(), Some(true));
                } else if host.is_empty() || host.len() < 3 {
                    self.session_status = (
                        "Mestre (Host) deve ter pelo menos 3 caracteres.".to_string(),
                        Some(true),
                    );
                } else if email.is_empty() || !email.contains('@') || !email.contains('.') {
                    self.session_status = (
                        "Informe um E-mail do Mestre válido.".to_string(),
                        Some(true),
                    );
                } else if self.session_password.len() < 6 {
                    self.session_status = (
                        "A Chave do Mestre deve ter mín. 6 caracteres.".to_string(),
                        Some(true),
                    );
                } else if !matches!(max_players, Ok(n) if (1..=32).contains(&n)) {
                    self.session_status = (
                        "Máx. de jogadores deve ser entre 1 e 32.".to_string(),
                        Some(true),
                    );
                } else {
                    let code = crate::mail::generate_confirmation_code();
                    self.session_expected_code = Some(code.clone());
                    self.session_is_sending = true;
                    let vis_label = match self.session_visibility {
                        SessionVisibility::Private => "Privada (Convite)",
                        SessionVisibility::Public => "Pública",
                    };
                    let description = format!(
                        "Mestre: {} | Máx. Jogadores: {} | Visibilidade: {}",
                        host,
                        self.session_max_players.trim(),
                        vis_label
                    );
                    self.session_status = (
                        format!(
                            "Registrando sessão '{}' e enviando e-mail para {}...",
                            name, email
                        ),
                        None,
                    );

                    return Task::perform(
                        crate::mail::register_and_send_confirmation_email(
                            email.to_string(),
                            host.to_string(),
                            self.session_password.clone(),
                            name.to_string(),
                            system.to_string(),
                            description,
                            code,
                        ),
                        Message::SessionEmailSent,
                    );
                }
            }
            Message::SessionEmailSent(result) => {
                self.session_is_sending = false;
                match result {
                    Ok(verif) => {
                        self.session_expected_code = Some(verif.codigo);
                        self.session_verification_token = Some(verif.token.clone());
                        self.session_awaiting_confirmation = true;
                        self.session_code_input.clear();
                        self.session_status = (
                            format!(
                                "E-mail enviado! Clique no link ({}) ou digite o código de 6 dígitos.",
                                verif.token
                            ),
                            Some(false),
                        );
                    }
                    Err(err) => {
                        self.session_status = (err, Some(true));
                    }
                }
            }
            Message::SessionCodeInputChanged(val) => {
                self.session_code_input = val;
            }
            Message::VerifySessionCode => {
                if self.session_is_sending {
                    return Task::none();
                }
                let entered = self.session_code_input.trim().to_string();
                let u = self.session_host.trim().to_string();

                if entered.is_empty() {
                    if let Some(token) = self.session_verification_token.clone() {
                        self.session_is_sending = true;
                        self.session_status = (
                            "Consultando se o link do e-mail já foi confirmado...".to_string(),
                            None,
                        );
                        return Task::perform(
                            crate::mail::check_verification_status(token),
                            Message::SessionCodeVerified,
                        );
                    }
                    self.session_status = (
                        "Digite o código de 6 dígitos ou clique no link do e-mail.".to_string(),
                        Some(true),
                    );
                } else {
                    self.session_is_sending = true;
                    self.session_status = (
                        "Validando código e criando sessão/Mestre no servidor principal..."
                            .to_string(),
                        None,
                    );
                    return Task::perform(
                        crate::mail::confirm_player_on_verifier(u, entered),
                        Message::SessionCodeVerified,
                    );
                }
            }
            Message::SessionCodeVerified(result) => {
                self.session_is_sending = false;
                match result {
                    Ok(resp) if resp.confirmado => {
                        self.session_confirmed = true;
                        self.session_awaiting_confirmation = false;
                        self.username = self.session_host.trim().to_string();
                        self.access_key = self.session_password.clone();
                        self.session_status = (
                            format!(
                                "Sessão '{}' e Mestre '{}' (id={}) criados! Clique em [ Ir para Login ↵ ].",
                                self.session_name.trim(),
                                resp.handle,
                                resp.usuario_id
                            ),
                            Some(false),
                        );
                    }
                    Ok(_) => {
                        self.session_status = (
                            "Ainda aguardando confirmação pelo link ou código de 6 dígitos."
                                .to_string(),
                            Some(true),
                        );
                    }
                    Err(err) => {
                        self.session_status = (err, Some(true));
                    }
                }
            }
            Message::ResendSessionEmail => {
                if self.session_is_sending {
                    return Task::none();
                }
                let code = crate::mail::generate_confirmation_code();
                self.session_expected_code = Some(code.clone());
                self.session_is_sending = true;
                let email = self.session_email.trim().to_string();
                let host = self.session_host.trim().to_string();
                let pass = self.session_password.clone();
                let name = self.session_name.trim().to_string();
                let system = self.session_system.as_str().to_string();
                let vis_label = match self.session_visibility {
                    SessionVisibility::Private => "Privada (Convite)",
                    SessionVisibility::Public => "Pública",
                };
                let description = format!(
                    "Mestre: {} | Máx. Jogadores: {} | Visibilidade: {}",
                    host,
                    self.session_max_players.trim(),
                    vis_label
                );
                self.session_status = (
                    format!("Reenviando e-mail de confirmação para {}...", email),
                    None,
                );
                return Task::perform(
                    crate::mail::register_and_send_confirmation_email(
                        email,
                        host,
                        pass,
                        name,
                        system,
                        description,
                        code,
                    ),
                    Message::SessionEmailSent,
                );
            }
            _ => {}
        }
        Task::none()
    }
}
