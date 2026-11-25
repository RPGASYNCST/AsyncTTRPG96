use iced::widget::text_editor;

use crate::ui::sprite::{
    CloakColor, FacialFeature, HairColor, HairStyle, HeadFeature, OutfitColor, OutfitStyle,
    SkinTone, WeaponType,
};

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
    TabPressed(bool),

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
