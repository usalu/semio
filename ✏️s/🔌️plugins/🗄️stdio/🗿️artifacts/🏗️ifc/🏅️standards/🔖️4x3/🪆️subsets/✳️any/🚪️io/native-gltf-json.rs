//! 🔎️ Natural JSON retains original token spans, bounded scalar decoding and last-member lookup without an owned JSON mirror.
use super::*;
const NONE: usize = usize::MAX;
#[derive(Clone, Copy, RetireOwned)]
pub(super) struct Token {
    pub kind: u8,
    pub start: usize,
    pub end: usize,
    pub first: usize,
    pub next: usize,
    pub count: usize,
    key: (usize, usize),
}
#[derive(Default, RetireOwned)]
pub(super) struct Catalog {
    pub tokens: Vec<Token>,
    count: usize,
    cursor: usize,
    source: (usize, usize),
    root_end: usize,
}
impl Catalog {
    fn byte(&mut self, source: &[u8], end: usize, port: &mut Port<'_, '_, '_>) -> Result<u8, ValueError> {
        if self.cursor >= end {
            return Err(invalid("natural glTF JSON ended inside a token"));
        }
        port.effect(1, || {
            let byte = source[self.cursor];
            self.cursor += 1;
            byte
        })
    }
    fn peek(&self, source: &[u8], end: usize) -> Option<u8> {
        (self.cursor < end).then(|| source[self.cursor])
    }
    fn trivia(&mut self, source: &[u8], end: usize, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
        while self.peek(source, end).is_some_and(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\n')) {
            self.byte(source, end, port)?;
        }
        Ok(())
    }
    fn hex(&mut self, source: &[u8], end: usize, port: &mut Port<'_, '_, '_>) -> Result<u32, ValueError> {
        let mut value = 0;
        for _ in 0..4 {
            let b = self.byte(source, end, port)?;
            value = (value << 4)
                | match b {
                    b'0'..=b'9' => (b - b'0') as u32,
                    b'a'..=b'f' => (b - b'a' + 10) as u32,
                    b'A'..=b'F' => (b - b'A' + 10) as u32,
                    _ => return Err(invalid("natural glTF JSON Unicode escape is malformed")),
                };
        }
        Ok(value)
    }
    fn character(&mut self, source: &[u8], end: usize, port: &mut Port<'_, '_, '_>) -> Result<Option<char>, ValueError> {
        let byte = self.byte(source, end, port)?;
        Ok(Some(match byte {
            b'"' => return Ok(None),
            b'\\' => match self.byte(source, end, port)? {
                b'"' => '"',
                b'\\' => '\\',
                b'/' => '/',
                b'b' => '\u{8}',
                b'f' => '\u{c}',
                b'n' => '\n',
                b'r' => '\r',
                b't' => '\t',
                b'u' => {
                    let mut scalar = self.hex(source, end, port)?;
                    if (0xd800..=0xdbff).contains(&scalar) {
                        if self.byte(source, end, port)? != b'\\' || self.byte(source, end, port)? != b'u' {
                            return Err(invalid("natural glTF JSON high surrogate is unpaired"));
                        }
                        let low = self.hex(source, end, port)?;
                        if !(0xdc00..=0xdfff).contains(&low) {
                            return Err(invalid("natural glTF JSON low surrogate is invalid"));
                        }
                        scalar = 0x10000 + ((scalar - 0xd800) << 10) + low - 0xdc00;
                    }
                    char::from_u32(scalar).ok_or_else(|| invalid("natural glTF JSON Unicode scalar is invalid"))?
                }
                _ => return Err(invalid("natural glTF JSON escape is invalid")),
            },
            0..=31 => return Err(invalid("natural glTF JSON string contains an unescaped control byte")),
            32..=127 => byte as char,
            _ => {
                let width = match byte {
                    0xc2..=0xdf => 2,
                    0xe0..=0xef => 3,
                    0xf0..=0xf4 => 4,
                    _ => return Err(invalid("natural glTF JSON UTF-8 lead byte is invalid")),
                };
                let mut bytes = [0u8; 4];
                bytes[0] = byte;
                for slot in &mut bytes[1..width] {
                    *slot = self.byte(source, end, port)?;
                }
                std::str::from_utf8(&bytes[..width]).ok().and_then(|s| s.chars().next()).ok_or_else(|| invalid("natural glTF JSON UTF-8 scalar is invalid"))?
            }
        }))
    }
    fn string(&mut self, source: &[u8], end: usize, port: &mut Port<'_, '_, '_>) -> Result<(usize, usize), ValueError> {
        let start = self.cursor;
        if self.byte(source, end, port)? != b'"' {
            return Err(invalid("natural glTF JSON object key is not a string"));
        }
        while self.character(source, end, port)?.is_some() {
            if self.cursor - start > 1024 * 1024 {
                return Err(limit("natural glTF JSON string exceeds one MiB admission"));
            }
        }
        Ok((start, self.cursor))
    }
    fn number(&mut self, source: &[u8], end: usize, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
        let start = self.cursor;
        if self.peek(source, end) == Some(b'-') {
            self.byte(source, end, port)?;
        }
        match self.byte(source, end, port)? {
            b'0' => {}
            b'1'..=b'9' => {
                while self.peek(source, end).is_some_and(|b| b.is_ascii_digit()) {
                    self.byte(source, end, port)?;
                    if self.cursor - start > 4096 {
                        return Err(limit("natural glTF JSON number exceeds 4096-byte admission"));
                    }
                }
            }
            _ => return Err(invalid("natural glTF JSON number lacks an integer mantissa")),
        }
        if self.peek(source, end) == Some(b'.') {
            self.byte(source, end, port)?;
            let before = self.cursor;
            while self.peek(source, end).is_some_and(|b| b.is_ascii_digit()) {
                self.byte(source, end, port)?;
                if self.cursor - start > 4096 {
                    return Err(limit("natural glTF JSON number exceeds 4096-byte admission"));
                }
            }
            if self.cursor == before {
                return Err(invalid("natural glTF JSON fractional mantissa is empty"));
            }
        }
        if self.peek(source, end).is_some_and(|b| matches!(b, b'e' | b'E')) {
            self.byte(source, end, port)?;
            if self.peek(source, end).is_some_and(|b| matches!(b, b'+' | b'-')) {
                self.byte(source, end, port)?;
            }
            let before = self.cursor;
            while self.peek(source, end).is_some_and(|b| b.is_ascii_digit()) {
                self.byte(source, end, port)?;
                if self.cursor - start > 4096 {
                    return Err(limit("natural glTF JSON number exceeds 4096-byte admission"));
                }
            }
            if self.cursor == before {
                return Err(invalid("natural glTF JSON exponent is empty"));
            }
        }
        Ok(())
    }
    fn value(&mut self, source: &[u8], end: usize, depth: usize, key: (usize, usize), store: bool, port: &mut Port<'_, '_, '_>) -> Result<usize, ValueError> {
        if depth >= MAX_DEPTH {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "natural glTF JSON exceeds 64 levels"));
        }
        self.trivia(source, end, port)?;
        let start = self.cursor;
        let kind = match self.peek(source, end).ok_or_else(|| invalid("natural glTF JSON value is missing"))? {
            b'{' => 1,
            b'[' => 2,
            b'"' => 3,
            b't' => 5,
            b'f' => 6,
            b'n' => 7,
            _ => 4,
        };
        let id = self.count;
        if id >= MAX_TOKENS {
            return Err(limit("natural glTF JSON token admission exceeded"));
        }
        port.effect(size_of::<Token>(), || {
            self.count += 1;
            if store {
                self.tokens.push(Token { kind, start, end: start, first: NONE, next: NONE, count: 0, key });
            }
        })?;
        let mut last = NONE;
        let mut children = 0;
        if kind == 1 || kind == 2 {
            self.byte(source, end, port)?;
            self.trivia(source, end, port)?;
            let close = if kind == 1 { b'}' } else { b']' };
            if self.peek(source, end) != Some(close) {
                loop {
                    let key = if kind == 1 {
                        let key = self.string(source, end, port)?;
                        self.trivia(source, end, port)?;
                        if self.byte(source, end, port)? != b':' {
                            return Err(invalid("natural glTF JSON object lacks a colon"));
                        }
                        key
                    } else {
                        (0, 0)
                    };
                    let child = self.value(source, end, depth + 1, key, store, port)?;
                    children += 1;
                    if kind == 1 && children > MAX_MEMBERS {
                        return Err(limit("natural glTF JSON object exceeds 4096 authored members"));
                    }
                    if store {
                        port.effect(size_of::<usize>(), || {
                            if last == NONE {
                                self.tokens[id].first = child;
                            } else {
                                self.tokens[last].next = child;
                            }
                            last = child;
                        })?;
                    }
                    self.trivia(source, end, port)?;
                    match self.byte(source, end, port)? {
                        b',' => self.trivia(source, end, port)?,
                        b if b == close => break,
                        _ => return Err(invalid("natural glTF JSON container separator is invalid")),
                    }
                }
            } else {
                self.byte(source, end, port)?;
            }
        } else if kind == 3 {
            self.string(source, end, port)?;
        } else if kind == 4 {
            self.number(source, end, port)?;
        } else {
            let word: &[u8] = match kind {
                5 => b"true",
                6 => b"false",
                _ => b"null",
            };
            for expected in word {
                if self.byte(source, end, port)? != *expected {
                    return Err(invalid("natural glTF JSON literal is malformed"));
                }
            }
        }
        if store {
            let at = self.cursor;
            port.effect(2 * size_of::<usize>(), || {
                self.tokens[id].end = at;
                self.tokens[id].count = children;
            })?;
        }
        Ok(id)
    }
    pub(super) fn receive(&mut self, source: &[u8], natural: Natural, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
        if !self.tokens.is_empty() {
            return Err(invalid("natural glTF token catalog was already populated"));
        }
        self.source = (source.as_ptr() as usize, source.len());
        for store in [false, true] {
            self.cursor = natural.json.0;
            self.count = 0;
            if source.get(self.cursor..self.cursor + 3) == Some(&[0xef, 0xbb, 0xbf]) {
                for _ in 0..3 {
                    self.byte(source, natural.json.1, port)?;
                }
            }
            self.value(source, natural.json.1, 0, (0, 0), store, port)?;
            self.root_end = self.cursor;
            self.trivia(source, natural.json.1, port)?;
            if self.cursor != natural.json.1 {
                return Err(invalid("natural glTF JSON contains trailing tokens"));
            }
            if natural.glb {
                for at in self.root_end..natural.json.1 {
                    if !port.effect(1, || source[at] == b' ')? {
                        return Err(invalid("natural GLB JSON padding is not canonical space"));
                    }
                }
            }
            if !store {
                port.vector(self.count, &mut self.tokens)?;
            }
        }
        if self.tokens.first().is_none_or(|t| t.kind != 1) {
            return Err(invalid("natural glTF root is not an object"));
        }
        Ok(())
    }
    fn check(&self, source: &[u8]) -> Result<(), ValueError> {
        if self.source != (source.as_ptr() as usize, source.len()) { Err(invalid("natural glTF token catalog changed original source")) } else { Ok(()) }
    }
    fn equal_span(&self, source: &[u8], span: (usize, usize), text: &str, port: &mut Port<'_, '_, '_>) -> Result<bool, ValueError> {
        self.check(source)?;
        let mut reader = Catalog { cursor: span.0, ..Default::default() };
        if reader.byte(source, span.1, port)? != b'"' {
            return Err(invalid("natural glTF string span is invalid"));
        }
        let mut expected = text.chars();
        let mut equal = true;
        while let Some(c) = reader.character(source, span.1, port)? {
            equal &= expected.next() == Some(c);
        }
        Ok(equal && expected.next().is_none())
    }
    pub(super) fn equal(&self, source: &[u8], id: usize, text: &str, port: &mut Port<'_, '_, '_>) -> Result<bool, ValueError> {
        let t = self.tokens.get(id).filter(|t| t.kind == 3).ok_or_else(|| invalid("natural glTF property requires a string"))?;
        self.equal_span(source, (t.start, t.end), text, port)
    }
    pub(super) fn member(&self, source: &[u8], id: usize, name: &str, port: &mut Port<'_, '_, '_>) -> Result<Option<usize>, ValueError> {
        self.check(source)?;
        let token = self.tokens.get(id).filter(|t| t.kind == 1).ok_or_else(|| invalid("natural glTF property requires an object"))?;
        let mut next = token.first;
        let mut found = None;
        while next != NONE {
            let t = self.tokens[next];
            if self.equal_span(source, t.key, name, port)? {
                found = Some(next);
            }
            next = t.next;
        }
        Ok(found)
    }
    pub(super) fn number_value(&self, source: &[u8], id: usize, port: &mut Port<'_, '_, '_>) -> Result<f64, ValueError> {
        self.check(source)?;
        let t = self.tokens.get(id).filter(|t| t.kind == 4).ok_or_else(|| invalid("natural glTF property requires a number"))?;
        let n = port.effect(t.end - t.start, || std::str::from_utf8(&source[t.start..t.end]).ok().and_then(|s| s.parse::<f64>().ok()))?.filter(|n| n.is_finite()).ok_or_else(|| invalid("natural glTF number is outside finite Binary64"))?;
        Ok(n)
    }
    pub(super) fn usize(&self, source: &[u8], id: usize, port: &mut Port<'_, '_, '_>) -> Result<usize, ValueError> {
        let n = self.number_value(source, id, port)?;
        if n < 0. || n.fract() != 0. || n > 9007199254740991. || n > usize::MAX as f64 {
            return Err(invalid("natural glTF unsigned property is outside exact integer domain"));
        }
        Ok(n as usize)
    }
    pub(super) fn text(&self, source: &[u8], id: usize, out: &mut String, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
        self.check(source)?;
        let t = self.tokens.get(id).filter(|t| t.kind == 3).ok_or_else(|| invalid("natural glTF property requires a string"))?;
        let mut reader = Catalog { cursor: t.start + 1, ..Default::default() };
        let mut length = 0;
        while let Some(c) = reader.character(source, t.end, port)? {
            length += c.len_utf8();
        }
        port.text(length, out)?;
        reader.cursor = t.start + 1;
        while let Some(c) = reader.character(source, t.end, port)? {
            port.effect(c.len_utf8(), || out.push(c))?;
        }
        Ok(())
    }
    pub(super) fn children(&self, id: usize) -> Result<Children<'_>, ValueError> {
        let t = self.tokens.get(id).filter(|t| t.kind == 2).ok_or_else(|| invalid("natural glTF property requires an array"))?;
        Ok(Children { tokens: &self.tokens, next: t.first })
    }
}
pub(super) struct Children<'a> {
    tokens: &'a [Token],
    next: usize,
}
impl Iterator for Children<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.next == NONE {
            return None;
        }
        let id = self.next;
        self.next = self.tokens[id].next;
        Some(id)
    }
}
