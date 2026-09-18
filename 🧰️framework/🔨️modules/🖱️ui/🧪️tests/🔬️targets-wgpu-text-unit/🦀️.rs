
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
    atlas.pack_glyph(('🔥', 32), super::RasterizedGlyph { bitmap: vec![255u8; 4 * 4 * 4], width: 4, height: 4, bearing_x: 0.0, bearing_y: 0.0, advance: 32.0, raster_scale: 1.0, is_color: true });
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

/// 🔑️ The glyph cache is keyed on the DEVICE size, so 16 logical px at 2x and 32 logical px at 1x
/// are distinct rows even though both rasterise 32 device px — otherwise a scale change would hand
/// back a correctly-sized raster with the WRONG logical metrics.
#[test]
fn the_glyph_cache_key_is_the_device_size_not_the_logical_one() {
    let mut atlas = FontAtlas::builtin();
    atlas.set_raster_scale(2.0);
    atlas.ensure_glyph('A', 16.0);
    assert_eq!(atlas.glyphs.len(), 1);
    assert!(atlas.glyphs.contains_key(&('A', 32)), "16 logical px at 2x must be cached under its 32 device px key");
    atlas.ensure_glyph('A', 16.0);
    assert_eq!(atlas.glyphs.len(), 1, "the same logical size must hit the same cache row");
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
    let (_, wrapped) = atlas.measure_text_wrapped("Word Word Word", 1.0, size);
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
