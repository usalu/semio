
use super::*;
use crate::math::FontContext;
use crate::syntax::parse_formula;

fn with_fonts<R>(f: impl FnOnce(&FontContext<'_>, &FontSet<'_>) -> R) -> R {
    let embedded = crate::world::embedded_fonts();
    let math = Font::from_bytes(embedded.math, 0).expect("parse math font");
    let serif = Font::from_bytes(embedded.serif, 0).expect("parse serif font");
    let mono = Font::from_bytes(embedded.mono, 0).expect("parse mono font");
    let emoji = Font::from_bytes(embedded.emoji, 0).expect("parse emoji font");
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
