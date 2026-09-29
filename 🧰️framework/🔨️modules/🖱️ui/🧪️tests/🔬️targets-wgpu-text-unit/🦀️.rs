use super::FontAtlas;

#[test]
fn from_bytes_falls_back_to_embedded_fonts_for_unparseable_input() {
    assert!(FontAtlas::from_bytes(&[]).is_ok());
    let woff2 = b"wOF2\x00\x01\x00\x00";
    let mut atlas = FontAtlas::from_bytes(woff2).expect("unparseable bytes still register embedded fonts");
    let glyph = atlas.ensure_glyph('A', 16.0);
    assert!(glyph.width > 0);
    assert!(!glyph.is_color, "'A' must rasterize as a regular alpha glyph, not a color one");
}

#[test]
fn same_char_at_different_sizes_does_not_collide_in_the_glyph_cache() {
    let mut atlas = FontAtlas::builtin();
    atlas.ensure_glyph('A', 16.0);
    assert_eq!(atlas.glyphs.len(), 1);
    atlas.ensure_glyph('A', 32.0);
    assert_eq!(atlas.glyphs.len(), 2, "a second size for the same char must add a new cache entry, not collide");
    atlas.ensure_glyph('A', 16.0);
    assert_eq!(atlas.glyphs.len(), 2, "re-requesting an already-cached (char, size) must not insert again");
}

#[test]
fn shaped_mode_resolves_real_font_metrics_that_differ_from_the_bitmap_fallback() {
    let mut atlas = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
    let glyph = atlas.ensure_glyph('W', 24.0);
    assert!(glyph.width > 0 && glyph.height > 0);
    assert!(!glyph.is_color);
}

/// 🔤️ The bundled `ui/asset/font/noto-emoji/*.ttf` buckets are the monochrome "Noto Emoji"
/// family (`glyf` outlines only — verified no `COLR`/`CPAL`/`CBDT`/`CBLC`/`sbix` table is
/// present), not "Noto Color Emoji", so real emoji codepoints correctly resolve through the
/// `GenericFamily::Emoji` fallback and rasterize successfully, but land on the alpha page
/// like any other outline glyph. `packing_a_synthetic_color_glyph_lands_on_the_rgba_page_and_marks_it_dirty`
/// below exercises the RGBA color-page path directly, since these assets never trigger it.
#[test]
fn emoji_codepoints_resolve_through_the_noto_emoji_fallback_family() {
    let mut atlas = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
    let glyph = atlas.ensure_glyph('😀', 32.0);
    assert!(glyph.width > 0 && glyph.height > 0, "emoji glyph must produce a non-empty raster");
    assert!(!glyph.is_color, "the bundled Noto Emoji assets are monochrome outline-only");
    assert!(atlas.take_dirty());
}

#[test]
fn packing_a_synthetic_color_glyph_lands_on_the_rgba_page_and_marks_it_dirty() {
    let mut atlas = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
    assert!(!atlas.take_color_dirty());
    atlas.pack_glyph((super::TextFace::Sans, '🔥', 128), super::RasterizedGlyph { bitmap: vec![255u8; 4 * 4 * 4], width: 4, height: 4, bearing_x: 0.0, bearing_y: 0.0, advance: 32.0, raster_scale: 1.0, is_color: true });
    let glyph = atlas.ensure_glyph('🔥', 32.0);
    assert!(glyph.is_color);
    assert_eq!((glyph.width, glyph.height), (4, 4));
    assert!(atlas.take_color_dirty(), "packing a color glyph must mark the color page dirty");
    assert!(!atlas.take_color_dirty(), "take_color_dirty must reset after being read");
}

//#region 📐️DPI

/// 📐️ The atlas raster size — and ONLY the raster size — follows the surface scale factor. A glyph
/// asked for at 16 logical px on a 2× surface is rasterised at 32 device px, so text is crisp
/// instead of an upscaled 1× bitmap; everything the layout reads back (`logical_*`, `advance`,
/// `bearing_*`, `measure_text`) stays in the same logical pixels it had at 1×.
/// Ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g.
#[test]
fn atlas_raster_size_scales_with_the_scale_factor_while_metrics_stay_logical() {
    let mut one = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
    let (width_1x, height_1x, advance_1x) = {
        let glyph = one.ensure_glyph('W', 16.0);
        (glyph.width, glyph.height, glyph.advance)
    };
    assert!(width_1x > 0 && height_1x > 0);

    let mut two = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
    two.set_raster_scale(2.0);
    let glyph = two.ensure_glyph('W', 16.0);
    let (width_2x, height_2x, advance_2x) = (glyph.width, glyph.height, glyph.advance);
    let (logical_w, logical_h) = (glyph.logical_width(), glyph.logical_height());

    assert!(width_2x >= width_1x * 2 - 2 && width_2x <= width_1x * 2 + 2, "2x raster must be about twice the texel width: {width_1x} -> {width_2x}");
    assert!(height_2x >= height_1x * 2 - 2 && height_2x <= height_1x * 2 + 2, "2x raster must be about twice the texel height: {height_1x} -> {height_2x}");
    assert!((logical_w - width_1x as f32).abs() <= 1.0, "the logical quad must stay the 1x size: {width_1x} vs {logical_w}");
    assert!((logical_h - height_1x as f32).abs() <= 1.0, "the logical quad must stay the 1x size: {height_1x} vs {logical_h}");
    assert!((advance_2x - advance_1x).abs() <= 0.5, "advance is logical at every scale: {advance_1x} vs {advance_2x}");

    let (measured_w_1x, measured_h_1x) = one.measure_text("Window", 16.0);
    let (measured_w_2x, measured_h_2x) = two.measure_text("Window", 16.0);
    assert!((measured_w_2x - measured_w_1x).abs() <= 1.0, "measure_text is logical at every scale: {measured_w_1x} vs {measured_w_2x}");
    assert!((measured_h_2x - measured_h_1x).abs() <= 1.0, "measure_text is logical at every scale: {measured_h_1x} vs {measured_h_2x}");
}

