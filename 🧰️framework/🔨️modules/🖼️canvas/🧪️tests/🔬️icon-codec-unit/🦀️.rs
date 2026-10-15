mod tests {
    use super::*;
    use std::sync::OnceLock;

    pub(crate) fn compiler_font_set() -> FontSet<'static> {
        static SET: OnceLock<FontSet<'static>> = OnceLock::new();
        *SET.get_or_init(|| {
            let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../📚️compiler/🌍️world/🔤️fonts");
            FontSet::from_directory(&directory).expect("compiler font assets must be readable from the repository")
        })
    }

    pub(crate) fn install_compiler_fonts_for_tests() {
        match install_compiler_fonts(compiler_font_set()) {
            Ok(()) | Err(CompilerFontsInstallError::AlreadyInstalled) => {}
            Err(failure) => panic!("{failure}"),
        }
    }

    fn round_trip(s: &str) {
        let icon = decode_icon(s).expect("decode");
        let encoded = encode_icon(&icon);
        let again = decode_icon(&encoded).expect("re-decode");
        assert_eq!(icon, again, "round-trip failed for {s} -> {encoded}");
    }

    #[test]
    fn icon_codec_round_trips_all_kinds() {
        round_trip("url:https://example.com/icon.png");
        round_trip(":smile:");
        round_trip("data:image/png;base64,iVBORw0KGgo=");
        round_trip("emoji:☺️");
        round_trip("math:x^2");
        round_trip("text:Hi");
        round_trip(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10"/></svg>"#);
        round_trip("capsule_J");
    }

    /// 💲️ The `$…$` delimiters are input sugar only — stripped, not echoed into `src`.
    #[test]
    fn icon_codec_bare_dollar_sugar_decodes_as_math() {
        assert_eq!(decode_icon("$x^2$"), Some(Icon::Math { src: "x^2".to_string() }));
    }

    #[test]
    fn icon_codec_decodes_themed_stem() {
        assert!(matches!(decode_icon("capsule_J"), Some(Icon::Themed { .. })));
    }

    #[test]
    fn icon_codec_decodes_catalog_stem() {
        assert!(matches!(decode_icon("plus"), Some(Icon::Catalog { .. })));
    }

    #[test]
    fn icon_codec_rejects_unknown_catalogish_stem() {
        assert!(decode_icon("unknown_icon_stem_with_underscore").is_none());
    }

    #[test]
    fn icon_codec_resolves_emoji_shortcode_to_svg() {
        install_compiler_fonts_for_tests();
        let r = resolve_icon_kind(":grinning:", |_| None);
        match r {
            ResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn icon_codec_resolves_catalog_shortcode_to_svg() {
        let r = resolve_icon_kind(":plus:", |_| None);
        match r {
            ResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn icon_codec_resolves_metabolism_shortcode_to_themed_svg() {
        let r = resolve_icon_kind(":capsule_J:", |_| None);
        match r {
            ResolvedIcon::SvgThemed(s) => assert!(s.contains("<svg")),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn icon_codec_resolves_text_to_svg() {
        install_compiler_fonts_for_tests();
        let r = resolve_icon_kind("text:Hi", |_| None);
        match r {
            ResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn icon_codec_resolves_math_to_svg_through_the_installed_fonts() {
        install_compiler_fonts_for_tests();
        match resolve_icon_kind("math:x^2", |_| None) {
            ResolvedIcon::SvgPlain(s) => assert!(s.contains("<path"), "{}", &s[..s.len().min(240)]),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn icon_codec_refuses_unparseable_compiler_fonts_and_a_second_install() {
        let real = compiler_font_set();
        let garbage: &'static [u8] = &[0u8; 16];
        assert_eq!(install_compiler_fonts(FontSet { serif: garbage, ..real }), Err(CompilerFontsInstallError::Unparseable { role: FontRole::Serif }));
        install_compiler_fonts_for_tests();
        assert_eq!(install_compiler_fonts(real), Err(CompilerFontsInstallError::AlreadyInstalled));
    }

    #[test]
    fn icon_codec_url_returns_none_for_sync_resolver() {
        assert!(matches!(resolve_icon_kind("url:https://example.com/x.png", |_| None), ResolvedIcon::None));
    }
}

#[test]
fn generated_shortcode_emoji_bindings_match_the_assets_snapshot_oracle() {
    let snapshot: serde_json::Value = serde_json::from_str(include_str!("../../../🖼️assets/🔣️icons/🔣️shortcodes.json")).expect("General Assets shortcode authority");
    let emoji = snapshot["emoji"].as_object().expect("emoji mapping");
    assert!(!emoji.is_empty());
    for (code, glyph) in emoji {
        let Some(icon_shortcodes::ShortcodeResolved::Emoji(actual)) = icon_shortcodes::icon_shortcode_resolve(code) else { panic!("{code}: expected emoji"); };
        assert_eq!(actual, glyph.as_str().expect("emoji glyph"), "{code}");
    }
    for key in snapshot["catalog"].as_array().expect("catalog identities") {
        let key = key.as_str().expect("catalog identity");
        if let Some(glyph) = emoji.get(key) {
            let Some(icon_shortcodes::ShortcodeResolved::Emoji(actual)) = icon_shortcodes::icon_shortcode_resolve(key) else { panic!("{key}: expected emoji collision"); };
            assert_eq!(actual, glyph.as_str().expect("emoji glyph"), "{key}");
        } else {
            assert!(matches!(icon_shortcodes::icon_shortcode_resolve(key), Some(icon_shortcodes::ShortcodeResolved::SvgPlain(_)) | Some(icon_shortcodes::ShortcodeResolved::SvgThemed(_))), "{key}");
        }
    }
}
