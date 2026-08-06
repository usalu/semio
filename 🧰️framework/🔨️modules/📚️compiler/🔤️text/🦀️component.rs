//! 🔤️ `compiler_text` — the compiler's text engine: font loading, HarfBuzz-class shaping, glyph
//! outline/raster extraction, and OpenType MATH table access. Wraps `rustybuzz`/`ttf-parser`
//! (rustybuzz's own re-export, so exactly one resolved `ttf-parser` version) entirely behind owned
//! types — no `rustybuzz::*`/`ttf_parser::*` type ever appears in this crate's public API, per the
//! "external libraries behind an interface" rule. All linear measurements this crate returns are in
//! **font design units** (the `units_per_em()` scale) unless documented otherwise — callers convert
//! to em/point space, this crate stays a thin, honest wrapper.

use rustybuzz::ttf_parser::{self, GlyphId};

//#region 🔖️Font
/// @emoji 🔤️ One loaded font face.
pub struct Font<'a> {
    face: rustybuzz::Face<'a>,
}

impl<'a> Font<'a> {
    /// @emoji 📂️ Parses font `index` (usually `0`) out of `data` (a whole OTF/TTF/TTC file).
    pub fn from_bytes(data: &'a [u8], index: u32) -> Option<Self> {
        Some(Self { face: rustybuzz::Face::from_slice(data, index)? })
    }

    pub fn units_per_em(&self) -> u16 {
        // Real fonts always report a units_per_em well within u16 range (1000/2048 are the
        // near-universal values); ttf-parser's own type is `i32` only because the accessor is
        // shared with a raw-table read that doesn't validate range up front.
        self.face.units_per_em() as u16
    }

    pub fn ascender(&self) -> i16 {
        self.face.ascender()
    }

    pub fn descender(&self) -> i16 {
        self.face.descender()
    }

    /// @emoji 🔍️ Maps a Unicode scalar to a glyph ID via the font's `cmap`, `None` if unmapped.
    pub fn glyph_index(&self, ch: char) -> Option<u16> {
        self.face.glyph_index(ch).map(|id| id.0)
    }

    pub fn glyph_hor_advance(&self, glyph_id: u16) -> Option<u16> {
        self.face.glyph_hor_advance(GlyphId(glyph_id))
    }

    /// @emoji 📦️ `(x_min, y_min, x_max, y_max)` in font design units — `None` for glyphs with no
    /// outline (space). Used to size a placed glyph's real ascent/descent instead of falling back
    /// to whole-font ascender/descender for every atom.
    pub fn glyph_bounding_box(&self, glyph_id: u16) -> Option<(i16, i16, i16, i16)> {
        let bbox = self.face.glyph_bounding_box(GlyphId(glyph_id))?;
        Some((bbox.x_min, bbox.y_min, bbox.x_max, bbox.y_max))
    }
}
//#endregion 🔖️Font

//#region 🔖️Shaping
/// @emoji 🧾️ One shaped glyph, positioned relative to its run's origin — font design units.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShapedGlyph {
    pub glyph_id: u16,
    pub cluster: u32,
    pub x_advance: i32,
    pub y_advance: i32,
    pub x_offset: i32,
    pub y_offset: i32,
}

/// @emoji 📏️ A shaped run of glyphs plus its total advance — font design units.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlyphRun {
    pub glyphs: Vec<ShapedGlyph>,
    pub advance: i32,
}

/// @emoji ✍️ Shapes `text` (left-to-right, no script/language override — every current caller is
/// short math/Latin/Greek runs where HarfBuzz's own auto-detection is correct) against `font`.
pub fn shape(font: &Font<'_>, text: &str) -> GlyphRun {
    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str(text);
    buffer.guess_segment_properties();
    let output = rustybuzz::shape(&font.face, &[], buffer);
    let infos = output.glyph_infos();
    let positions = output.glyph_positions();
    let mut glyphs = Vec::with_capacity(infos.len());
    let mut advance = 0i32;
    for (info, pos) in infos.iter().zip(positions.iter()) {
        glyphs.push(ShapedGlyph { glyph_id: info.glyph_id as u16, cluster: info.cluster, x_advance: pos.x_advance, y_advance: pos.y_advance, x_offset: pos.x_offset, y_offset: pos.y_offset });
        advance += pos.x_advance;
    }
    GlyphRun { glyphs, advance }
}
//#endregion 🔖️Shaping