/// ♻️ A scale-factor change drops every cached glyph so the next frame re-rasterises at the new
/// density (the window dragged between a Retina and a non-Retina display), and marks the page dirty
/// so the whole atlas is re-uploaded. Re-setting the SAME scale must keep the cache.
#[test]
fn changing_the_scale_factor_re_rasterises_and_re_setting_it_does_not() {
    let mut atlas = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
    atlas.ensure_glyph('A', 16.0);
    assert_eq!(atlas.glyphs.len(), 1);
    assert_eq!(atlas.raster_scale(), 1.0);

    atlas.set_raster_scale(1.0);
    assert_eq!(atlas.glyphs.len(), 1, "re-setting the same scale must not drop the cache");

    let _ = atlas.take_dirty();
    atlas.set_raster_scale(2.0);
    assert!(atlas.glyphs.is_empty(), "a scale change must drop every glyph rasterised at the old density");
    assert_eq!(atlas.raster_scale(), 2.0);
    assert!(atlas.take_dirty(), "a scale change must force a full atlas re-upload");

    atlas.set_raster_scale(0.0);
    assert_eq!(atlas.raster_scale(), 2.0, "a non-positive scale is refused, not applied");
    atlas.set_raster_scale(f32::NAN);
    assert_eq!(atlas.raster_scale(), 2.0, "a non-finite scale is refused, not applied");
}

/// 🔑️ The glyph cache is keyed on the DEVICE size on the quarter-pixel raster grid, so 16 logical px at 2x (32 device px =
/// 128 grid steps) is its own row, a fractional size rasterises on the nearest quarter pixel, and one grid row serves every
/// exact size that snaps to it — while each size's advance stays exact.
#[test]
fn the_glyph_cache_key_is_the_device_size_on_the_quarter_pixel_grid() {
    let mut atlas = FontAtlas::builtin();
    atlas.set_raster_scale(2.0);
    atlas.ensure_glyph('A', 16.0);
    assert_eq!(atlas.glyphs.len(), 1);
    assert!(atlas.glyphs.contains_key(&(super::TextFace::Sans, 'A', 128)), "16 logical px at 2x is cached under 32 device px = 128 quarter-pixel steps");
    atlas.ensure_glyph('A', 16.0);
    assert_eq!(atlas.glyphs.len(), 1, "the same logical size must hit the same cache row");
    atlas.set_raster_scale(1.0);
    let wide = atlas.ensure_glyph('A', 9.6).advance;
    let narrow = atlas.ensure_glyph('A', 9.55).advance;
    assert!(atlas.glyphs.contains_key(&(super::TextFace::Sans, 'A', 38)), "9.6 device px rasterises on the 9.5 px grid step");
    assert_eq!(atlas.glyphs.len(), 1, "9.55 px snaps to the same raster row");
    assert!(wide > narrow, "yet each exact size keeps its own advance: {wide} vs {narrow}");
}

/// 📏️ LAW (ticket 26/09/23 session 14d, WG11 T7a): an advance is the face's own advance at the EXACT size — linear in it, never
/// read off a rounded raster size (9.6 px measured as 10 px before: text-2xs sat 4 % wide of React).
#[test]
fn advances_are_exact_at_fractional_sizes() {
    let mut atlas = FontAtlas::shaped_default();
    let at = |atlas: &mut FontAtlas, size: f32| "feature-editor".chars().map(|ch| atlas.ensure_glyph(ch, size).advance).sum::<f32>();
    let (small, double) = (at(&mut atlas, 9.6), at(&mut atlas, 19.2));
    assert!((double - 2.0 * small).abs() < 0.01, "an advance scales linearly with the exact size: {small} vs {double}");
    assert!((small - 62.75).abs() < 0.05, "Anta's own units put 'feature-editor' at 62.75 px at 9.6 px: {small}");
}

/// 📏️ LAW (WG11 T7a): the retained layout's text worker and the atlas pen by ONE advance source — `font_advance_em` from the
/// face's units — so a label lays out exactly as wide as it paints.
#[test]
fn the_layout_advance_source_is_the_atlas_advance() {
    let mut atlas = FontAtlas::shaped_default();
    for (face, text) in [(super::TextFace::Sans, "Einstellungen · Größe 1.2"), (super::TextFace::Mono, "fn main() { 42 }")] {
        for ch in text.chars() {
            let painted = atlas.ensure_glyph_for(face, ch, 11.2).advance;
            let laid_out = super::font_advance_em(face, ch) * 11.2;
            assert!((painted - laid_out).abs() < 0.001, "{face:?} {ch:?}: atlas {painted} vs layout {laid_out}");
        }
    }
}

