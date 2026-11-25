use iced::{Size, Task};

use crate::ui::app::RetroApp;
use crate::ui::models::{Message, Screen};

impl RetroApp {
    pub(crate) fn update_auth(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UsernameChanged(val) => {
                self.username = val;
            }
            Message::AccessKeyChanged(val) => {
                self.access_key = val;
            }
            Message::Connect => {
                if self.login_in_progress {
                    return Task::none();
                }
                let u = self.username.trim();
                let p = &self.access_key;
                if u.is_empty() {
                    self.status_is_error = true;
                    self.status_info = Some("Informe o usuário.".to_string());
                } else if p.is_empty() {
                    self.status_is_error = true;
                    self.status_info = Some("Informe a senha.".to_string());
                } else {
                    self.login_in_progress = true;
                    self.status_is_error = false;
                    self.status_info = Some(format!("Autenticando '{}'...", u));
                    return Task::perform(
                        crate::mail::authenticate_player(u.to_string(), p.clone()),
                        Message::LoginFinished,
                    );
                }
            }
            Message::LoginFinished(result) => {
                self.login_in_progress = false;
                match result {
                    Ok(resp) => {
                        self.auth_token = Some(resp.token);
                        self.active_sessao_id = resp.sessao_id;
                        self.active_sessao_nome = resp.sessao_nome.clone();
                        self.active_sessao_sistema = resp.sessao_sistema;
                        self.active_players_id = resp.players_id;
                        self.status_is_error = false;

                        if !resp.tem_ficha {
                            self.screen = Screen::CreateSheet;
                            self.sheet_is_saving = false;
                            self.sheet_saved = false;
                            if self.sheet_name.trim().is_empty() {
                                self.sheet_name = resp.usuario.nome.clone();
                            }
                            self.sheet_status = (
                                "Pronto para salvar e submeter a ficha para a mesa.".to_string(),
                                None,
                            );
                            return iced::window::oldest()
                                .and_then(|id| iced::window::resize(id, Size::new(700.0, 520.0)));
                        }

                        self.status_info = Some(format!(
                            "Conectado à mesa '{}' como {}!",
                            resp.sessao_nome, resp.usuario.nome
                        ));
                    }
                    Err(err) => {
                        self.status_is_error = true;
                        self.status_info = Some(err);
                    }
                }
            }
            Message::Options => {
                self.status_is_error = false;
                self.status_info =
                    Some("Servidor: 127.0.0.1:8080 | Verificador: :8081".to_string());
            }

            Message::PlayerUsernameChanged(val) => {
                self.player_username = val;
            }
            Message::PlayerPasswordChanged(val) => {
                self.player_password = val;
            }
            Message::PlayerConfirmPasswordChanged(val) => {
                self.player_confirm_password = val;
            }
            Message::PlayerInviteCodeChanged(val) => {
                self.player_invite_code = val;
            }
            Message::PlayerHelp => {
                self.player_status = (
                    "Preencha seu usuário, chave de acesso e código de convite da mesa."
                        .to_string(),
                    None,
                );
            }
            Message::SubmitCreatePlayer => {
                if self.player_is_sending {
                    return Task::none();
                }

                let u = self.player_username.trim();
                let c = self.player_invite_code.trim();

                if u.is_empty() {
                    self.player_status = ("Informe o Handle / Usuário.".to_string(), Some(true));
                } else if u.len() < 3 {
                    self.player_status = (
                        "Usuário deve ter pelo menos 3 caracteres.".to_string(),
                        Some(true),
                    );
                } else if self.player_password.is_empty() {
                    self.player_status = ("Informe a Chave de Acesso.".to_string(), Some(true));
                } else if self.player_password.len() < 6 {
                    self.player_status = (
                        "Chave de Acesso deve ter mín. 6 caracteres.".to_string(),
                        Some(true),
                    );
                } else if self.player_password != self.player_confirm_password {
                    self.player_status =
                        ("As Chaves de Acesso não coincidem.".to_string(), Some(true));
                } else if c.is_empty() {
                    self.player_status = (
                        "Informe o Código de Convite da mesa.".to_string(),
                        Some(true),
                    );
                } else {
                    self.player_is_sending = true;
                    self.player_status = (
                        format!("Cadastrando jogador '{}' no servidor principal...", u),
                        None,
                    );

                    return Task::perform(
                        crate::mail::register_player_direct(
                            u.to_string(),
                            self.player_password.clone(),
                            c.to_string(),
                        ),
                        Message::PlayerCreated,
                    );
                }
            }
            Message::PlayerCreated(result) => {
                self.player_is_sending = false;
                match result {
                    Ok(usuario) => {
                        self.player_confirmed = true;
                        self.username = self.player_username.trim().to_string();
                        self.access_key = self.player_password.clone();
                        self.player_status = (
                            format!(
                                "Jogador '{}' (id={}) cadastrado com sucesso! Clique em [ Ir para Login ↵ ].",
                                usuario.nome, usuario.id
                            ),
                            Some(false),
                        );
                    }
                    Err(err) => {
                        self.player_status = (err, Some(true));
                    }
                }
            }
            _ => {}
        }
        Task::none()
    }
}