//#region 🔖️Outline
/// @emoji ✏️ One glyph outline as an SVG path `d` string, font design units, Y-up (SVG's own Y-down
/// convention is the caller's problem — the caller already has to apply a translate/scale transform
/// per placement, and folding a Y-flip into that one transform is simpler than flipping every curve
/// control point here).
struct PathBuilder {
    d: String,
}

impl ttf_parser::OutlineBuilder for PathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.d.push_str(&format!("M{x} {y} "));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.d.push_str(&format!("L{x} {y} "));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.d.push_str(&format!("Q{x1} {y1} {x} {y} "));
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.d.push_str(&format!("C{x1} {y1} {x2} {y2} {x} {y} "));
    }
    fn close(&mut self) {
        self.d.push_str("Z ");
    }
}

/// @emoji ✏️ Extracts `glyph_id`'s outline from `font` as an SVG path `d` string in font design
/// units (Y-up) — `None` for glyphs with no outline (space, or a color/raster-only glyph).
pub fn outline_glyph_path(font: &Font<'_>, glyph_id: u16) -> Option<String> {
    let mut builder = PathBuilder { d: String::new() };
    font.face.outline_glyph(GlyphId(glyph_id), &mut builder)?;
    Some(builder.d.trim_end().to_string())
}
//#endregion 🔖️Outline

//#region 🔖️Raster
/// @emoji 🖼️ One extracted color/bitmap glyph — already-encoded image bytes (PNG for the CBDT path
/// this crate's only raster consumer, Noto Color Emoji, uses) plus placement metrics in font design
/// units at the returned `pixels_per_em` strike.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RasterGlyph {
    pub data: Vec<u8>,
    pub width: u16,
    pub height: u16,
    pub x: i16,
    pub y: i16,
    pub pixels_per_em: u16,
}

/// @emoji 🖼️ Looks up `glyph_id`'s embedded raster image (CBDT/CBLC or `sbix`) at the strike
/// closest to `pixels_per_em` — `None` for fonts/glyphs with no embedded raster data.
pub fn glyph_raster_image(font: &Font<'_>, glyph_id: u16, pixels_per_em: u16) -> Option<RasterGlyph> {
    let image = font.face.glyph_raster_image(GlyphId(glyph_id), pixels_per_em)?;
    Some(RasterGlyph { data: image.data.to_vec(), width: image.width, height: image.height, x: image.x, y: image.y, pixels_per_em: image.pixels_per_em })
}
//#endregion 🔖️Raster

//#region 🔖️Math
/// @emoji 🧮️ The OpenType `MATH` table's `MathConstants` subtable, extracted into an owned struct —
/// field names match the spec's own snake_case names 1:1. All device-table-adjusted values (font
/// design units, or thousandths-of-a-percent for the two `*_percent_scale_down` fields, matching the
/// spec) — see <https://learn.microsoft.com/en-us/typography/opentype/spec/math#mathconstants-table>.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MathConstants {
    pub script_percent_scale_down: i16,
    pub script_script_percent_scale_down: i16,
    pub axis_height: i16,
    pub accent_base_height: i16,
    pub flattened_accent_base_height: i16,
    pub subscript_shift_down: i16,
    pub subscript_top_max: i16,
    pub subscript_baseline_drop_min: i16,
    pub superscript_shift_up: i16,
    pub superscript_shift_up_cramped: i16,
    pub superscript_bottom_min: i16,
    pub superscript_baseline_drop_max: i16,
    pub sub_superscript_gap_min: i16,
    pub superscript_bottom_max_with_subscript: i16,
    pub space_after_script: i16,
    pub upper_limit_gap_min: i16,
    pub upper_limit_baseline_rise_min: i16,
    pub lower_limit_gap_min: i16,
    pub lower_limit_baseline_drop_min: i16,
    pub stack_top_shift_up: i16,
    pub stack_top_display_style_shift_up: i16,
    pub stack_bottom_shift_down: i16,
    pub stack_bottom_display_style_shift_down: i16,
    pub stack_gap_min: i16,
    pub stack_display_style_gap_min: i16,
    pub stretch_stack_top_shift_up: i16,
    pub stretch_stack_bottom_shift_down: i16,
    pub stretch_stack_gap_above_min: i16,
    pub stretch_stack_gap_below_min: i16,
    pub fraction_numerator_shift_up: i16,
    pub fraction_numerator_display_style_shift_up: i16,
    pub fraction_denominator_shift_down: i16,
    pub fraction_denominator_display_style_shift_down: i16,
    pub fraction_numerator_gap_min: i16,
    pub fraction_num_display_style_gap_min: i16,
    pub fraction_rule_thickness: i16,
    pub fraction_denominator_gap_min: i16,
    pub fraction_denom_display_style_gap_min: i16,
    pub skewed_fraction_horizontal_gap: i16,
    pub skewed_fraction_vertical_gap: i16,
    pub overbar_vertical_gap: i16,
    pub overbar_rule_thickness: i16,
    pub overbar_extra_ascender: i16,
    pub underbar_vertical_gap: i16,
    pub underbar_rule_thickness: i16,
    pub underbar_extra_descender: i16,
    pub radical_vertical_gap: i16,
    pub radical_display_style_vertical_gap: i16,
    pub radical_rule_thickness: i16,
    pub radical_extra_ascender: i16,
    pub radical_kern_before_degree: i16,
    pub radical_kern_after_degree: i16,
    pub radical_degree_bottom_raise_percent: i16,
}

