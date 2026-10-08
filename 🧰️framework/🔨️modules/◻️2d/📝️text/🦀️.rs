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
pub fn drawing_text_fallback_extent(content: &(impl semio_framework_value::paged::Utf8Text + ?Sized), size: f64) -> [f64; 2] {
    let mut columns = 0;
    let mut current = 0;
    let mut count = 1;
    let mut carriage = false;
    for index in 0..content.text_chunk_count() {
        for byte in content.text_chunk(index).unwrap_or("").bytes() {
            if byte == b'\n' && carriage { carriage = false; continue; }
            if byte == b'\n' || byte == b'\r' { columns = columns.max(current); current = 0; count += 1; }
            else if byte & 0xc0 != 0x80 { current += 1; }
            carriage = byte == b'\r';
        }
    }
    columns = columns.max(current);
    [columns as f64 * size * 0.6, count as f64 * size * DRAWING_TEXT_LINE_HEIGHT]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
