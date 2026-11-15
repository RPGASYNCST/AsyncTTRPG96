use rand::RngExt;
use serde::{Deserialize, Serialize};

pub const SPRITE_W: usize = 32;
pub const SPRITE_H: usize = 36;
pub const SCALE: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SkinTone {
    #[default]
    Media,
    Clara,
    Morena,
    EscuraEbano,
    ElficaPalida,
    DouradaSolar,
    CinzaDrow,
    RoxaDrow,
    VerdeOrc,
    VerdeGoblin,
    VermelhaTiefling,
    AzulDraconato,
    DouradaDraconato,
    BrancaGelo,
    BronzeForjado,
    OssoMortoVivo,
}

impl SkinTone {
    pub const ALL: &'static [Self] = &[
        Self::Media,
        Self::Clara,
        Self::Morena,
        Self::EscuraEbano,
        Self::ElficaPalida,
        Self::DouradaSolar,
        Self::CinzaDrow,
        Self::RoxaDrow,
        Self::VerdeOrc,
        Self::VerdeGoblin,
        Self::VermelhaTiefling,
        Self::AzulDraconato,
        Self::DouradaDraconato,
        Self::BrancaGelo,
        Self::BronzeForjado,
        Self::OssoMortoVivo,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Media => "Média / Bronzeada",
            Self::Clara => "Clara / Rosada",
            Self::Morena => "Morena",
            Self::EscuraEbano => "Negra / Ébano",
            Self::ElficaPalida => "Élfica Pálida / Lunar",
            Self::DouradaSolar => "Élfica Solar / Aasimar",
            Self::CinzaDrow => "Cinza Obsidiana (Drow / Duergar)",
            Self::RoxaDrow => "Violeta Noturno (Drow / Feérico)",
            Self::VerdeOrc => "Verde Oliva (Meio-Orc)",
            Self::VerdeGoblin => "Verde Esmeralda (Goblin / Verdan)",
            Self::VermelhaTiefling => "Rubra Infernal (Tiefling)",
            Self::AzulDraconato => "Escama Azul (Draconato / Tritão)",
            Self::DouradaDraconato => "Escama de Ouro (Draconato)",
            Self::BrancaGelo => "Alva Glacial (Goliath / Gelo)",
            Self::BronzeForjado => "Bronze Metálico (Warforged)",
            Self::OssoMortoVivo => "Pálido Morto-Vivo (Dhampir)",
        }
    }

    pub fn colors(&self) -> ([u8; 3], [u8; 3]) {
        match self {
            Self::Media => ([218, 156, 109], [178, 118, 76]),
            Self::Clara => ([245, 205, 174], [208, 160, 128]),
            Self::Morena => ([152, 98, 62], [114, 70, 42]),
            Self::EscuraEbano => ([94, 60, 42], [64, 38, 26]),
            Self::ElficaPalida => ([238, 224, 212], [196, 178, 164]),
            Self::DouradaSolar => ([238, 198, 118], [194, 150, 74]),
            Self::CinzaDrow => ([136, 140, 152], [96, 100, 112]),
            Self::RoxaDrow => ([142, 116, 168], [102, 78, 128]),
            Self::VerdeOrc => ([118, 166, 92], [82, 124, 60]),
            Self::VerdeGoblin => ([86, 186, 104], [54, 138, 70]),
            Self::VermelhaTiefling => ([198, 84, 78], [148, 52, 48]),
            Self::AzulDraconato => ([86, 138, 188], [56, 98, 142]),
            Self::DouradaDraconato => ([216, 172, 68], [164, 124, 38]),
            Self::BrancaGelo => ([214, 232, 245], [166, 190, 212]),
            Self::BronzeForjado => ([184, 134, 82], [136, 92, 48]),
            Self::OssoMortoVivo => ([206, 208, 192], [156, 160, 142]),
        }
    }
}

impl std::fmt::Display for SkinTone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HeadFeature {
    #[default]
    OrelhasElficas,
    OrelhasLongasFeericas,
    Humana,
    ChifresTiefling,
    ChifresNegros,
    ChifresCarneiro,
    PresasOrc,
    HaloCelestial,
    CoroaReal,
    TiaraArcana,
    GalhosDruida,
    BandanaPirata,
    OlhosBrilhantesFogo,
    OlhosBrilhantesGelo,
}

impl HeadFeature {
    pub const ALL: &'static [Self] = &[
        Self::OrelhasElficas,
        Self::OrelhasLongasFeericas,
        Self::Humana,
        Self::ChifresTiefling,
        Self::ChifresNegros,
        Self::ChifresCarneiro,
        Self::PresasOrc,
        Self::HaloCelestial,
        Self::CoroaReal,
        Self::TiaraArcana,
        Self::GalhosDruida,
        Self::BandanaPirata,
        Self::OlhosBrilhantesFogo,
        Self::OlhosBrilhantesGelo,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OrelhasElficas => "Orelhas Pontiagudas (Elfo)",
            Self::OrelhasLongasFeericas => "Orelhas Longas (Feérico / Firbolg)",
            Self::Humana => "Orelhas Humanas / Anão",
            Self::ChifresTiefling => "Chifres Curvos Dourados (Tiefling)",
            Self::ChifresNegros => "Chifres de Obsidiana (Demônio)",
            Self::ChifresCarneiro => "Chifres Enrolados (Minotauro / Sátiro)",
            Self::PresasOrc => "Presas Inferiores (Meio-Orc)",
            Self::HaloCelestial => "Auréola Sagrada (Aasimar / Paladino)",
            Self::CoroaReal => "Coroa de Ouro Real (Nobre / Rei)",
            Self::TiaraArcana => "Tiara c/ Safira (Arcanista)",
            Self::GalhosDruida => "Galhos da Floresta (Druida / Ancião)",
            Self::BandanaPirata => "Bandana Carmesim (Corsário / Monge)",
            Self::OlhosBrilhantesFogo => "Olhos Flamejantes (Bruxo / Fogo)",
            Self::OlhosBrilhantesGelo => "Olhos Astrais Ciano (Místico / Gelo)",
        }
    }
}

impl std::fmt::Display for HeadFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HairStyle {
    #[default]
    LongoAventureiro,
    CurtoGuerreiro,
    RaboDeCavalo,
    TrancasAnao,
    CoqueSamurai,
    LongoOndulado,
    MoicanoBarbaro,
    CarecaMonge,
    LongoElfoLiso,
    FranjaLateral,
    CurtoBaguncado,
    AfroVolumoso,
    DreadlocksLongos,
    ChiquinhasDuplas,
    TrancaUnicaLateral,
    CabeloPartidoMeio,
    TopeteReais,
    CalvoCoroa,
    CoroaDeLouros,
    CapuzLadino,
    ChapeuMago,
    ElmoCavaleiro,
    ElmoViking,
    ElmoEspartano,
    ElmoAladoValquiria,
    ElmoCruzado,
    ChapeuBardo,
    TricornePirata,
    TurbanteDeserto,
    CapuzXamaLobo,
    CapuzNecromante,
    MitraClerigo,
}

impl HairStyle {
    pub const ALL: &'static [Self] = &[
        Self::LongoAventureiro,
        Self::CurtoGuerreiro,
        Self::RaboDeCavalo,
        Self::TrancasAnao,
        Self::CoqueSamurai,
        Self::LongoOndulado,
        Self::MoicanoBarbaro,
        Self::CarecaMonge,
        Self::LongoElfoLiso,
        Self::FranjaLateral,
        Self::CurtoBaguncado,
        Self::AfroVolumoso,
        Self::DreadlocksLongos,
        Self::ChiquinhasDuplas,
        Self::TrancaUnicaLateral,
        Self::CabeloPartidoMeio,
        Self::TopeteReais,
        Self::CalvoCoroa,
        Self::CoroaDeLouros,
        Self::CapuzLadino,
        Self::ChapeuMago,
        Self::ElmoCavaleiro,
        Self::ElmoViking,
        Self::ElmoEspartano,
        Self::ElmoAladoValquiria,
        Self::ElmoCruzado,
        Self::ChapeuBardo,
        Self::TricornePirata,
        Self::TurbanteDeserto,
        Self::CapuzXamaLobo,
        Self::CapuzNecromante,
        Self::MitraClerigo,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LongoAventureiro => "Longo c/ Franja (Aventureiro)",
            Self::CurtoGuerreiro => "Curto Militar (Guerreiro)",
            Self::RaboDeCavalo => "Rabo de Cavalo Alto (Espadachim)",
            Self::TrancasAnao => "Tranças Duplas (Anão / Viking)",
            Self::CoqueSamurai => "Coque de Batalha (Monge / Samurai)",
            Self::LongoOndulado => "Longo Volumoso (Bardo / Feiticeiro)",
            Self::MoicanoBarbaro => "Moicano Selvagem (Bárbaro)",
            Self::CarecaMonge => "Careca / Raspado (Monge)",
            Self::LongoElfoLiso => "Longo Liso Élfico (Realeza)",
            Self::FranjaLateral => "Franja Caída no Olho (Sombrio)",
            Self::CurtoBaguncado => "Curto Espetado / Bagunçado",
            Self::AfroVolumoso => "Black Power / Crespo Volumoso",
            Self::DreadlocksLongos => "Dreadlocks Longos c/ Contas",
            Self::ChiquinhasDuplas => "Maria-Chiquinha Dupla",
            Self::TrancaUnicaLateral => "Trança Longa no Ombro",
            Self::CabeloPartidoMeio => "Médio Partido ao Meio (Cortesão)",
            Self::TopeteReais => "Topete Fidalgo (Pompadour)",
            Self::CalvoCoroa => "Calvo Lateral (Frade / Sábio)",
            Self::CoroaDeLouros => "Cabelo Curto + Coroa de Louros",
            Self::CapuzLadino => "Capuz Sombrio (Ladino / Patrulheiro)",
            Self::ChapeuMago => "Chapéu Pontudo Arcano (Mago)",
            Self::ElmoCavaleiro => "Elmo Fechado de Aço (Cavaleiro)",
            Self::ElmoViking => "Elmo c/ Chifres (Guerreiro do Norte)",
            Self::ElmoEspartano => "Elmo c/ Crista Vermelha (Gladiador)",
            Self::ElmoAladoValquiria => "Elmo Alado Dourado (Valquíria / Paladino)",
            Self::ElmoCruzado => "Grande Elmo Templário (Cruzado)",
            Self::ChapeuBardo => "Chapéu c/ Pluma Branca (Bardo)",
            Self::TricornePirata => "Chapéu Tricórnio (Capitão Pirata)",
            Self::TurbanteDeserto => "Turbante Real c/ Joia (Deserto)",
            Self::CapuzXamaLobo => "Pele de Lobo na Cabeça (Xamã)",
            Self::CapuzNecromante => "Capuz c/ Caveira (Necromante)",
            Self::MitraClerigo => "Mitra Sagrada Dourada (Sumo Clérigo)",
        }
    }
}

