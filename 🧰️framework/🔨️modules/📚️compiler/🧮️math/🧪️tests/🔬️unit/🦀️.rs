
use super::*;
use crate::syntax::parse_formula;

fn with_fonts<R>(f: impl FnOnce(&FontContext<'_>) -> R) -> R {
    let fonts = crate::world::embedded_fonts();
    let math = Font::from_bytes(fonts.math, 0).expect("parse math font");
    let serif = Font::from_bytes(fonts.serif, 0).expect("parse serif font");
    let mono = Font::from_bytes(fonts.mono, 0).expect("parse mono font");
    let emoji = Font::from_bytes(fonts.emoji, 0).expect("parse emoji font");
    let ctx = FontContext { math: &math, serif: &serif, mono: &mono, emoji: &emoji };
    f(&ctx)
}

fn layout_src(fonts: &FontContext<'_>, src: &str) -> MathBox {
    let node = parse_formula(src).unwrap_or_else(|e| panic!("parse {src:?} failed: {e}"));
    layout(fonts, &node)
}

#[test]
fn simple_symbol_has_positive_width_and_at_least_one_glyph() {
    with_fonts(|fonts| {
        let node = MathNode::Symbol("x".to_string());
        let box_ = layout(fonts, &node);
        assert!(box_.width > 0.0);
        assert!(!box_.items.is_empty());
    });
}

#[test]
fn superscript_is_narrower_and_shifted_above_the_baseline() {
    with_fonts(|fonts| {
        let box_ = layout_src(fonts, "x^2");
        assert!(box_.width > 0.0);
        assert!(box_.height > 0.0, "a superscript must raise the box's height");
        // Two items: the base glyph and the (smaller, higher) exponent glyph.
        assert_eq!(box_.items.len(), 2);
    });
}

#[test]
fn subscript_extends_the_depth_not_the_height() {
    with_fonts(|fonts| {
        let base_only = layout_src(fonts, "x");
        let subscripted = layout_src(fonts, "x_1");
        assert!(subscripted.depth > base_only.depth, "a subscript must extend depth: {} vs {}", subscripted.depth, base_only.depth);
    });
}

#[test]
fn fraction_stacks_numerator_over_denominator_around_the_axis() {
    with_fonts(|fonts| {
        let box_ = layout_src(fonts, "frac(a, b)");
        assert!(box_.height > 0.0 && box_.depth > 0.0, "a fraction must have both height and depth: {box_:?}");
        let rule_count = box_.items.iter().filter(|item| matches!(item, PlacedItem::Rule { .. })).count();
        assert_eq!(rule_count, 1, "a fraction must draw exactly one rule");
    });
}

#[test]
fn sqrt_draws_a_radical_sign_and_a_top_rule() {
    with_fonts(|fonts| {
        let box_ = layout_src(fonts, "sqrt(x)");
        let rule_count = box_.items.iter().filter(|item| matches!(item, PlacedItem::Rule { .. })).count();
        assert_eq!(rule_count, 1, "a radical must draw exactly one top rule");
        assert!(box_.width > layout_src(fonts, "x").width, "the radical sign must add width beyond the radicand alone");
    });
}

#[test]
fn matrix_lays_out_a_grid_wrapped_in_parens() {
    with_fonts(|fonts| {
        let box_ = layout_src(fonts, "mat(1, 2; 3, 4)");
        // 4 number glyphs + 2 stretchy paren glyphs.
        assert_eq!(box_.items.iter().filter(|i| matches!(i, PlacedItem::Glyph { .. })).count(), 6);
    });
}

#[test]
fn stretchy_parens_are_taller_for_taller_content() {
    with_fonts(|fonts| {
        let short = layout_src(fonts, "(x)");
        let tall = layout_src(fonts, "(frac(a, b))");
        assert!(tall.height + tall.depth > short.height + short.depth, "parens around a fraction must be taller than parens around a bare symbol");
    });
}

#[test]
fn emoji_shortcode_places_an_image_item() {
    with_fonts(|fonts| {
        let box_ = layout_src(fonts, ":rocket:");
        assert_eq!(box_.items.len(), 1);
        assert!(matches!(box_.items[0], PlacedItem::Image { .. }), "known shortcode must render as an image, got {:?}", box_.items[0]);
    });
}

#[test]
fn unknown_emoji_shortcode_falls_back_to_visible_text_not_a_blank_box() {
    with_fonts(|fonts| {
        let box_ = layout_src(fonts, ":not-a-real-shortcode:");
        assert!(!box_.items.is_empty(), "an unresolved shortcode must still render something visible");
        assert!(box_.items.iter().all(|i| matches!(i, PlacedItem::Glyph { .. })), "fallback must be text glyphs: {:?}", box_.items);
    });
}

#[test]
fn binary_operator_inserts_visible_spacing_between_operands() {
    with_fonts(|fonts| {
        let plain_sum = hbox(vec![layout_src(fonts, "x"), layout_src(fonts, "y")]);
        let spaced_sum = layout_src(fonts, "x + y");
        assert!(spaced_sum.width > plain_sum.width, "an explicit `+` must take more width than bare juxtaposition");
    });
}

#[test]
fn accent_adds_height_above_the_base() {
    with_fonts(|fonts| {
        let base = layout_src(fonts, "x");
        let accented = layout_src(fonts, "hat(x)");
        assert!(accented.height > base.height, "an accent must raise the box's height above the bare base");
    });
}

#[test]
fn unknown_call_name_renders_the_name_and_wrapped_args_rather_than_silently_dropping() {
    with_fonts(|fonts| {
        let box_ = layout_src(fonts, "mystery(x, y)");
        assert!(!box_.items.is_empty());
        assert!(box_.items.len() >= 4, "expected the label glyphs plus at least two stretchy delimiters plus content: {}", box_.items.len());
    });
}

#[test]
fn layout_raw_text_shapes_arbitrary_strings_without_parsing_notation_syntax() {
    with_fonts(|fonts| {
        // `_ ; < > !` are all special characters in math notation — a raw-text caller must be
        // able to include them literally, which `layout_raw_text` (no parser involved) allows.
        let box_ = layout_raw_text(fonts, "a_b; c<d!");
        assert!(box_.width > 0.0);
        assert!(!box_.items.is_empty());
    });
}

#[test]
fn layout_raw_emoji_places_a_raster_image_for_a_known_glyph() {
    with_fonts(|fonts| {
        let box_ = layout_raw_emoji(fonts, "🚀");
        assert_eq!(box_.items.len(), 1);
        assert!(matches!(box_.items[0], PlacedItem::Image { .. }));
    });
}

#[test]
fn layout_raw_code_shapes_via_the_mono_font() {
    with_fonts(|fonts| {
        let box_ = layout_raw_code(fonts, "fn main() {}");
        assert!(box_.width > 0.0);
        assert!(box_.items.iter().all(|i| matches!(i, PlacedItem::Glyph { font: FontKind::Mono, .. })));
    });
}
