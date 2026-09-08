
use super::*;

#[test]
fn embedded_fonts_are_non_empty_and_look_like_sfnt() {
    let fonts = embedded_fonts();
    for (role, bytes) in [(FontRole::Math, fonts.math), (FontRole::Serif, fonts.serif), (FontRole::SerifItalic, fonts.serif_italic), (FontRole::Mono, fonts.mono), (FontRole::Emoji, fonts.emoji)] {
        assert!(bytes.len() > 1024, "{role:?} font is suspiciously small: {} bytes", bytes.len());
        // Every sfnt font (OTF/TTF) starts with one of these four-byte tags.
        let tag = &bytes[0..4];
        assert!(tag == [0x00, 0x01, 0x00, 0x00] || tag == b"OTTO" || tag == b"true" || tag == b"ttcf", "{role:?} font does not start with a recognized sfnt tag: {tag:?}");
    }
}

#[test]
fn font_set_get_matches_field_by_role() {
    let fonts = embedded_fonts();
    assert_eq!(fonts.get(FontRole::Math).as_ptr(), fonts.math.as_ptr());
    assert_eq!(fonts.get(FontRole::Emoji).as_ptr(), fonts.emoji.as_ptr());
}