impl std::fmt::Display for HairStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HairColor {
    #[default]
    PretoCorvo,
    Castanho,
    CastanhoClaro,
    LoiroDourado,
    LoiroPlatinado,
    RuivoFogo,
    RuivoCobre,
    BrancoPrata,
    CinzaVeterano,
    AzulNoturno,
    AzulCeleste,
    VerdeSilvestre,
    RoxoArcano,
    RosaFeerico,
    CarmesimSangue,
}

impl HairColor {
    pub const ALL: &'static [Self] = &[
        Self::PretoCorvo,
        Self::Castanho,
        Self::CastanhoClaro,
        Self::LoiroDourado,
        Self::LoiroPlatinado,
        Self::RuivoFogo,
        Self::RuivoCobre,
        Self::BrancoPrata,
        Self::CinzaVeterano,
        Self::AzulNoturno,
        Self::AzulCeleste,
        Self::VerdeSilvestre,
        Self::RoxoArcano,
        Self::RosaFeerico,
        Self::CarmesimSangue,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PretoCorvo => "Preto Corvo",
            Self::Castanho => "Castanho Escuro",
            Self::CastanhoClaro => "Castanho Avelã",
            Self::LoiroDourado => "Loiro Dourado",
            Self::LoiroPlatinado => "Loiro Platinado",
            Self::RuivoFogo => "Ruivo Fogo",
            Self::RuivoCobre => "Ruivo Acobreado",
            Self::BrancoPrata => "Branco Prateado",
            Self::CinzaVeterano => "Grisalho Veterano",
            Self::AzulNoturno => "Azul Noturno",
            Self::AzulCeleste => "Azul Ciano / Gelo",
            Self::VerdeSilvestre => "Verde Musgo / Silvestre",
            Self::RoxoArcano => "Roxo Ametista",
            Self::RosaFeerico => "Rosa Feérico",
            Self::CarmesimSangue => "Carmesim Profundo",
        }
    }

    pub fn colors(&self) -> ([u8; 3], [u8; 3]) {
        match self {
            Self::PretoCorvo => ([58, 60, 66], [36, 38, 42]),
            Self::Castanho => ([116, 74, 46], [82, 50, 30]),
            Self::CastanhoClaro => ([158, 112, 68], [118, 80, 46]),
            Self::LoiroDourado => ([226, 188, 78], [182, 142, 46]),
            Self::LoiroPlatinado => ([242, 232, 192], [198, 186, 142]),
            Self::RuivoFogo => ([198, 68, 42], [146, 44, 24]),
            Self::RuivoCobre => ([178, 96, 48], [132, 66, 30]),
            Self::BrancoPrata => ([226, 230, 238], [172, 178, 190]),
            Self::CinzaVeterano => ([142, 146, 154], [98, 102, 110]),
            Self::AzulNoturno => ([58, 92, 158], [38, 62, 114]),
            Self::AzulCeleste => ([92, 196, 232], [58, 144, 182]),
            Self::VerdeSilvestre => ([74, 152, 84], [48, 108, 56]),
            Self::RoxoArcano => ([142, 78, 184], [98, 48, 134]),
            Self::RosaFeerico => ([218, 116, 162], [168, 76, 118]),
            Self::CarmesimSangue => ([156, 34, 46], [106, 20, 28]),
        }
    }
}

impl std::fmt::Display for HairColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FacialFeature {
    #[default]
    Nenhum,
    BarbaLonga,
    BarbaGiganteMago,
    BarbaTrancada,
    BarbaBifurcada,
    BarbaLenhador,
    BarbaRealAparada,
    BarbaCurta,
    BarbaPorFazer,
    CosteletasLongas,
    Cavanhaque,
    CavanhaquePontudo,
    BigodeImperial,
    BigodeFinoMosqueteiro,
    BigodeFerradura,
    BigodeDali,
    MascaraSombra,
    MascaraPrata,
    MascaraCaveira,
    MascaraPeste,
    Cicatriz,
    CicatrizCruzada,
    TapaOlho,
    MonoculoNobre,
    PinturaGuerraAzul,
    PinturaGuerraVermelha,
    TatuagemRunica,
    SardasCampones,
    OculosArtifice,
    VendasOraculo,
}

impl FacialFeature {
    pub const ALL: &'static [Self] = &[
        Self::Nenhum,
        Self::BarbaLonga,
        Self::BarbaGiganteMago,
        Self::BarbaTrancada,
        Self::BarbaBifurcada,
        Self::BarbaLenhador,
        Self::BarbaRealAparada,
        Self::BarbaCurta,
        Self::BarbaPorFazer,
        Self::CosteletasLongas,
        Self::Cavanhaque,
        Self::CavanhaquePontudo,
        Self::BigodeImperial,
        Self::BigodeFinoMosqueteiro,
        Self::BigodeFerradura,
        Self::BigodeDali,
        Self::MascaraSombra,
        Self::MascaraPrata,
        Self::MascaraCaveira,
        Self::MascaraPeste,
        Self::Cicatriz,
        Self::CicatrizCruzada,
        Self::TapaOlho,
        Self::MonoculoNobre,
        Self::PinturaGuerraAzul,
        Self::PinturaGuerraVermelha,
        Self::TatuagemRunica,
        Self::SardasCampones,
        Self::OculosArtifice,
        Self::VendasOraculo,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Nenhum => "Nenhum (Rosto Limpo)",
            Self::BarbaLonga => "Barba Longa (Anão / Mago)",
            Self::BarbaGiganteMago => "Barba Gigante até o Cinto (Arquimago)",
            Self::BarbaTrancada => "Barba Trançada c/ Ouro (Anão)",
            Self::BarbaBifurcada => "Barba Dupla Pontuda (Rei Anão)",
            Self::BarbaLenhador => "Barba Cheia Rústica (Lenhador)",
            Self::BarbaRealAparada => "Barba Real Quadrada c/ Fio Dourado",
            Self::BarbaCurta => "Barba Cerrada (Veterano)",
            Self::BarbaPorFazer => "Barba por Fazer / Sombreada",
            Self::CosteletasLongas => "Costeletas Longas (Mutton Chops)",
            Self::Cavanhaque => "Cavanhaque Nobre",
            Self::CavanhaquePontudo => "Cavanhaque Longo Pontudo (Feiticeiro)",
            Self::BigodeImperial => "Bigode Imperial (Fidalgo)",
            Self::BigodeFinoMosqueteiro => "Bigode Fino + Mosca (Mosqueteiro)",
            Self::BigodeFerradura => "Bigode Ferradura (Guerreiro)",
            Self::BigodeDali => "Bigode Curvado Pontudo (Alquimista)",
            Self::MascaraSombra => "Máscara de Sombra (Assassino)",
            Self::MascaraPrata => "Meia-Máscara de Prata",
            Self::MascaraCaveira => "Máscara de Caveira (Necromante)",
            Self::MascaraPeste => "Máscara de Bico (Médico da Peste)",
            Self::Cicatriz => "Cicatriz Vertical no Olho",
            Self::CicatrizCruzada => "Cicatriz em X na Bochecha",
            Self::TapaOlho => "Tapa-Olho de Couro (Corsário)",
            Self::MonoculoNobre => "Monóculo de Ouro (Lorde / Erudito)",
            Self::PinturaGuerraAzul => "Pintura de Guerra Tribal (Azul)",
            Self::PinturaGuerraVermelha => "Pintura de Sangue (Berserker)",
            Self::TatuagemRunica => "Tatuagem Rúnica Arcana (Roxa)",
            Self::SardasCampones => "Sardas & Bochechas Coradas",
            Self::OculosArtifice => "Óculos de Latão (Artífice)",
            Self::VendasOraculo => "Venda Mística (Oráculo Cego)",
        }
    }
}

impl std::fmt::Display for FacialFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OutfitStyle {
    #[default]
    CouroLadino,
    ArmaduraPlacas,
    CotaDeMalha,
    PaladinoReal,
    MantoArcano,
    TunicaClerigo,
    PeitoralBarbaro,
    VesteMonge,
    CasacoBardo,
    TrajePatrulheiro,
    MantoDruida,
    ArmaduraNegra,
    ColeteArtifice,
    MantoBruxo,
    SobretudoCacador,
    ArmaduraEscamas,
    ArmaduraSamurai,
    BrigandinaMercenario,
    CamisaCampones,
    VestidoNobre,
    CasacoPirata,
    TunicaAssassino,
    ArmaduraGladiador,
    MantoArquimago,
    HabitoInquisidor,
    ArmaduraOsso,
    TrajeAlquimista,
    TunicaDeserto,
    ArmaduraGelo,
    GibaoMosqueteiro,
}