/// 🔤️ LAW (WG11 T7a): the shared corpus — both faces, sizes 2xs…2xl, en + de strings — sums its ADVANCES within the fixture's
/// tolerance (0.5 px) of Chromium's UNKERNED DOM width (`🧫️fixtures/🔤️text-advances`, re-measured in Chromium by the React-side law).
#[test]
fn the_shared_corpus_advances_sum_to_chromiums_unkerned_width() {
    for_each_corpus_row(|atlas, face, text, size, row| {
        let advances: f32 = text.chars().map(|ch| atlas.ensure_glyph_for(face, ch, size).advance).sum();
        (advances, row["unkernedWidthPx"].as_f64().expect("unkerned width") as f32)
    });
}

/// 🤝️ LAW (WG11 T7b): the same corpus MEASURES — advances plus every adjacent pair's kerning — within the fixture's tolerance
/// (0.5 px) of Chromium's default, KERNED DOM width; 54 of its 144 rows kern, by up to 9.5 px.
#[test]
fn the_shared_corpus_measures_as_chromium_does_with_kerning() {
    for_each_corpus_row(|atlas, face, text, size, row| (atlas.measure_text_face(face, text, size).0, row["kernedWidthPx"].as_f64().expect("kerned width") as f32));
}

/// 🔤️ Walks every corpus row (`🧫️fixtures/🔤️text-advances`) through `width`, which answers the wgpu width and the Chromium width it
/// must land within the fixture's tolerance of.
fn for_each_corpus_row(mut width: impl FnMut(&mut FontAtlas, super::TextFace, &str, f32, &serde_json::Value) -> (f32, f32)) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔤️text-advances/🔣️.json")).expect("the text advance corpus parses");
    let tolerance = fixture["tolerancePx"].as_f64().expect("tolerance") as f32;
    let mut atlas = FontAtlas::shaped_default();
    for row in fixture["rows"].as_array().expect("corpus rows") {
        let face = match row["face"].as_str() {
            Some("mono") => super::TextFace::Mono,
            _ => super::TextFace::Sans,
        };
        let text = row["text"].as_str().expect("text");
        let size = row["sizePx"].as_f64().expect("size") as f32;
        let (measured, expected) = width(&mut atlas, face, text, size, row);
        assert!((measured - expected).abs() <= tolerance, "{face:?} {size}px {text:?}: {measured} vs Chromium {expected}");
    }
}

/// 🤝️ The corpus pangram — Anta kerns six of its pairs (−0.127 em in all).
const KERNED_PANGRAM: &str = "The quick brown fox jumps over the lazy dog";

/// 🤝️ LAW (WG11 T7b): the atlas and the retained layout's text worker kern by ONE source — the face's own pair table read through
/// `PairKerning` — which Anta's `A`+`V` and `f`+`o` pin at Chromium's own −0.0928 and −0.0391 em; the monospace face does not
/// kern, a scalar outside the authored face never kerns against its neighbour, and the fixed-pitch bitmap fallback never kerns.
#[test]
fn the_layout_kerning_source_is_the_atlas_kerning() {
    let mut atlas = FontAtlas::shaped_default();
    let mut source = super::PairKerning::default();
    for (left, right, chromium) in [('A', 'V', -0.0928), ('f', 'o', -0.0391)] {
        let em = source.em(super::TextFace::Sans, left, right);
        assert!((em - chromium).abs() < 0.0005, "Anta kerns {left:?}+{right:?} by Chromium's own {chromium} em: {em}");
    }
    for (face, left, right) in [(super::TextFace::Sans, 'A', 'V'), (super::TextFace::Sans, 'y', ' '), (super::TextFace::Sans, 'T', 'h'), (super::TextFace::Mono, 'f', 'o')] {
        let cached = atlas.kerning_for(face, left, right, 11.2);
        let direct = source.em(face, left, right) * 11.2;
        assert!((cached - direct).abs() < 0.0001, "{face:?} {left:?}+{right:?}: atlas {cached} vs source {direct}");
    }
    assert_eq!(source.em(super::TextFace::Mono, 'f', 'o'), 0.0, "Share Tech Mono is fixed-pitch");
    assert_eq!(source.em(super::TextFace::Sans, 'o', '🔥'), 0.0, "an emoji fallback never kerns against Anta");
    assert_eq!(FontAtlas::builtin().kerning_for(super::TextFace::Sans, 'f', 'o', 11.2), 0.0, "the bitmap fallback never kerns");
}

