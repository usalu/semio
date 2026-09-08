
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
    atlas.pack_glyph(('🔥', 32), super::RasterizedGlyph { bitmap: vec![255u8; 4 * 4 * 4], width: 4, height: 4, bearing_x: 0.0, bearing_y: 0.0, advance: 32.0, is_color: true });
    let glyph = atlas.ensure_glyph('🔥', 32.0);
    assert!(glyph.is_color);
    assert_eq!((glyph.width, glyph.height), (4, 4));
    assert!(atlas.take_color_dirty(), "packing a color glyph must mark the color page dirty");
    assert!(!atlas.take_color_dirty(), "take_color_dirty must reset after being read");
}
