//! 🔎️ Text extraction and search: the cell text, text between points, character classes, word, line and url spans and the find. See [`super`].

use super::*;

impl VtScreen {
    fn cell_text(row: &Row, from: usize, to: usize) -> String {
        let mut out = String::new();
        for cell in row.cells.iter().take(to + 1).skip(from) {
            if cell.width != 0 && cell.ch != '\0' {
                out.push(cell.ch);
            }
        }
        out
    }

    /// 🗞️ The text between two cells, inclusive, with soft-wrapped rows joined and trailing blanks trimmed.
    pub fn text_between(&self, a: CellPoint, b: CellPoint) -> String {
        let (a, b) = if a <= b { (a, b) } else { (b, a) };
        let mut out = String::new();
        for abs in a.row..=b.row {
            let Some(row) = self.row_at(abs) else { continue };
            let from = if abs == a.row { usize::from(a.col) } else { 0 };
            let to = if abs == b.row { usize::from(b.col) } else { row.cells.len().saturating_sub(1) };
            let mut text = Self::cell_text(row, from, to);
            if !row.wrapped {
                text.truncate(text.trim_end().len());
            }
            out.push_str(&text);
            if !row.wrapped && abs != b.row {
                out.push('\n');
            }
        }
        out
    }

    fn char_class(cell: &Cell) -> u8 {
        let c = cell.ch;
        if c == ' ' || c == '\0' {
            0
        } else if "()[]{}<>\"'`|,;".contains(c) {
            1
        } else {
            2
        }
    }

    /// 🔡️ The word, delimiter run or blank run around `point`.
    pub fn word_span(&self, point: CellPoint) -> (CellPoint, CellPoint) {
        let Some(row) = self.row_at(point.row) else { return (point, point) };
        let mut at = usize::from(point.col).min(row.cells.len().saturating_sub(1));
        while at > 0 && row.cells[at].width == 0 {
            at -= 1;
        }
        let class = Self::char_class(&row.cells[at]);
        let mut start = at;
        while start > 0 && (row.cells[start - 1].width == 0 || Self::char_class(&row.cells[start - 1]) == class) {
            start -= 1;
        }
        let mut end = at;
        while end + 1 < row.cells.len() && (row.cells[end + 1].width == 0 || Self::char_class(&row.cells[end + 1]) == class) {
            end += 1;
        }
        (CellPoint { row: point.row, col: start as u16 }, CellPoint { row: point.row, col: end as u16 })
    }

    /// 🧵️ The whole logical line (all soft-wrapped rows) holding `point`.
    pub fn line_span(&self, point: CellPoint) -> (CellPoint, CellPoint) {
        let mut first = point.row;
        while first > self.first_row() && self.row_at(first - 1).is_some_and(|row| row.wrapped) {
            first -= 1;
        }
        let mut last = point.row;
        while self.row_at(last).is_some_and(|row| row.wrapped) && self.row_at(last + 1).is_some() {
            last += 1;
        }
        (CellPoint { row: first, col: 0 }, CellPoint { row: last, col: self.size.width - 1 })
    }

    /// 🔗️ The URL under `point`, if the blank-delimited word there contains `://`.
    pub fn url_at(&self, point: CellPoint) -> Option<String> {
        let row = self.row_at(point.row)?;
        let at = usize::from(point.col);
        let blank = |i: usize| row.cells.get(i).is_none_or(|cell| cell.ch == ' ');
        if blank(at) {
            return None;
        }
        let mut start = at;
        while start > 0 && !blank(start - 1) {
            start -= 1;
        }
        let mut end = at;
        while !blank(end + 1) {
            end += 1;
        }
        let word = Self::cell_text(row, start, end);
        let word = word.trim_end_matches(['.', ',', ';', ':', '!', '?', ')', ']', '}', '>', '\'', '"']);
        let scheme = word.find("://")?;
        (scheme > 0 && word[..scheme].chars().all(|c| c.is_ascii_alphabetic()) && word.len() > scheme + 3).then(|| word.to_string())
    }

    /// 🔭️ Every case-insensitive occurrence of `needle` in a single row, oldest first.
    pub fn find(&self, needle: &str) -> Vec<Match> {
        let needle: Vec<char> = needle.chars().flat_map(char::to_lowercase).collect();
        if needle.is_empty() {
            return Vec::new();
        }
        let mut hits = Vec::new();
        let last = self.screen_top() + u64::from(self.size.height);
        let first = if self.alt_active { self.screen_top() } else { self.first_row() };
        for abs in first..last {
            let Some(row) = self.row_at(abs) else { continue };
            let glyphs: Vec<(char, u16)> = row.cells.iter().enumerate().filter(|(_, cell)| cell.width != 0).map(|(x, cell)| (cell.ch.to_lowercase().next().unwrap_or(cell.ch), x as u16)).collect();
            if glyphs.len() < needle.len() {
                continue;
            }
            for start in 0..=glyphs.len() - needle.len() {
                if glyphs[start..start + needle.len()].iter().map(|(c, _)| *c).eq(needle.iter().copied()) {
                    let col = glyphs[start].1;
                    let end = glyphs[start + needle.len() - 1].1;
                    let tail = u16::from(row.cells[usize::from(end)].width.max(1));
                    hits.push(Match { row: abs, col, cells: end - col + tail });
                    if hits.len() >= MAX_MATCHES {
                        return hits;
                    }
                }
            }
        }
        hits
    }
}