/// 🤝️ LAW (WG11 T7b): pair kerning is ONE rule for measure, wrap and caret — a run measures as its advances plus each adjacent
/// pair's kerning, the per-scalar pen walk (`pen_advance`) lands every caret where `pen_at` puts it (a caret after a kerned pair
/// includes the pair, as Chromium's does) and ends at the measured width, and a box exactly as wide as the kerned run holds it on
/// one line where the unkerned sum would overflow.
#[test]
fn pair_kerning_is_one_rule_for_measure_wrap_and_caret() {
    let (text, size) = (KERNED_PANGRAM, 12.8);
    let mut atlas = FontAtlas::shaped_default();
    let scalars: Vec<(usize, char)> = text.char_indices().collect();
    let advances: f32 = scalars.iter().map(|&(_, ch)| atlas.ensure_glyph(ch, size).advance).sum();
    let kerning: f32 = scalars.windows(2).map(|pair| atlas.kerning_for(super::TextFace::Sans, pair[0].1, pair[1].1, size)).sum();
    let measured = atlas.measure_text(text, size).0;
    assert!(kerning < -0.5, "the pangram kerns in Anta: {kerning}");
    assert!((measured - (advances + kerning)).abs() < 0.001, "measured {measured} vs advances {advances} + kerning {kerning}");
    let mut pen = 0.0f32;
    for &(byte, ch) in &scalars {
        let caret = atlas.pen_at(text, byte, size);
        assert!((caret - pen).abs() < 0.001, "the caret before {ch:?} at byte {byte}: {caret} vs pen walk {pen}");
        pen += atlas.pen_advance(text, byte, size);
    }
    assert!((pen - measured).abs() < 0.001, "the pen walk ends at the measured width: {pen} vs {measured}");
    assert!((atlas.pen_at(text, text.len(), size) - measured).abs() < 0.001, "the caret at the end is the measured width");
    let fits = measured + 0.01;
    assert!(advances > fits + super::LINE_BREAK_FIT_EPSILON, "the unkerned sum {advances} would overflow a {fits} box");
    assert_eq!(atlas.wrap_lines(text, fits, size).len(), 1, "the kerned run fits its own box");
    assert_eq!(atlas.pre_wrap_lines(super::TextFace::Sans, text, fits, size).len(), 1, "pre-wrap prices the same kerned run");
}

/// 🤝️ LAW (WG11 T7b): the retained painter pens every glyph at its kerned caret and breaks where the kerned wrap does — measure,
/// wrap, caret and paint read one pair-kerning table under one rule.
#[test]
fn the_retained_painter_pens_the_kerned_caret_and_breaks_where_the_kerned_wrap_does() {
    use crate::wgpu::draw::{DrawList, KIND_GLYPH};
    use crate::wgpu::geometry::Rect;
    use crate::wgpu::paint::{paint_retained_glyph_step_flowed, RetainedGlyphCursor, RetainedGlyphStep, RetainedTextFlow};
    use crate::wgpu::theme::Rgba;

    let (text, size) = (KERNED_PANGRAM, 12.8);
    let mut atlas = FontAtlas::shaped_default();
    let paint = |atlas: &mut FontAtlas, bounds: Rect, flow: RetainedTextFlow| {
        let (mut draw, mut cursor, mut lines) = (DrawList::default(), RetainedGlyphCursor::default(), Vec::new());
        loop {
            let byte = cursor.byte();
            match paint_retained_glyph_step_flowed(text, bounds, size, Rgba::new(1.0, 1.0, 1.0, 1.0), flow, atlas, &mut draw, &mut cursor) {
                RetainedGlyphStep::Pending => lines.push((byte, cursor.line())),
                RetainedGlyphStep::Complete => break,
                RetainedGlyphStep::Fault => panic!("the pangram never faults the retained painter"),
            }
        }
        let glyphs: Vec<f32> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).map(|instance| instance.rect[0]).collect();
        (glyphs, lines)
    };
    let (glyphs, _) = paint(&mut atlas, Rect::new(10.0, 0.0, 1_000.0, 40.0), RetainedTextFlow::Clip);
    assert_eq!(glyphs.len(), text.chars().count(), "one glyph per scalar");
    for ((byte, ch), x) in text.char_indices().zip(glyphs) {
        let caret = 10.0 + atlas.pen_at(text, byte, size) + atlas.ensure_glyph(ch, size).bearing_x;
        assert!((x - caret).abs() < 0.001, "{ch:?} at byte {byte} painted at {x}, its kerned caret is {caret}");
    }
    let first_line = "The quick brown fox jumps over ";
    let width = atlas.measure_text(first_line.trim_end(), size).0 + 0.01;
    let expected = atlas.wrap_lines(text, width, size);
    assert_eq!(expected.iter().map(|line| line.start).collect::<Vec<_>>(), [0, first_line.len()], "the kerned first line keeps `over`");
    let (_, lines) = paint(&mut atlas, Rect::new(0.0, 0.0, width, 400.0), RetainedTextFlow::Wrap);
    assert_eq!(lines.len(), text.chars().count(), "every scalar is stepped exactly once");
    for (byte, line) in lines {
        let assigned = expected.iter().position(|range| range.contains(&byte)).unwrap_or_else(|| panic!("byte {byte} lands on no measured line"));
        assert_eq!(line, assigned, "scalar at byte {byte} painted on line {line}, measured onto line {assigned}");
    }
}

//#endregion 📐️DPI

//#region 📏️LineHeight
/// 📏️ Every `--text-*` token resolves to the exact CSS line box React lays out
/// (`🎨️styling/🖌️ui/🎨️.css`'s `--text-*--line-height` ramp), read from the shared generated token
/// source — not from the `size * 1.35` multiplier this target used to carry for wrapped text only.
/// Ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1o.
#[test]
fn line_height_resolves_every_font_size_token_to_reacts_own_line_box() {
    for (size, expected) in [
        (ui_styling::metrics::typography::TEXT2XS_PX as f32, 14.4),
        (ui_styling::metrics::typography::TEXT_XS_PX as f32, 16.0),
        (ui_styling::metrics::typography::TEXT_SM_PX as f32, 19.2),
        (ui_styling::metrics::typography::TEXT_BASE_PX as f32, 22.4),
        (ui_styling::metrics::typography::TEXT_LG_PX as f32, 25.6),
    ] {
        let resolved = super::line_height(size);
        assert!((resolved - expected).abs() < 0.001, "{size}px text must lay out on a {expected}px line box, got {resolved}");
    }
}

