use crate::mail::{LoginResposta, UsuarioLogado};
use crate::ui::app::RetroApp;
use crate::ui::models::{Message, RpgSystem, Screen};
use crate::ui::sprite::{FacialFeature, HairStyle, OutfitStyle};

fn mock_login_response(tem_ficha: bool) -> LoginResposta {
    LoginResposta {
        token: "token-teste-123".to_string(),
        usuario: UsuarioLogado {
            id: 42,
            login: "aventureiro".to_string(),
            nome: "Aventureiro".to_string(),
            admin: false,
        },
        sessao_id: 7,
        sessao_nome: "Mesa Teste".to_string(),
        sessao_sistema: "D&D".to_string(),
        players_id: 99,
        tem_ficha,
    }
}

#[test]
fn test_initial_state_and_titles() {
    let mut app = RetroApp::new();
    assert_eq!(app.screen, Screen::Login);
    assert_eq!(app.title(), "Sessão de Acesso - RPG ASYNC");

    let _ = app.update(Message::NavigateTo(Screen::CreateSession));
    assert_eq!(app.screen, Screen::CreateSession);
    assert_eq!(app.title(), "RPGASYNCST — Criar Sessão");

    let _ = app.update(Message::NavigateTo(Screen::CreatePlayer));
    assert_eq!(app.screen, Screen::CreatePlayer);
    assert_eq!(app.title(), "RPGASYNCST — Criar Jogador");
}

#[test]
fn test_login_validation_rejects_empty_fields() {
    let mut app = RetroApp::new();

    let _ = app.update(Message::Connect);
    assert!(app.status_is_error);
    assert_eq!(app.status_info.as_deref(), Some("Informe o usuário."));

    let _ = app.update(Message::UsernameChanged("jogador".to_string()));
    let _ = app.update(Message::Connect);
    assert!(app.status_is_error);
    assert_eq!(app.status_info.as_deref(), Some("Informe a senha."));
}

#[test]
fn test_login_finished_routes_by_tem_ficha() {
    let mut app = RetroApp::new();
    let _ = app.update(Message::LoginFinished(Ok(mock_login_response(false))));
    assert_eq!(app.screen, Screen::CreateSheet);
    assert_eq!(app.auth_token.as_deref(), Some("token-teste-123"));
    assert_eq!(app.active_sessao_id, 7);
    assert_eq!(app.active_players_id, 99);

    let mut app_with_sheet = RetroApp::new();
    let _ = app_with_sheet.update(Message::LoginFinished(Ok(mock_login_response(true))));
    assert_eq!(app_with_sheet.screen, Screen::Login);
    assert!(!app_with_sheet.status_is_error);
    assert!(
        app_with_sheet
            .status_info
            .as_deref()
            .unwrap_or("")
            .contains("Mesa Teste")
    );
}

#[test]
fn test_create_session_validation_and_system() {
    let mut app = RetroApp::new();
    let _ = app.update(Message::NavigateTo(Screen::CreateSession));
    assert_eq!(app.session_system, RpgSystem::DnD);

    let _ = app.update(Message::SubmitCreateSession);
    assert_eq!(app.session_status.1, Some(true));

    let _ = app.update(Message::SessionNameChanged("Campanha".to_string()));
    let _ = app.update(Message::SessionHostChanged("ab".to_string()));
    let _ = app.update(Message::SubmitCreateSession);
    assert_eq!(app.session_status.1, Some(true));

    let _ = app.update(Message::SessionHostChanged("Mestre".to_string()));
    let _ = app.update(Message::SessionEmailChanged("email-invalido".to_string()));
    let _ = app.update(Message::SubmitCreateSession);
    assert_eq!(app.session_status.1, Some(true));

    let _ = app.update(Message::SessionEmailChanged("mestre@teste.com".to_string()));
    let _ = app.update(Message::SessionPasswordChanged("123".to_string()));
    let _ = app.update(Message::SubmitCreateSession);
    assert_eq!(app.session_status.1, Some(true));

    let _ = app.update(Message::SessionPasswordChanged("123456".to_string()));
    let _ = app.update(Message::SessionMaxPlayersChanged("0".to_string()));
    let _ = app.update(Message::SubmitCreateSession);
    assert_eq!(app.session_status.1, Some(true));
}