impl OutfitStyle {
    pub const ALL: &'static [Self] = &[
        Self::CouroLadino,
        Self::ArmaduraPlacas,
        Self::CotaDeMalha,
        Self::PaladinoReal,
        Self::MantoArcano,
        Self::TunicaClerigo,
        Self::PeitoralBarbaro,
        Self::VesteMonge,
        Self::CasacoBardo,
        Self::TrajePatrulheiro,
        Self::MantoDruida,
        Self::ArmaduraNegra,
        Self::ColeteArtifice,
        Self::MantoBruxo,
        Self::SobretudoCacador,
        Self::ArmaduraEscamas,
        Self::ArmaduraSamurai,
        Self::BrigandinaMercenario,
        Self::CamisaCampones,
        Self::VestidoNobre,
        Self::CasacoPirata,
        Self::TunicaAssassino,
        Self::ArmaduraGladiador,
        Self::MantoArquimago,
        Self::HabitoInquisidor,
        Self::ArmaduraOsso,
        Self::TrajeAlquimista,
        Self::TunicaDeserto,
        Self::ArmaduraGelo,
        Self::GibaoMosqueteiro,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CouroLadino => "Armadura de Couro Batido (Ladino)",
            Self::ArmaduraPlacas => "Armadura Completa de Placas (Guerreiro)",
            Self::CotaDeMalha => "Cota de Malha de Aço (Cruzado)",
            Self::PaladinoReal => "Placas Douradas c/ Tabardo (Paladino)",
            Self::MantoArcano => "Manto Arcano Bordado (Mago)",
            Self::TunicaClerigo => "Túnica Sagrada c/ Símbolo (Clérigo)",
            Self::PeitoralBarbaro => "Tiras de Combate & Peles (Bárbaro)",
            Self::VesteMonge => "Kimono de Artes Marciais (Monge)",
            Self::CasacoBardo => "Gibão Nobre c/ Babados (Bardo)",
            Self::TrajePatrulheiro => "Túnica de Caçador (Patrulheiro)",
            Self::MantoDruida => "Vestes de Folhas & Vinhas (Druida)",
            Self::ArmaduraNegra => "Armadura de Espinhos (Cavaleiro Negro)",
            Self::ColeteArtifice => "Colete c/ Poções & Engrenagens (Artífice)",
            Self::MantoBruxo => "Vestes Cultistas do Pacto (Bruxo)",
            Self::SobretudoCacador => "Sobretudo Longo de Couro (Caçador de Bruxas)",
            Self::ArmaduraEscamas => "Armadura de Escamas de Dragão",
            Self::ArmaduraSamurai => "Armadura Oriental O-Yoroi (Samurai)",
            Self::BrigandinaMercenario => "Brigandina Rebitada (Mercenário)",
            Self::CamisaCampones => "Camisa de Linho + Suspensório (Taverna)",
            Self::VestidoNobre => "Traje de Gala da Corte Real (Nobre)",
            Self::CasacoPirata => "Casaco de Capitão c/ Ouro (Corsário)",
            Self::TunicaAssassino => "Traje de Mestre Assassino c/ Faixa",
            Self::ArmaduraGladiador => "Peitoral de Arena c/ Ombreira (Gladiador)",
            Self::MantoArquimago => "Toga Astral Estrelada (Arquimago)",
            Self::HabitoInquisidor => "Sobrepeliz Negra c/ Cruz (Inquisidor)",
            Self::ArmaduraOsso => "Couraça de Ossos & Crânio (Necromante)",
            Self::TrajeAlquimista => "Avental de Couro c/ Frascos (Alquimista)",
            Self::TunicaDeserto => "Vestes de Seda do Deserto (Nômade)",
            Self::ArmaduraGelo => "Placas de Cristal Glacial (Cavaleiro Gélido)",
            Self::GibaoMosqueteiro => "Gibão Real c/ Cruz Prateada (Mosqueteiro)",
        }
    }
}

impl std::fmt::Display for OutfitStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OutfitColor {
    #[default]
    MarromCouro,
    AcoPrata,
    FerroEscuro,
    AzulReal,
    AzulGelo,
    VermelhoCarmesim,
    VinhoImperial,
    VerdeFloresta,
    VerdeEsmeralda,
    PretoSombrio,
    DouradoSolar,
    BronzeAntigo,
    RoxoMistico,
    BrancoMarfim,
    LaranjaFogo,
}

impl OutfitColor {
    pub const ALL: &'static [Self] = &[
        Self::MarromCouro,
        Self::AcoPrata,
        Self::FerroEscuro,
        Self::AzulReal,
        Self::AzulGelo,
        Self::VermelhoCarmesim,
        Self::VinhoImperial,
        Self::VerdeFloresta,
        Self::VerdeEsmeralda,
        Self::PretoSombrio,
        Self::DouradoSolar,
        Self::BronzeAntigo,
        Self::RoxoMistico,
        Self::BrancoMarfim,
        Self::LaranjaFogo,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MarromCouro => "Marrom Couro Rústico",
            Self::AcoPrata => "Aço Polido / Prata",
            Self::FerroEscuro => "Ferro Negro / Titânio",
            Self::AzulReal => "Azul Real",
            Self::AzulGelo => "Azul Glacial / Celeste",
            Self::VermelhoCarmesim => "Vermelho Carmesim",
            Self::VinhoImperial => "Bordô / Vinho Imperial",
            Self::VerdeFloresta => "Verde Floresta",
            Self::VerdeEsmeralda => "Verde Esmeralda Vivo",
            Self::PretoSombrio => "Preto Sombrio / Obsidiana",
            Self::DouradoSolar => "Dourado Solar",
            Self::BronzeAntigo => "Bronze Antigo / Cobre",
            Self::RoxoMistico => "Roxo Místico / Arcano",
            Self::BrancoMarfim => "Branco Marfim / Sagrado",
            Self::LaranjaFogo => "Âmbar / Laranja Flamejante",
        }
    }

    pub fn colors(&self) -> ([u8; 3], [u8; 3]) {
        match self {
            Self::MarromCouro => ([122, 86, 54], [88, 58, 34]),
            Self::AcoPrata => ([168, 174, 184], [116, 122, 134]),
            Self::FerroEscuro => ([92, 98, 108], [60, 64, 72]),
            Self::AzulReal => ([62, 104, 176], [40, 70, 128]),
            Self::AzulGelo => ([102, 182, 222], [66, 134, 172]),
            Self::VermelhoCarmesim => ([172, 52, 52], [120, 32, 32]),
            Self::VinhoImperial => ([124, 34, 56], [82, 20, 36]),
            Self::VerdeFloresta => ([64, 132, 74], [42, 92, 48]),
            Self::VerdeEsmeralda => ([52, 172, 108], [32, 122, 74]),
            Self::PretoSombrio => ([62, 62, 68], [38, 38, 44]),
            Self::DouradoSolar => ([208, 168, 64], [156, 120, 38]),
            Self::BronzeAntigo => ([168, 112, 62], [122, 76, 38]),
            Self::RoxoMistico => ([128, 68, 168], [86, 42, 118]),
            Self::BrancoMarfim => ([232, 228, 218], [182, 176, 164]),
            Self::LaranjaFogo => ([214, 112, 42], [162, 74, 24]),
        }
    }
}

impl std::fmt::Display for OutfitColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CloakColor {
    #[default]
    CinzaSombra,
    SemCapa,
    VermelhoReal,
    VerdePatrulheiro,
    AzulMistico,
    PretoNoturno,
    BrancoArminho,
    RoxoImperial,
    DouradoCelestial,
    MarromViajante,
    PeleDeLobo,
    AsasAnjo,
    AsasDemonio,
    MochilaAventureiro,
}

impl CloakColor {
    pub const ALL: &'static [Self] = &[
        Self::CinzaSombra,
        Self::SemCapa,
        Self::VermelhoReal,
        Self::VerdePatrulheiro,
        Self::AzulMistico,
        Self::PretoNoturno,
        Self::BrancoArminho,
        Self::RoxoImperial,
        Self::DouradoCelestial,
        Self::MarromViajante,
        Self::PeleDeLobo,
        Self::AsasAnjo,
        Self::AsasDemonio,
        Self::MochilaAventureiro,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CinzaSombra => "Capa Cinza Sombra",
            Self::SemCapa => "Sem Capa",
            Self::VermelhoReal => "Capa Vermelho Real",
            Self::VerdePatrulheiro => "Capa Verde Patrulheiro",
            Self::AzulMistico => "Capa Azul Mística",
            Self::PretoNoturno => "Capa Preto Noturno",
            Self::BrancoArminho => "Manto Branco de Arminho Real",
            Self::RoxoImperial => "Capa Roxa Imperial",
            Self::DouradoCelestial => "Capa Dourada Solar",
            Self::MarromViajante => "Capa Marrom de Viajante",
            Self::PeleDeLobo => "Manto de Pele de Lobo Cinzento",
            Self::AsasAnjo => "Asas Celestiais Brancas (Aasimar)",
            Self::AsasDemonio => "Asas Dracônicas / Abissais",
            Self::MochilaAventureiro => "Mochila de Couro + Saco de Dormir",
        }
    }

    pub fn colors(&self) -> Option<([u8; 3], [u8; 3])> {
        match self {
            Self::SemCapa | Self::AsasAnjo | Self::AsasDemonio | Self::MochilaAventureiro => None,
            Self::CinzaSombra => Some(([92, 94, 100], [64, 66, 72])),
            Self::VermelhoReal => Some(([156, 42, 46], [108, 26, 30])),
            Self::VerdePatrulheiro => Some(([58, 112, 64], [38, 78, 42])),
            Self::AzulMistico => Some(([56, 86, 148], [36, 58, 106])),
            Self::PretoNoturno => Some(([48, 48, 54], [28, 28, 34])),
            Self::BrancoArminho => Some(([234, 234, 238], [184, 186, 194])),
            Self::RoxoImperial => Some(([118, 54, 152], [78, 32, 106])),
            Self::DouradoCelestial => Some(([212, 172, 66], [162, 124, 38])),
            Self::MarromViajante => Some(([118, 82, 52], [82, 54, 32])),
            Self::PeleDeLobo => Some(([128, 124, 118], [86, 82, 76])),
        }
    }
}

impl std::fmt::Display for CloakColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WeaponType {
    #[default]
    AdagaDupla,
    DuasAdagas,
    EspadaLonga,
    EspadaMontante,
    Katana,
    Rapieira,
    EspadaFlamejante,
    MachadoBatalha,
    MarteloGuerra,
    MacaEstrela,
    LancaAlabarda,
    FoiceSombria,
    CajadoArcano,
    CajadoDruida,
    OrbeGrimorio,
    ArcoCaca,
    BestaPesada,
    AlaudeBardo,
}

impl WeaponType {
    pub const ALL: &'static [Self] = &[
        Self::AdagaDupla,
        Self::DuasAdagas,
        Self::EspadaLonga,
        Self::EspadaMontante,
        Self::Katana,
        Self::Rapieira,
        Self::EspadaFlamejante,
        Self::MachadoBatalha,
        Self::MarteloGuerra,
        Self::MacaEstrela,
        Self::LancaAlabarda,
        Self::FoiceSombria,
        Self::CajadoArcano,
        Self::CajadoDruida,
        Self::OrbeGrimorio,
        Self::ArcoCaca,
        Self::BestaPesada,
        Self::AlaudeBardo,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AdagaDupla => "Espada Curta + Espada nas Costas",
            Self::DuasAdagas => "Adagas Gêmeas de Assassino",
            Self::EspadaLonga => "Espada Longa + Escudo de Aço",
            Self::EspadaMontante => "Espada Montante Gigante (Zweihänder)",
            Self::Katana => "Katana Curva (Lâmina Oriental)",
            Self::Rapieira => "Rapieira de Duelo (Bardo / Ladino)",
            Self::EspadaFlamejante => "Espada Longa Flamejante",
            Self::MachadoBatalha => "Machado de Batalha Duplo",
            Self::MarteloGuerra => "Martelo de Guerra Sagrado",
            Self::MacaEstrela => "Maça-Estrela + Escudo Templário",
            Self::LancaAlabarda => "Alabarda / Lança Real",
            Self::FoiceSombria => "Foice da Ceifa (Necromante / Bruxo)",
            Self::CajadoArcano => "Cajado Arcano c/ Cristal Azul",
            Self::CajadoDruida => "Cajado da Natureza + Grimório",
            Self::OrbeGrimorio => "Grimório Aberto + Fogo Fátuo",
            Self::ArcoCaca => "Arco Longo Élfico",
            Self::BestaPesada => "Besta de Caçador (Crossbow)",
            Self::AlaudeBardo => "Alaúde Musical (Bardo)",
        }
    }
}