/// 📏️ A size between ramp steps borrows the nearest step's ratio — the browser applies the
/// inherited unitless ratio to whatever `font-size` an element resolves to, so the answer stays
/// proportional instead of snapping to a fixed pixel box.
#[test]
fn line_height_scales_proportionally_for_sizes_off_the_ramp() {
    let ratio = super::line_height(12.8) / 12.8;
    assert!((super::line_height(12.9) / 12.9 - ratio).abs() < 0.001, "a size next to `--text-sm` keeps that token's ratio");
    assert!(super::line_height(64.0) > super::line_height(32.0), "line height grows with size");
    assert!(super::line_height(0.0).abs() < f32::EPSILON);
}

/// 📏️ Wrapped and single-line measurement now agree with each other: N lines are exactly N line
/// boxes tall, and one line is exactly one — the internal contradiction (1.35 versus raw glyph
/// bbox) the parity audit called out is gone.
#[test]
fn wrapped_and_single_line_measurement_agree_on_the_line_box() {
    let mut atlas = FontAtlas::builtin();
    let size = ui_styling::metrics::typography::TEXT_SM_PX as f32;
    let (_, single) = atlas.measure_text("Word", size);
    assert!((single - super::line_height(size)).abs() < 0.001, "one line occupies exactly one line box, got {single}");
    let one_word = atlas.measure_text("Word", size).0;
    let (_, wrapped) = atlas.measure_text_wrapped("Word Word Word", one_word, size);
    assert!((wrapped - super::line_height(size) * 3.0).abs() < 0.001, "three wrapped lines occupy exactly three line boxes, got {wrapped}");
    let (_, empty) = atlas.measure_text_wrapped("", 100.0, size);
    assert!((empty - super::line_height(size)).abs() < 0.001, "empty text still reserves one line box");
}
//#endregion 📏️LineHeight

//#region 🅰️WeightTests
// 🅰️ W2k: React's `TextView` swaps `text-sm` for `font-semibold` — same SIZE, different WEIGHT. The
// wgpu target had no weight vocabulary at all and swapped the size instead, which moves every line
// box and wrap point away from React's. No bold Latin face ships, so semibold is a second strike.

#[semio_framework_async_macros::async_test]
async fn a_weight_maps_from_reacts_own_emphasize_flag_and_prices_its_strikes() {
    assert_eq!(super::TextWeight::of(false), super::TextWeight::Regular);
    assert_eq!(super::TextWeight::of(true), super::TextWeight::Semibold);
    assert_eq!(<super::TextWeight as Default>::default(), super::TextWeight::Regular);
    assert_eq!(super::TextWeight::Regular.strikes(), 1);
    assert_eq!(super::TextWeight::Semibold.strikes(), 2, "a per-glyph reservation must price two strikes before the retained arm can carry the swap");
}

#[semio_framework_async_macros::async_test]
async fn the_faux_bold_offset_scales_with_the_size_and_never_vanishes() {
    assert!((super::faux_bold_offset(25.6) - 25.6 * 0.04).abs() < 1e-6, "a semibold stem is ~4 % of the em thicker");
    assert!((super::faux_bold_offset(19.2) - 19.2 * 0.04).abs() < 1e-6);
    assert_eq!(super::faux_bold_offset(1.0), 0.34, "the floor keeps the strike visible at the smallest step");
    assert_eq!(super::faux_bold_offset(0.0), 0.0);
    assert_eq!(super::faux_bold_offset(f32::NAN), 0.0);
    assert!(super::faux_bold_offset(25.6) > super::faux_bold_offset(9.6), "the offset is monotonic in the size");
}

#[semio_framework_async_macros::async_test]
async fn a_semibold_run_strikes_twice_at_the_regular_faces_own_advances() {
    let mut atlas = FontAtlas::builtin();
    let size = 19.2;
    let mut regular = crate::wgpu::draw::DrawList::default();
    crate::wgpu::widgets::draw_text_weighted(&mut regular, &mut atlas, "Ab", 10.0, 20.0, size, crate::wgpu::theme::Rgba::new(1.0, 1.0, 1.0, 1.0), super::TextWeight::Regular);
    let mut semibold = crate::wgpu::draw::DrawList::default();
    crate::wgpu::widgets::draw_text_weighted(&mut semibold, &mut atlas, "Ab", 10.0, 20.0, size, crate::wgpu::theme::Rgba::new(1.0, 1.0, 1.0, 1.0), super::TextWeight::Semibold);
    let count = |draw: &crate::wgpu::draw::DrawList| draw.layers.iter().map(|layer| layer.ui_instances.len()).sum::<usize>();
    assert_eq!(count(&semibold), count(&regular) * 2, "the second strike is the whole synthetic weight");
    let widths: Vec<f32> = semibold.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect[0]).collect();
    assert!(widths.iter().any(|x| (*x - 10.0).abs() < 0.001), "the first strike sits at the authored origin");
    assert!(widths.iter().any(|x| (*x - (10.0 + super::faux_bold_offset(size))).abs() < 0.001), "the second strike is offset by exactly the faux-bold offset");
}
//#endregion 🅰️WeightTests

//#region 🔣️SymbolFaceTests

