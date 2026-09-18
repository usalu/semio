//! 🔤️ COS syntax (ISO 32000-1 §7.2–7.3): a cursor lexer/parser over the object grammar, the
//! indirect-object and brute-force scanners the cross-reference layer builds on, and the
//! canonical object writer every emitted byte goes through. Shared by both standards of the
//! artifact (PDF 1.4 and 1.7 have the same lexical grammar).

use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDecimal, PdfDictEntry, PdfObject};
use std::collections::HashMap;

//#region 🔖️Error
/// 🚨 Typed engine error — never silent fabrication. `Unsupported` names a feature this codec
/// refuses to guess at (e.g. an encryption revision it does not implement).
#[derive(Clone, Debug, PartialEq)]
pub enum PdfEngineError {
    NotPdf,
    Unsupported(String),
    Malformed(String),
}

impl std::fmt::Display for PdfEngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfEngineError::NotPdf => write!(f, "pdf: not a PDF file (missing %PDF- magic)"),
            PdfEngineError::Unsupported(s) => write!(f, "pdf: unsupported: {s}"),
            PdfEngineError::Malformed(s) => write!(f, "pdf: malformed: {s}"),
        }
    }
}
impl std::error::Error for PdfEngineError {}

pub type PResult<T> = Result<T, PdfEngineError>;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn malformed<T>(msg: impl Into<String>) -> PResult<T> {
    Err(PdfEngineError::Malformed(msg.into()))
}
//#endregion 🔖️Error

//#region 🔖️Characters
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n' | 0x0C | 0x00)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_delim(b: u8) -> bool {
    matches!(b, b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'/' | b'%')
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn hex_val(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => 0,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_subslice(data: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    if from > data.len() || needle.is_empty() {
        return None;
    }
    data[from..].windows(needle.len()).position(|w| w == needle).map(|p| p + from)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_last_subslice(data: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > data.len() {
        return None;
    }
    (0..=data.len() - needle.len()).rev().find(|&i| &data[i..i + needle.len()] == needle)
}
//#endregion 🔖️Characters

//#region 🔖️Lexer
/// 🎫 One token of a content stream or object body: a value, a bare keyword (an operator or
/// `obj`/`stream`/…), or the end of input.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Object(PdfObject),
    Keyword(String),
    End,
}

/// 🔍 Cursor-based recursive-descent lexer/parser over the PDF COS object grammar. Used for
/// top-level `N G obj ... endobj` parsing, values nested inside arrays/dicts, and content streams.
pub struct Lexer<'a> {
    pub data: &'a [u8],
    pub pos: usize,
}

