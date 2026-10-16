//! 🔤️ Shared original text wrapping reference for font and renderer laws.
use super::FontAtlas;

pub const KERNED_PANGRAM: &str = "The quick brown fox jumps over the lazy dog";

pub fn reference_break_indices(atlas: &mut FontAtlas, text: &str, max_width: f32, size: f32) -> Vec<usize> {
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