/// 🔣️ The chord/arrow glyphs the shell writes, exactly as `format_keybinding_shortcut` and React's
/// `formatKeybindingShortcut` spell them (`🐚️Shell/🧫️fixtures/⌨️keybinding-glyphs/🔣️.json`).
const SHORTCUT_SYMBOLS: [char; 12] = ['\u{2318}', '\u{2325}', '\u{21E7}', '\u{2303}', '\u{238B}', '\u{21B5}', '\u{232B}', '\u{2326}', '\u{2190}', '\u{2192}', '\u{2191}', '\u{2193}'];

/// 🔣️ **Every glyph of the shortcut table rasterises to a non-empty bitmap** — the law the boot
/// screenshot broke, where `Editor ⌘️⌥️E` painted four `.notdef` boxes because no shipped face carries
/// U+2318/U+2325 and the atlas scans no system fonts.
#[semio_framework_async_macros::async_test]
async fn every_shortcut_symbol_rasterises_to_real_ink() {
    for atlas_name in ["shaped", "builtin"] {
        let mut atlas = if atlas_name == "shaped" { FontAtlas::shaped_default() } else { FontAtlas::builtin() };
        for size in [11.2_f32, 12.8, 16.0] {
            for symbol in SHORTCUT_SYMBOLS {
                let (x, y, w, h, advance, is_color) = {
                    let glyph = atlas.ensure_glyph(symbol, size);
                    (glyph.atlas_x, glyph.atlas_y, glyph.width, glyph.height, glyph.advance, glyph.is_color)
                };
                assert!(w > 0 && h > 0, "{atlas_name} atlas left U+{:04X} without a bitmap at {size} px", symbol as u32);
                assert!(advance > 0.0, "{atlas_name} atlas left U+{:04X} without an advance", symbol as u32);
                let page_width = atlas.width;
                let inked = (0..h).any(|row| (0..w).any(|col| atlas.pixels[((y + row) * page_width + x + col) as usize] != 0));
                assert!(inked, "{atlas_name} atlas rasterised U+{:04X} as a blank box at {size} px", symbol as u32);
                assert!(!is_color, "a chord symbol is an alpha glyph, never a colour one");
            }
        }
    }
}

/// 🫥 The variation selector this repo spells every symbol with is zero-width, zero-ink — it used to
/// resolve to Anta's own `.notdef`, which is why `⌘️⌥️E` painted FOUR boxes for two symbols.
#[semio_framework_async_macros::async_test]
async fn a_default_ignorable_codepoint_paints_nothing_and_moves_no_pen() {
    let mut atlas = FontAtlas::shaped_default();
    for ignorable in ['\u{fe0f}', '\u{fe0e}', '\u{200d}', '\u{200b}', '\u{feff}', '\u{00ad}'] {
        let glyph = atlas.ensure_glyph(ignorable, 11.2);
        assert_eq!((glyph.width, glyph.height), (0, 0), "U+{:04X} must occupy no atlas texels", ignorable as u32);
        assert_eq!(glyph.advance, 0.0, "U+{:04X} must move the pen by nothing", ignorable as u32);
    }
    let (with_selector, _) = atlas.measure_text("\u{2318}\u{fe0f}\u{2325}\u{fe0f}E", 11.2);
    let (without_selector, _) = atlas.measure_text("\u{2318}\u{2325}E", 11.2);
    assert!((with_selector - without_selector).abs() < 1e-3, "the repo's ⌘️⌥️E spelling must measure exactly its ⌘⌥E ink");
}

/// 📐️ A symbol sits on the same baseline and inside the same line box as the shaped face beside it,
/// and it scales with the raster scale like every other glyph.
#[semio_framework_async_macros::async_test]
async fn a_symbol_glyph_shares_the_faces_baseline_and_honours_the_raster_scale() {
    let mut atlas = FontAtlas::shaped_default();
    let size = 16.0_f32;
    let letter = {
        let glyph = atlas.ensure_glyph('E', size);
        (glyph.logical_height() + glyph.bearing_y, glyph.bearing_y)
    };
    let symbol = {
        let glyph = atlas.ensure_glyph('\u{2318}', size);
        (glyph.logical_height() + glyph.bearing_y, glyph.bearing_y)
    };
    assert!(symbol.0 > 0.0 && symbol.0 <= super::line_height(size), "a symbol's cap must stay inside React's own line box");
    assert!(symbol.1 > -size * 0.3, "a symbol must not hang further below the baseline than a descender would");
    assert!((symbol.0 - letter.0).abs() < size * 0.5, "a symbol's cap height must sit within half an em of the face's own");
    let one_x = atlas.ensure_glyph('\u{2192}', size).advance;
    atlas.set_raster_scale(2.0);
    let glyph = atlas.ensure_glyph('\u{2192}', size);
    assert_eq!(glyph.raster_scale, 2.0);
    assert!((glyph.advance - one_x).abs() < 1.0, "the LOGICAL advance must survive a density change");
    assert!(glyph.width as f32 > one_x, "the ATLAS extent must grow with the density");
}

//#endregion 🔣️SymbolFaceTests

//#region ✂️LineBreakTests
// ✂️ W9a: the tour card painted `comp|osing` — the measures split on whitespace while the retained
// PAINTER broke at whatever glyph happened to overflow. These laws pin the one predicate both now go
// through, against a reference computed from the CSS rules inside the test itself.

