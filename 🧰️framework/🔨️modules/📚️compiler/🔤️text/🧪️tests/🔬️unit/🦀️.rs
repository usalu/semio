
use super::*;

fn math_font_bytes() -> &'static [u8] {
    crate::world::embedded_fonts().math
}

fn serif_font_bytes() -> &'static [u8] {
    crate::world::embedded_fonts().serif
}

#[test]
fn loads_embedded_fonts_and_reports_sane_metrics() {
    let font = Font::from_bytes(math_font_bytes(), 0).expect("parse Libertinus Math");
    assert!(font.units_per_em() >= 1000, "unexpected units_per_em: {}", font.units_per_em());
    assert!(font.ascender() > 0);
}

#[test]
fn glyph_index_resolves_ascii_letters() {
    let font = Font::from_bytes(serif_font_bytes(), 0).expect("parse Libertinus Serif");
    let id = font.glyph_index('x').expect("Libertinus Serif must cover 'x'");
    assert_ne!(id, 0, "glyph 0 is .notdef");
}

#[test]
fn shape_produces_one_glyph_per_ascii_letter_with_positive_advance() {
    let font = Font::from_bytes(serif_font_bytes(), 0).expect("parse");
    let run = shape(&font, "x");
    assert_eq!(run.glyphs.len(), 1);
    assert!(run.advance > 0);
}

#[test]
fn outline_glyph_path_is_nonempty_for_a_real_letter() {
    let font = Font::from_bytes(serif_font_bytes(), 0).expect("parse");
    let id = font.glyph_index('x').expect("glyph index");
    let path = outline_glyph_path(&font, id).expect("'x' must have an outline");
    assert!(path.starts_with('M'), "path must start with a moveto: {path:?}");
    assert!(path.contains('Z'), "path must close: {path:?}");
}

#[test]
fn math_constants_are_present_on_the_math_font() {
    let font = Font::from_bytes(math_font_bytes(), 0).expect("parse");
    let constants = math_constants(&font).expect("Libertinus Math must have a MATH table");
    assert!(constants.axis_height > 0, "axis_height should be positive: {}", constants.axis_height);
    assert!(constants.fraction_rule_thickness > 0);
    assert!(constants.script_percent_scale_down > 0 && constants.script_percent_scale_down <= 100);
}

#[test]
fn math_constants_are_absent_on_a_non_math_font() {
    let font = Font::from_bytes(serif_font_bytes(), 0).expect("parse");
    assert!(math_constants(&font).is_none(), "Libertinus Serif has no MATH table");
}

#[test]
fn math_stretch_variants_exist_for_parenthesis_on_the_math_font() {
    let font = Font::from_bytes(math_font_bytes(), 0).expect("parse");
    let paren = font.glyph_index('(').expect("Libertinus Math must cover '('");
    let variants = math_stretch_variants(&font, paren, true);
    assert!(!variants.is_empty(), "Libertinus Math should declare vertical stretch variants for '('");
    // The font orders variants smallest-first; every later one must be at least as large.
    for pair in variants.windows(2) {
        assert!(pair[1].advance >= pair[0].advance, "variants must be non-decreasing: {variants:?}");
    }
}

#[test]
fn glyph_raster_image_extracts_a_png_from_the_emoji_font() {
    let font_bytes = crate::world::embedded_fonts().emoji;
    let font = Font::from_bytes(font_bytes, 0).expect("parse Noto Color Emoji subset");
    // U+1F680 ROCKET must be present — the subset was curated for exactly this kind of usage.
    let id = font.glyph_index('🚀').expect("emoji subset must cover the rocket emoji");
    let raster = glyph_raster_image(&font, id, 96).expect("CBDT glyph must yield a raster image");
    assert!(!raster.data.is_empty());
    assert_eq!(&raster.data[1..4], b"PNG", "CBDT payload must be PNG-encoded: {:?}", &raster.data[..raster.data.len().min(8)]);
    assert!(raster.width > 0 && raster.height > 0);
}