impl<'a> Lexer<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn at(&self, offset: usize) -> Self {
        Self { data: self.data, pos: offset }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn peek(&self) -> Option<u8> {
        self.data.get(self.pos).copied()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn peek_at(&self, n: usize) -> Option<u8> {
        self.data.get(self.pos + n).copied()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn skip_ws(&mut self) {
        loop {
            match self.peek() {
                Some(b) if is_ws(b) => {
                    self.pos += 1;
                }
                Some(b'%') => {
                    while let Some(c) = self.peek() {
                        self.pos += 1;
                        if c == b'\n' || c == b'\r' {
                            break;
                        }
                    }
                }
                _ => break,
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_regular_run(&mut self) -> &'a [u8] {
        let start = self.pos;
        while let Some(b) = self.peek() {
            if is_ws(b) || is_delim(b) {
                break;
            }
            self.pos += 1;
        }
        &self.data[start..self.pos]
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn starts_with(&self, kw: &[u8]) -> bool {
        self.data.get(self.pos..self.pos + kw.len()) == Some(kw)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn consume_keyword(&mut self, kw: &[u8]) -> bool {
        if self.starts_with(kw) {
            self.pos += kw.len();
            true
        } else {
            false
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn parse_number(&mut self) -> PResult<PdfObject> {
        let start = self.pos;
        if matches!(self.peek(), Some(b'+') | Some(b'-')) {
            self.pos += 1;
        }
        let mut is_real = false;
        let mut saw_digit = false;
        while let Some(b) = self.peek() {
            match b {
                b'0'..=b'9' => {
                    saw_digit = true;
                    self.pos += 1;
                }
                b'.' => {
                    is_real = true;
                    self.pos += 1;
                }
                b'+' | b'-' => {
                    self.pos += 1;
                }
                _ => break,
            }
        }
        if !saw_digit {
            return malformed("expected number");
        }
        let raw = std::str::from_utf8(&self.data[start..self.pos]).unwrap_or("0");
        // 🩹 Lenient like every shipping reader: collapse repeated signs/dots real generators emit.
        let mut text = String::with_capacity(raw.len());
        let mut seen_dot = false;
        for (index, character) in raw.chars().enumerate() {
            match character {
                '-' | '+' if index == 0 => text.push(character),
                '-' | '+' => {}
                '.' if !seen_dot => {
                    seen_dot = true;
                    text.push('.');
                }
                '.' => {}
                digit => text.push(digit),
            }
        }
        if is_real {
            PdfDecimal::parse(&text).map(PdfObject::Real).map_err(PdfEngineError::Malformed)
        } else {
            match text.parse::<i64>() {
                Ok(value) => Ok(PdfObject::Int(value)),
                Err(_) => PdfDecimal::parse(&format!("{text}.")).map(PdfObject::Real).map_err(PdfEngineError::Malformed),
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_name(&mut self) -> PdfObject {
        self.pos += 1;
        let mut out = Vec::new();
        while let Some(b) = self.peek() {
            if is_ws(b) || is_delim(b) {
                break;
            }
            if b == b'#' && self.peek_at(1).is_some_and(|c| c.is_ascii_hexdigit()) && self.peek_at(2).is_some_and(|c| c.is_ascii_hexdigit()) {
                out.push((hex_val(self.data[self.pos + 1]) << 4) | hex_val(self.data[self.pos + 2]));
                self.pos += 3;
                continue;
            }
            out.push(b);
            self.pos += 1;
        }
        PdfObject::Name(String::from_utf8(out.clone()).unwrap_or_else(|_| out.iter().map(|&b| b as char).collect()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_literal_string(&mut self) -> PResult<PdfObject> {
        self.pos += 1;
        let mut depth = 1i32;
        let mut out = Vec::new();
        while let Some(b) = self.peek() {
            self.pos += 1;
            match b {
                b'(' => {
                    depth += 1;
                    out.push(b);
                }
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(PdfObject::Str(out));
                    }
                    out.push(b);
                }
                b'\\' => match self.peek() {
                    Some(b'n') => {
                        out.push(b'\n');
                        self.pos += 1;
                    }
                    Some(b'r') => {
                        out.push(b'\r');
                        self.pos += 1;
                    }
                    Some(b't') => {
                        out.push(b'\t');
                        self.pos += 1;
                    }
                    Some(b'b') => {
                        out.push(0x08);
                        self.pos += 1;
                    }
                    Some(b'f') => {
                        out.push(0x0C);
                        self.pos += 1;
                    }
                    Some(b'\r') => {
                        self.pos += 1;
                        if self.peek() == Some(b'\n') {
                            self.pos += 1;
                        }
                    }
                    Some(b'\n') => {
                        self.pos += 1;
                    }
                    Some(d) if (b'0'..=b'7').contains(&d) => {
                        let mut v: u32 = 0;
                        let mut n = 0;
                        while n < 3 {
                            match self.peek() {
                                Some(dd) if (b'0'..=b'7').contains(&dd) => {
                                    v = v * 8 + (dd - b'0') as u32;
                                    self.pos += 1;
                                    n += 1;
                                }
                                _ => break,
                            }
                        }
                        out.push((v & 0xFF) as u8);
                    }
                    Some(other) => {
                        out.push(other);
                        self.pos += 1;
                    }
                    None => {}
                },
                other => out.push(other),
            }
        }
        malformed("unterminated literal string")
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_hex_string(&mut self) -> PResult<PdfObject> {
        self.pos += 1;
        let mut nibbles = Vec::new();
        loop {
            match self.peek() {
                Some(b'>') => {
                    self.pos += 1;
                    break;
                }
                Some(b) if b.is_ascii_hexdigit() => {
                    nibbles.push(hex_val(b));
                    self.pos += 1;
                }
                Some(b) if is_ws(b) => {
                    self.pos += 1;
                }
                None => return malformed("unterminated hex string"),
                Some(_) => {
                    self.pos += 1;
                }
            }
        }
        if nibbles.len() % 2 == 1 {
            nibbles.push(0);
        }
        Ok(PdfObject::Str(nibbles.chunks(2).map(|c| (c[0] << 4) | c[1]).collect()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_array(&mut self) -> PResult<PdfObject> {
        self.pos += 1;
        let mut items = Vec::new();
        loop {
            self.skip_ws();
            match self.peek() {
                Some(b']') => {
                    self.pos += 1;
                    break;
                }
                None => return malformed("unterminated array"),
                _ => match self.next_token()? {
                    Token::Object(value) => items.push(value),
                    Token::Keyword(_) => {}
                    Token::End => return malformed("unterminated array"),
                },
            }
        }
        Ok(PdfObject::Array(items))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_dict_or_stream(&mut self, allow_stream: bool) -> PResult<PdfObject> {
        self.pos += 2;
        let mut entries = Vec::new();
        loop {
            self.skip_ws();
            if self.starts_with(b">>") {
                self.pos += 2;
                break;
            }
            match self.peek() {
                None => return malformed("unterminated dictionary"),
                Some(b'/') => {}
                Some(_) => {
                    // 🩹 Skip a stray value/keyword where a key belongs (malformed generators).
                    match self.next_token()? {
                        Token::End => return malformed("unterminated dictionary"),
                        _ => continue,
                    }
                }
            }
            let key = match self.parse_name() {
                PdfObject::Name(n) => n,
                _ => unreachable!(),
            };
            self.skip_ws();
            if self.starts_with(b">>") {
                entries.push(PdfDictEntry { key, value: PdfObject::Null });
                continue;
            }
            let value = match self.next_token()? {
                Token::Object(value) => value,
                Token::Keyword(_) | Token::End => PdfObject::Null,
            };
            entries.push(PdfDictEntry { key, value });
        }
        if allow_stream {
            let save = self.pos;
            self.skip_ws();
            if self.consume_keyword(b"stream") {
                if self.peek() == Some(b'\r') {
                    self.pos += 1;
                }
                if self.peek() == Some(b'\n') {
                    self.pos += 1;
                }
                let data_start = self.pos;
                let declared_len = entries.iter().find(|e| e.key == "Length").and_then(|e| match &e.value {
                    PdfObject::Int(i) if *i >= 0 => Some(*i as usize),
                    _ => None,
                });
                let data_end = match declared_len {
                    Some(len) if data_start + len <= self.data.len() && Self::endstream_follows(self.data, data_start + len) => data_start + len,
                    _ => find_subslice(self.data, data_start, b"endstream").map(|end| Self::trim_stream_eol(self.data, data_start, end)).unwrap_or(self.data.len()),
                };
                let raw = self.data[data_start..data_end.min(self.data.len())].to_vec();
                self.pos = data_end;
                self.skip_ws();
                let _ = self.consume_keyword(b"endstream");
                return Ok(PdfObject::Stream { dict: entries, data: raw, filters: Vec::new() });
            }
            self.pos = save;
        }
        Ok(PdfObject::Dict(entries))
    }

    /// 📏 Whether `endstream` (after optional EOL) follows `at` — validates a declared `/Length`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn endstream_follows(data: &[u8], at: usize) -> bool {
        let mut lex = Lexer::new(data).at(at);
        lex.skip_ws();
        lex.starts_with(b"endstream")
    }

    /// 📏 Drops the EOL that precedes `endstream` when the length had to be searched for.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn trim_stream_eol(data: &[u8], start: usize, end: usize) -> usize {
        let mut end = end;
        if end > start && data[end - 1] == b'\n' {
            end -= 1;
        }
        if end > start && data[end - 1] == b'\r' {
            end -= 1;
        }
        end
    }

    /// 🎯 Parses one value: number, `N G R` reference, name, string, array, dict/stream,
    /// `true`/`false`/`null`. A bare keyword yields `Null` (use [`Lexer::next_token`] where
    /// keywords matter).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn parse_object(&mut self) -> PResult<PdfObject> {
        match self.next_token()? {
            Token::Object(value) => Ok(value),
            Token::Keyword(_) => Ok(PdfObject::Null),
            Token::End => malformed("unexpected end of input"),
        }
    }

    /// 🎫 Reads the next token: a value (with `N G R` references folded), a keyword, or the end.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn next_token(&mut self) -> PResult<Token> {
        self.skip_ws();
        match self.peek() {
            None => Ok(Token::End),
            Some(b'/') => Ok(Token::Object(self.parse_name())),
            Some(b'(') => self.parse_literal_string().map(Token::Object),
            Some(b'<') if self.peek_at(1) == Some(b'<') => self.parse_dict_or_stream(true).map(Token::Object),
            Some(b'<') => self.parse_hex_string().map(Token::Object),
            Some(b'[') => self.parse_array().map(Token::Object),
            Some(b']') | Some(b'>') | Some(b')') | Some(b'{') | Some(b'}') => {
                self.pos += 1;
                self.next_token()
            }
            Some(b'-') | Some(b'+') | Some(b'.') | Some(b'0'..=b'9') => {
                let first = match self.parse_number() {
                    Ok(value) => value,
                    Err(_) => {
                        self.pos += 1;
                        return self.next_token();
                    }
                };
                if let PdfObject::Int(num) = first {
                    if num >= 0 {
                        let save = self.pos;
                        self.skip_ws();
                        if matches!(self.peek(), Some(b'0'..=b'9')) {
                            if let Ok(PdfObject::Int(gen)) = self.parse_number() {
                                if gen >= 0 {
                                    self.skip_ws();
                                    if self.consume_keyword(b"R") && self.peek().is_none_or(|b| is_ws(b) || is_delim(b)) {
                                        return Ok(Token::Object(PdfObject::Ref(ObjRef { num: num as u32, gen: gen as u16 })));
                                    }
                                }
                            }
                        }
                        self.pos = save;
                    }
                }
                Ok(Token::Object(first))
            }
            Some(_) => {
                let run = self.read_regular_run();
                if run.is_empty() {
                    self.pos += 1;
                    return self.next_token();
                }
                Ok(match run {
                    b"true" => Token::Object(PdfObject::Bool(true)),
                    b"false" => Token::Object(PdfObject::Bool(false)),
                    b"null" => Token::Object(PdfObject::Null),
                    other => Token::Keyword(String::from_utf8_lossy(other).into_owned()),
                })
            }
        }
    }
}
//#endregion 🔖️Lexer

//#region 🔖️IndirectObjects
/// 📦️ Parses one `N G obj ... endobj` at `offset`. Returns the parsed value and the id it
/// actually declared (used by the brute-force scanner, which doesn't trust its own guessed id).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_indirect_at(data: &[u8], offset: usize) -> PResult<(ObjRef, PdfObject)> {
    let mut lex = Lexer::new(data).at(offset);
    lex.skip_ws();
    let num = match lex.parse_number()? {
        PdfObject::Int(i) if i >= 0 => i as u32,
        _ => return malformed("bad object number"),
    };
    lex.skip_ws();
    let gen = match lex.parse_number()? {
        PdfObject::Int(i) if i >= 0 => i as u16,
        _ => return malformed("bad generation number"),
    };
    lex.skip_ws();
    if !lex.consume_keyword(b"obj") {
        return malformed("expected 'obj' keyword");
    }
    lex.skip_ws();
    let value = if lex.starts_with(b"endobj") { PdfObject::Null } else { lex.parse_object()? };
    lex.skip_ws();
    let _ = lex.consume_keyword(b"endobj");
    Ok((ObjRef { num, gen }, value))
}

/// 🩹 Brute-force fallback: scans the whole buffer for `N G obj` patterns — used when structured
/// xref parsing fails outright (damaged/`%%EOF`-free files). Real readers all do this; the last
/// occurrence of a given object number wins (later generation/incremental update).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn brute_force_scan(data: &[u8]) -> HashMap<u32, (ObjRef, usize)> {
    let mut found: HashMap<u32, (ObjRef, usize)> = HashMap::new();
    let mut i = 0usize;
    while i < data.len() {
        if data[i].is_ascii_digit() && (i == 0 || is_ws(data[i - 1]) || is_delim(data[i - 1])) {
            let start = i;
            let mut lex = Lexer::new(data).at(start);
            if let Ok(PdfObject::Int(num)) = lex.parse_number() {
                if num >= 0 {
                    lex.skip_ws();
                    if let Ok(PdfObject::Int(gen)) = lex.parse_number() {
                        if gen >= 0 {
                            lex.skip_ws();
                            if lex.consume_keyword(b"obj") {
                                found.insert(num as u32, (ObjRef { num: num as u32, gen: gen as u16 }, start));
                                i = lex.pos;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    found
}
//#endregion 🔖️IndirectObjects

//#region 🔖️Writer
/// 🔢️ Canonical number text: integers bare, reals exact and exponent-free (PDF has no exponent
/// syntax), `-0` folded to `0`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn number_text(value: f64) -> String {
    if !value.is_finite() {
        return "0".into();
    }
    if value.fract() == 0.0 && value.abs() < 1e15 {
        return format!("{}", value as i64);
    }
    let decimal = PdfDecimal::from_f64(value);
    let mut text = decimal.to_string();
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    if text == "-0" {
        text = "0".into();
    }
    text
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_name(out: &mut Vec<u8>, name: &str) {
    out.push(b'/');
    for byte in name.bytes() {
        if (33..=126).contains(&byte) && !is_delim(byte) && byte != b'#' {
            out.push(byte);
        } else {
            out.extend_from_slice(format!("#{byte:02X}").as_bytes());
        }
    }
}

/// 🔤️ Writes a string operand: literal when printable ASCII (escaping the §7.3.4.2 specials and
/// balanced-parenthesis-free), hex otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_string(out: &mut Vec<u8>, bytes: &[u8]) {
    if bytes.iter().all(|byte| matches!(byte, 0x20..=0x7e)) {
        out.push(b'(');
        for byte in bytes {
            if matches!(byte, b'(' | b')' | b'\\') {
                out.push(b'\\');
            }
            out.push(*byte);
        }
        out.push(b')');
    } else {
        out.push(b'<');
        for byte in bytes {
            out.extend_from_slice(format!("{byte:02X}").as_bytes());
        }
        out.push(b'>');
    }
}

/// ✍️ Writes one direct object in canonical form. Streams are written with their `data` AS IS
/// and `/Length` set from it — the caller is responsible for having applied the stream's
/// filters (@see `filters::encode_stream`) so the dictionary and the bytes agree.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_object(out: &mut Vec<u8>, object: &PdfObject) {
    match object {
        PdfObject::Null => out.extend_from_slice(b"null"),
        PdfObject::Bool(value) => out.extend_from_slice(if *value { b"true" } else { b"false" }),
        PdfObject::Int(value) => out.extend_from_slice(value.to_string().as_bytes()),
        PdfObject::Real(value) => out.extend_from_slice(number_text(value.to_f64().unwrap_or(0.0)).as_bytes()),
        PdfObject::Str(bytes) => write_string(out, bytes),
        PdfObject::Name(name) => write_name(out, name),
        PdfObject::Array(items) => {
            out.push(b'[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(b' ');
                }
                write_object(out, item);
            }
            out.push(b']');
        }
        PdfObject::Dict(entries) => write_dict(out, entries),
        PdfObject::Ref(reference) => out.extend_from_slice(format!("{} {} R", reference.num, reference.gen).as_bytes()),
        PdfObject::Stream { dict, data, .. } => {
            let mut entries: Vec<PdfDictEntry> = dict.iter().filter(|entry| entry.key != "Length").cloned().collect();
            entries.push(PdfDictEntry::new("Length", PdfObject::Int(data.len() as i64)));
            write_dict(out, &entries);
            out.extend_from_slice(b"\nstream\n");
            out.extend_from_slice(data);
            out.extend_from_slice(b"\nendstream");
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_dict(out: &mut Vec<u8>, entries: &[PdfDictEntry]) {
    out.extend_from_slice(b"<<");
    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            out.push(b' ');
        }
        write_name(out, &entry.key);
        if !matches!(entry.value, PdfObject::Name(_) | PdfObject::Array(_) | PdfObject::Dict(_) | PdfObject::Str(_) | PdfObject::Stream { .. }) {
            out.push(b' ');
        }
        write_object(out, &entry.value);
    }
    out.extend_from_slice(b">>");
}

/// 🧾 Serializes one direct object to bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn object_bytes(object: &PdfObject) -> Vec<u8> {
    let mut out = Vec::new();
    write_object(&mut out, object);
    out
}
//#endregion 🔖️Writer

//#region 🔖️DictHelpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_get<'a>(entries: &'a [PdfDictEntry], key: &str) -> Option<&'a PdfObject> {
    entries.iter().find(|entry| entry.key == key).map(|entry| &entry.value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_put(entries: &mut Vec<PdfDictEntry>, key: &str, value: PdfObject) {
    match entries.iter_mut().find(|entry| entry.key == key) {
        Some(entry) => entry.value = value,
        None => entries.push(PdfDictEntry::new(key, value)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_drop(entries: &mut Vec<PdfDictEntry>, key: &str) {
    entries.retain(|entry| entry.key != key);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_i64(entries: &[PdfDictEntry], key: &str) -> Option<i64> {
    dict_get(entries, key).and_then(PdfObject::as_i64)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_f64(entries: &[PdfDictEntry], key: &str) -> Option<f64> {
    dict_get(entries, key).and_then(PdfObject::as_f64)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_name<'a>(entries: &'a [PdfDictEntry], key: &str) -> Option<&'a str> {
    dict_get(entries, key).and_then(PdfObject::as_name)
}
//#endregion 🔖️DictHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
