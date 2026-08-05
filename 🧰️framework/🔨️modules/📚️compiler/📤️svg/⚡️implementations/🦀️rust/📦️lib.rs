//! 📤️ `compiler_svg` — serializes a `compiler_math::MathBox` into an SVG string: glyph outlines
//! become `<path>` fills, rules become `<rect>`, raster (emoji) glyphs become base64 `<image>` data
//! URLs. Emits only `svg`/`g`/`path`/`rect`/`image` — a strict subset of what `usvg` parses and
//! what this repo's existing `vello_svg::append_tree` / animate's path collector already consume,
//! replacing `typst_svg::svg_merged`'s role for the two Typst call sites.

use base64::Engine;
use compiler_math::{FontKind, MathBox, PlacedItem};
use compiler_text::Font;

//#region 🔖️Options
#[derive(Clone, Copy, Debug)]
pub struct SvgOptions {
    pub font_size_pt: f32,
    pub margin_pt: f32,
}

impl Default for SvgOptions {
    fn default() -> Self {
        Self { font_size_pt: 28.0, margin_pt: 3.0 }
    }
}

pub struct FontSet<'a> {
    pub math: &'a Font<'a>,
    pub serif: &'a Font<'a>,
    pub mono: &'a Font<'a>,
}
//#endregion 🔖️Options
//
// Note: this crate never embeds raw caller text into SVG markup — `MathNode::Text`/`Symbol`/
// `Emoji` atoms are always converted to glyph outline `<path>` data or base64 `<image>` bytes
// during layout (`compiler_math`), never written as literal XML text content. There is
// consequently no XML-injection surface here to escape against.

//#region 🔖️Render
fn font_for<'a>(fonts: &'a FontSet<'_>, kind: FontKind) -> Option<&'a Font<'a>> {
    match kind {
        FontKind::Math => Some(fonts.math),
        FontKind::Serif => Some(fonts.serif),
        FontKind::Mono => Some(fonts.mono),
        FontKind::Emoji => None, // emoji glyphs are pre-rasterized `PlacedItem::Image`, never `Glyph`
    }
}

/// @emoji 📐️ A `PlacedItem::Glyph`'s fields bundled into one struct purely to keep [`write_glyph`]
/// under clippy's argument-count lint — no behavior beyond that.
struct GlyphPlacement {
    font: FontKind,
    glyph_id: u16,
    x: f32,
    y: f32,
    scale_y: f32,
}

fn write_glyph(out: &mut String, fonts: &FontSet<'_>, placement: &GlyphPlacement, font_size_pt: f32) {
    let Some(font) = font_for(fonts, placement.font) else { return };
    let Some(path) = compiler_text::outline_glyph_path(font, placement.glyph_id) else { return };
    let units_per_em = font.units_per_em() as f32;
    let scale = font_size_pt / units_per_em;
    // Font design space is Y-up; SVG is Y-down. The glyph's own outline is in raw font units
    // (unscaled) — one transform per placement does the translate, unit scale, extra vertical
    // stretch (`scale_y`, from stretchy delimiters/radicals), and the Y-flip together.
    out.push_str(&format!(
        r#"<path transform="translate({:.3} {:.3}) scale({:.6} {:.6})" d="{path}"/>"#,
        placement.x * font_size_pt,
        -placement.y * font_size_pt,
        scale,
        -scale * placement.scale_y
    ));
}

fn write_rule(out: &mut String, x: f32, y: f32, width: f32, height: f32, font_size_pt: f32) {
    let px = x * font_size_pt;
    let py = -y * font_size_pt;
    let pw = width * font_size_pt;
    let ph = height * font_size_pt;
    out.push_str(&format!(r#"<rect x="{:.3}" y="{:.3}" width="{:.3}" height="{:.3}"/>"#, px, py - ph, pw, ph));
}

fn write_image(out: &mut String, data: &[u8], x: f32, y: f32, width: f32, height: f32, font_size_pt: f32) {
    let px = x * font_size_pt;
    let py = -y * font_size_pt - height * font_size_pt;
    let pw = width * font_size_pt;
    let ph = height * font_size_pt;
    let encoded = base64::engine::general_purpose::STANDARD.encode(data);
    out.push_str(&format!(r#"<image x="{px:.3}" y="{py:.3}" width="{pw:.3}" height="{ph:.3}" href="data:image/png;base64,{encoded}"/>"#));
}

/// @emoji 🖨️ Renders `math_box` as a complete, standalone SVG document string.
pub fn render_svg(math_box: &MathBox, fonts: &FontSet<'_>, options: SvgOptions) -> String {
    let width_pt = math_box.width * options.font_size_pt + options.margin_pt * 2.0;
    let height_pt = (math_box.height + math_box.depth) * options.font_size_pt + options.margin_pt * 2.0;
    let baseline_y = options.margin_pt + math_box.height * options.font_size_pt;

    let mut body = String::new();
    for item in &math_box.items {
        match item {
            PlacedItem::Glyph { font, glyph_id, x, y, scale_y } => write_glyph(&mut body, fonts, &GlyphPlacement { font: *font, glyph_id: *glyph_id, x: *x, y: *y, scale_y: *scale_y }, options.font_size_pt),
            PlacedItem::Rule { x, y, width, height } => write_rule(&mut body, *x, *y, *width, *height, options.font_size_pt),
            PlacedItem::Image { data, x, y, width, height } => write_image(&mut body, data, *x, *y, *width, *height, options.font_size_pt),
        }
    }

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width_pt:.3} {height_pt:.3}" width="{width_pt:.3}" height="{height_pt:.3}"><g transform="translate({:.3} {baseline_y:.3})" fill="currentColor">{body}</g></svg>"#,
        options.margin_pt,
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use compiler_math::FontContext;
    use compiler_syntax::parse_formula;

    fn with_fonts<R>(f: impl FnOnce(&FontContext<'_>, &FontSet<'_>) -> R) -> R {
        let embedded = compiler_world::embedded_fonts();
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
        let box_ = compiler_math::layout(layout_ctx, &node);
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
}
//#endregion 🧪️Tests
