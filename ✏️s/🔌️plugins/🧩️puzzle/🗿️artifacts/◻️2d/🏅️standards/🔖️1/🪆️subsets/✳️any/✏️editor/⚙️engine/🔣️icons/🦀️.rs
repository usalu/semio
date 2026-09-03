//! 🔣️ Puzzle 2d app engine — the metabolism icon table: `build.rs` walks the shared
//! `🌱️metabolism/🔣️icons` asset directory and generates a `board_metabolism_icon_svg(key)` match arm
//! per SVG, keyed by the bare stem after the repo's emoji filename prefix. The board's icon codec
//! reaches it through [`puzzle_themed_icon_lookup`].

mod board_metabolism_icons {
    include!(concat!(env!("OUT_DIR"), "/board_metabolism_icon_match.rs"));
}

/// 🔣️ Resolves a board catalog icon key to its SVG source, or `None` when the key is not a
/// metabolism asset (the codec then falls back to typst-math / emoji rendering).
pub fn puzzle_themed_icon_lookup(key: &str) -> Option<&'static str> {
    board_metabolism_icons::board_metabolism_icon_svg(key)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::puzzle_themed_icon_lookup;
    use crate::editor::puzzle2d::engine::canvas;

    #[test]
    fn svg_icon_append_smoke() {
        let mut scene = canvas::Scene::new();
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10" fill="#ffffff"/><path d="M0 0 L10 10" stroke="#000000" stroke-width="1"/></svg>"##;
        canvas::svg_icon::append_svg_str(&mut scene, svg).expect("parse svg");
        let fg = canvas::Color::from_rgba8(200, 10, 10, 255);
        let bg = canvas::Color::from_rgba8(10, 200, 10, 255);
        let mut scene2 = canvas::Scene::new();
        canvas::svg_icon::append_svg_str_themed(&mut scene2, svg, fg, bg).expect("parse themed");
    }

    #[test]
    fn board_icon_codec_resolves_catalog_key_via_themed_lookup() {
        let r = canvas::icon_codec::board_resolve_icon_kind("capsule_J", puzzle_themed_icon_lookup);
        match r {
            canvas::icon_codec::BoardResolvedIcon::SvgThemed(s) => {
                assert!(s.contains("<svg"), "catalog metabolism key should resolve via themed lookup");
            }
            other => panic!("unexpected resolution for catalog capsule_J: {other:?}"),
        }
    }

    #[test]
    fn board_icon_codec_resolves_typst_math_to_svg_plain() {
        let r = canvas::icon_codec::board_resolve_icon_kind("typst:$x^2$", puzzle_themed_icon_lookup);
        match r {
            canvas::icon_codec::BoardResolvedIcon::SvgPlain(s) => {
                assert!(s.contains("<svg"), "{}", &s[..s.len().min(240)]);
            }
            other => panic!("unexpected resolution: {other:?}"),
        }
    }

    #[test]
    fn board_icon_codec_resolves_emoji_prefix_without_tofu() {
        let r = canvas::icon_codec::board_resolve_icon_kind("emoji:☺️", puzzle_themed_icon_lookup);
        match r {
            canvas::icon_codec::BoardResolvedIcon::SvgPlain(s) => {
                assert!(s.contains("<svg"), "{}", &s[..s.len().min(240)]);
                assert!(!s.contains('\u{fffd}'), "expected no U+FFFD replacement in emoji SVG, got {}", &s[..s.len().min(400)]);
            }
            other => panic!("unexpected resolution: {other:?}"),
        }
    }

    #[test]
    fn svg_icon_content_bounds_follows_nested_group_translate() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200"><g transform="translate(72 88)"><rect width="12" height="12" fill="rgb(8,8,8)"/></g></svg>"#;
        let (x, y, w, h) = canvas::svg_icon::svg_icon_content_bounds_from_str(svg).expect("parse");
        assert!((70.0..=74.0).contains(&x), "expected translated art near x≈72, got {x}");
        assert!((86.0..=90.0).contains(&y), "expected translated art near y≈88, got {y}");
        assert!(w > 10.0 && w < 14.0 && h > 10.0 && h < 14.0, "expected ~12×12 bbox, got {w}×{h}");
    }

    #[test]
    fn svg_icon_content_bounds_includes_visible_image_abs_box() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><image href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==" x="30" y="40" width="50" height="50"/></svg>"##;
        let (x, y, w, h) = canvas::svg_icon::svg_icon_content_bounds_from_str(svg).expect("parse");
        assert!((x - 30.0).abs() < 2.0, "expected image bbox near x=30, got {x}");
        assert!((y - 40.0).abs() < 2.0, "expected image bbox near y=40, got {y}");
        assert!((w - 50.0).abs() < 2.0 && (h - 50.0).abs() < 2.0, "expected ~50×50 bbox, got {w}×{h}");
    }
}
//#endregion 🧪️Tests