/// @emoji 🧮️ Reads `font`'s `MATH` table constants — `None` if the font has no `MATH` table.
pub fn math_constants(font: &Font<'_>) -> Option<MathConstants> {
    let math = font.face.tables().math?;
    let c = math.constants?;
    Some(MathConstants {
        script_percent_scale_down: c.script_percent_scale_down(),
        script_script_percent_scale_down: c.script_script_percent_scale_down(),
        axis_height: c.axis_height().value,
        accent_base_height: c.accent_base_height().value,
        flattened_accent_base_height: c.flattened_accent_base_height().value,
        subscript_shift_down: c.subscript_shift_down().value,
        subscript_top_max: c.subscript_top_max().value,
        subscript_baseline_drop_min: c.subscript_baseline_drop_min().value,
        superscript_shift_up: c.superscript_shift_up().value,
        superscript_shift_up_cramped: c.superscript_shift_up_cramped().value,
        superscript_bottom_min: c.superscript_bottom_min().value,
        superscript_baseline_drop_max: c.superscript_baseline_drop_max().value,
        sub_superscript_gap_min: c.sub_superscript_gap_min().value,
        superscript_bottom_max_with_subscript: c.superscript_bottom_max_with_subscript().value,
        space_after_script: c.space_after_script().value,
        upper_limit_gap_min: c.upper_limit_gap_min().value,
        upper_limit_baseline_rise_min: c.upper_limit_baseline_rise_min().value,
        lower_limit_gap_min: c.lower_limit_gap_min().value,
        lower_limit_baseline_drop_min: c.lower_limit_baseline_drop_min().value,
        stack_top_shift_up: c.stack_top_shift_up().value,
        stack_top_display_style_shift_up: c.stack_top_display_style_shift_up().value,
        stack_bottom_shift_down: c.stack_bottom_shift_down().value,
        stack_bottom_display_style_shift_down: c.stack_bottom_display_style_shift_down().value,
        stack_gap_min: c.stack_gap_min().value,
        stack_display_style_gap_min: c.stack_display_style_gap_min().value,
        stretch_stack_top_shift_up: c.stretch_stack_top_shift_up().value,
        stretch_stack_bottom_shift_down: c.stretch_stack_bottom_shift_down().value,
        stretch_stack_gap_above_min: c.stretch_stack_gap_above_min().value,
        stretch_stack_gap_below_min: c.stretch_stack_gap_below_min().value,
        fraction_numerator_shift_up: c.fraction_numerator_shift_up().value,
        fraction_numerator_display_style_shift_up: c.fraction_numerator_display_style_shift_up().value,
        fraction_denominator_shift_down: c.fraction_denominator_shift_down().value,
        fraction_denominator_display_style_shift_down: c.fraction_denominator_display_style_shift_down().value,
        fraction_numerator_gap_min: c.fraction_numerator_gap_min().value,
        fraction_num_display_style_gap_min: c.fraction_num_display_style_gap_min().value,
        fraction_rule_thickness: c.fraction_rule_thickness().value,
        fraction_denominator_gap_min: c.fraction_denominator_gap_min().value,
        fraction_denom_display_style_gap_min: c.fraction_denom_display_style_gap_min().value,
        skewed_fraction_horizontal_gap: c.skewed_fraction_horizontal_gap().value,
        skewed_fraction_vertical_gap: c.skewed_fraction_vertical_gap().value,
        overbar_vertical_gap: c.overbar_vertical_gap().value,
        overbar_rule_thickness: c.overbar_rule_thickness().value,
        overbar_extra_ascender: c.overbar_extra_ascender().value,
        underbar_vertical_gap: c.underbar_vertical_gap().value,
        underbar_rule_thickness: c.underbar_rule_thickness().value,
        underbar_extra_descender: c.underbar_extra_descender().value,
        radical_vertical_gap: c.radical_vertical_gap().value,
        radical_display_style_vertical_gap: c.radical_display_style_vertical_gap().value,
        radical_rule_thickness: c.radical_rule_thickness().value,
        radical_extra_ascender: c.radical_extra_ascender().value,
        radical_kern_before_degree: c.radical_kern_before_degree().value,
        radical_kern_after_degree: c.radical_kern_after_degree().value,
        radical_degree_bottom_raise_percent: c.radical_degree_bottom_raise_percent(),
    })
}

