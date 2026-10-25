//! 📍️ LSP positions: zero-based line and UTF-16 character, mapped to and from the byte offsets of the Jack language service.
//!
//! See [LSP 3.17 Position](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#position).

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// 🧭️ Line table over one document text; `\n`, `\r\n` and a lone `\r` end a line.
pub struct LineIndex<'a> {
    text: &'a str,
    starts: Vec<usize>,
}

impl<'a> LineIndex<'a> {
    pub fn new(text: &'a str) -> Self {
        let bytes = text.as_bytes();
        let mut starts = vec![0];
        let mut index = 0;
        while index < bytes.len() {
            match bytes[index] {
                b'\n' => starts.push(index + 1),
                b'\r' => {
                    if bytes.get(index + 1) == Some(&b'\n') {
                        index += 1;
                    }
                    starts.push(index + 1);
                }
                _ => {}
            }
            index += 1;
        }
        Self { text, starts }
    }

    pub fn line_count(&self) -> usize {
        self.starts.len()
    }

    fn line_end(&self, line: usize) -> usize {
        let start = self.starts[line];
        let next = self.starts.get(line + 1).copied().unwrap_or(self.text.len());
        let row = &self.text.as_bytes()[start..next];
        let content = row.strip_suffix(b"\r\n").or_else(|| row.strip_suffix(b"\n")).or_else(|| row.strip_suffix(b"\r")).unwrap_or(row);
        start + content.len()
    }

    pub fn line_length(&self, line: u32) -> u32 {
        let line = line as usize;
        if line >= self.starts.len() {
            return 0;
        }
        self.text[self.starts[line]..self.line_end(line)].encode_utf16().count() as u32
    }

    pub fn position(&self, offset: usize) -> Position {
        let mut offset = offset.min(self.text.len());
        while !self.text.is_char_boundary(offset) {
            offset -= 1;
        }
        let line = self.starts.partition_point(|start| *start <= offset) - 1;
        let end = offset.min(self.line_end(line));
        Position { line: line as u32, character: self.text[self.starts[line]..end].encode_utf16().count() as u32 }
    }

    pub fn offset(&self, position: Position) -> usize {
        let line = position.line as usize;
        if line >= self.starts.len() {
            return self.text.len();
        }
        let start = self.starts[line];
        let end = self.line_end(line);
        let mut units = 0u32;
        for (index, character) in self.text[start..end].char_indices() {
            if units >= position.character {
                return start + index;
            }
            units += character.len_utf16() as u32;
        }
        end
    }

    pub fn range(&self, start: usize, end: usize) -> Range {
        Range { start: self.position(start), end: self.position(end.max(start)) }
    }

    pub fn whole(&self) -> Range {
        self.range(0, self.text.len())
    }
}
