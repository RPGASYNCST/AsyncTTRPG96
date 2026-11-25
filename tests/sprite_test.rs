use AsyncTTRPG96::ui::sprite::{
    FacialFeature, HairStyle, OutfitStyle, SpriteConfig, render_sprite_png,
};

#[test]
fn test_default_sprite_renders_valid_png() {
    let cfg = SpriteConfig::default();
    let (w, h, png_bytes, b64) = render_sprite_png(&cfg);

    assert!(w > 0 && h > 0);
    assert!(png_bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]));
    assert!(!b64.is_empty());
}

#[test]
fn test_sprite_config_serde_roundtrip() {
    let cfg = SpriteConfig::randomize();
    let json = serde_json::to_string(&cfg).expect("serializacao deve funcionar");
    let decoded: SpriteConfig =
        serde_json::from_str(&json).expect("desserializacao deve funcionar");
    assert_eq!(cfg, decoded);
}

#[test]
fn test_all_hair_facial_and_outfit_variants_render() {
    for &hair in HairStyle::ALL {
        let cfg = SpriteConfig {
            hair_style: hair,
            ..Default::default()
        };
        let (_, _, bytes, _) = render_sprite_png(&cfg);
        assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
    }

    for &facial in FacialFeature::ALL {
        let cfg = SpriteConfig {
            facial,
            ..Default::default()
        };
        let (_, _, bytes, _) = render_sprite_png(&cfg);
        assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
    }

    for &outfit in OutfitStyle::ALL {
        let cfg = SpriteConfig {
            outfit_style: outfit,
            ..Default::default()
        };
        let (_, _, bytes, _) = render_sprite_png(&cfg);
        assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
    }
}
