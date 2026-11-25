pub mod auth;
pub mod session;
pub mod sheet;

use iced::{Size, Task};

use crate::ui::app::RetroApp;
use crate::ui::models::{Message, Screen};

impl RetroApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DragWindow => iced::window::oldest().and_then(iced::window::drag),
            Message::Minimize => {
                iced::window::oldest().and_then(|id| iced::window::minimize(id, true))
            }
            Message::Close => std::process::exit(0),
            Message::TabPressed(shift) => {
                if shift {
                    iced::widget::operation::focus_previous()
                } else {
                    iced::widget::operation::focus_next()
                }
            }
            Message::NavigateTo(screen) => self.navigate_to(screen),

            Message::UsernameChanged(_)
            | Message::AccessKeyChanged(_)
            | Message::Connect
            | Message::LoginFinished(_)
            | Message::Options
            | Message::PlayerUsernameChanged(_)
            | Message::PlayerPasswordChanged(_)
            | Message::PlayerConfirmPasswordChanged(_)
            | Message::PlayerInviteCodeChanged(_)
            | Message::PlayerHelp
            | Message::SubmitCreatePlayer
            | Message::PlayerCreated(_) => self.update_auth(message),

            Message::SessionNameChanged(_)
            | Message::SessionSystemChanged(_)
            | Message::SessionHostChanged(_)
            | Message::SessionEmailChanged(_)
            | Message::SessionPasswordChanged(_)
            | Message::SessionMaxPlayersChanged(_)
            | Message::SessionVisibilityChanged(_)
            | Message::SessionHelp
            | Message::SubmitCreateSession
            | Message::SessionEmailSent(_)
            | Message::SessionCodeInputChanged(_)
            | Message::VerifySessionCode
            | Message::SessionCodeVerified(_)
            | Message::ResendSessionEmail => self.update_session(message),

            Message::SheetNameChanged(_)
            | Message::SheetRaceChanged(_)
            | Message::SheetClassChanged(_)
            | Message::SheetAlignmentChanged(_)
            | Message::SheetItemsChanged(_)
            | Message::SheetGoldChanged(_)
            | Message::SheetBioAction(_)
            | Message::SheetAttrForChanged(_)
            | Message::SheetAttrDesChanged(_)
            | Message::SheetAttrConChanged(_)
            | Message::SheetAttrIntChanged(_)
            | Message::SheetAttrSabChanged(_)
            | Message::SheetAttrCarChanged(_)
            | Message::SheetRollAttributes
            | Message::SheetOpenSpriteEditor
            | Message::SheetLoadPortrait
            | Message::SheetPortraitLoaded(_)
            | Message::SheetHelp
            | Message::SubmitCreateSheet
            | Message::SheetSaved(_)
            | Message::SpriteSkinChanged(_)
            | Message::SpriteHeadChanged(_)
            | Message::SpriteHairStyleChanged(_)
            | Message::SpriteHairColorChanged(_)
            | Message::SpriteFacialChanged(_)
            | Message::SpriteOutfitStyleChanged(_)
            | Message::SpriteOutfitColorChanged(_)
            | Message::SpriteCloakChanged(_)
            | Message::SpriteWeaponChanged(_)
            | Message::SpriteRandomize
            | Message::SpriteResetDefault
            | Message::SpriteApplyToSheet => self.update_sheet(message),
        }
    }

    fn navigate_to(&mut self, screen: Screen) -> Task<Message> {
        self.screen = screen;
        self.status_info = None;
        let target_size = match screen {
            Screen::Login => Size::new(396.0, 326.0),
            Screen::CreateSession => {
                if !self.username.trim().is_empty() && self.session_host.is_empty() {
                    self.session_host = self.username.clone();
                }
                self.session_is_sending = false;
                self.session_awaiting_confirmation = false;
                self.session_confirmed = false;
                self.session_code_input.clear();
                self.session_status = ("Aguardando configuração da nova sessão.".to_string(), None);
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
        iced::window::oldest().and_then(move |id| iced::window::resize(id, target_size))
    }
}
