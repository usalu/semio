//! 📝️ Explicit line layout in local drawing coordinates, shared by canvas and export.
pub const DRAWING_TEXT_LINE_HEIGHT: f64 = 1.2;

pub struct DrawingTextLines<'a> { remaining: Option<&'a str> }

pub fn drawing_text_lines(content: &str) -> DrawingTextLines<'_> {
    DrawingTextLines { remaining: Some(content) }
}

impl<'a> Iterator for DrawingTextLines<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.remaining.take()?;
        if let Some(index) = remaining.find(['\r', '\n']) {
            let width = if remaining[index..].starts_with("\r\n") { 2 } else { 1 };
            self.remaining = Some(&remaining[index + width..]);
            Some(&remaining[..index])
        } else { Some(remaining) }
    }
}

/// 📏️ Font-independent fallback extent, not a shaped-glyph measurement.
pub fn drawing_text_fallback_extent(content: &str, size: f64) -> [f64; 2] {
    let mut columns = 0;
    let mut count = 0;
    for line in drawing_text_lines(content) { columns = columns.max(line.chars().count()); count += 1; }
    [columns as f64 * size * 0.6, count as f64 * size * DRAWING_TEXT_LINE_HEIGHT]
}