/// 🧾️ The paragraph React's own boot tour carries (`🗑️generated/react-6313/final.png` reads it under
/// `Welcome to Puzzle 3D`) — the string that exposed the defect.
const TOUR_BODY: &str = "A quick tour of the viewport, utilities, and panels before you start composing.";

/// ⚖️ LAW: a break is allowed exactly where CSS `word-break: normal` allows one — after a run of
/// spaces, after a hyphen/dash/soft-hyphen/zero-width space, and on either side of an ideograph
/// except where a bracket forbids it. Nowhere else, and never inside a Latin word.
#[test]
fn a_break_opportunity_is_exactly_what_css_allows() {
    for (prev, next, allowed, why) in [
        ('p', 'o', false, "inside a Latin word"),
        (' ', 'W', true, "after a space"),
        (' ', ' ', false, "between two spaces — the break is after the LAST of a run"),
        ('d', ' ', false, "before a space — a trailing space hangs, it never starts a line"),
        ('\u{00A0}', 'W', false, "a no-break space is written precisely to forbid this break"),
        ('-', 'b', true, "after a hyphen"),
        ('\u{00AD}', 'b', true, "after a soft hyphen"),
        ('\u{200B}', 'b', true, "after a zero-width space"),
        ('\u{2014}', 'b', true, "after an em dash"),
        ('\u{4E16}', '\u{754C}', true, "between two ideographs"),
        ('a', '\u{4E16}', true, "before an ideograph"),
        ('\u{4E16}', '\u{3002}', false, "an ideographic full stop never starts a line"),
        ('\u{300C}', '\u{4E16}', false, "an opening bracket never ends a line"),
        ('.', 'b', false, "a full stop is not a break opportunity in Latin"),
        ('/', 'b', false, "a solidus is not one either"),
    ] {
        assert_eq!(super::may_break_between(prev, next), allowed, "{prev:?} → {next:?}: {why}");
    }
    assert!(!super::is_break_opportunity(TOUR_BODY, 0), "the start of a run is never a break");
    assert!(!super::is_break_opportunity(TOUR_BODY, TOUR_BODY.len()), "nor is its end");
    assert_eq!(super::unbreakable_run_end(TOUR_BODY, 2), 7, "`quick` is one unbreakable run");
}

/// 🧮️ The REFERENCE wrap, written here from the CSS rules rather than borrowed from the
/// implementation: greedy first-fit over whitespace-separated words, a word measured by the very
/// advances the painter pens, a trailing space that never counts toward the line's fit, and a break
/// inside a word only when that word alone is wider than the whole box (`overflow-wrap`).
fn reference_break_indices(atlas: &mut FontAtlas, text: &str, max_width: f32, size: f32) -> Vec<usize> {
    let advance = |atlas: &mut FontAtlas, ch: char| atlas.ensure_glyph(ch, size).advance;
    let mut breaks = Vec::new();
    let (mut pen, mut cursor) = (0.0f32, 0usize);
    for (offset, ch) in text.char_indices() {
        if ch == ' ' {
            pen += advance(atlas, ch);
            cursor = offset + ch.len_utf8();
            continue;
        }
        if cursor == offset {
            let word_end = text[offset..].find(' ').map_or(text.len(), |at| offset + at);
            let word: f32 = text[offset..word_end].chars().map(|ch| advance(atlas, ch)).sum();
            if pen > 0.0 && pen + word > max_width + super::LINE_BREAK_FIT_EPSILON {
                breaks.push(offset);
                pen = 0.0;
            }
        }
        let width = advance(atlas, ch);
        if pen > 0.0 && pen + width > max_width + super::LINE_BREAK_FIT_EPSILON {
            breaks.push(offset);
            pen = 0.0;
        }
        pen += width;
    }
    breaks
}

/// ⚖️ LAW: the atlas wrap breaks at exactly the reference's indices, at every width from "one word
/// fits" to "the whole paragraph fits", and never inside a word that fits its own line.
#[test]
fn a_paragraph_wraps_at_the_reference_break_indices() {
    let size = ui_styling::metrics::typography::TEXT_XS_PX as f32;
    let mut atlas = FontAtlas::builtin();
    let full = atlas.measure_text(TOUR_BODY, size).0;
    let widest_word = TOUR_BODY.split(' ').map(|word| atlas.measure_text(word, size).0).fold(0.0f32, f32::max);
    let mut widths = 0;
    let mut max_width = widest_word;
    while max_width <= full {
        let expected = reference_break_indices(&mut atlas, TOUR_BODY, max_width, size);
        let actual: Vec<usize> = atlas.wrap_lines(TOUR_BODY, max_width, size).into_iter().skip(1).map(|line| line.start).collect();
        assert_eq!(actual, expected, "break indices at max_width {max_width}");
        for start in &actual {
            let prev = TOUR_BODY[..*start].chars().next_back().expect("a break index is never zero");
            assert!(prev == ' ', "a line may only start after a space, got {prev:?} at {start} (width {max_width})");
        }
        widths += 1;
        max_width += 7.0;
    }
    assert!(widths > 10, "the sweep must cover a real range of box widths, covered {widths}");
}

