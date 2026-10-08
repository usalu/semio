mod tests {
    use super::*;

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
        let r = resolve_icon_kind("text:Hi", |_| None);
        match r {
            ResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
            other => panic!("unexpected: {other:?}"),
        }
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
