use iced::widget::{
    Space, button, column, container, image, mouse_area, pick_list, radio, row, text, text_editor,
    text_input,
};
use iced::{Alignment, Element, Length, Size, Task};

use crate::ui::sprite::{
    CloakColor, FacialFeature, HairColor, HairStyle, HeadFeature, OutfitColor, OutfitStyle,
    SkinTone, SpriteConfig, WeaponType, render_sprite_png,
};
use crate::ui::theme::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Login,
    CreateSession,
    CreatePlayer,
    CreateSheet,
    SpriteEditor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RpgSystem {
    #[default]
    DnD,
}

impl RpgSystem {
    pub const ALL: &'static [RpgSystem] = &[RpgSystem::DnD];

    pub fn as_str(&self) -> &'static str {
        match self {
            RpgSystem::DnD => "D&D",
        }
    }
}

impl std::fmt::Display for RpgSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DndRace {
    #[default]
    Elfo,
    Humano,
    Anao,
    Halfling,
    MeioElfo,
    MeioOrc,
    Tiefling,
    Draconato,
    Gnomo,
}

impl DndRace {
    pub const ALL: &'static [DndRace] = &[
        DndRace::Elfo,
        DndRace::Humano,
        DndRace::Anao,
        DndRace::Halfling,
        DndRace::MeioElfo,
        DndRace::MeioOrc,
        DndRace::Tiefling,
        DndRace::Draconato,
        DndRace::Gnomo,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            DndRace::Elfo => "Elfo",
            DndRace::Humano => "Humano",
            DndRace::Anao => "Anão",
            DndRace::Halfling => "Halfling",
            DndRace::MeioElfo => "Meio-Elfo",
            DndRace::MeioOrc => "Meio-Orc",
            DndRace::Tiefling => "Tiefling",
            DndRace::Draconato => "Draconato",
            DndRace::Gnomo => "Gnomo",
        }
    }
}

impl std::fmt::Display for DndRace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DndClass {
    #[default]
    Ladino,
    Guerreiro,
    Mago,
    Clerigo,
    Paladino,
    Patrulheiro,
    Barbaro,
    Bardo,
    Bruxo,
    Druida,
    Feiticeiro,
    Monge,
}

impl DndClass {
    pub const ALL: &'static [DndClass] = &[
        DndClass::Ladino,
        DndClass::Guerreiro,
        DndClass::Mago,
        DndClass::Clerigo,
        DndClass::Paladino,
        DndClass::Patrulheiro,
        DndClass::Barbaro,
        DndClass::Bardo,
        DndClass::Bruxo,
        DndClass::Druida,
        DndClass::Feiticeiro,
        DndClass::Monge,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            DndClass::Ladino => "Ladino (Rogue)",
            DndClass::Guerreiro => "Guerreiro (Fighter)",
            DndClass::Mago => "Mago (Wizard)",
            DndClass::Clerigo => "Clérigo (Cleric)",
            DndClass::Paladino => "Paladino (Paladin)",
            DndClass::Patrulheiro => "Patrulheiro (Ranger)",
            DndClass::Barbaro => "Bárbaro (Barbarian)",
            DndClass::Bardo => "Bardo (Bard)",
            DndClass::Bruxo => "Bruxo (Warlock)",
            DndClass::Druida => "Druida (Druid)",
            DndClass::Feiticeiro => "Feiticeiro (Sorcerer)",
            DndClass::Monge => "Monge (Monk)",
        }
    }
}

impl std::fmt::Display for DndClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DndAlignment {
    #[default]
    CaoticoNeutro,
    LealBom,
    NeutroBom,
    CaoticoBom,
    LealNeutro,
    NeutroPuro,
    LealMau,
    NeutroMau,
    CaoticoMau,
}

impl DndAlignment {
    pub const ALL: &'static [DndAlignment] = &[
        DndAlignment::CaoticoNeutro,
        DndAlignment::LealBom,
        DndAlignment::NeutroBom,
        DndAlignment::CaoticoBom,
        DndAlignment::LealNeutro,
        DndAlignment::NeutroPuro,
        DndAlignment::LealMau,
        DndAlignment::NeutroMau,
        DndAlignment::CaoticoMau,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            DndAlignment::CaoticoNeutro => "Caótico e Neutro",
            DndAlignment::LealBom => "Leal e Bom",
            DndAlignment::NeutroBom => "Neutro e Bom",
            DndAlignment::CaoticoBom => "Caótico e Bom",
            DndAlignment::LealNeutro => "Leal e Neutro",
            DndAlignment::NeutroPuro => "Neutro (Puro)",
            DndAlignment::LealMau => "Leal e Mau",
            DndAlignment::NeutroMau => "Neutro e Mau",
            DndAlignment::CaoticoMau => "Caótico e Mau",
        }
    }
}

impl std::fmt::Display for DndAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SessionVisibility {
    #[default]
    Private,
    Public,
}

#[derive(Debug, Clone)]
pub enum Message {
    DragWindow,
    Minimize,
    Close,

    NavigateTo(Screen),

    UsernameChanged(String),
    AccessKeyChanged(String),
    Connect,
    LoginFinished(Result<crate::mail::LoginResposta, String>),
    Options,

    SessionNameChanged(String),
    SessionSystemChanged(RpgSystem),
    SessionHostChanged(String),
    SessionEmailChanged(String),
    SessionPasswordChanged(String),
    SessionMaxPlayersChanged(String),
    SessionVisibilityChanged(SessionVisibility),
    SessionHelp,
    SubmitCreateSession,
    SessionEmailSent(Result<crate::mail::SolicitarVerificacaoResposta, String>),
    SessionCodeInputChanged(String),
    VerifySessionCode,
    SessionCodeVerified(Result<crate::mail::ConfirmarVerificacaoResposta, String>),
    ResendSessionEmail,

    PlayerUsernameChanged(String),
    PlayerPasswordChanged(String),
    PlayerConfirmPasswordChanged(String),
    PlayerInviteCodeChanged(String),
    PlayerHelp,
    SubmitCreatePlayer,
    PlayerCreated(Result<crate::mail::UsuarioLogado, String>),

    SheetNameChanged(String),
    SheetRaceChanged(DndRace),
    SheetClassChanged(DndClass),
    SheetAlignmentChanged(DndAlignment),
    SheetItemsChanged(String),
    SheetGoldChanged(String),
    SheetBioAction(text_editor::Action),
    SheetAttrForChanged(String),
    SheetAttrDesChanged(String),
    SheetAttrConChanged(String),
    SheetAttrIntChanged(String),
    SheetAttrSabChanged(String),
    SheetAttrCarChanged(String),
    SheetRollAttributes,
    SheetOpenSpriteEditor,
    SheetLoadPortrait,
    SheetPortraitLoaded(Result<Option<crate::mail::LoadedPortrait>, String>),
    SheetHelp,
    SubmitCreateSheet,
    SheetSaved(Result<crate::mail::SavedSheetResponse, String>),

    SpriteSkinChanged(SkinTone),
    SpriteHeadChanged(HeadFeature),
    SpriteHairStyleChanged(HairStyle),
    SpriteHairColorChanged(HairColor),
    SpriteFacialChanged(FacialFeature),
    SpriteOutfitStyleChanged(OutfitStyle),
    SpriteOutfitColorChanged(OutfitColor),
    SpriteCloakChanged(CloakColor),
    SpriteWeaponChanged(WeaponType),
    SpriteRandomize,
    SpriteResetDefault,
    SpriteApplyToSheet,
}

