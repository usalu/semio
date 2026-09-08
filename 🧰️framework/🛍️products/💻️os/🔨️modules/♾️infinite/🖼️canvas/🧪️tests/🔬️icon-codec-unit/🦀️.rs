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

    #[test]
    fn icon_codec_bare_dollar_sugar_decodes_as_math() {
        // The `$…$` delimiters are input sugar only — stripped, not echoed into `src`.
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
        let r = board_resolve_icon_kind(":grinning:", |_| None);
        match r {
            BoardResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn icon_codec_resolves_catalog_shortcode_to_svg() {
        let r = board_resolve_icon_kind(":plus:", |_| None);
        match r {
            BoardResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn icon_codec_resolves_metabolism_shortcode_to_themed_svg() {
        let r = board_resolve_icon_kind(":capsule_J:", |_| None);
        match r {
            BoardResolvedIcon::SvgThemed(s) => assert!(s.contains("<svg")),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn icon_codec_resolves_text_to_svg() {
        let r = board_resolve_icon_kind("text:Hi", |_| None);
        match r {
            BoardResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn icon_codec_url_returns_none_for_sync_resolver() {
        assert!(matches!(board_resolve_icon_kind("url:https://example.com/x.png", |_| None), BoardResolvedIcon::None));
    }
}