/// @emoji 📐️ `MathGlyphInfo.MathItalicsCorrectionInfo` for one glyph — `0` if the font has no entry
/// (correct default: no correction).
pub fn math_italics_correction(font: &Font<'_>, glyph_id: u16) -> i16 {
    font.face.tables().math.and_then(|m| m.glyph_info).and_then(|info| info.italic_corrections).and_then(|table| table.get(GlyphId(glyph_id))).map_or(0, |v| v.value)
}

/// @emoji 🎯️ `MathGlyphInfo.MathTopAccentAttachment` for one glyph — the X position (font design
/// units, from the glyph's own origin) an accent should be centered over. `None` falls back to the
/// glyph's horizontal midpoint (the spec's own documented default).
pub fn math_top_accent_attachment(font: &Font<'_>, glyph_id: u16) -> Option<i16> {
    font.face.tables().math?.glyph_info?.top_accent_attachments?.get(GlyphId(glyph_id)).map(|v| v.value)
}

/// @emoji 📏️ One alternate glyph from `MathVariants` — progressively larger/taller stand-ins for a
/// base glyph (e.g. bigger parentheses), ordered smallest-first by the font itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StretchVariant {
    pub glyph_id: u16,
    /// Advance along the stretch axis (width for horizontal, height for vertical) — font design units.
    pub advance: u16,
}

/// @emoji 📏️ `MathVariants.VerticalGlyphCoverage`/`HorizontalGlyphCoverage` for `base_glyph` —
/// empty if the font declares no variants for it (every current caller then falls back to scaling
/// the base glyph's own outline).
pub fn math_stretch_variants(font: &Font<'_>, base_glyph_id: u16, vertical: bool) -> Vec<StretchVariant> {
    let Some(math) = font.face.tables().math else { return Vec::new() };
    let Some(variants) = math.variants else { return Vec::new() };
    let coverage = if vertical { variants.vertical_constructions } else { variants.horizontal_constructions };
    let Some(construction) = coverage.get(GlyphId(base_glyph_id)) else { return Vec::new() };
    construction.variants.into_iter().map(|v| StretchVariant { glyph_id: v.variant_glyph.0, advance: v.advance_measurement }).collect()
}

/// @emoji 📏️ The minimum vertical extent (ascent + descent) `MATH` guarantees for `glyph_id` — used
/// to size a placed glyph's box when no shaped run gives us one (e.g. a directly-picked stretch
/// variant). Falls back to the font's global ascender/descender when the glyph has no per-glyph
/// vertical extents in `MATH` (`MathVariants` doesn't carry per-glyph bounding boxes, so this uses
/// `hhea`/`OS/2`-level metrics — a documented approximation, not a real per-glyph tight bound).
pub fn glyph_vertical_extent(font: &Font<'_>) -> (i16, i16) {
    (font.face.ascender(), font.face.descender())
}
//#endregion 🔖️Math

//#region 🧪️Tests
#[cfg(test)]
mod tests {
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
}
//#endregion 🧪️Tests