impl std::fmt::Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SpriteConfig {
    pub skin: SkinTone,
    pub head_feature: HeadFeature,
    pub hair_style: HairStyle,
    pub hair_color: HairColor,
    pub facial: FacialFeature,
    pub outfit_style: OutfitStyle,
    pub outfit_color: OutfitColor,
    pub cloak: CloakColor,
    pub weapon: WeaponType,
}

impl SpriteConfig {
    pub fn randomize() -> Self {
        let mut rng = rand::rng();
        Self {
            skin: SkinTone::ALL[rng.random_range(0..SkinTone::ALL.len())],
            head_feature: HeadFeature::ALL[rng.random_range(0..HeadFeature::ALL.len())],
            hair_style: HairStyle::ALL[rng.random_range(0..HairStyle::ALL.len())],
            hair_color: HairColor::ALL[rng.random_range(0..HairColor::ALL.len())],
            facial: FacialFeature::ALL[rng.random_range(0..FacialFeature::ALL.len())],
            outfit_style: OutfitStyle::ALL[rng.random_range(0..OutfitStyle::ALL.len())],
            outfit_color: OutfitColor::ALL[rng.random_range(0..OutfitColor::ALL.len())],
            cloak: CloakColor::ALL[rng.random_range(0..CloakColor::ALL.len())],
            weapon: WeaponType::ALL[rng.random_range(0..WeaponType::ALL.len())],
        }
    }
}

