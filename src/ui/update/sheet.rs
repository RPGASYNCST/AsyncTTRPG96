use iced::widget::image;
use iced::{Size, Task};

use crate::ui::app::RetroApp;
use crate::ui::models::{Message, Screen};
use crate::ui::sprite::{SpriteConfig, render_sprite_png};

impl RetroApp {
    pub(crate) fn update_sheet(&mut self, message: Message) -> Task<Message> {
        match message {
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
            _ => {}
        }
        Task::none()
    }
}