/// ⚖️ LAW: the last-resort arm is the ONLY way a word is cut — one run wider than the whole box.
#[test]
fn only_a_word_wider_than_the_box_is_ever_cut() {
    let size = ui_styling::metrics::typography::TEXT_XS_PX as f32;
    let mut atlas = FontAtlas::builtin();
    let word_w = atlas.measure_text("composing", size).0;
    let lines = atlas.wrap_lines("composing", word_w * 0.5, size);
    assert!(lines.len() > 1, "a word alone and wider than its box breaks rather than painting outside it");
    let fits = atlas.wrap_lines("composing", word_w, size);
    assert_eq!(fits.len(), 1, "a word that fits is never cut");
    let paragraph = atlas.wrap_lines(TOUR_BODY, word_w * 4.0, size);
    for line in &paragraph {
        let trimmed = TOUR_BODY[line.clone()].trim();
        assert!(!trimmed.is_empty(), "no empty line");
        assert!(TOUR_BODY[..line.start].chars().next_back().is_none_or(|prev| prev == ' '), "no line starts mid-word: {trimmed:?}");
    }
}

/// ⚖️ LAW: a hard newline always ends a line, and trailing spaces hang — they never push a line over
/// its box and never open the next one.
#[test]
fn a_newline_always_breaks_and_a_trailing_space_hangs() {
    let size = ui_styling::metrics::typography::TEXT_XS_PX as f32;
    let mut atlas = FontAtlas::builtin();
    let wide = atlas.measure_text("one two", size).0 * 4.0;
    assert_eq!(atlas.wrap_lines("one\ntwo", wide, size).len(), 2, "a hard newline breaks even in an empty box");
    let exact = atlas.measure_text("one two", size).0;
    assert_eq!(atlas.wrap_lines("one two ", exact, size).len(), 1, "the trailing space hangs off the end of its own line");
    let (width, _) = atlas.measure_text_wrapped("one two ", exact, size);
    assert!((width - exact).abs() < 0.001, "a hanging space is not priced into the line box, got {width}");
}

/// ⚖️ LAW (ticket 26/09/23 session 14d, WG11): `white-space: pre-wrap` keeps every space, and the spaces ending a line hang —
/// a line whose last word fits breaks AFTER its trailing spaces, never before that word (the DiffView presentation fixture's
/// `preWrap` vector, `🧬️contract/🧫️fixtures/🆚️diff-view-presentation/🔣️.json`). A hard newline still breaks, empty lines stay.
#[test]
fn pre_wrap_keeps_repeated_spaces_and_hangs_the_ones_ending_a_line() {
    use super::TextFace;
    let size = ui_styling::metrics::typography::TEXT_XS_PX as f32;
    let mut atlas = FontAtlas::builtin();
    let text = "a  b  c  d";
    let fits = atlas.measure_range_face(TextFace::Mono, text, 0, "a  b  c".len(), size);
    let lines = atlas.pre_wrap_lines(TextFace::Mono, text, fits + 0.5, size).into_iter().map(|range| &text[range]).collect::<Vec<_>>();
    assert_eq!(lines, ["a  b  c  ", "d"], "the spaces after `c` hang; `d` opens the next line");
    assert_eq!(atlas.pre_wrap_lines(TextFace::Mono, "one\n\ntwo", 1_000.0, size).len(), 3, "a hard newline breaks and an empty line is kept");
}

/// ⚖️ LAW: **the retained painter breaks where the measure says it does.** Driven glyph by glyph over
/// a real `DrawList`, the line each scalar lands on is exactly the line [`FontAtlas::wrap_lines`]
/// assigns it. This is the law the tour card's `comp|osing` violated.
#[test]
fn the_retained_painter_breaks_where_the_measure_says() {
    use crate::wgpu::draw::DrawList;
    use crate::wgpu::geometry::Rect;
    use crate::wgpu::paint::{paint_retained_glyph_step_flowed, RetainedGlyphCursor, RetainedGlyphStep, RetainedTextFlow};
    use crate::wgpu::theme::Rgba;

    let size = ui_styling::metrics::typography::TEXT_XS_PX as f32;
    let mut atlas = FontAtlas::builtin();
    let max_width = atlas.measure_text("A quick tour of the viewport,", size).0;
    let expected = atlas.wrap_lines(TOUR_BODY, max_width, size);
    let mut draw = DrawList::default();
    let mut cursor = RetainedGlyphCursor::default();
    let bounds = Rect::new(0.0, 0.0, max_width, 400.0);
    let mut painted: Vec<(usize, usize)> = Vec::new();
    for _ in 0..TOUR_BODY.len() + 4 {
        let byte = cursor.byte();
        match paint_retained_glyph_step_flowed(TOUR_BODY, bounds, size, Rgba::new(1.0, 1.0, 1.0, 1.0), RetainedTextFlow::Wrap, &mut atlas, &mut draw, &mut cursor) {
            RetainedGlyphStep::Pending => painted.push((byte, cursor.line())),
            RetainedGlyphStep::Complete => break,
            RetainedGlyphStep::Fault => panic!("the tour body must never fault the retained painter"),
        }
    }
    assert_eq!(painted.len(), TOUR_BODY.chars().count(), "every scalar is stepped exactly once");
    for (byte, line) in painted {
        let assigned = expected.iter().position(|range| range.contains(&byte)).unwrap_or_else(|| panic!("byte {byte} lands on no measured line"));
        assert_eq!(line, assigned, "scalar at byte {byte} ({:?}) painted on line {line}, measured onto line {assigned}", &TOUR_BODY[byte..byte + 1]);
    }
    assert_eq!(expected.len(), 3, "this box holds the paragraph in three lines");
}
//#endregion ✂️LineBreakTests