pub fn render_sprite_png(cfg: &SpriteConfig) -> (u32, u32, Vec<u8>, String) {
    let bg = [179u8, 179, 179, 255];
    let outline = [20u8, 20, 22, 255];
    let white = [248u8, 248, 250, 255];
    let gold = [218u8, 176, 68, 255];
    let gold_dark = [164u8, 122, 34, 255];
    let steel = [192u8, 200, 212, 255];
    let steel_dark = [126u8, 134, 148, 255];
    let wood = [132u8, 82, 44, 255];
    let wood_dark = [92u8, 54, 26, 255];
    let red_gem = [218u8, 48, 54, 255];
    let cyan_glow = [82u8, 218, 252, 255];
    let fire_orange = [246u8, 128, 38, 255];
    let leaf_green = [68u8, 168, 76, 255];

    let (skin_main, skin_sh) = cfg.skin.colors();
    let (hair_main, hair_sh) = cfg.hair_color.colors();
    let (out_main, out_sh) = cfg.outfit_color.colors();

    let to_rgba = |rgb: [u8; 3]| [rgb[0], rgb[1], rgb[2], 255u8];
    let c_skin = to_rgba(skin_main);
    let c_skin_sh = to_rgba(skin_sh);
    let c_hair = to_rgba(hair_main);
    let c_hair_sh = to_rgba(hair_sh);
    let c_out = to_rgba(out_main);
    let c_out_sh = to_rgba(out_sh);

    let mut grid = [[bg; SPRITE_W]; SPRITE_H];

    fn set_px(grid: &mut [[[u8; 4]; SPRITE_W]; SPRITE_H], x: i32, y: i32, col: [u8; 4]) {
        if x >= 0 && x < SPRITE_W as i32 && y >= 0 && y < SPRITE_H as i32 {
            grid[y as usize][x as usize] = col;
        }
    }

    fn fill_rect(
        grid: &mut [[[u8; 4]; SPRITE_W]; SPRITE_H],
        x0: i32,
        y0: i32,
        w: i32,
        h: i32,
        col: [u8; 4],
    ) {
        for yy in y0..(y0 + h) {
            for xx in x0..(x0 + w) {
                if xx >= 0 && xx < SPRITE_W as i32 && yy >= 0 && yy < SPRITE_H as i32 {
                    grid[yy as usize][xx as usize] = col;
                }
            }
        }
    }

    if let Some((clk_m, clk_s)) = cfg.cloak.colors() {
        let c_clk = to_rgba(clk_m);
        let c_clk_sh = to_rgba(clk_s);
        fill_rect(&mut grid, 10, 15, 14, 14, c_clk_sh);
        fill_rect(&mut grid, 11, 16, 13, 12, c_clk);
        fill_rect(&mut grid, 22, 18, 4, 11, c_clk_sh);
        for y in 16..29 {
            set_px(&mut grid, 9, y, outline);
            set_px(&mut grid, 25, y, outline);
        }
        for x in 10..26 {
            set_px(&mut grid, x, 29, outline);
        }
        if cfg.cloak == CloakColor::BrancoArminho {
            fill_rect(&mut grid, 9, 14, 14, 3, white);
            set_px(&mut grid, 11, 15, outline);
            set_px(&mut grid, 15, 15, outline);
            set_px(&mut grid, 19, 15, outline);
        } else if cfg.cloak == CloakColor::PeleDeLobo {
            fill_rect(&mut grid, 8, 14, 16, 3, to_rgba([148, 142, 134]));
            fill_rect(&mut grid, 9, 15, 14, 2, to_rgba([98, 94, 88]));
        }
    } else {
        match cfg.cloak {
            CloakColor::AsasAnjo => {
                fill_rect(&mut grid, 2, 11, 7, 13, outline);
                fill_rect(&mut grid, 3, 12, 6, 11, white);
                fill_rect(&mut grid, 4, 14, 4, 8, to_rgba([212, 224, 242]));
                fill_rect(&mut grid, 23, 11, 7, 13, outline);
                fill_rect(&mut grid, 23, 12, 6, 11, white);
                fill_rect(&mut grid, 24, 14, 4, 8, to_rgba([212, 224, 242]));
            }
            CloakColor::AsasDemonio => {
                let wing_main = to_rgba([138, 34, 42]);
                let wing_mem = to_rgba([82, 22, 28]);
                fill_rect(&mut grid, 2, 11, 7, 12, outline);
                fill_rect(&mut grid, 3, 12, 6, 10, wing_main);
                fill_rect(&mut grid, 4, 14, 4, 7, wing_mem);
                fill_rect(&mut grid, 23, 11, 7, 12, outline);
                fill_rect(&mut grid, 23, 12, 6, 10, wing_main);
                fill_rect(&mut grid, 24, 14, 4, 7, wing_mem);
            }
            CloakColor::MochilaAventureiro => {
                fill_rect(&mut grid, 21, 14, 5, 10, outline);
                fill_rect(&mut grid, 22, 15, 3, 8, wood);
                fill_rect(&mut grid, 21, 12, 4, 3, to_rgba([168, 56, 56]));
            }
            _ => {}
        }
    }

    if cfg.weapon == WeaponType::AdagaDupla {
        for i in 0..7 {
            set_px(&mut grid, 21 + i, 11 - i, outline);
            set_px(&mut grid, 21 + i, 12 - i, wood);
            set_px(&mut grid, 22 + i, 12 - i, outline);
        }
        set_px(&mut grid, 20, 11, gold);
        set_px(&mut grid, 21, 12, gold);
        set_px(&mut grid, 22, 13, gold);
    } else if cfg.weapon == WeaponType::ArcoCaca
        || cfg.outfit_style == OutfitStyle::TrajePatrulheiro
    {
        fill_rect(&mut grid, 21, 10, 3, 6, wood_dark);
        set_px(&mut grid, 22, 8, white);
        set_px(&mut grid, 23, 9, red_gem);
    }

    fill_rect(&mut grid, 11, 25, 4, 4, steel_dark);
    fill_rect(&mut grid, 9, 29, 6, 4, c_out_sh);
    fill_rect(&mut grid, 10, 30, 4, 2, c_out);

    fill_rect(&mut grid, 17, 25, 4, 4, steel_dark);
    fill_rect(&mut grid, 17, 29, 6, 4, c_out_sh);
    fill_rect(&mut grid, 18, 30, 4, 2, c_out);

    for x in 9..15 {
        set_px(&mut grid, x, 33, outline);
    }
    for x in 17..23 {
        set_px(&mut grid, x, 33, outline);
    }
    for y in 26..33 {
        set_px(&mut grid, 9, y, outline);
        set_px(&mut grid, 15, y, outline);
        set_px(&mut grid, 16, y, outline);
        set_px(&mut grid, 22, y, outline);
    }

    fill_rect(&mut grid, 10, 15, 12, 11, outline);
    match cfg.outfit_style {
        OutfitStyle::CouroLadino => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_out);
            fill_rect(&mut grid, 11, 16, 10, 2, to_rgba([96, 100, 108]));
            set_px(&mut grid, 13, 18, outline);
            set_px(&mut grid, 14, 19, outline);
            set_px(&mut grid, 17, 18, outline);
            set_px(&mut grid, 16, 19, outline);
            set_px(&mut grid, 15, 20, outline);
        }
        OutfitStyle::ArmaduraPlacas => {
            fill_rect(&mut grid, 11, 16, 10, 9, steel);
            fill_rect(&mut grid, 12, 17, 8, 4, white);
            fill_rect(&mut grid, 13, 18, 6, 3, c_out);
        }
        OutfitStyle::CotaDeMalha => {
            fill_rect(&mut grid, 11, 16, 10, 9, steel_dark);
            for y in (16..24).step_by(2) {
                for x in (11..21).step_by(2) {
                    set_px(&mut grid, x, y, steel);
                }
            }
            fill_rect(&mut grid, 13, 17, 6, 6, c_out);
        }
        OutfitStyle::PaladinoReal => {
            fill_rect(&mut grid, 11, 16, 10, 9, gold);
            fill_rect(&mut grid, 13, 16, 6, 11, c_out);
            fill_rect(&mut grid, 15, 17, 2, 6, gold);
            fill_rect(&mut grid, 14, 18, 4, 2, white);
        }
        OutfitStyle::MantoArcano => {
            fill_rect(&mut grid, 11, 16, 10, 12, c_out);
            fill_rect(&mut grid, 14, 16, 4, 12, gold);
            set_px(&mut grid, 15, 18, cyan_glow);
            set_px(&mut grid, 16, 18, cyan_glow);
        }
        OutfitStyle::TunicaClerigo => {
            fill_rect(&mut grid, 11, 16, 10, 10, c_out);
            fill_rect(&mut grid, 15, 17, 2, 7, white);
            fill_rect(&mut grid, 13, 19, 6, 2, white);
        }
        OutfitStyle::PeitoralBarbaro => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_skin);
            fill_rect(&mut grid, 12, 17, 2, 7, c_out_sh);
            fill_rect(&mut grid, 18, 17, 2, 7, c_out_sh);
            fill_rect(&mut grid, 11, 23, 10, 3, wood_dark);
        }
        OutfitStyle::VesteMonge => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_out);
            for i in 0..5 {
                set_px(&mut grid, 12 + i, 16 + i, gold);
                set_px(&mut grid, 13 + i, 16 + i, c_out_sh);
            }
        }
        OutfitStyle::CasacoBardo => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_out);
            fill_rect(&mut grid, 14, 16, 4, 6, white);
            set_px(&mut grid, 13, 18, gold);
            set_px(&mut grid, 18, 18, gold);
            set_px(&mut grid, 13, 20, gold);
            set_px(&mut grid, 18, 20, gold);
        }
        OutfitStyle::TrajePatrulheiro => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_out);
            fill_rect(&mut grid, 11, 16, 10, 2, wood_dark);
            for i in 0..6 {
                set_px(&mut grid, 12 + i, 17 + i, wood);
            }
        }
        OutfitStyle::MantoDruida => {
            fill_rect(&mut grid, 11, 16, 10, 10, wood);
            fill_rect(&mut grid, 12, 16, 8, 4, leaf_green);
            set_px(&mut grid, 13, 21, leaf_green);
            set_px(&mut grid, 18, 20, leaf_green);
        }
        OutfitStyle::ArmaduraNegra => {
            fill_rect(&mut grid, 11, 16, 10, 9, to_rgba([44, 46, 54]));
            fill_rect(&mut grid, 13, 17, 6, 5, c_out_sh);
            set_px(&mut grid, 9, 15, steel);
            set_px(&mut grid, 22, 15, steel);
            set_px(&mut grid, 15, 18, red_gem);
            set_px(&mut grid, 16, 18, red_gem);
        }
        OutfitStyle::ColeteArtifice => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_out);
            fill_rect(&mut grid, 12, 18, 2, 3, cyan_glow);
            fill_rect(&mut grid, 18, 18, 2, 3, red_gem);
            fill_rect(&mut grid, 15, 17, 2, 4, gold);
        }
        OutfitStyle::MantoBruxo => {
            fill_rect(&mut grid, 11, 16, 10, 12, c_out_sh);
            fill_rect(&mut grid, 13, 16, 6, 11, c_out);
            set_px(&mut grid, 15, 18, fire_orange);
            set_px(&mut grid, 16, 18, fire_orange);
            set_px(&mut grid, 15, 19, gold);
            set_px(&mut grid, 16, 19, gold);
        }
        OutfitStyle::SobretudoCacador => {
            fill_rect(&mut grid, 10, 16, 12, 13, c_out_sh);
            fill_rect(&mut grid, 11, 16, 3, 12, c_out);
            fill_rect(&mut grid, 18, 16, 3, 12, c_out);
            fill_rect(&mut grid, 14, 16, 4, 6, white);
            set_px(&mut grid, 13, 18, gold);
            set_px(&mut grid, 18, 18, gold);
            set_px(&mut grid, 13, 20, gold);
            set_px(&mut grid, 18, 20, gold);
        }
        OutfitStyle::ArmaduraEscamas => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_out_sh);
            for y in 16..24 {
                for x in 11..21 {
                    if (x + y) % 2 == 0 {
                        set_px(&mut grid, x, y, c_out);
                    } else if (x + y) % 3 == 0 {
                        set_px(&mut grid, x, y, gold);
                    }
                }
            }
        }
        OutfitStyle::ArmaduraSamurai => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_out);
            for y in (17..23).step_by(2) {
                fill_rect(&mut grid, 11, y, 10, 1, gold);
            }
            fill_rect(&mut grid, 8, 15, 3, 4, c_out_sh);
            fill_rect(&mut grid, 21, 15, 3, 4, c_out_sh);
        }
        OutfitStyle::BrigandinaMercenario => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_out);
            for y in (17..23).step_by(2) {
                set_px(&mut grid, 12, y, steel);
                set_px(&mut grid, 15, y, steel);
                set_px(&mut grid, 16, y, steel);
                set_px(&mut grid, 19, y, steel);
            }
        }
        OutfitStyle::CamisaCampones => {
            fill_rect(&mut grid, 11, 16, 10, 9, white);
            fill_rect(&mut grid, 12, 16, 2, 9, c_out);
            fill_rect(&mut grid, 18, 16, 2, 9, c_out);
            set_px(&mut grid, 15, 17, wood_dark);
            set_px(&mut grid, 16, 18, wood_dark);
        }
        OutfitStyle::VestidoNobre => {
            fill_rect(&mut grid, 11, 16, 10, 12, c_out);
            fill_rect(&mut grid, 13, 16, 6, 12, c_out_sh);
            fill_rect(&mut grid, 14, 16, 4, 11, gold);
            fill_rect(&mut grid, 12, 16, 8, 2, white);
        }
        OutfitStyle::CasacoPirata => {
            fill_rect(&mut grid, 11, 16, 10, 11, c_out);
            fill_rect(&mut grid, 14, 16, 4, 7, white);
            for y in (17..23).step_by(2) {
                set_px(&mut grid, 13, y, gold);
                set_px(&mut grid, 18, y, gold);
            }
        }
        OutfitStyle::TunicaAssassino => {
            fill_rect(&mut grid, 11, 16, 10, 10, white);
            fill_rect(&mut grid, 12, 16, 8, 9, c_out);
            fill_rect(&mut grid, 11, 21, 10, 3, red_gem);
            set_px(&mut grid, 13, 17, red_gem);
            set_px(&mut grid, 18, 17, red_gem);
        }
        OutfitStyle::ArmaduraGladiador => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_skin);
            fill_rect(&mut grid, 7, 15, 6, 5, gold);
            fill_rect(&mut grid, 11, 17, 9, 2, wood_dark);
            fill_rect(&mut grid, 11, 21, 10, 4, gold_dark);
        }
        OutfitStyle::MantoArquimago => {
            fill_rect(&mut grid, 11, 16, 10, 13, c_out);
            fill_rect(&mut grid, 11, 16, 10, 2, gold);
            set_px(&mut grid, 13, 19, gold);
            set_px(&mut grid, 18, 20, cyan_glow);
            set_px(&mut grid, 14, 24, cyan_glow);
            set_px(&mut grid, 17, 25, gold);
        }
        OutfitStyle::HabitoInquisidor => {
            fill_rect(&mut grid, 11, 16, 10, 11, to_rgba([40, 40, 46]));
            fill_rect(&mut grid, 12, 16, 8, 2, red_gem);
            fill_rect(&mut grid, 15, 17, 2, 7, gold);
            fill_rect(&mut grid, 13, 19, 6, 2, gold);
        }
        OutfitStyle::ArmaduraOsso => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_out_sh);
            for y in (17..23).step_by(2) {
                fill_rect(&mut grid, 12, y, 8, 1, white);
            }
            fill_rect(&mut grid, 15, 16, 2, 7, outline);
        }
        OutfitStyle::TrajeAlquimista => {
            fill_rect(&mut grid, 11, 16, 10, 9, c_out);
            fill_rect(&mut grid, 12, 18, 8, 7, wood);
            set_px(&mut grid, 13, 20, leaf_green);
            set_px(&mut grid, 15, 20, red_gem);
            set_px(&mut grid, 17, 20, cyan_glow);
        }
        OutfitStyle::TunicaDeserto => {
            fill_rect(&mut grid, 11, 16, 10, 11, c_out);
            fill_rect(&mut grid, 12, 16, 8, 4, gold);
            fill_rect(&mut grid, 11, 21, 10, 2, red_gem);
        }
        OutfitStyle::ArmaduraGelo => {
            fill_rect(&mut grid, 11, 16, 10, 9, cyan_glow);
            fill_rect(&mut grid, 12, 17, 8, 6, white);
            fill_rect(&mut grid, 14, 18, 4, 4, c_out);
        }
        OutfitStyle::GibaoMosqueteiro => {
            fill_rect(&mut grid, 11, 16, 10, 10, c_out);
            fill_rect(&mut grid, 15, 17, 2, 7, white);
            fill_rect(&mut grid, 13, 19, 6, 2, white);
            fill_rect(&mut grid, 15, 19, 2, 2, gold);
        }
    }
    fill_rect(&mut grid, 11, 22, 10, 2, c_out_sh);
    fill_rect(&mut grid, 15, 22, 2, 2, gold);

    fill_rect(&mut grid, 7, 16, 4, 8, outline);
    fill_rect(&mut grid, 8, 17, 2, 6, c_out);
    fill_rect(&mut grid, 7, 21, 3, 3, c_out_sh);

    fill_rect(&mut grid, 21, 16, 4, 8, outline);
    fill_rect(&mut grid, 22, 17, 2, 6, c_out);
    fill_rect(&mut grid, 22, 21, 3, 3, c_out_sh);

    fill_rect(&mut grid, 10, 5, 12, 11, outline);
    fill_rect(&mut grid, 11, 7, 10, 8, c_skin);
    fill_rect(&mut grid, 11, 13, 10, 2, c_skin_sh);

    fill_rect(&mut grid, 12, 9, 2, 1, outline);
    fill_rect(&mut grid, 17, 9, 3, 1, outline);
    set_px(&mut grid, 12, 10, white);
    set_px(&mut grid, 13, 10, outline);
    set_px(&mut grid, 17, 10, outline);
    set_px(&mut grid, 18, 10, white);
    fill_rect(&mut grid, 14, 13, 3, 1, c_skin_sh);

    match cfg.head_feature {
        HeadFeature::OrelhasElficas => {
            fill_rect(&mut grid, 8, 8, 2, 3, outline);
            set_px(&mut grid, 9, 9, c_skin);
            set_px(&mut grid, 8, 8, c_skin);
            fill_rect(&mut grid, 22, 8, 2, 3, outline);
            set_px(&mut grid, 22, 9, c_skin);
            set_px(&mut grid, 23, 8, c_skin);
        }
        HeadFeature::OrelhasLongasFeericas => {
            fill_rect(&mut grid, 7, 8, 3, 3, outline);
            fill_rect(&mut grid, 7, 9, 3, 1, c_skin);
            fill_rect(&mut grid, 22, 8, 3, 3, outline);
            fill_rect(&mut grid, 22, 9, 3, 1, c_skin);
        }
        HeadFeature::Humana => {
            set_px(&mut grid, 9, 9, outline);
            set_px(&mut grid, 9, 10, c_skin_sh);
            set_px(&mut grid, 22, 9, outline);
            set_px(&mut grid, 22, 10, c_skin_sh);
        }
        HeadFeature::ChifresTiefling => {
            fill_rect(&mut grid, 8, 3, 2, 4, outline);
            set_px(&mut grid, 9, 4, gold);
            set_px(&mut grid, 8, 3, gold);
            fill_rect(&mut grid, 22, 3, 2, 4, outline);
            set_px(&mut grid, 22, 4, gold);
            set_px(&mut grid, 23, 3, gold);
        }
        HeadFeature::ChifresNegros => {
            fill_rect(&mut grid, 8, 1, 2, 6, outline);
            set_px(&mut grid, 9, 2, steel_dark);
            set_px(&mut grid, 9, 4, steel_dark);
            fill_rect(&mut grid, 22, 1, 2, 6, outline);
            set_px(&mut grid, 22, 2, steel_dark);
            set_px(&mut grid, 22, 4, steel_dark);
        }
        HeadFeature::ChifresCarneiro => {
            fill_rect(&mut grid, 7, 5, 3, 5, outline);
            fill_rect(&mut grid, 8, 6, 2, 3, gold_dark);
            fill_rect(&mut grid, 22, 5, 3, 5, outline);
            fill_rect(&mut grid, 22, 6, 2, 3, gold_dark);
        }
        HeadFeature::PresasOrc => {
            set_px(&mut grid, 13, 13, white);
            set_px(&mut grid, 13, 12, white);
            set_px(&mut grid, 18, 13, white);
            set_px(&mut grid, 18, 12, white);
        }
        HeadFeature::HaloCelestial => {
            fill_rect(&mut grid, 11, 1, 10, 1, gold);
            set_px(&mut grid, 10, 2, gold);
            set_px(&mut grid, 21, 2, gold);
            fill_rect(&mut grid, 12, 2, 8, 1, white);
        }
        HeadFeature::CoroaReal => {
            fill_rect(&mut grid, 11, 3, 10, 3, gold);
            set_px(&mut grid, 11, 2, gold);
            set_px(&mut grid, 15, 2, gold);
            set_px(&mut grid, 16, 2, gold);
            set_px(&mut grid, 20, 2, gold);
            set_px(&mut grid, 15, 4, red_gem);
            set_px(&mut grid, 16, 4, red_gem);
        }
        HeadFeature::TiaraArcana => {
            fill_rect(&mut grid, 11, 6, 10, 1, gold);
            fill_rect(&mut grid, 15, 5, 2, 2, cyan_glow);
        }
        HeadFeature::GalhosDruida => {
            fill_rect(&mut grid, 8, 2, 2, 5, wood);
            set_px(&mut grid, 7, 2, leaf_green);
            set_px(&mut grid, 9, 1, leaf_green);
            fill_rect(&mut grid, 22, 2, 2, 5, wood);
            set_px(&mut grid, 24, 2, leaf_green);
            set_px(&mut grid, 22, 1, leaf_green);
        }
        HeadFeature::BandanaPirata => {
            fill_rect(&mut grid, 10, 6, 12, 2, red_gem);
            fill_rect(&mut grid, 22, 7, 2, 3, red_gem);
        }
        HeadFeature::OlhosBrilhantesFogo => {
            fill_rect(&mut grid, 12, 10, 2, 1, fire_orange);
            fill_rect(&mut grid, 17, 10, 2, 1, fire_orange);
        }
        HeadFeature::OlhosBrilhantesGelo => {
            fill_rect(&mut grid, 12, 10, 2, 1, cyan_glow);
            fill_rect(&mut grid, 17, 10, 2, 1, cyan_glow);
        }
    }

    match cfg.hair_style {
        HairStyle::LongoAventureiro => {
            fill_rect(&mut grid, 11, 4, 10, 3, c_hair);
            fill_rect(&mut grid, 10, 6, 2, 9, c_hair_sh);
            fill_rect(&mut grid, 20, 6, 2, 9, c_hair_sh);
            for x in 11..21 {
                set_px(&mut grid, x, 3, outline);
            }
        }
        HairStyle::CurtoGuerreiro => {
            fill_rect(&mut grid, 11, 4, 10, 3, c_hair);
            fill_rect(&mut grid, 11, 6, 2, 2, c_hair_sh);
            fill_rect(&mut grid, 19, 6, 2, 2, c_hair_sh);
            for x in 11..21 {
                set_px(&mut grid, x, 3, outline);
            }
        }
        HairStyle::RaboDeCavalo => {
            fill_rect(&mut grid, 11, 4, 10, 3, c_hair);
            fill_rect(&mut grid, 14, 2, 4, 2, c_hair_sh);
            fill_rect(&mut grid, 21, 5, 3, 7, c_hair_sh);
            set_px(&mut grid, 20, 5, gold);
        }
        HairStyle::TrancasAnao => {
            fill_rect(&mut grid, 11, 4, 10, 3, c_hair);
            fill_rect(&mut grid, 9, 7, 2, 11, c_hair_sh);
            fill_rect(&mut grid, 21, 7, 2, 11, c_hair_sh);
            fill_rect(&mut grid, 9, 15, 2, 1, gold);
            fill_rect(&mut grid, 21, 15, 2, 1, gold);
        }
        HairStyle::CoqueSamurai => {
            fill_rect(&mut grid, 11, 5, 10, 2, c_hair);
            fill_rect(&mut grid, 14, 1, 4, 4, c_hair_sh);
            fill_rect(&mut grid, 14, 4, 4, 1, gold);
        }
        HairStyle::LongoOndulado => {
            fill_rect(&mut grid, 10, 3, 12, 4, c_hair);
            fill_rect(&mut grid, 9, 6, 3, 11, c_hair);
            fill_rect(&mut grid, 20, 6, 3, 11, c_hair_sh);
        }
        HairStyle::MoicanoBarbaro => {
            fill_rect(&mut grid, 14, 1, 4, 6, c_hair);
            fill_rect(&mut grid, 15, 2, 2, 4, c_hair_sh);
        }
        HairStyle::CarecaMonge => {
            for x in 11..21 {
                set_px(&mut grid, x, 5, outline);
            }
        }
        HairStyle::CapuzLadino => {
            fill_rect(&mut grid, 9, 3, 14, 5, c_out_sh);
            fill_rect(&mut grid, 9, 6, 2, 9, c_out_sh);
            fill_rect(&mut grid, 21, 6, 2, 9, c_out_sh);
            for x in 9..23 {
                set_px(&mut grid, x, 2, outline);
            }
        }
        HairStyle::ChapeuMago => {
            fill_rect(&mut grid, 7, 6, 18, 2, c_out_sh);
            fill_rect(&mut grid, 10, 3, 12, 3, c_out);
            fill_rect(&mut grid, 12, 1, 8, 2, c_out_sh);
            fill_rect(&mut grid, 10, 5, 12, 1, gold);
        }
        HairStyle::ElmoCavaleiro => {
            fill_rect(&mut grid, 10, 4, 12, 5, steel);
            fill_rect(&mut grid, 15, 5, 2, 6, steel_dark);
            fill_rect(&mut grid, 10, 8, 2, 6, steel_dark);
            fill_rect(&mut grid, 20, 8, 2, 6, steel_dark);
        }
        HairStyle::ElmoViking => {
            fill_rect(&mut grid, 10, 4, 12, 4, steel_dark);
            fill_rect(&mut grid, 10, 7, 12, 1, gold);
            fill_rect(&mut grid, 7, 2, 3, 4, white);
            fill_rect(&mut grid, 22, 2, 3, 4, white);
        }
        HairStyle::ElmoEspartano => {
            fill_rect(&mut grid, 13, 1, 6, 3, red_gem);
            fill_rect(&mut grid, 10, 4, 12, 4, gold);
            fill_rect(&mut grid, 10, 8, 2, 6, gold_dark);
            fill_rect(&mut grid, 20, 8, 2, 6, gold_dark);
        }
        HairStyle::ChapeuBardo => {
            fill_rect(&mut grid, 8, 5, 16, 2, c_out);
            fill_rect(&mut grid, 11, 3, 10, 2, c_out_sh);
            fill_rect(&mut grid, 19, 1, 4, 4, white);
        }
        HairStyle::CapuzNecromante => {
            fill_rect(&mut grid, 9, 3, 14, 5, to_rgba([36, 36, 42]));
            fill_rect(&mut grid, 9, 6, 2, 9, to_rgba([36, 36, 42]));
            fill_rect(&mut grid, 21, 6, 2, 9, to_rgba([36, 36, 42]));
            fill_rect(&mut grid, 14, 4, 4, 3, white);
        }
        HairStyle::MitraClerigo => {
            fill_rect(&mut grid, 11, 2, 10, 5, white);
            fill_rect(&mut grid, 13, 1, 6, 1, gold);
            fill_rect(&mut grid, 15, 2, 2, 5, gold);
            fill_rect(&mut grid, 11, 6, 10, 1, gold);
        }
        HairStyle::LongoElfoLiso => {
            fill_rect(&mut grid, 11, 4, 10, 3, c_hair);
            fill_rect(&mut grid, 10, 6, 2, 14, c_hair);
            fill_rect(&mut grid, 20, 6, 2, 14, c_hair);
            fill_rect(&mut grid, 11, 6, 10, 1, gold);
        }
        HairStyle::FranjaLateral => {
            fill_rect(&mut grid, 11, 4, 10, 3, c_hair);
            fill_rect(&mut grid, 11, 7, 5, 4, c_hair_sh);
            fill_rect(&mut grid, 10, 6, 2, 6, c_hair_sh);
        }
        HairStyle::CurtoBaguncado => {
            fill_rect(&mut grid, 10, 4, 12, 3, c_hair);
            set_px(&mut grid, 11, 2, c_hair);
            set_px(&mut grid, 14, 2, c_hair);
            set_px(&mut grid, 17, 2, c_hair_sh);
            set_px(&mut grid, 20, 3, c_hair);
        }
        HairStyle::AfroVolumoso => {
            fill_rect(&mut grid, 8, 2, 16, 6, c_hair);
            fill_rect(&mut grid, 7, 4, 3, 7, c_hair_sh);
            fill_rect(&mut grid, 22, 4, 3, 7, c_hair_sh);
        }
        HairStyle::DreadlocksLongos => {
            fill_rect(&mut grid, 10, 3, 12, 4, c_hair);
            for x in (9..23).step_by(2) {
                fill_rect(&mut grid, x, 6, 1, 11, c_hair_sh);
                set_px(&mut grid, x, 13, gold);
            }
        }
        HairStyle::ChiquinhasDuplas => {
            fill_rect(&mut grid, 11, 4, 10, 3, c_hair);
            fill_rect(&mut grid, 7, 6, 3, 9, c_hair_sh);
            fill_rect(&mut grid, 22, 6, 3, 9, c_hair_sh);
            set_px(&mut grid, 9, 6, red_gem);
            set_px(&mut grid, 22, 6, red_gem);
        }
        HairStyle::TrancaUnicaLateral => {
            fill_rect(&mut grid, 11, 4, 10, 3, c_hair);
            fill_rect(&mut grid, 9, 6, 3, 13, c_hair_sh);
            fill_rect(&mut grid, 9, 17, 3, 1, gold);
        }
        HairStyle::CabeloPartidoMeio => {
            fill_rect(&mut grid, 10, 4, 5, 4, c_hair);
            fill_rect(&mut grid, 17, 4, 5, 4, c_hair);
            fill_rect(&mut grid, 10, 7, 2, 4, c_hair_sh);
            fill_rect(&mut grid, 20, 7, 2, 4, c_hair_sh);
        }
        HairStyle::TopeteReais => {
            fill_rect(&mut grid, 11, 2, 11, 5, c_hair);
            fill_rect(&mut grid, 12, 1, 8, 2, c_hair_sh);
        }
        HairStyle::CalvoCoroa => {
            fill_rect(&mut grid, 9, 6, 2, 5, c_hair_sh);
            fill_rect(&mut grid, 21, 6, 2, 5, c_hair_sh);
        }
        HairStyle::CoroaDeLouros => {
            fill_rect(&mut grid, 11, 4, 10, 3, c_hair);
            fill_rect(&mut grid, 10, 5, 12, 2, leaf_green);
            set_px(&mut grid, 15, 5, gold);
            set_px(&mut grid, 16, 5, gold);
        }
        HairStyle::ElmoAladoValquiria => {
            fill_rect(&mut grid, 10, 4, 12, 4, gold);
            fill_rect(&mut grid, 6, 2, 4, 5, white);
            fill_rect(&mut grid, 22, 2, 4, 5, white);
            fill_rect(&mut grid, 15, 5, 2, 5, gold_dark);
        }
        HairStyle::ElmoCruzado => {
            fill_rect(&mut grid, 10, 4, 12, 11, steel);
            fill_rect(&mut grid, 15, 5, 2, 9, gold);
            fill_rect(&mut grid, 11, 9, 10, 2, gold);
            fill_rect(&mut grid, 12, 10, 3, 1, outline);
            fill_rect(&mut grid, 17, 10, 3, 1, outline);
        }
        HairStyle::TricornePirata => {
            fill_rect(&mut grid, 7, 5, 18, 2, outline);
            fill_rect(&mut grid, 9, 3, 14, 2, c_out_sh);
            fill_rect(&mut grid, 8, 4, 16, 1, gold);
            fill_rect(&mut grid, 20, 2, 3, 3, red_gem);
        }
        HairStyle::TurbanteDeserto => {
            fill_rect(&mut grid, 9, 2, 14, 6, c_out);
            fill_rect(&mut grid, 10, 3, 12, 4, white);
            fill_rect(&mut grid, 15, 4, 2, 3, red_gem);
            set_px(&mut grid, 15, 3, gold);
            set_px(&mut grid, 16, 3, gold);
        }
        HairStyle::CapuzXamaLobo => {
            fill_rect(&mut grid, 9, 2, 14, 6, to_rgba([128, 124, 118]));
            fill_rect(&mut grid, 9, 3, 2, 10, to_rgba([96, 92, 86]));
            fill_rect(&mut grid, 21, 3, 2, 10, to_rgba([96, 92, 86]));
            set_px(&mut grid, 10, 1, to_rgba([96, 92, 86]));
            set_px(&mut grid, 21, 1, to_rgba([96, 92, 86]));
            set_px(&mut grid, 13, 4, red_gem);
            set_px(&mut grid, 18, 4, red_gem);
        }
    }

    match cfg.facial {
        FacialFeature::Nenhum => {}
        FacialFeature::BarbaLonga => {
            fill_rect(&mut grid, 11, 12, 10, 6, c_hair);
            fill_rect(&mut grid, 12, 14, 8, 5, c_hair_sh);
        }
        FacialFeature::BarbaGiganteMago => {
            fill_rect(&mut grid, 11, 12, 10, 7, c_hair);
            fill_rect(&mut grid, 12, 16, 8, 7, c_hair_sh);
            fill_rect(&mut grid, 13, 21, 6, 3, c_hair);
        }
        FacialFeature::BarbaTrancada => {
            fill_rect(&mut grid, 11, 12, 10, 4, c_hair);
            fill_rect(&mut grid, 12, 15, 3, 5, c_hair_sh);
            fill_rect(&mut grid, 17, 15, 3, 5, c_hair_sh);
            fill_rect(&mut grid, 12, 18, 3, 1, gold);
            fill_rect(&mut grid, 17, 18, 3, 1, gold);
        }
        FacialFeature::BarbaBifurcada => {
            fill_rect(&mut grid, 11, 12, 10, 4, c_hair);
            fill_rect(&mut grid, 11, 16, 4, 5, c_hair_sh);
            fill_rect(&mut grid, 17, 16, 4, 5, c_hair_sh);
        }
        FacialFeature::BarbaLenhador => {
            fill_rect(&mut grid, 10, 11, 12, 6, c_hair);
            fill_rect(&mut grid, 11, 13, 10, 4, c_hair_sh);
        }
        FacialFeature::BarbaRealAparada => {
            fill_rect(&mut grid, 11, 12, 10, 4, c_hair);
            fill_rect(&mut grid, 12, 15, 8, 1, gold);
        }
        FacialFeature::BarbaCurta => {
            fill_rect(&mut grid, 11, 12, 10, 3, c_hair_sh);
            fill_rect(&mut grid, 14, 13, 3, 1, c_skin_sh);
        }
        FacialFeature::BarbaPorFazer => {
            for x in (11..21).step_by(2) {
                set_px(&mut grid, x, 12, c_hair_sh);
                set_px(&mut grid, x + 1, 14, c_hair_sh);
            }
        }
        FacialFeature::CosteletasLongas => {
            fill_rect(&mut grid, 11, 9, 2, 6, c_hair);
            fill_rect(&mut grid, 19, 9, 2, 6, c_hair);
        }
        FacialFeature::Cavanhaque => {
            fill_rect(&mut grid, 13, 13, 6, 3, c_hair_sh);
        }
        FacialFeature::CavanhaquePontudo => {
            fill_rect(&mut grid, 13, 12, 6, 2, c_hair);
            fill_rect(&mut grid, 14, 14, 4, 3, c_hair_sh);
            fill_rect(&mut grid, 15, 17, 2, 2, c_hair_sh);
        }
        FacialFeature::BigodeImperial => {
            fill_rect(&mut grid, 11, 12, 10, 2, c_hair);
            set_px(&mut grid, 10, 11, c_hair);
            set_px(&mut grid, 21, 11, c_hair);
        }
        FacialFeature::BigodeFinoMosqueteiro => {
            fill_rect(&mut grid, 12, 12, 8, 1, c_hair);
            fill_rect(&mut grid, 15, 14, 2, 2, c_hair_sh);
        }
        FacialFeature::BigodeFerradura => {
            fill_rect(&mut grid, 12, 12, 8, 1, c_hair);
            fill_rect(&mut grid, 12, 12, 2, 4, c_hair);
            fill_rect(&mut grid, 18, 12, 2, 4, c_hair);
        }
        FacialFeature::BigodeDali => {
            fill_rect(&mut grid, 12, 12, 8, 1, c_hair);
            set_px(&mut grid, 10, 11, c_hair);
            set_px(&mut grid, 9, 10, c_hair);
            set_px(&mut grid, 21, 11, c_hair);
            set_px(&mut grid, 22, 10, c_hair);
        }
        FacialFeature::MascaraSombra => {
            fill_rect(&mut grid, 11, 11, 10, 4, outline);
        }
        FacialFeature::MascaraPrata => {
            fill_rect(&mut grid, 11, 11, 10, 4, steel);
            fill_rect(&mut grid, 12, 12, 8, 2, steel_dark);
        }
        FacialFeature::MascaraCaveira => {
            fill_rect(&mut grid, 11, 11, 10, 4, white);
            set_px(&mut grid, 13, 13, outline);
            set_px(&mut grid, 15, 13, outline);
            set_px(&mut grid, 17, 13, outline);
            set_px(&mut grid, 19, 13, outline);
        }
        FacialFeature::MascaraPeste => {
            fill_rect(&mut grid, 11, 9, 10, 6, white);
            fill_rect(&mut grid, 8, 11, 6, 3, steel_dark);
            fill_rect(&mut grid, 12, 9, 2, 2, red_gem);
            fill_rect(&mut grid, 17, 9, 2, 2, red_gem);
        }
        FacialFeature::Cicatriz => {
            set_px(&mut grid, 18, 8, red_gem);
            set_px(&mut grid, 18, 11, red_gem);
            set_px(&mut grid, 18, 12, red_gem);
        }
        FacialFeature::CicatrizCruzada => {
            set_px(&mut grid, 12, 11, red_gem);
            set_px(&mut grid, 13, 12, red_gem);
            set_px(&mut grid, 14, 11, red_gem);
            set_px(&mut grid, 12, 12, red_gem);
        }
        FacialFeature::TapaOlho => {
            fill_rect(&mut grid, 11, 9, 10, 1, outline);
            fill_rect(&mut grid, 17, 9, 3, 3, outline);
        }
        FacialFeature::MonoculoNobre => {
            fill_rect(&mut grid, 17, 9, 3, 3, gold);
            set_px(&mut grid, 18, 10, cyan_glow);
            fill_rect(&mut grid, 19, 12, 1, 3, gold);
        }
        FacialFeature::PinturaGuerraAzul => {
            fill_rect(&mut grid, 11, 10, 3, 2, cyan_glow);
            fill_rect(&mut grid, 17, 10, 3, 2, cyan_glow);
        }
        FacialFeature::PinturaGuerraVermelha => {
            fill_rect(&mut grid, 11, 8, 2, 5, red_gem);
            fill_rect(&mut grid, 18, 8, 2, 5, red_gem);
        }
        FacialFeature::TatuagemRunica => {
            let rune = to_rgba([158, 72, 224]);
            set_px(&mut grid, 12, 8, rune);
            set_px(&mut grid, 11, 11, rune);
            set_px(&mut grid, 12, 12, rune);
            set_px(&mut grid, 11, 13, rune);
        }
        FacialFeature::SardasCampones => {
            let blush = to_rgba([228, 122, 118]);
            set_px(&mut grid, 11, 11, blush);
            set_px(&mut grid, 13, 11, wood);
            set_px(&mut grid, 18, 11, wood);
            set_px(&mut grid, 20, 11, blush);
        }
        FacialFeature::OculosArtifice => {
            fill_rect(&mut grid, 11, 9, 4, 3, gold);
            fill_rect(&mut grid, 17, 9, 4, 3, gold);
            fill_rect(&mut grid, 12, 10, 2, 1, cyan_glow);
            fill_rect(&mut grid, 18, 10, 2, 1, cyan_glow);
            fill_rect(&mut grid, 15, 10, 2, 1, outline);
        }
        FacialFeature::VendasOraculo => {
            fill_rect(&mut grid, 11, 9, 10, 3, white);
            fill_rect(&mut grid, 11, 11, 10, 1, steel);
        }
    }

    match cfg.weapon {
        WeaponType::AdagaDupla => {
            set_px(&mut grid, 6, 22, gold);
            set_px(&mut grid, 7, 23, gold);
            for i in 0..6 {
                set_px(&mut grid, 5 - i, 23 + i, outline);
                set_px(&mut grid, 6 - i, 23 + i, steel);
                set_px(&mut grid, 6 - i, 24 + i, steel_dark);
                set_px(&mut grid, 7 - i, 24 + i, outline);
            }
        }
        WeaponType::DuasAdagas => {
            for i in 0..5 {
                set_px(&mut grid, 6 - i, 23 + i, steel);
                set_px(&mut grid, 5 - i, 23 + i, outline);
                set_px(&mut grid, 25 + i, 23 + i, steel);
                set_px(&mut grid, 26 + i, 23 + i, outline);
            }
            set_px(&mut grid, 7, 22, gold);
            set_px(&mut grid, 24, 22, gold);
        }
        WeaponType::EspadaLonga => {
            fill_rect(&mut grid, 4, 9, 3, 12, outline);
            fill_rect(&mut grid, 5, 10, 1, 10, steel);
            fill_rect(&mut grid, 3, 21, 5, 2, gold);
            fill_rect(&mut grid, 22, 17, 6, 8, outline);
            fill_rect(&mut grid, 23, 18, 4, 6, steel);
            fill_rect(&mut grid, 24, 19, 2, 4, c_out);
        }
        WeaponType::EspadaMontante => {
            fill_rect(&mut grid, 3, 4, 4, 17, outline);
            fill_rect(&mut grid, 4, 5, 2, 15, steel);
            fill_rect(&mut grid, 1, 20, 8, 2, gold);
            fill_rect(&mut grid, 4, 22, 2, 5, wood_dark);
        }
        WeaponType::Katana => {
            for y in 6..21 {
                let x = if y < 11 { 5 } else { 4 };
                set_px(&mut grid, x - 1, y, outline);
                set_px(&mut grid, x, y, white);
                set_px(&mut grid, x + 1, y, steel_dark);
            }
            fill_rect(&mut grid, 3, 21, 4, 1, gold);
            fill_rect(&mut grid, 4, 22, 2, 4, outline);
        }
        WeaponType::Rapieira => {
            fill_rect(&mut grid, 5, 8, 1, 13, steel);
            fill_rect(&mut grid, 3, 19, 5, 3, gold);
            set_px(&mut grid, 4, 18, gold);
            set_px(&mut grid, 6, 18, gold);
        }
        WeaponType::EspadaFlamejante => {
            fill_rect(&mut grid, 3, 7, 4, 14, fire_orange);
            fill_rect(&mut grid, 4, 8, 2, 12, gold);
            fill_rect(&mut grid, 2, 21, 6, 2, gold_dark);
        }
        WeaponType::MachadoBatalha => {
            fill_rect(&mut grid, 4, 10, 2, 18, wood);
            fill_rect(&mut grid, 1, 11, 8, 6, outline);
            fill_rect(&mut grid, 2, 12, 6, 4, steel);
        }
        WeaponType::MarteloGuerra => {
            fill_rect(&mut grid, 4, 11, 2, 17, wood);
            fill_rect(&mut grid, 1, 9, 8, 5, outline);
            fill_rect(&mut grid, 2, 10, 6, 3, gold);
        }
        WeaponType::MacaEstrela => {
            fill_rect(&mut grid, 4, 13, 2, 12, wood);
            fill_rect(&mut grid, 2, 8, 6, 6, steel_dark);
            fill_rect(&mut grid, 3, 9, 4, 4, steel);
            fill_rect(&mut grid, 22, 16, 6, 9, outline);
            fill_rect(&mut grid, 23, 17, 4, 7, white);
            fill_rect(&mut grid, 24, 18, 2, 5, red_gem);
        }
        WeaponType::LancaAlabarda => {
            fill_rect(&mut grid, 4, 3, 2, 29, wood);
            fill_rect(&mut grid, 3, 1, 4, 5, steel);
            fill_rect(&mut grid, 1, 5, 4, 4, steel_dark);
        }
        WeaponType::FoiceSombria => {
            fill_rect(&mut grid, 4, 6, 2, 25, wood_dark);
            fill_rect(&mut grid, 4, 4, 9, 3, outline);
            fill_rect(&mut grid, 5, 5, 7, 2, steel);
            set_px(&mut grid, 11, 7, steel);
        }
        WeaponType::CajadoArcano => {
            fill_rect(&mut grid, 4, 6, 2, 24, wood);
            fill_rect(&mut grid, 3, 3, 4, 4, outline);
            fill_rect(&mut grid, 4, 4, 2, 2, cyan_glow);
        }
        WeaponType::CajadoDruida => {
            fill_rect(&mut grid, 4, 6, 2, 24, wood);
            fill_rect(&mut grid, 2, 3, 6, 4, leaf_green);
            set_px(&mut grid, 4, 4, gold);
            fill_rect(&mut grid, 22, 19, 5, 6, wood_dark);
            fill_rect(&mut grid, 23, 20, 3, 4, white);
        }
        WeaponType::OrbeGrimorio => {
            fill_rect(&mut grid, 2, 14, 5, 5, cyan_glow);
            fill_rect(&mut grid, 3, 15, 3, 3, white);
            fill_rect(&mut grid, 22, 19, 6, 5, red_gem);
            fill_rect(&mut grid, 23, 20, 4, 3, white);
        }
        WeaponType::ArcoCaca => {
            for y in 11..26 {
                set_px(&mut grid, 4, y, wood);
                set_px(&mut grid, 7, y, white);
            }
            set_px(&mut grid, 5, 10, wood);
            set_px(&mut grid, 6, 10, wood);
            set_px(&mut grid, 5, 26, wood);
            set_px(&mut grid, 6, 26, wood);
        }
        WeaponType::BestaPesada => {
            fill_rect(&mut grid, 1, 18, 8, 3, wood_dark);
            fill_rect(&mut grid, 2, 15, 2, 7, steel);
            fill_rect(&mut grid, 5, 16, 1, 5, white);
        }
        WeaponType::AlaudeBardo => {
            fill_rect(&mut grid, 2, 16, 6, 7, outline);
            fill_rect(&mut grid, 3, 17, 4, 5, wood);
            fill_rect(&mut grid, 4, 11, 2, 6, wood_dark);
            set_px(&mut grid, 4, 19, gold);
            set_px(&mut grid, 5, 19, gold);
        }
    }

    let out_w = SPRITE_W * SCALE;
    let out_h = SPRITE_H * SCALE;
    let mut rgba = vec![0u8; out_w * out_h * 4];

    for y in 0..out_h {
        let sy = y / SCALE;
        for x in 0..out_w {
            let sx = x / SCALE;
            let px = grid[sy][sx];
            let idx = (y * out_w + x) * 4;
            rgba[idx..idx + 4].copy_from_slice(&px);
        }
    }

    let png_bytes = encode_rgba_png(out_w as u32, out_h as u32, &rgba);
    let b64 = encode_base64(&png_bytes);
    (out_w as u32, out_h as u32, png_bytes, b64)
}

