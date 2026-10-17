//! 🔤️ Text geometry of the editor: the range overlap test, the byte, line and column conversions and the character boundary helpers. See [`super`].

use super::*;

// #region 🔖️EditorState
pub(super) fn ranges_overlap(a_start: usize, a_end: usize, b_start: usize, b_end: usize) -> bool {
    a_start < b_end && b_start < a_end
}

pub(super) fn hit_byte_in_line(line: &str, world_x: f64, line_origin_x: f64, font_px: f64) -> usize {
    if line.is_empty() {
        return 0;
    }
    let mut boundaries = vec![0usize];
    for (index, _) in line.char_indices() {
        if index > 0 {
            boundaries.push(index);
        }
    }
    if boundaries.last().copied() != Some(line.len()) {
        boundaries.push(line.len());
    }
    for pair in boundaries.windows(2) {
        let start = pair[0];
        let end = pair[1];
        let x0 = canvas_text::label_byte_world_x(line, start, line_origin_x, font_px);
        let x1 = canvas_text::label_byte_world_x(line, end, line_origin_x, font_px);
        if world_x < (x0 + x1) * 0.5 {
            return start;
        }
    }
    line.len()
}

pub(super) fn offset_line_col(text: &str, offset: usize) -> (usize, usize) {
    let clamped = offset.min(text.len());
    let mut line = 0usize;
    let mut last = 0usize;
    for (i, ch) in text.char_indices() {
        if i >= clamped {
            break;
        }
        if ch == '\n' {
            line += 1;
            last = i + 1;
        }
    }
    (line, clamped - last)
}

pub(super) fn offset_at_line_col(text: &str, line: usize, col: usize) -> usize {
    let mut current_line = 0usize;
    let mut line_start = 0usize;
    for (i, ch) in text.char_indices() {
        if current_line == line {
            let line_end = text[line_start..].find('\n').map_or(text.len(), |idx| line_start + idx);
            return line_start + col.min(line_end.saturating_sub(line_start));
        }
        if ch == '\n' {
            current_line += 1;
            line_start = i + 1;
        }
    }
    if current_line == line {
        return line_start + col.min(text.len().saturating_sub(line_start));
    }
    text.len()
}

pub(super) fn offset_to_world(host: &EditorHost, offset: usize) -> (f64, f64) {
    let (line, byte) = offset_line_col(&host.text, offset);
    let line_text = host.text.split('\n').nth(line).unwrap_or("");
    let x = canvas_text::label_byte_world_x(line_text, byte, host.line_origin_x(), host.font_px);
    let y = host.line_y(line);
    (x, y)
}

pub(super) fn position_to_offset(text: &str, pos: &TextPosJson) -> usize {
    offset_at_line_col(text, pos.line as usize, pos.character as usize)
}

/// 📍️ The char boundary at `index` (clamped to the text), or the one before it when `index` splits a character.
pub(super) fn char_boundary_at_or_before(text: &str, index: usize) -> usize {
    let index = index.min(text.len());
    if text.is_char_boundary(index) {
        index
    } else {
        prev_char_boundary(text, index)
    }
}

pub(super) fn prev_char_boundary(text: &str, index: usize) -> usize {
    text[..index].char_indices().next_back().map_or(0, |(i, _)| i)
}

pub(super) fn next_char_boundary(text: &str, index: usize) -> usize {
    text[index..].char_indices().nth(1).map_or(text.len(), |(i, _)| index + i)
}
// #endregion 🔖️EditorState