pub const RETRO_HEADER_FONT: iced::Font = iced::Font {
    family: iced::font::Family::Name("Silkscreen"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const RETRO_UI_FONT: iced::Font = iced::Font::with_name("VT323");

pub struct RetroApp {
    screen: Screen,

    d20_handle: image::Handle,
    logo_handle: image::Handle,
    portrait_handle: image::Handle,

    active_sessao_id: u64,
    active_sessao_nome: String,
    active_sessao_sistema: String,
    active_players_id: u64,

    username: String,
    access_key: String,
    status_info: Option<String>,
    status_is_error: bool,
    login_in_progress: bool,
    auth_token: Option<String>,

    session_name: String,
    session_system: RpgSystem,
    session_host: String,
    session_email: String,
    session_password: String,
    session_max_players: String,
    session_visibility: SessionVisibility,
    session_status: (String, Option<bool>),
    session_is_sending: bool,
    session_awaiting_confirmation: bool,
    session_expected_code: Option<String>,
    session_verification_token: Option<String>,
    session_code_input: String,
    session_confirmed: bool,

    player_username: String,
    player_password: String,
    player_confirm_password: String,
    player_invite_code: String,
    player_status: (String, Option<bool>),
    player_is_sending: bool,
    player_confirmed: bool,

    sheet_name: String,
    sheet_race: DndRace,
    sheet_class: DndClass,
    sheet_alignment: DndAlignment,
    sheet_items: String,
    sheet_gold: String,
    sheet_bio: text_editor::Content,
    sheet_attr_for: String,
    sheet_attr_des: String,
    sheet_attr_con: String,
    sheet_attr_int: String,
    sheet_attr_sab: String,
    sheet_attr_car: String,
    sheet_portrait_filename: Option<String>,
    sheet_portrait_base64: Option<String>,
    sheet_status: (String, Option<bool>),
    sheet_is_saving: bool,
    sheet_saved: bool,

    sprite_config: SpriteConfig,
    sprite_preview_handle: image::Handle,
    sprite_preview_b64: String,
    sprite_status: String,
}

impl Default for RetroApp {
    fn default() -> Self {
        Self::new()
    }
}

impl RetroApp {
    pub fn new() -> Self {
        let default_sprite = SpriteConfig::default();
        let (_, _, sprite_png, sprite_b64) = render_sprite_png(&default_sprite);
        Self {
            screen: Screen::Login,
            d20_handle: image::Handle::from_bytes(
                &include_bytes!("../../assets/icons/d20.png")[..],
            ),
            logo_handle: image::Handle::from_bytes(
                &include_bytes!("../../assets/icons/logo.png")[..],
            ),
            portrait_handle: image::Handle::from_bytes(
                &include_bytes!("../../assets/icons/portrait_default.png")[..],
            ),
            active_sessao_id: 1,
            active_sessao_nome: "Crônicas de Kharos".to_string(),
            active_sessao_sistema: "D&D".to_string(),
            active_players_id: 1,
            username: String::new(),
            access_key: String::new(),
            status_info: None,
            status_is_error: true,
            login_in_progress: false,
            auth_token: None,
            session_name: String::new(),
            session_system: RpgSystem::DnD,
            session_host: String::new(),
            session_email: String::new(),
            session_password: String::new(),
            session_max_players: "5".to_string(),
            session_visibility: SessionVisibility::Private,
            session_status: ("Aguardando configuração da nova sessão.".to_string(), None),
            session_is_sending: false,
            session_awaiting_confirmation: false,
            session_expected_code: None,
            session_verification_token: None,
            session_code_input: String::new(),
            session_confirmed: false,
            player_username: String::new(),
            player_password: String::new(),
            player_confirm_password: String::new(),
            player_invite_code: String::new(),
            player_status: (
                "Aguardando preenchimento das credenciais do jogador.".to_string(),
                None,
            ),
            player_is_sending: false,
            player_confirmed: false,
            sheet_name: "Eldrin Sombra".to_string(),
            sheet_race: DndRace::Elfo,
            sheet_class: DndClass::Ladino,
            sheet_alignment: DndAlignment::CaoticoNeutro,
            sheet_items: "Espada Curta, Armadura de Couro, Gazetas, Tochas, Ração".to_string(),
            sheet_gold: "120".to_string(),
            sheet_bio: text_editor::Content::with_text(
                "Órfão das ruas de Kharos, sobrevivente dos becos de engrenagens. \
Especialista em obter o que não lhe pertence e caçar segredos da fenda temporal...",
            ),
            sheet_attr_for: "14".to_string(),
            sheet_attr_des: "16".to_string(),
            sheet_attr_con: "12".to_string(),
            sheet_attr_int: "12".to_string(),
            sheet_attr_sab: "10".to_string(),
            sheet_attr_car: "11".to_string(),
            sheet_portrait_filename: None,
            sheet_portrait_base64: None,
            sheet_status: (
                "Pronto para salvar e submeter a ficha para a mesa.".to_string(),
                None,
            ),
            sheet_is_saving: false,
            sheet_saved: false,
            sprite_config: default_sprite,
            sprite_preview_handle: image::Handle::from_bytes(sprite_png),
            sprite_preview_b64: sprite_b64,
            sprite_status:
                "Personalize as camadas 16-bit do seu personagem e clique em [ Aplicar Sprite ↵ ]."
                    .to_string(),
        }
    }

    fn refresh_sprite_preview(&mut self) {
        let (_, _, png_bytes, b64) = render_sprite_png(&self.sprite_config);
        self.sprite_preview_handle = image::Handle::from_bytes(png_bytes);
        self.sprite_preview_b64 = b64;
    }

    pub fn title(&self) -> String {
        match self.screen {
            Screen::Login => "Sessão de Acesso - RPG ASYNC".to_string(),
            Screen::CreateSession => "RPGASYNCST — Criar Sessão".to_string(),
            Screen::CreatePlayer => "RPGASYNCST — Criar Jogador".to_string(),
            Screen::CreateSheet => {
                format!("RPGASYNCST - Criar Ficha: {}", self.active_sessao_nome)
            }
            Screen::SpriteEditor => {
                format!(
                    "RPGASYNCST - Editor de Sprite 16-Bit: {}",
                    self.sheet_name.trim()
                )
            }
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DragWindow => {
                return iced::window::oldest().and_then(iced::window::drag);
            }
            Message::Minimize => {
                return iced::window::oldest().and_then(|id| iced::window::minimize(id, true));
            }
            Message::Close => {
                std::process::exit(0);
            }

            Message::NavigateTo(screen) => {
                self.screen = screen;
                self.status_info = None;
                let target_size = match screen {
                    Screen::Login => Size::new(396.0, 310.0),
                    Screen::CreateSession => {
                        if !self.username.trim().is_empty() && self.session_host.is_empty() {
                            self.session_host = self.username.clone();
                        }
                        self.session_is_sending = false;
                        self.session_awaiting_confirmation = false;
                        self.session_confirmed = false;
                        self.session_code_input.clear();
                        self.session_status =
                            ("Aguardando configuração da nova sessão.".to_string(), None);
                        Size::new(540.0, 425.0)
                    }
                    Screen::CreatePlayer => {
                        self.player_is_sending = false;
                        self.player_confirmed = false;
                        self.player_status = (
                            "Aguardando preenchimento das credenciais do jogador.".to_string(),
                            None,
                        );
                        Size::new(540.0, 345.0)
                    }
                    Screen::CreateSheet => {
                        self.sheet_is_saving = false;
                        self.sheet_saved = false;
                        self.sheet_status = (
                            "Pronto para salvar e submeter a ficha para a mesa.".to_string(),
                            None,
                        );
                        Size::new(700.0, 520.0)
                    }
                    Screen::SpriteEditor => {
                        self.refresh_sprite_preview();
                        Size::new(660.0, 495.0)
                    }
                };
                return iced::window::oldest()
                    .and_then(move |id| iced::window::resize(id, target_size));
            }

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

            Message::SheetNameChanged(val) => {
                self.sheet_name = val;
            }
            Message::SheetRaceChanged(val) => {
                self.sheet_race = val;
            }
            Message::SheetClassChanged(val) => {
                self.sheet_class = val;
            }
            Message::SheetAlignmentChanged(val) => {
                self.sheet_alignment = val;
            }
            Message::SheetItemsChanged(val) => {
                self.sheet_items = val;
            }
            Message::SheetGoldChanged(val) => {
                self.sheet_gold = val;
            }
            Message::SheetBioAction(action) => {
                self.sheet_bio.perform(action);
            }
            Message::SheetAttrForChanged(val) => {
                self.sheet_attr_for = val;
            }
            Message::SheetAttrDesChanged(val) => {
                self.sheet_attr_des = val;
            }
            Message::SheetAttrConChanged(val) => {
                self.sheet_attr_con = val;
            }
            Message::SheetAttrIntChanged(val) => {
                self.sheet_attr_int = val;
            }
            Message::SheetAttrSabChanged(val) => {
                self.sheet_attr_sab = val;
            }
            Message::SheetAttrCarChanged(val) => {
                self.sheet_attr_car = val;
            }
            Message::SheetRollAttributes => {
                let f = crate::mail::roll_4d6_drop_lowest();
                let d = crate::mail::roll_4d6_drop_lowest();
                let c = crate::mail::roll_4d6_drop_lowest();
                let i = crate::mail::roll_4d6_drop_lowest();
                let s = crate::mail::roll_4d6_drop_lowest();
                let ca = crate::mail::roll_4d6_drop_lowest();
                self.sheet_attr_for = f.to_string();
                self.sheet_attr_des = d.to_string();
                self.sheet_attr_con = c.to_string();
                self.sheet_attr_int = i.to_string();
                self.sheet_attr_sab = s.to_string();
                self.sheet_attr_car = ca.to_string();
                self.sheet_status = (
                    format!(
                        "Rolagem 4d6d1 concluída: FOR={f}, DES={d}, CON={c}, INT={i}, SAB={s}, CAR={ca}."
                    ),
                    Some(false),
                );
            }
            Message::SheetOpenSpriteEditor => {
                self.screen = Screen::SpriteEditor;
                self.refresh_sprite_preview();
                self.sprite_status =
                    "Personalize as camadas 16-bit do seu personagem e clique em [ Aplicar Sprite ↵ ]."
                        .to_string();
                return iced::window::oldest()
                    .and_then(|id| iced::window::resize(id, Size::new(660.0, 495.0)));
            }
            Message::SheetLoadPortrait => {
                self.sheet_status = (
                    "Selecione uma imagem (.png, .jpg, .bmp, .gif) para o retrato do seu personagem..."
                        .to_string(),
                    None,
                );
                return Task::perform(
                    crate::mail::pick_portrait_image(),
                    Message::SheetPortraitLoaded,
                );
            }
            Message::SheetPortraitLoaded(result) => match result {
                Ok(Some(loaded)) => {
                    self.portrait_handle = image::Handle::from_bytes(loaded.png_bytes);
                    self.sheet_portrait_filename = Some(loaded.file_name.clone());
                    self.sheet_portrait_base64 = Some(loaded.base64_png);
                    self.sheet_status = (
                        format!(
                            "Imagem '{}' carregada como retrato do personagem!",
                            loaded.file_name
                        ),
                        Some(false),
                    );
                }
                Ok(None) => {
                    self.sheet_status = ("Seleção de imagem cancelada.".to_string(), None);
                }
                Err(err) => {
                    self.sheet_status = (err, Some(true));
                }
            },
            Message::SheetHelp => {
                self.sheet_status = (
                    "Defina identidade, atributos (ou role 4d6d1), equipamentos e biografia."
                        .to_string(),
                    None,
                );
            }
            Message::SubmitCreateSheet => {
                if self.sheet_is_saving {
                    return Task::none();
                }

                let nome_p = self.sheet_name.trim();
                if nome_p.is_empty() {
                    self.sheet_status = (
                        "Informe o Nome do Personagem antes de salvar.".to_string(),
                        Some(true),
                    );
                    return Task::none();
                }

                let parse_attr = |s: &str| -> Option<u8> {
                    s.trim().parse::<u8>().ok().filter(|v| (1..=30).contains(v))
                };

                let (Some(a_for), Some(a_des), Some(a_con), Some(a_int), Some(a_sab), Some(a_car)) = (
                    parse_attr(&self.sheet_attr_for),
                    parse_attr(&self.sheet_attr_des),
                    parse_attr(&self.sheet_attr_con),
                    parse_attr(&self.sheet_attr_int),
                    parse_attr(&self.sheet_attr_sab),
                    parse_attr(&self.sheet_attr_car),
                ) else {
                    self.sheet_status = (
                        "Todos os atributos (FOR, DES, CON, INT, SAB, CAR) devem ser entre 1 e 30."
                            .to_string(),
                        Some(true),
                    );
                    return Task::none();
                };

                let Ok(po) = self.sheet_gold.trim().parse::<u32>() else {
                    self.sheet_status = (
                        "Informe um valor numérico válido para Moedas de Ouro (PO).".to_string(),
                        Some(true),
                    );
                    return Task::none();
                };

                let Some(token) = self.auth_token.clone() else {
                    self.sheet_status = (
                        "Sessão expirada. Volte ao login e conecte novamente.".to_string(),
                        Some(true),
                    );
                    return Task::none();
                };

                let bio_text = self.sheet_bio.text();
                let sheet_json = serde_json::json!({
                    "nome_personagem": nome_p,
                    "raca": self.sheet_race.as_str(),
                    "classe": self.sheet_class.as_str(),
                    "alinhamento": self.sheet_alignment.as_str(),
                    "atributos": {
                        "forca": a_for,
                        "destreza": a_des,
                        "constituicao": a_con,
                        "inteligencia": a_int,
                        "sabedoria": a_sab,
                        "carisma": a_car
                    },
                    "equipamento": {
                        "itens_iniciais": self.sheet_items.trim(),
                        "moedas_ouro_po": po
                    },
                    "biografia": bio_text.trim(),
                    "retrato": {
                        "arquivo": self.sheet_portrait_filename.as_deref().unwrap_or("portrait_default.png"),
                        "png_base64": self.sheet_portrait_base64,
                        "sprite_config": self.sprite_config
                    }
                });

                self.sheet_is_saving = true;
                self.sheet_status = (
                    format!(
                        "Salvando e submetendo a ficha de '{}' para a mesa '{}'...",
                        nome_p, self.active_sessao_nome
                    ),
                    None,
                );

                return Task::perform(
                    crate::mail::save_character_sheet(
                        token,
                        self.active_sessao_id,
                        self.active_players_id,
                        self.active_sessao_sistema.clone(),
                        nome_p.to_string(),
                        self.sheet_race.as_str().to_string(),
                        self.sheet_class.as_str().to_string(),
                        self.sheet_alignment.as_str().to_string(),
                        sheet_json,
                    ),
                    Message::SheetSaved,
                );
            }
            Message::SheetSaved(result) => {
                self.sheet_is_saving = false;
                match result {
                    Ok(saved) => {
                        self.sheet_saved = true;
                        self.sheet_status = (
                            format!(
                                "Ficha de '{}' (id={}) salva com sucesso na mesa '{}'!",
                                self.sheet_name.trim(),
                                saved.id,
                                self.active_sessao_nome
                            ),
                            Some(false),
                        );
                    }
                    Err(err) => {
                        self.sheet_status = (err, Some(true));
                    }
                }
            }

            Message::SpriteSkinChanged(val) => {
                self.sprite_config.skin = val;
                self.refresh_sprite_preview();
            }
            Message::SpriteHeadChanged(val) => {
                self.sprite_config.head_feature = val;
                self.refresh_sprite_preview();
            }
            Message::SpriteHairStyleChanged(val) => {
                self.sprite_config.hair_style = val;
                self.refresh_sprite_preview();
            }
            Message::SpriteHairColorChanged(val) => {
                self.sprite_config.hair_color = val;
                self.refresh_sprite_preview();
            }
            Message::SpriteFacialChanged(val) => {
                self.sprite_config.facial = val;
                self.refresh_sprite_preview();
            }
            Message::SpriteOutfitStyleChanged(val) => {
                self.sprite_config.outfit_style = val;
                self.refresh_sprite_preview();
            }
            Message::SpriteOutfitColorChanged(val) => {
                self.sprite_config.outfit_color = val;
                self.refresh_sprite_preview();
            }
            Message::SpriteCloakChanged(val) => {
                self.sprite_config.cloak = val;
                self.refresh_sprite_preview();
            }
            Message::SpriteWeaponChanged(val) => {
                self.sprite_config.weapon = val;
                self.refresh_sprite_preview();
            }
            Message::SpriteRandomize => {
                self.sprite_config = SpriteConfig::randomize();
                self.refresh_sprite_preview();
                self.sprite_status = format!(
                    "Sprite aleatório gerado: {} | {} | {}.",
                    self.sprite_config.skin.as_str(),
                    self.sprite_config.outfit_style.as_str(),
                    self.sprite_config.weapon.as_str()
                );
            }
            Message::SpriteResetDefault => {
                self.sprite_config = SpriteConfig::default();
                self.refresh_sprite_preview();
                self.sprite_status = "Sprite restaurado para o modelo padrão.".to_string();
            }
            Message::SpriteApplyToSheet => {
                let (_, _, png_bytes, b64) = render_sprite_png(&self.sprite_config);
                self.portrait_handle = image::Handle::from_bytes(png_bytes);
                self.sheet_portrait_filename = Some("custom_sprite_16bit.png".to_string());
                self.sheet_portrait_base64 = Some(b64);
                self.screen = Screen::CreateSheet;
                self.sheet_status = (
                    "Sprite 16-bit personalizado aplicado ao retrato da ficha!".to_string(),
                    Some(false),
                );
                return iced::window::oldest()
                    .and_then(|id| iced::window::resize(id, Size::new(700.0, 520.0)));
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let (min_label, title_text, close_label) = match self.screen {
            Screen::Login => ("—", "Sessão de Acesso - RPG ASYNC".to_string(), "✕"),
            Screen::CreateSession => ("[-]", "RPGASYNCST — Criar Sessão".to_string(), "✕"),
            Screen::CreatePlayer => ("[-]", "RPGASYNCST — Criar Jogador".to_string(), "✕"),
            Screen::CreateSheet => (
                "[-]",
                format!("RPGASYNCST - Criar Ficha: {}", self.active_sessao_nome),
                "✕",
            ),
            Screen::SpriteEditor => (
                "[-]",
                format!(
                    "RPGASYNCST - Editor de Sprite 16-Bit: {}",
                    self.sheet_name.trim()
                ),
                "✕",
            ),
        };

        let minimize_btn = button(
            container(text(min_label).size(15).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .width(24)
        .height(22)
        .padding(0)
        .style(title_button_style)
        .on_press(Message::Minimize);

        let title_center = mouse_area(
            container(
                text(title_text)
                    .size(18)
                    .font(RETRO_UI_FONT)
                    .color(TEXT_LIGHT),
            )
            .width(Length::Fill)
            .height(22)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(title_bar_segment),
        )
        .on_press(Message::DragWindow);

        let close_btn = button(
            container(text(close_label).size(14).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .width(24)
        .height(22)
        .padding(0)
        .style(title_button_style)
        .on_press(Message::Close);

        let title_bar = row![minimize_btn, title_center, close_btn]
            .spacing(1)
            .align_y(Alignment::Center);

        let content = match self.screen {
            Screen::Login => self.view_login(),
            Screen::CreateSession => self.view_create_session(),
            Screen::CreatePlayer => self.view_create_player(),
            Screen::CreateSheet => self.view_create_sheet(),
            Screen::SpriteEditor => self.view_sprite_editor(),
        };

        container(column![title_bar, content])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(outer_window_frame)
            .into()
    }

    fn view_login(&self) -> Element<'_, Message> {
        let d20_icon = image(self.d20_handle.clone())
            .width(50)
            .height(56)
            .filter_method(image::FilterMethod::Nearest);

        let top_header_box = etched_panel(
            row![
                d20_icon,
                Space::new().width(12),
                text("SESSÃO DE ACESSO")
                    .size(20)
                    .font(RETRO_HEADER_FONT)
                    .color(TEXT_DARK),
            ]
            .align_y(Alignment::Center),
            [10, 14],
        );

        let user_input = sunken_3d(
            text_input("", &self.username)
                .font(RETRO_UI_FONT)
                .size(18)
                .style(login_input_style)
                .padding([3, 6])
                .on_input(Message::UsernameChanged),
            INPUT_BG,
            1.5,
        );

        let key_input = sunken_3d(
            text_input("", &self.access_key)
                .font(RETRO_UI_FONT)
                .size(18)
                .style(login_input_style)
                .secure(true)
                .padding([3, 6])
                .on_input(Message::AccessKeyChanged),
            INPUT_BG,
            1.5,
        );

        let left_fields = column![
            text("Usuário")
                .size(19)
                .font(RETRO_UI_FONT)
                .color(TEXT_DARK),
            Space::new().height(3),
            user_input,
            Space::new().height(12),
            text("Senha").size(19).font(RETRO_UI_FONT).color(TEXT_DARK),
            Space::new().height(3),
            key_input,
            if let Some(msg) = &self.status_info {
                let color = if self.status_is_error {
                    STATUS_ERROR
                } else {
                    STATUS_SUCCESS
                };
                Element::from(column![
                    Space::new().height(4),
                    text(msg).size(15).font(RETRO_UI_FONT).color(color),
                ])
            } else {
                Space::new().height(0).into()
            }
        ]
        .width(205);

        let connect_btn = button(
            container(text("Conectar").size(18).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(default_button_style)
        .width(114)
        .padding([4, 8])
        .on_press(Message::Connect);

        let sheet_btn = button(
            container(text("Criar sessão").size(18).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(114)
        .padding([4, 8])
        .on_press(Message::NavigateTo(Screen::CreateSession));

        let player_btn = button(
            container(text("Criar jogador").size(18).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(114)
        .padding([4, 8])
        .on_press(Message::NavigateTo(Screen::CreatePlayer));

        let options_btn = button(
            container(text("Opções").size(18).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(114)
        .padding([4, 8])
        .on_press(Message::Options);

        let right_buttons = column![
            connect_btn,
            Space::new().height(6),
            sheet_btn,
            Space::new().height(6),
            player_btn,
            Space::new().height(6),
            options_btn,
        ]
        .align_x(Alignment::Center);

        let bottom_form_box = etched_panel(
            row![left_fields, Space::new().width(22), right_buttons].align_y(Alignment::Center),
            [14, 16],
        );

        container(column![
            top_header_box,
            Space::new().height(4),
            bottom_form_box,
        ])
        .padding(6)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn field_row<'a>(
        label: &'a str,
        label_width: f32,
        input: Element<'a, Message>,
    ) -> Element<'a, Message> {
        row![
            container(text(label).size(17).font(RETRO_UI_FONT).color(TEXT_DARK))
                .width(label_width)
                .align_y(Alignment::Center),
            input,
        ]
        .spacing(6)
        .align_y(Alignment::Center)
        .into()
    }

    fn view_create_session(&self) -> Element<'_, Message> {
        let logo_icon = image(self.logo_handle.clone())
            .width(96)
            .height(96)
            .filter_method(image::FilterMethod::Linear);

        let logo_col = container(logo_icon)
            .padding([12, 6])
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        let session_group = group_box(
            "1. Dados da Sessão & Mestre",
            RETRO_UI_FONT,
            column![
                Self::field_row(
                    "Nome da Sessão:",
                    122.0,
                    sunken_3d(
                        text_input("Crônicas de Kharos", &self.session_name)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .padding([2, 5])
                            .on_input(Message::SessionNameChanged),
                        INPUT_BG,
                        1.5,
                    )
                ),
                Self::field_row(
                    "Opção de RPG:",
                    122.0,
                    sunken_3d(
                        pick_list(
                            RpgSystem::ALL,
                            Some(self.session_system),
                            Message::SessionSystemChanged,
                        )
                        .font(RETRO_UI_FONT)
                        .text_size(17)
                        .width(Length::Fill)
                        .padding([2, 5])
                        .style(next_pick_list_style)
                        .menu_style(next_pick_list_menu_style),
                        INPUT_BG,
                        1.5,
                    )
                ),
                Self::field_row(
                    "Mestre (Host):",
                    122.0,
                    sunken_3d(
                        text_input("Lip1Z", &self.session_host)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .padding([2, 5])
                            .on_input(Message::SessionHostChanged),
                        INPUT_BG,
                        1.5,
                    )
                ),
                Self::field_row(
                    "E-mail do Mestre:",
                    122.0,
                    sunken_3d(
                        text_input("mestre@chronos.local", &self.session_email)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .padding([2, 5])
                            .on_input(Message::SessionEmailChanged),
                        INPUT_BG,
                        1.5,
                    )
                ),
                Self::field_row(
                    "Chave do Mestre:",
                    122.0,
                    sunken_3d(
                        text_input("", &self.session_password)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .secure(true)
                            .padding([2, 5])
                            .on_input(Message::SessionPasswordChanged),
                        INPUT_BG,
                        1.5,
                    )
                ),
            ]
            .spacing(5),
            [12, 10],
        );

        let top_section = row![logo_col, Space::new().width(8), session_group]
            .align_y(Alignment::Center)
            .width(Length::Fill);

        let node_group = if self.session_awaiting_confirmation {
            group_box(
                "2. Confirmação por E-mail do Mestre",
                RETRO_UI_FONT,
                column![
                    text("Um código de 6 dígitos e link foram enviados para o e-mail do Mestre.")
                        .size(16)
                        .font(RETRO_UI_FONT)
                        .color(TEXT_DARK),
                    Self::field_row(
                        "Código (6 dígitos):",
                        155.0,
                        sunken_3d(
                            text_input("000000", &self.session_code_input)
                                .font(RETRO_UI_FONT)
                                .size(17)
                                .style(login_input_style)
                                .padding([2, 5])
                                .on_input(Message::SessionCodeInputChanged),
                            INPUT_BG,
                            1.5,
                        )
                    ),
                ]
                .spacing(6),
                [12, 10],
            )
        } else {
            let vis_radios = row![
                radio(
                    "Privada (Convite)",
                    SessionVisibility::Private,
                    Some(self.session_visibility),
                    Message::SessionVisibilityChanged,
                )
                .size(14)
                .text_size(17)
                .font(RETRO_UI_FONT)
                .style(next_radio_style),
                Space::new().width(20),
                radio(
                    "Pública",
                    SessionVisibility::Public,
                    Some(self.session_visibility),
                    Message::SessionVisibilityChanged,
                )
                .size(14)
                .text_size(17)
                .font(RETRO_UI_FONT)
                .style(next_radio_style),
            ]
            .align_y(Alignment::Center);

            group_box(
                "2. Parâmetros da Mesa",
                RETRO_UI_FONT,
                column![
                    Self::field_row(
                        "Máx. de Jogadores:",
                        155.0,
                        sunken_3d(
                            text_input("5", &self.session_max_players)
                                .font(RETRO_UI_FONT)
                                .size(17)
                                .style(login_input_style)
                                .padding([2, 5])
                                .on_input(Message::SessionMaxPlayersChanged),
                            INPUT_BG,
                            1.5,
                        )
                    ),
                    row![
                        container(
                            text("Visibilidade da Mesa:")
                                .size(17)
                                .font(RETRO_UI_FONT)
                                .color(TEXT_DARK)
                        )
                        .width(155.0)
                        .align_y(Alignment::Center),
                        vis_radios,
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                ]
                .spacing(5),
                [12, 10],
            )
        };

        let (status_msg, status_err) = &self.session_status;
        let status_color = match status_err {
            Some(true) => STATUS_ERROR,
            Some(false) => STATUS_SUCCESS,
            None => TEXT_DARK,
        };
        let status_bar = status_bar_box(
            text(format!("Status: {}", status_msg))
                .size(16)
                .font(RETRO_UI_FONT)
                .color(status_color),
        );

        let left_btn = if self.session_awaiting_confirmation {
            let mut b = button(
                container(
                    text(if self.session_is_sending {
                        "[ Enviando... ]"
                    } else {
                        "[ Reenviar E-mail ]"
                    })
                    .size(17)
                    .font(RETRO_UI_FONT),
                )
                .align_x(Alignment::Center),
            )
            .style(login_button_style)
            .padding([4, 10]);

            if !self.session_is_sending {
                b = b.on_press(Message::ResendSessionEmail);
            }
            b
        } else {
            button(
                container(text("[ Ajuda ]").size(17).font(RETRO_UI_FONT))
                    .align_x(Alignment::Center),
            )
            .style(login_button_style)
            .padding([4, 10])
            .on_press(Message::SessionHelp)
        };

        let cancel_btn = button(
            container(text("[ Cancelar ]").size(17).font(RETRO_UI_FONT)).align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([4, 10])
        .on_press(Message::NavigateTo(Screen::Login));

        let submit_btn = if self.session_confirmed {
            button(
                container(text("[ Ir para Login ↵ ]").size(17).font(RETRO_UI_FONT))
                    .align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([4, 12])
            .on_press(Message::NavigateTo(Screen::Login))
        } else if self.session_awaiting_confirmation {
            let mut b = button(
                container(text("[ Validar Código ↵ ]").size(17).font(RETRO_UI_FONT))
                    .align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([4, 12]);
            if !self.session_is_sending {
                b = b.on_press(Message::VerifySessionCode);
            }
            b
        } else {
            let mut b = button(
                container(
                    text(if self.session_is_sending {
                        "[ Enviando... ]"
                    } else {
                        "[ Criar Sessão ↵ ]"
                    })
                    .size(17)
                    .font(RETRO_UI_FONT),
                )
                .align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([4, 12]);
            if !self.session_is_sending {
                b = b.on_press(Message::SubmitCreateSession);
            }
            b
        };

        let action_bar = row![
            left_btn,
            Space::new().width(Length::Fill),
            cancel_btn,
            Space::new().width(8),
            submit_btn,
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill);

        container(column![
            top_section,
            Space::new().height(6),
            node_group,
            Space::new().height(8),
            status_bar,
            Space::new().height(8),
            action_bar,
        ])
        .padding([8, 10])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn view_create_player(&self) -> Element<'_, Message> {
        let logo_icon = image(self.logo_handle.clone())
            .width(96)
            .height(96)
            .filter_method(image::FilterMethod::Linear);

        let logo_col = container(logo_icon)
            .padding([12, 6])
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        let id_group = group_box(
            "1. Identificação do Jogador",
            RETRO_UI_FONT,
            column![
                Self::field_row(
                    "Handle / Usuário:",
                    122.0,
                    sunken_3d(
                        text_input("Kaelen", &self.player_username)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .padding([2, 5])
                            .on_input(Message::PlayerUsernameChanged),
                        INPUT_BG,
                        1.5,
                    )
                ),
                Self::field_row(
                    "Chave de Acesso:",
                    122.0,
                    sunken_3d(
                        text_input("", &self.player_password)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .secure(true)
                            .padding([2, 5])
                            .on_input(Message::PlayerPasswordChanged),
                        INPUT_BG,
                        1.5,
                    )
                ),
                Self::field_row(
                    "Confirmar Chave:",
                    122.0,
                    sunken_3d(
                        text_input("", &self.player_confirm_password)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .secure(true)
                            .padding([2, 5])
                            .on_input(Message::PlayerConfirmPasswordChanged),
                        INPUT_BG,
                        1.5,
                    )
                ),
            ]
            .spacing(5),
            [12, 10],
        );

        let top_section = row![logo_col, Space::new().width(8), id_group]
            .align_y(Alignment::Center)
            .width(Length::Fill);

        let bottom_group = group_box(
            "2. Ingresso na Mesa",
            RETRO_UI_FONT,
            column![Self::field_row(
                "Código de Convite:",
                155.0,
                sunken_3d(
                    text_input("Crônicas de Kharos", &self.player_invite_code)
                        .font(RETRO_UI_FONT)
                        .size(17)
                        .style(login_input_style)
                        .padding([2, 5])
                        .on_input(Message::PlayerInviteCodeChanged),
                    INPUT_BG,
                    1.5,
                )
            ),]
            .spacing(5),
            [12, 10],
        );

        let (status_msg, status_err) = &self.player_status;
        let status_color = match status_err {
            Some(true) => STATUS_ERROR,
            Some(false) => STATUS_SUCCESS,
            None => TEXT_DARK,
        };
        let status_bar = status_bar_box(
            text(format!("Status: {}", status_msg))
                .size(16)
                .font(RETRO_UI_FONT)
                .color(status_color),
        );

        let left_btn = button(
            container(text("[ Ajuda ]").size(17).font(RETRO_UI_FONT)).align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([4, 10])
        .on_press(Message::PlayerHelp);

        let cancel_btn = button(
            container(text("[ Cancelar ]").size(17).font(RETRO_UI_FONT)).align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([4, 10])
        .on_press(Message::NavigateTo(Screen::Login));

        let submit_btn = if self.player_confirmed {
            button(
                container(text("[ Ir para Login ↵ ]").size(17).font(RETRO_UI_FONT))
                    .align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([4, 12])
            .on_press(Message::NavigateTo(Screen::Login))
        } else {
            let mut b = button(
                container(
                    text(if self.player_is_sending {
                        "[ Cadastrando... ]"
                    } else {
                        "[ Criar Jogador ↵ ]"
                    })
                    .size(17)
                    .font(RETRO_UI_FONT),
                )
                .align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([4, 12]);
            if !self.player_is_sending {
                b = b.on_press(Message::SubmitCreatePlayer);
            }
            b
        };

        let action_bar = row![
            left_btn,
            Space::new().width(Length::Fill),
            cancel_btn,
            Space::new().width(8),
            submit_btn,
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill);

        container(column![
            top_section,
            Space::new().height(6),
            bottom_group,
            Space::new().height(8),
            status_bar,
            Space::new().height(8),
            action_bar,
        ])
        .padding([8, 10])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn attr_row<'a>(
        label: &'a str,
        value: &'a str,
        on_change: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message> {
        row![
            container(text(label).size(15).font(RETRO_UI_FONT).color(TEXT_DARK))
                .width(Length::Fill)
                .align_y(Alignment::Center),
            container(sunken_3d(
                text_input("10", value)
                    .font(RETRO_UI_FONT)
                    .size(16)
                    .style(login_input_style)
                    .padding([1, 4])
                    .on_input(on_change),
                INPUT_BG,
                1.5,
            ))
            .width(36),
        ]
        .spacing(4)
        .align_y(Alignment::Center)
        .into()
    }

    fn view_create_sheet(&self) -> Element<'_, Message> {
        let portrait_img = image(self.portrait_handle.clone())
            .width(108)
            .height(118)
            .filter_method(image::FilterMethod::Nearest);

        let display_char_name = if self.sheet_name.trim().is_empty() {
            "Sem Nome"
        } else {
            self.sheet_name.trim()
        };

        let gen_ia_btn = button(
            container(text("[ Customizar ↵ ]").size(15).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(126)
        .padding([2, 6])
        .on_press(Message::SheetOpenSpriteEditor);

        let load_portrait_btn = button(
            container(text("[ Carregar ]").size(15).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(104)
        .padding([2, 6])
        .on_press(Message::SheetLoadPortrait);

        let retrato_panel = sheet_section_panel(
            "Retrato",
            RETRO_UI_FONT,
            column![
                container(portrait_img)
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
                Space::new().height(2),
                container(
                    text(display_char_name)
                        .size(19)
                        .font(RETRO_UI_FONT)
                        .color(TEXT_DARK)
                )
                .width(Length::Fill)
                .align_x(Alignment::Center),
                Space::new().height(3),
                container(gen_ia_btn)
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
                Space::new().height(3),
                container(load_portrait_btn)
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            ]
            .align_x(Alignment::Center),
            [6, 8],
        );

        let roll_btn = button(
            container(text("[ Rolar 4d6d1 🎲 ]").size(15).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(Length::Fill)
        .padding([3, 6])
        .on_press(Message::SheetRollAttributes);

        let atributos_panel = sheet_section_panel(
            "Atributos",
            RETRO_UI_FONT,
            column![
                Self::attr_row(
                    "FOR (Força):",
                    &self.sheet_attr_for,
                    Message::SheetAttrForChanged
                ),
                Self::attr_row(
                    "DES (Destreza):",
                    &self.sheet_attr_des,
                    Message::SheetAttrDesChanged
                ),
                Self::attr_row(
                    "CON (Constituição):",
                    &self.sheet_attr_con,
                    Message::SheetAttrConChanged
                ),
                Self::attr_row(
                    "INT (Inteligência):",
                    &self.sheet_attr_int,
                    Message::SheetAttrIntChanged
                ),
                Self::attr_row(
                    "SAB (Sabedoria):",
                    &self.sheet_attr_sab,
                    Message::SheetAttrSabChanged
                ),
                Self::attr_row(
                    "CAR (Carisma):",
                    &self.sheet_attr_car,
                    Message::SheetAttrCarChanged
                ),
                Space::new().height(4),
                roll_btn,
            ]
            .spacing(3),
            [6, 8],
        );

        let left_col = column![retrato_panel, Space::new().height(6), atributos_panel].width(166);

        let panel_identidade = etched_panel(
            column![
                container(
                    text("Novo Personagem - Registro de Ficha")
                        .size(20)
                        .font(RETRO_UI_FONT)
                        .color(TEXT_DARK)
                )
                .padding([4, 8]),
                horizontal_groove(),
                container(
                    column![
                        text("1. Identidade e Classe")
                            .size(18)
                            .font(RETRO_UI_FONT)
                            .color(TEXT_DARK),
                        Space::new().height(2),
                        Self::field_row(
                            "Nome do Personagem:",
                            138.0,
                            sunken_3d(
                                text_input("Eldrin Sombra", &self.sheet_name)
                                    .font(RETRO_UI_FONT)
                                    .size(17)
                                    .style(login_input_style)
                                    .padding([2, 5])
                                    .on_input(Message::SheetNameChanged),
                                INPUT_BG,
                                1.5,
                            )
                        ),
                        Self::field_row(
                            "Raça / Origem:",
                            138.0,
                            pick_list(
                                DndRace::ALL,
                                Some(self.sheet_race),
                                Message::SheetRaceChanged,
                            )
                            .font(RETRO_UI_FONT)
                            .text_size(17)
                            .width(Length::Fill)
                            .padding([2, 6])
                            .style(sheet_pick_list_style)
                            .menu_style(next_pick_list_menu_style)
                            .into()
                        ),
                        Self::field_row(
                            "Classe / Perfil:",
                            138.0,
                            pick_list(
                                DndClass::ALL,
                                Some(self.sheet_class),
                                Message::SheetClassChanged,
                            )
                            .font(RETRO_UI_FONT)
                            .text_size(17)
                            .width(Length::Fill)
                            .padding([2, 6])
                            .style(sheet_pick_list_style)
                            .menu_style(next_pick_list_menu_style)
                            .into()
                        ),
                        Self::field_row(
                            "Alinhamento:",
                            138.0,
                            pick_list(
                                DndAlignment::ALL,
                                Some(self.sheet_alignment),
                                Message::SheetAlignmentChanged,
                            )
                            .font(RETRO_UI_FONT)
                            .text_size(17)
                            .width(Length::Fill)
                            .padding([2, 6])
                            .style(sheet_pick_list_style)
                            .menu_style(next_pick_list_menu_style)
                            .into()
                        ),
                    ]
                    .spacing(4)
                )
                .padding([6, 8])
                .width(Length::Fill),
            ]
            .width(Length::Fill),
            0,
        );

        let panel_equipamento = sheet_section_panel(
            "2. Equipamento & Recursos",
            RETRO_UI_FONT,
            column![
                Self::field_row(
                    "Itens Iniciais:",
                    138.0,
                    sunken_3d(
                        text_input(
                            "Espada Curta, Armadura de Couro, Gazetas, Tochas, Ração",
                            &self.sheet_items
                        )
                        .font(RETRO_UI_FONT)
                        .size(16)
                        .style(login_input_style)
                        .padding([3, 5])
                        .on_input(Message::SheetItemsChanged),
                        INPUT_BG,
                        1.5,
                    )
                ),
                row![
                    container(
                        text("Moedas de Ouro (PO):")
                            .size(17)
                            .font(RETRO_UI_FONT)
                            .color(TEXT_DARK)
                    )
                    .width(138.0)
                    .align_y(Alignment::Center),
                    container(sunken_3d(
                        text_input("120", &self.sheet_gold)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .padding([2, 5])
                            .on_input(Message::SheetGoldChanged),
                        INPUT_BG,
                        1.5,
                    ))
                    .width(72),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            ]
            .spacing(5),
            [6, 8],
        );

        let help_btn = button(
            container(text("[ Ajuda ]").size(16).font(RETRO_UI_FONT)).align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([3, 8])
        .on_press(Message::SheetHelp);

        let cancel_btn = button(
            container(text("[ Cancelar ]").size(16).font(RETRO_UI_FONT)).align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([3, 8])
        .on_press(Message::NavigateTo(Screen::Login));

        let save_btn = {
            let label = if self.sheet_is_saving {
                "[ Salvando... ]"
            } else if self.sheet_saved {
                "[ Ficha Salva ✔ ]"
            } else {
                "[ Salvar Ficha ↵ ]"
            };
            let mut b = button(
                container(text(label).size(16).font(RETRO_UI_FONT)).align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([3, 10]);
            if !self.sheet_is_saving {
                b = b.on_press(Message::SubmitCreateSheet);
            }
            b
        };

        let bio_editor = sunken_3d(
            text_editor(&self.sheet_bio)
                .font(RETRO_UI_FONT)
                .size(16)
                .height(68)
                .padding([4, 6])
                .style(next_text_editor_style)
                .on_action(Message::SheetBioAction),
            INPUT_BG,
            1.5,
        );

        let panel_biografia = sheet_section_panel(
            "3. Histórico do Personagem (Biografia)",
            RETRO_UI_FONT,
            column![
                bio_editor,
                Space::new().height(5),
                row![
                    help_btn,
                    Space::new().width(Length::Fill),
                    cancel_btn,
                    Space::new().width(6),
                    save_btn,
                ]
                .align_y(Alignment::Center)
                .width(Length::Fill),
            ],
            [6, 8],
        );

        let right_col = column![
            panel_identidade,
            Space::new().height(6),
            panel_equipamento,
            Space::new().height(6),
            panel_biografia,
        ]
        .width(Length::Fill);

        let main_columns = row![left_col, Space::new().width(8), right_col]
            .width(Length::Fill)
            .height(Length::Fill);

        let (status_msg, status_err) = &self.sheet_status;
        let status_color = match status_err {
            Some(true) => STATUS_ERROR,
            Some(false) => STATUS_SUCCESS,
            None => TEXT_DARK,
        };

        let bottom_footer = row![
            container(status_bar_box(
                text(format!("Status: {}", status_msg))
                    .size(16)
                    .font(RETRO_UI_FONT)
                    .color(status_color),
            ))
            .width(Length::Fill),
            Space::new().width(3),
            container(status_bar_box(
                text("RPGASYNCST Daemon v1.4")
                    .size(16)
                    .font(RETRO_UI_FONT)
                    .color(TEXT_DARK),
            ))
            .width(175),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill);

        container(column![
            container(main_columns)
                .padding([6, 8])
                .width(Length::Fill)
                .height(Length::Fill),
            bottom_footer,
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn view_sprite_editor(&self) -> Element<'_, Message> {
        let preview_img = image(self.sprite_preview_handle.clone())
            .width(136)
            .height(152)
            .filter_method(image::FilterMethod::Nearest);

        let display_char_name = if self.sheet_name.trim().is_empty() {
            "Aventureiro"
        } else {
            self.sheet_name.trim()
        };

        let random_btn = button(
            container(text("[ Aleatório 🎲 ]").size(16).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(Length::Fill)
        .padding([4, 6])
        .on_press(Message::SpriteRandomize);

        let reset_btn = button(
            container(text("[ Restaurar Padrão ]").size(16).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(Length::Fill)
        .padding([4, 6])
        .on_press(Message::SpriteResetDefault);

        let preview_panel = sheet_section_panel(
            "Pré-Visualização 16-Bit",
            RETRO_UI_FONT,
            column![
                sunken_3d(
                    container(preview_img)
                        .width(Length::Fill)
                        .padding([8, 6])
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center),
                    NEXT_WINDOW_GRAY,
                    1.5,
                ),
                Space::new().height(6),
                container(
                    text(display_char_name)
                        .size(20)
                        .font(RETRO_UI_FONT)
                        .color(TEXT_DARK)
                )
                .width(Length::Fill)
                .align_x(Alignment::Center),
                container(
                    text(format!(
                        "{} • {}",
                        self.sheet_race.as_str(),
                        self.sheet_class.as_str()
                    ))
                    .size(15)
                    .font(RETRO_UI_FONT)
                    .color(TEXT_MUTED)
                )
                .width(Length::Fill)
                .align_x(Alignment::Center),
                Space::new().height(10),
                random_btn,
                Space::new().height(5),
                reset_btn,
            ]
            .align_x(Alignment::Center),
            [8, 10],
        );

        let left_col = column![preview_panel].width(185);

        let group_cabeca = sheet_section_panel(
            "1. Anatomia, Rosto & Cabelo",
            RETRO_UI_FONT,
            column![
                Self::field_row(
                    "Tom de Pele:",
                    138.0,
                    pick_list(
                        SkinTone::ALL,
                        Some(self.sprite_config.skin),
                        Message::SpriteSkinChanged,
                    )
                    .font(RETRO_UI_FONT)
                    .text_size(16)
                    .width(Length::Fill)
                    .padding([2, 6])
                    .style(sheet_pick_list_style)
                    .menu_style(next_pick_list_menu_style)
                    .into()
                ),
                Self::field_row(
                    "Orelhas / Traços:",
                    138.0,
                    pick_list(
                        HeadFeature::ALL,
                        Some(self.sprite_config.head_feature),
                        Message::SpriteHeadChanged,
                    )
                    .font(RETRO_UI_FONT)
                    .text_size(16)
                    .width(Length::Fill)
                    .padding([2, 6])
                    .style(sheet_pick_list_style)
                    .menu_style(next_pick_list_menu_style)
                    .into()
                ),
                Self::field_row(
                    "Cabelo / Elmo:",
                    138.0,
                    pick_list(
                        HairStyle::ALL,
                        Some(self.sprite_config.hair_style),
                        Message::SpriteHairStyleChanged,
                    )
                    .font(RETRO_UI_FONT)
                    .text_size(16)
                    .width(Length::Fill)
                    .padding([2, 6])
                    .style(sheet_pick_list_style)
                    .menu_style(next_pick_list_menu_style)
                    .into()
                ),
                Self::field_row(
                    "Cor do Cabelo:",
                    138.0,
                    pick_list(
                        HairColor::ALL,
                        Some(self.sprite_config.hair_color),
                        Message::SpriteHairColorChanged,
                    )
                    .font(RETRO_UI_FONT)
                    .text_size(16)
                    .width(Length::Fill)
                    .padding([2, 6])
                    .style(sheet_pick_list_style)
                    .menu_style(next_pick_list_menu_style)
                    .into()
                ),
                Self::field_row(
                    "Barba / Rosto:",
                    138.0,
                    pick_list(
                        FacialFeature::ALL,
                        Some(self.sprite_config.facial),
                        Message::SpriteFacialChanged,
                    )
                    .font(RETRO_UI_FONT)
                    .text_size(16)
                    .width(Length::Fill)
                    .padding([2, 6])
                    .style(sheet_pick_list_style)
                    .menu_style(next_pick_list_menu_style)
                    .into()
                ),
            ]
            .spacing(4),
            [6, 8],
        );

        let cancel_btn = button(
            container(text("[ Voltar p/ Ficha ]").size(16).font(RETRO_UI_FONT))
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([4, 10])
        .on_press(Message::NavigateTo(Screen::CreateSheet));

        let apply_btn = button(
            container(text("[ Aplicar Sprite ↵ ]").size(16).font(RETRO_UI_FONT))
                .align_x(Alignment::Center),
        )
        .style(default_button_style)
        .padding([4, 12])
        .on_press(Message::SpriteApplyToSheet);

        let group_equipamento = sheet_section_panel(
            "2. Traje, Armadura, Capa & Armamento",
            RETRO_UI_FONT,
            column![
                Self::field_row(
                    "Traje / Armadura:",
                    138.0,
                    pick_list(
                        OutfitStyle::ALL,
                        Some(self.sprite_config.outfit_style),
                        Message::SpriteOutfitStyleChanged,
                    )
                    .font(RETRO_UI_FONT)
                    .text_size(16)
                    .width(Length::Fill)
                    .padding([2, 6])
                    .style(sheet_pick_list_style)
                    .menu_style(next_pick_list_menu_style)
                    .into()
                ),
                Self::field_row(
                    "Cor Principal:",
                    138.0,
                    pick_list(
                        OutfitColor::ALL,
                        Some(self.sprite_config.outfit_color),
                        Message::SpriteOutfitColorChanged,
                    )
                    .font(RETRO_UI_FONT)
                    .text_size(16)
                    .width(Length::Fill)
                    .padding([2, 6])
                    .style(sheet_pick_list_style)
                    .menu_style(next_pick_list_menu_style)
                    .into()
                ),
                Self::field_row(
                    "Capa / Costas:",
                    138.0,
                    pick_list(
                        CloakColor::ALL,
                        Some(self.sprite_config.cloak),
                        Message::SpriteCloakChanged,
                    )
                    .font(RETRO_UI_FONT)
                    .text_size(16)
                    .width(Length::Fill)
                    .padding([2, 6])
                    .style(sheet_pick_list_style)
                    .menu_style(next_pick_list_menu_style)
                    .into()
                ),
                Self::field_row(
                    "Arma Equipada:",
                    138.0,
                    pick_list(
                        WeaponType::ALL,
                        Some(self.sprite_config.weapon),
                        Message::SpriteWeaponChanged,
                    )
                    .font(RETRO_UI_FONT)
                    .text_size(16)
                    .width(Length::Fill)
                    .padding([2, 6])
                    .style(sheet_pick_list_style)
                    .menu_style(next_pick_list_menu_style)
                    .into()
                ),
                Space::new().height(8),
                row![
                    Space::new().width(Length::Fill),
                    cancel_btn,
                    Space::new().width(8),
                    apply_btn,
                ]
                .align_y(Alignment::Center)
                .width(Length::Fill),
            ]
            .spacing(4),
            [6, 8],
        );

        let right_col =
            column![group_cabeca, Space::new().height(6), group_equipamento].width(Length::Fill);

        let main_columns = row![left_col, Space::new().width(8), right_col]
            .width(Length::Fill)
            .height(Length::Fill);

        let bottom_footer = row![
            container(status_bar_box(
                text(format!("Status: {}", self.sprite_status))
                    .size(16)
                    .font(RETRO_UI_FONT)
                    .color(TEXT_DARK),
            ))
            .width(Length::Fill),
            Space::new().width(3),
            container(status_bar_box(
                text("Sprite Engine 16-Bit v1.4")
                    .size(16)
                    .font(RETRO_UI_FONT)
                    .color(TEXT_DARK),
            ))
            .width(175),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill);

        container(column![
            container(main_columns)
                .padding([6, 8])
                .width(Length::Fill)
                .height(Length::Fill),
            bottom_footer,
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

pub fn run_gui() -> iced::Result {
    let app_icon =
        iced::window::icon::from_file_data(include_bytes!("../../assets/icons/logo.png"), None)
            .ok();

    iced::application(RetroApp::new, RetroApp::update, RetroApp::view)
        .title(RetroApp::title)
        .font(include_bytes!("../../assets/fonts/Silkscreen-Regular.ttf").as_slice())
        .font(include_bytes!("../../assets/fonts/Silkscreen-Bold.ttf").as_slice())
        .font(include_bytes!("../../assets/fonts/VT323-Regular.ttf").as_slice())
        .default_font(RETRO_UI_FONT)
        .window(iced::window::Settings {
            size: iced::Size::new(396.0, 310.0),
            position: iced::window::Position::Centered,
            resizable: false,
            decorations: false,
            icon: app_icon,
            ..Default::default()
        })
        .run()
}
