
use super::*;
use crate::math::FontContext;
use crate::syntax::parse_formula;

fn with_fonts<R>(f: impl FnOnce(&FontContext<'_>, &FontSet<'_>) -> R) -> R {
    let set = crate::world::tests::test_fonts();
    let math = Font::from_bytes(set.math, 0).expect("parse math font");
    let serif = Font::from_bytes(set.serif, 0).expect("parse serif font");
    let mono = Font::from_bytes(set.mono, 0).expect("parse mono font");
    let emoji = Font::from_bytes(set.emoji, 0).expect("parse emoji font");
    let layout_ctx = FontContext { math: &math, serif: &serif, mono: &mono, emoji: &emoji };
    let svg_ctx = FontSet { math: &math, serif: &serif, mono: &mono };
    f(&layout_ctx, &svg_ctx)
}

fn render_src(layout_ctx: &FontContext<'_>, svg_ctx: &FontSet<'_>, src: &str) -> String {
    let node = parse_formula(src).unwrap_or_else(|e| panic!("parse {src:?} failed: {e}"));
    let box_ = crate::math::layout(layout_ctx, &node);
    render_svg(&box_, svg_ctx, SvgOptions::default())
}

#[test]
fn renders_a_well_formed_svg_document_with_expected_root_attributes() {
    with_fonts(|layout_ctx, svg_ctx| {
        let svg = render_src(layout_ctx, svg_ctx, "x^2");
        assert!(svg.starts_with("<svg "), "must start with an <svg> root: {svg}");
        assert!(svg.contains("viewBox="));
        assert!(svg.contains("xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(svg.ends_with("</svg>"));
        assert!(svg.contains("<path"), "x^2 must emit at least one glyph path: {svg}");
    });
}

#[test]
fn fraction_svg_contains_a_rect_for_the_bar() {
    with_fonts(|layout_ctx, svg_ctx| {
        let svg = render_src(layout_ctx, svg_ctx, "frac(a, b)");
        assert!(svg.contains("<rect"), "a fraction must render a <rect> bar: {svg}");
    });
}

#[test]
fn emoji_svg_contains_an_embedded_png_image() {
    with_fonts(|layout_ctx, svg_ctx| {
        let svg = render_src(layout_ctx, svg_ctx, ":rocket:");
        assert!(svg.contains("<image"), "an emoji shortcode must render an <image>: {svg}");
        assert!(svg.contains("data:image/png;base64,"));
    });
}

#[test]
fn emoji_image_payload_is_the_standard_base64_of_a_png_the_third_party_oracle_decodes() {
    use base64::Engine as _;
    with_fonts(|layout_ctx, svg_ctx| {
        let svg = render_src(layout_ctx, svg_ctx, ":rocket:");
        let payload = svg.split("data:image/png;base64,").nth(1).and_then(|rest| rest.split('"').next()).expect("embedded payload");
        let decoded = base64::engine::general_purpose::STANDARD.decode(payload).expect("the oracle decodes the payload");
        assert!(decoded.starts_with(b"\x89PNG\r\n\x1a\n"), "payload must be a PNG");
        assert_eq!(base64::engine::general_purpose::STANDARD.encode(&decoded), payload);
        assert_eq!(semio_framework_io_base64::base64_standard_decode(payload).expect("first-party decode"), decoded);
    });
}

#[test]
fn rfc4648_vectors_encode_identically_in_the_first_party_codec_and_the_oracle() {
    use base64::Engine as _;
    for (plain, encoded) in [("", ""), ("f", "Zg=="), ("fo", "Zm8="), ("foo", "Zm9v"), ("foob", "Zm9vYg=="), ("fooba", "Zm9vYmE="), ("foobar", "Zm9vYmFy")] {
        assert_eq!(semio_framework_io_base64::base64_standard_encode(plain.as_bytes()), encoded);
        assert_eq!(base64::engine::general_purpose::STANDARD.encode(plain.as_bytes()), encoded);
    }
}

#[test]
fn viewbox_dimensions_are_positive_and_account_for_margin() {
    with_fonts(|layout_ctx, svg_ctx| {
        let svg = render_src(layout_ctx, svg_ctx, "x");
        let view_box = svg.split("viewBox=\"").nth(1).and_then(|s| s.split('"').next()).expect("viewBox attribute");
        let parts: Vec<f32> = view_box.split_whitespace().map(|p| p.parse().expect("numeric viewBox component")).collect();
        assert_eq!(parts.len(), 4);
        assert!(parts[2] > 2.0 * SvgOptions::default().margin_pt, "width must exceed the bare margin: {parts:?}");
        assert!(parts[3] > 2.0 * SvgOptions::default().margin_pt, "height must exceed the bare margin: {parts:?}");
    });
}
