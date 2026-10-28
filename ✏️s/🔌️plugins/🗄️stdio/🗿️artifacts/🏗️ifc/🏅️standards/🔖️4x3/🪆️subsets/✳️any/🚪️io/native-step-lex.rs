//! 🔤️ Funded lexical scans borrow original TXT spans and decode directly into retained token storage.
use super::{MAX_RECORD_BYTES, Port, TxtSnapshot, invalid, limit};
use semio_framework_value::{RetireOwned, ValueError};
const MAX_NUMBER_BYTES: usize = 65_536;
#[derive(Clone, Copy, Default, RetireOwned)]
pub(super) struct Position {
    line: usize,
    column: usize,
    pub offset: usize,
}
#[derive(Clone, Copy, Default)]
pub(super) struct Token {
    pub kind: u8,
    pub start: Position,
    pub end: Position,
    pub length: usize,
    pub id: u64,
    word: [u8; 32],
    word_len: usize,
    pub name: bool,
    pub real: bool,
}
impl Token {
    pub fn is(&self, word: &str) -> bool {
        self.kind == b'K' && self.word_len == word.len() && self.word.get(..self.word_len) == Some(word.as_bytes())
    }
}
#[derive(Clone, Copy, Default)]
pub(super) struct Lexer {
    pub position: Position,
    record: usize,
}
impl Lexer {
    fn raw(&self, source: &TxtSnapshot) -> Option<u8> {
        let line = source.lines.get(self.position.line)?;
        line.as_bytes()
            .get(self.position.column)
            .copied()
            .or_else(|| if self.position.line + 1 < source.lines.len() || source.trailing_newline { source.line_ending.as_str().as_bytes().get(self.position.column.saturating_sub(line.len())).copied() } else { None })
    }
    fn byte(&mut self, source: &TxtSnapshot, port: &mut Port<'_, '_, '_>) -> Result<Option<u8>, ValueError> {
        let Some(byte) = self.raw(source) else {
            return Ok(None);
        };
        port.at(self.position.offset);
        if self.position.offset.saturating_sub(self.record) >= MAX_RECORD_BYTES {
            return Err(limit("STEP record exceeds byte admission"));
        }
        port.effect(1, || {
            self.position.offset += 1;
            self.position.column += 1;
            let line = &source.lines[self.position.line];
            let end = line.len() + if self.position.line + 1 < source.lines.len() || source.trailing_newline { source.line_ending.as_str().len() } else { 0 };
            if self.position.column == end {
                self.position.line += 1;
                self.position.column = 0;
            }
        })?;
        Ok(Some(byte))
    }
    fn peek(&mut self, source: &TxtSnapshot, port: &mut Port<'_, '_, '_>) -> Result<Option<u8>, ValueError> {
        loop {
            match self.raw(source) {
                Some(b) if b < 32 || b == 127 => {
                    self.byte(source, port)?;
                }
                other => return Ok(other),
            }
        }
    }
    fn bump(&mut self, source: &TxtSnapshot, port: &mut Port<'_, '_, '_>) -> Result<Option<u8>, ValueError> {
        self.peek(source, port)?;
        self.byte(source, port)
    }
    fn prefix(&mut self, source: &TxtSnapshot, text: &[u8], port: &mut Port<'_, '_, '_>) -> Result<bool, ValueError> {
        let mut candidate = *self;
        for &b in text {
            if candidate.bump(source, port)? != Some(b) {
                return Ok(false);
            }
        }
        Ok(true)
    }
    fn consume(&mut self, source: &TxtSnapshot, text: &[u8], port: &mut Port<'_, '_, '_>) -> Result<bool, ValueError> {
        if !self.prefix(source, text, port)? {
            return Ok(false);
        }
        for _ in text {
            self.bump(source, port)?;
        }
        Ok(true)
    }
    fn prints(&mut self, source: &TxtSnapshot, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
        while self.consume(source, b"\\N\\", port)? || self.consume(source, b"\\F\\", port)? {}
        Ok(())
    }
    fn trivia(&mut self, source: &TxtSnapshot, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
        loop {
            if self.peek(source, port)? == Some(b' ') {
                self.bump(source, port)?;
            } else if self.consume(source, b"\\N\\", port)? || self.consume(source, b"\\F\\", port)? {
            } else if self.consume(source, b"/*", port)? {
                loop {
                    if self.consume(source, b"*/", port)? {
                        break;
                    }
                    if self.bump(source, port)?.is_none() {
                        return Err(invalid("unterminated STEP comment"));
                    }
                }
            } else {
                return Ok(());
            }
        }
    }
    fn hex(&mut self, source: &TxtSnapshot, count: usize, port: &mut Port<'_, '_, '_>) -> Result<u32, ValueError> {
        let mut code = 0u32;
        for _ in 0..count {
            self.prints(source, port)?;
            let byte = self.bump(source, port)?.ok_or_else(|| invalid("truncated STEP hexadecimal escape"))?;
            let digit = (byte as char).to_digit(16).ok_or_else(|| invalid("invalid STEP hexadecimal escape"))?;
            code = (code << 4) | digit;
        }
        Ok(code)
    }
    fn scalar(&mut self, source: &TxtSnapshot, first: u8, port: &mut Port<'_, '_, '_>) -> Result<char, ValueError> {
        let width = match first {
            0..=127 => 1,
            0xC2..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xF4 => 4,
            _ => return Err(invalid("invalid UTF-8 in STEP string")),
        };
        let mut bytes = [0; 4];
        bytes[0] = first;
        for byte in bytes.iter_mut().take(width).skip(1) {
            *byte = self.bump(source, port)?.ok_or_else(|| invalid("truncated UTF-8 in STEP string"))?;
        }
        std::str::from_utf8(&bytes[..width]).ok().and_then(|s| s.chars().next()).ok_or_else(|| invalid("invalid UTF-8 in STEP string"))
    }
    fn emit(output: &mut Option<&mut String>, length: &mut usize, c: char, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
        *length = length.checked_add(c.len_utf8()).ok_or_else(|| limit("STEP token decoded extent overflow"))?;
        if let Some(text) = output {
            if c.len_utf8() > text.capacity() - text.len() {
                return Err(invalid("STEP token exceeded its original admitted backing"));
            }
            port.effect(c.len_utf8(), || text.push(c))?;
        }
        Ok(())
    }
    fn string(&mut self, source: &TxtSnapshot, mut output: Option<&mut String>, port: &mut Port<'_, '_, '_>) -> Result<usize, ValueError> {
        if self.bump(source, port)? != Some(b'\'') {
            return Err(invalid("STEP string opener required"));
        }
        let (mut length, mut alphabet) = (0usize, 'A');
        loop {
            let start = self.position.offset;
            let byte = self.bump(source, port)?.ok_or_else(|| invalid("unterminated STEP string"))?;
            let c = match byte {
                b'\'' => {
                    if self.peek(source, port)? == Some(b'\'') {
                        self.bump(source, port)?;
                        '\''
                    } else {
                        break;
                    }
                }
                b'\\' => {
                    let escape = self.bump(source, port)?.ok_or_else(|| invalid("truncated STEP escape"))?;
                    match escape {
                        b'\\' => '\\',
                        b'N' | b'F' => {
                            if self.bump(source, port)? != Some(b'\\') {
                                return Err(invalid("unterminated STEP print directive"));
                            }
                            continue;
                        }
                        b'P' => {
                            let page = self.bump(source, port)?.ok_or_else(|| invalid("missing STEP alphabet"))?;
                            if !(b'A'..=b'I').contains(&page) || self.bump(source, port)? != Some(b'\\') {
                                return Err(invalid("invalid STEP alphabet selector"));
                            }
                            alphabet = page as char;
                            continue;
                        }
                        b'S' => {
                            if self.bump(source, port)? != Some(b'\\') {
                                return Err(invalid("missing STEP PAGE delimiter"));
                            }
                            let payload = self.bump(source, port)?.ok_or_else(|| invalid("missing STEP PAGE payload"))?;
                            super::super::part21::part21_alphabet_character(alphabet, payload as char).ok_or_else(|| {
                                port.at(start);
                                invalid("undefined STEP ISO 8859 page position")
                            })?
                        }
                        b'X' => {
                            if matches!(self.peek(source, port)?, Some(b'2' | b'4')) {
                                let width = self.bump(source, port)?.unwrap();
                                if self.bump(source, port)? != Some(b'\\') {
                                    return Err(invalid("missing STEP Unicode delimiter"));
                                }
                                let (mut high, mut count) = (None, 0usize);
                                loop {
                                    self.prints(source, port)?;
                                    if self.consume(source, b"\\X0\\", port)? {
                                        if count == 0 || high.is_some() {
                                            return Err(invalid("empty STEP Unicode run or unpaired surrogate"));
                                        }
                                        break;
                                    }
                                    let code = self.hex(source, if width == b'2' { 4 } else { 8 }, port)?;
                                    count += 1;
                                    let scalar = if width == b'2' {
                                        if let Some(previous) = high.take() {
                                            if !(0xDC00..=0xDFFF).contains(&code) {
                                                return Err(invalid("unpaired STEP UTF-16 surrogate"));
                                            }
                                            Some(0x10000 + ((previous - 0xD800) << 10) + code - 0xDC00)
                                        } else if (0xD800..=0xDBFF).contains(&code) {
                                            high = Some(code);
                                            None
                                        } else {
                                            Some(code)
                                        }
                                    } else {
                                        Some(code)
                                    };
                                    if let Some(scalar) = scalar {
                                        Self::emit(&mut output, &mut length, char::from_u32(scalar).ok_or_else(|| invalid("invalid STEP Unicode scalar"))?, port)?;
                                    }
                                }
                                continue;
                            } else {
                                if self.bump(source, port)? != Some(b'\\') {
                                    return Err(invalid("missing STEP arbitrary-byte delimiter"));
                                }
                                char::from_u32(self.hex(source, 2, port)?).ok_or_else(|| invalid("invalid STEP arbitrary byte"))?
                            }
                        }
                        _ => return Err(invalid("unsupported STEP string escape")),
                    }
                }
                other => self.scalar(source, other, port)?,
            };
            Self::emit(&mut output, &mut length, c, port)?;
        }
        Ok(length)
    }
    pub(super) fn token(&mut self, source: &TxtSnapshot, port: &mut Port<'_, '_, '_>) -> Result<Token, ValueError> {
        self.trivia(source, port)?;
        let start = self.position;
        port.at(start.offset);
        let mut token = Token { start, ..Default::default() };
        let Some(byte) = self.peek(source, port)? else {
            return Ok(token);
        };
        match byte {
            b'\'' => {
                token.kind = b'S';
                token.length = self.string(source, None, port)?;
            }
            b'"' => {
                token.kind = b'B';
                self.bump(source, port)?;
                self.prints(source, port)?;
                let first = self.bump(source, port)?.ok_or_else(|| invalid("unterminated STEP binary"))?;
                if !(b'0'..=b'3').contains(&first) {
                    return Err(invalid("STEP binary requires unused-bit prefix"));
                }
                token.length = 1;
                let mut first_hex = true;
                loop {
                    self.prints(source, port)?;
                    let b = self.bump(source, port)?.ok_or_else(|| invalid("unterminated STEP binary"))?;
                    if b == b'"' {
                        if token.length == 1 && first != b'0' {
                            return Err(invalid("empty STEP binary has nonzero unused bits"));
                        }
                        break;
                    }
                    let digit = (b as char).to_digit(16).ok_or_else(|| invalid("invalid STEP binary hex digit"))?;
                    if first_hex && digit >= (16u32 >> (first - b'0')) {
                        return Err(invalid("STEP binary high padding bits are not zero"));
                    }
                    first_hex = false;
                    token.length += 1;
                }
            }
            b'#' => {
                token.kind = b'R';
                self.bump(source, port)?;
                let mut count = 0;
                while self.peek(source, port)?.is_some_and(|b| b.is_ascii_digit()) {
                    let digit = self.bump(source, port)?.unwrap() - b'0';
                    token.id = token.id.checked_mul(10).and_then(|n| n.checked_add(digit as u64)).ok_or_else(|| invalid("STEP reference exceeds unsigned64"))?;
                    count += 1;
                }
                if count == 0 {
                    return Err(invalid("STEP reference requires decimal digits"));
                }
            }
            b'.' => {
                token.kind = b'E';
                self.bump(source, port)?;
                if !self.peek(source, port)?.is_some_and(|b| b.is_ascii_uppercase() || b == b'_') {
                    return Err(invalid("STEP enumeration requires a valid name"));
                }
                while self.peek(source, port)?.is_some_and(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_') {
                    self.bump(source, port)?;
                    token.length += 1;
                }
                if token.length == 0 || self.bump(source, port)? != Some(b'.') {
                    return Err(invalid("invalid STEP enumeration"));
                }
            }
            b'+' | b'-' | b'0'..=b'9' => {
                token.kind = b'N';
                if matches!(byte, b'+' | b'-') {
                    self.bump(source, port)?;
                    token.length += 1;
                }
                let mut digits = 0;
                while self.peek(source, port)?.is_some_and(|b| b.is_ascii_digit()) {
                    self.bump(source, port)?;
                    digits += 1;
                    token.length += 1;
                }
                if digits == 0 {
                    return Err(invalid("STEP number requires a decimal mantissa"));
                }
                if self.peek(source, port)? == Some(b'.') {
                    token.real = true;
                    self.bump(source, port)?;
                    token.length += 1;
                    while self.peek(source, port)?.is_some_and(|b| b.is_ascii_digit()) {
                        self.bump(source, port)?;
                        token.length += 1;
                    }
                }
                if self.peek(source, port)? == Some(b'E') {
                    if !token.real {
                        return Err(invalid("STEP real exponent requires a decimal mantissa"));
                    }
                    self.bump(source, port)?;
                    token.length += 1;
                    if matches!(self.peek(source, port)?, Some(b'+' | b'-')) {
                        self.bump(source, port)?;
                        token.length += 1;
                    }
                    let mut digits = 0;
                    while self.peek(source, port)?.is_some_and(|b| b.is_ascii_digit()) {
                        self.bump(source, port)?;
                        digits += 1;
                        token.length += 1;
                    }
                    if digits == 0 {
                        return Err(invalid("STEP real exponent requires digits"));
                    }
                }
                if token.length > MAX_NUMBER_BYTES {
                    return Err(limit("STEP numeric conversion exceeds bounded token admission"));
                }
            }
            b'A'..=b'Z' | b'_' => {
                token.kind = b'K';
                token.name = true;
                while let Some(b) = self.peek(source, port)? {
                    if !(b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_' || b == b'-') {
                        break;
                    }
                    self.bump(source, port)?;
                    if token.word_len < token.word.len() {
                        token.word[token.word_len] = b;
                    }
                    token.word_len += 1;
                    token.length += 1;
                    if b == b'-' {
                        token.name = false;
                    }
                }
            }
            b'(' | b')' | b',' | b'=' | b';' | b'$' | b'*' => {
                token.kind = byte;
                self.bump(source, port)?;
                if byte == b';' {
                    self.record = self.position.offset;
                }
            }
            _ => return Err(invalid("unexpected STEP token byte")),
        }
        token.end = self.position;
        port.at(start.offset);
        Ok(token)
    }
    pub(super) fn decode(&self, source: &TxtSnapshot, token: Token, out: &mut String, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
        let mut cursor = Self { position: token.start, record: token.start.offset };
        port.text(token.length, out)?;
        if token.kind == b'S' {
            let length = cursor.string(source, Some(out), port)?;
            if length != token.length {
                return Err(invalid("STEP string changed its decoded extent"));
            }
        } else {
            if token.kind == b'B' || token.kind == b'E' {
                cursor.bump(source, port)?;
            }
            for _ in 0..token.length {
                if token.kind == b'B' {
                    cursor.prints(source, port)?;
                }
                let b = cursor.bump(source, port)?.ok_or_else(|| invalid("STEP token changed source extent"))?;
                let b = if token.kind == b'B' { b.to_ascii_uppercase() } else { b };
                port.effect(1, || out.push(b as char))?;
            }
        }
        port.at(token.start.offset);
        Ok(())
    }
}