#[test]
fn test_create_player_validation() {
    let mut app = RetroApp::new();
    let _ = app.update(Message::NavigateTo(Screen::CreatePlayer));

    let _ = app.update(Message::SubmitCreatePlayer);
    assert_eq!(app.player_status.1, Some(true));

    let _ = app.update(Message::PlayerUsernameChanged("Kaelen".to_string()));
    let _ = app.update(Message::PlayerPasswordChanged("123456".to_string()));
    let _ = app.update(Message::PlayerConfirmPasswordChanged("654321".to_string()));
    let _ = app.update(Message::PlayerInviteCodeChanged("Mesa".to_string()));
    let _ = app.update(Message::SubmitCreatePlayer);
    assert_eq!(app.player_status.0, "As Chaves de Acesso não coincidem.");
    assert_eq!(app.player_status.1, Some(true));
}

#[test]
fn test_sheet_roll_attributes_and_validation() {
    let mut app = RetroApp::new();
    let _ = app.update(Message::SheetRollAttributes);
    assert_eq!(app.sheet_status.1, Some(false));

    for attr in [
        &app.sheet_attr_for,
        &app.sheet_attr_des,
        &app.sheet_attr_con,
        &app.sheet_attr_int,
        &app.sheet_attr_sab,
        &app.sheet_attr_car,
    ] {
        let parsed = attr.parse::<u8>().expect("atributo deve ser numerico");
        assert!((3..=18).contains(&parsed));
    }

    let _ = app.update(Message::SheetNameChanged("   ".to_string()));
    let _ = app.update(Message::SubmitCreateSheet);
    assert_eq!(app.sheet_status.1, Some(true));

    let _ = app.update(Message::SheetNameChanged("Eldrin".to_string()));
    let _ = app.update(Message::SheetAttrForChanged("99".to_string()));
    let _ = app.update(Message::SubmitCreateSheet);
    assert_eq!(app.sheet_status.1, Some(true));

    let _ = app.update(Message::SheetAttrForChanged("14".to_string()));
    let _ = app.update(Message::SheetGoldChanged("abc".to_string()));
    let _ = app.update(Message::SubmitCreateSheet);
    assert_eq!(app.sheet_status.1, Some(true));

    let _ = app.update(Message::SheetGoldChanged("120".to_string()));
    let _ = app.update(Message::SubmitCreateSheet);
    assert_eq!(app.sheet_status.1, Some(true));
}

#[test]
fn test_sprite_editor_customization_and_apply() {
    let mut app = RetroApp::new();
    let _ = app.update(Message::SheetOpenSpriteEditor);
    assert_eq!(app.screen, Screen::SpriteEditor);

    let _ = app.update(Message::SpriteHairStyleChanged(HairStyle::LongoOndulado));
    let _ = app.update(Message::SpriteFacialChanged(FacialFeature::BarbaLonga));
    let _ = app.update(Message::SpriteOutfitStyleChanged(OutfitStyle::MantoArcano));

    assert_eq!(app.sprite_config.hair_style, HairStyle::LongoOndulado);
    assert_eq!(app.sprite_config.facial, FacialFeature::BarbaLonga);
    assert_eq!(app.sprite_config.outfit_style, OutfitStyle::MantoArcano);

    let _ = app.update(Message::SpriteApplyToSheet);
    assert_eq!(app.screen, Screen::CreateSheet);
    assert_eq!(
        app.sheet_portrait_filename.as_deref(),
        Some("custom_sprite_16bit.png")
    );
    assert!(app.sheet_portrait_base64.is_some());
    assert_eq!(app.sheet_status.1, Some(false));
}