fn encode_rgba_png(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    let row_bytes = (width as usize) * 4;
    let mut raw_scanlines = Vec::with_capacity((row_bytes + 1) * (height as usize));
    for y in 0..(height as usize) {
        raw_scanlines.push(0u8);
        let start = y * row_bytes;
        raw_scanlines.extend_from_slice(&rgba[start..start + row_bytes]);
    }

    let mut zlib = Vec::with_capacity(raw_scanlines.len() + 64);
    zlib.extend_from_slice(&[0x78, 0x01]);
    let chunks: Vec<&[u8]> = raw_scanlines.chunks(65_535).collect();
    for (i, chunk) in chunks.iter().enumerate() {
        let is_last = i + 1 == chunks.len();
        zlib.push(if is_last { 0x01 } else { 0x00 });
        let len = chunk.len() as u16;
        let nlen = !len;
        zlib.extend_from_slice(&len.to_le_bytes());
        zlib.extend_from_slice(&nlen.to_le_bytes());
        zlib.extend_from_slice(chunk);
    }
    zlib.extend_from_slice(&adler32(&raw_scanlines).to_be_bytes());

    let mut png = Vec::with_capacity(zlib.len() + 128);
    png.extend_from_slice(&[137, 80, 78, 71, 13, 10, 26, 10]);

    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    write_png_chunk(&mut png, b"IHDR", &ihdr);
    write_png_chunk(&mut png, b"IDAT", &zlib);
    write_png_chunk(&mut png, b"IEND", &[]);
    png
}

fn write_png_chunk(out: &mut Vec<u8>, chunk_type: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(chunk_type);
    out.extend_from_slice(data);
    let mut crc = 0xFFFF_FFFFu32;
    for &b in chunk_type.iter().chain(data.iter()) {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    out.extend_from_slice(&(!crc).to_be_bytes());
}

fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % 65_521;
        b = (b + a) % 65_521;
    }
    (b << 16) | a
}

fn encode_base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };

        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}
