//! 📝️ Text representation codec surface for `stdio.json` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type JsonSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_rfc8259::subsets::base::schema::snapshot::*;
use crate::STDIO_JSON_DOCUMENT_SCHEMA;
use semio_framework_diagnostic::TextSpan;
use framework_schema::ArtifactSchema;
use semio_framework_diagnostic::TextError;

/// 🔓️ Parses a complete RFC8259 JSON text into a [`JsonValue`], rejecting trailing content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_json_text(text: &str) -> Result<JsonValue, TextError> {
    let mut parser = Parser::new(text);
    let value = parser.parse_value()?;
    parser.skip_ws();
    if parser.pos != parser.bytes.len() {
        return Err(parser.err("trailing characters after JSON value"));
    }
    Ok(value)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod pack_codec {
use crate::standards::v_rfc8259::subsets::base::schema::snapshot::*;
use crate::standards::v_rfc8259::subsets::base::io::binary::snapshot::owned_pack::*;
impl store::ArtifactDsl for JsonSnapshot{
 const EXTENSION:&'static str="json";
 fn envelope_id()->&'static str{"stdio.json"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
  let body=match store::semio_format::split_text_preamble(text){Ok((envelope,body))=>{if !envelope.matches_identity("stdio.json",store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("JSON logical text identity differs").to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))}body},Err(_)=>text};
  let record=semio_framework_dsl_record::parse_exact(body,&Snapshot::__dsl_spec(),&semio_framework_dsl_record::ParseOptions::default())?;
  JsonSnapshot::try_from(Snapshot::__dsl_from_record(&record)?).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
 }
 fn print_dsl(&self)->String{
  let body=semio_framework_dsl_record::print(&Snapshot::from(self).__dsl_to_record(),&Snapshot::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);
  let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.json",store::semio_format::Component::Dsl,1).expect("valid JSON identity");store::semio_format::wrap_text(&envelope,&body)
 }
}
}

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v_rfc8259::subsets::base::schema::snapshot::*;
use crate::STDIO_JSON_DOCUMENT_SCHEMA;
use semio_framework_diagnostic::TextSpan;
use framework_schema::ArtifactSchema;
use semio_framework_diagnostic::TextError;

/// 🚶️ Byte-cursor recursive-descent RFC8259 parser with 1-based line/column tracking for
/// `TextError` spans. Operates on the UTF-8 byte slice of a valid `&str` — multi-byte characters
/// inside string literals are re-assembled from their continuation bytes in [`Self::parse_string`].
pub(crate) struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
    line: u32,
    col: u32,
}

impl<'a> Parser<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn new(text: &'a str) -> Self {
        Self { bytes: text.as_bytes(), pos: 0, line: 1, col: 1 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn advance(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.pos += 1;
        if byte == b'\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(byte)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn span(&self) -> TextSpan {
        TextSpan::at(self.line, self.col)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn err(&self, message: impl Into<String>) -> TextError {
        TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message, self.span())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.advance();
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn expect(&mut self, byte: u8) -> Result<(), TextError> {
        match self.peek() {
            Some(b) if b == byte => {
                self.advance();
                Ok(())
            }
            Some(other) => Err(self.err(format!("expected '{}', found '{}'", byte as char, other as char))),
            None => Err(self.err(format!("expected '{}', found end of input", byte as char))),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_value(&mut self) -> Result<JsonValue, TextError> {
        self.skip_ws();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => Ok(JsonValue::String { value: self.parse_string()? }),
            Some(b't') => self.parse_literal("true", JsonValue::Bool { value: true }),
            Some(b'f') => self.parse_literal("false", JsonValue::Bool { value: false }),
            Some(b'n') => self.parse_literal("null", JsonValue::Null),
            Some(b'-') | Some(b'0'..=b'9') => self.parse_number(),
            Some(other) => Err(self.err(format!("unexpected character '{}'", other as char))),
            None => Err(self.err("unexpected end of input, expected a value")),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_literal(&mut self, literal: &str, value: JsonValue) -> Result<JsonValue, TextError> {
        for expected in literal.bytes() {
            match self.advance() {
                Some(b) if b == expected => {}
                _ => return Err(self.err(format!("expected literal '{literal}'"))),
            }
        }
        Ok(value)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_object(&mut self) -> Result<JsonValue, TextError> {
        self.expect(b'{')?;
        let mut members = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.advance();
            return Ok(JsonValue::Object { members });
        }
        loop {
            self.skip_ws();
            if self.peek() != Some(b'"') {
                return Err(self.err("expected a string member key"));
            }
            let key = self.parse_string()?;
            self.skip_ws();
            self.expect(b':')?;
            let value = self.parse_value()?;
            members.push(JsonMember { key, value });
            self.skip_ws();
            match self.peek() {
                Some(b',') => {
                    self.advance();
                }
                Some(b'}') => {
                    self.advance();
                    break;
                }
                Some(other) => return Err(self.err(format!("expected ',' or '}}', found '{}'", other as char))),
                None => return Err(self.err("unterminated object, expected ',' or '}'")),
            }
        }
        Ok(JsonValue::Object { members })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_array(&mut self) -> Result<JsonValue, TextError> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.advance();
            return Ok(JsonValue::Array { items });
        }
        loop {
            let value = self.parse_value()?;
            items.push(value);
            self.skip_ws();
            match self.peek() {
                Some(b',') => {
                    self.advance();
                }
                Some(b']') => {
                    self.advance();
                    break;
                }
                Some(other) => return Err(self.err(format!("expected ',' or ']', found '{}'", other as char))),
                None => return Err(self.err("unterminated array, expected ',' or ']'")),
            }
        }
        Ok(JsonValue::Array { items })
    }

    /// 🔤️ Parses a quoted string, decoding escapes (incl. `\uXXXX` surrogate pairs) into their
    /// literal characters — the LITERAL decoded value is stored (never the wire escape form), the
    /// same convention `stdio.xml`'s `XmlNode::Text` uses for entity decoding.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_string(&mut self) -> Result<String, TextError> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            match self.advance() {
                None => return Err(self.err("unterminated string literal")),
                Some(b'"') => break,
                Some(b'\\') => match self.advance() {
                    Some(b'"') => out.push('"'),
                    Some(b'\\') => out.push('\\'),
                    Some(b'/') => out.push('/'),
                    Some(b'b') => out.push('\u{0008}'),
                    Some(b'f') => out.push('\u{000C}'),
                    Some(b'n') => out.push('\n'),
                    Some(b'r') => out.push('\r'),
                    Some(b't') => out.push('\t'),
                    Some(b'u') => out.push(self.parse_unicode_escape()?),
                    Some(other) => return Err(self.err(format!("invalid escape sequence '\\{}'", other as char))),
                    None => return Err(self.err("unterminated escape sequence")),
                },
                Some(b) if b < 0x20 => return Err(self.err("unescaped control character in string literal")),
                Some(b) if b < 0x80 => out.push(b as char),
                Some(lead) => {
                    let extra = if lead >= 0xF0 {
                        3
                    } else if lead >= 0xE0 {
                        2
                    } else {
                        1
                    };
                    let mut buf = vec![lead];
                    for _ in 0..extra {
                        match self.advance() {
                            Some(cont) => buf.push(cont),
                            None => return Err(self.err("truncated UTF-8 sequence in string literal")),
                        }
                    }
                    let decoded = std::str::from_utf8(&buf).map_err(|_| self.err("invalid UTF-8 sequence in string literal"))?;
                    out.push_str(decoded);
                }
            }
        }
        Ok(out)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_unicode_escape(&mut self) -> Result<char, TextError> {
        let high = self.parse_hex4()?;
        if (0xD800..=0xDBFF).contains(&high) {
            if self.advance() != Some(b'\\') {
                return Err(self.err("expected low surrogate after high surrogate"));
            }
            if self.advance() != Some(b'u') {
                return Err(self.err("expected \\u low surrogate after high surrogate"));
            }
            let low = self.parse_hex4()?;
            if !(0xDC00..=0xDFFF).contains(&low) {
                return Err(self.err("invalid low surrogate"));
            }
            let combined = 0x10000 + ((high - 0xD800) << 10) + (low - 0xDC00);
            char::from_u32(combined).ok_or_else(|| self.err("invalid surrogate pair"))
        } else if (0xDC00..=0xDFFF).contains(&high) {
            Err(self.err("unpaired low surrogate"))
        } else {
            char::from_u32(high).ok_or_else(|| self.err("invalid \\u escape"))
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_hex4(&mut self) -> Result<u32, TextError> {
        let mut value = 0u32;
        for _ in 0..4 {
            let byte = self.advance().ok_or_else(|| self.err("unexpected end of input in \\u escape"))?;
            let digit = match byte {
                b'0'..=b'9' => (byte - b'0') as u32,
                b'a'..=b'f' => (byte - b'a' + 10) as u32,
                b'A'..=b'F' => (byte - b'A' + 10) as u32,
                _ => return Err(self.err("invalid hex digit in \\u escape")),
            };
            value = value * 16 + digit;
        }
        Ok(value)
    }

    /// 🔢️ Captures the ORIGINAL number lexeme verbatim per RFC8259 §6 grammar
    /// (`-? (0 | [1-9][0-9]*) (.[0-9]+)? ([eE][+-]?[0-9]+)?`) — never parsed into `f64`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_number(&mut self) -> Result<JsonValue, TextError> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.advance();
        }
        match self.peek() {
            Some(b'0') => {
                self.advance();
            }
            Some(b'1'..=b'9') => {
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.advance();
                }
            }
            _ => return Err(self.err("invalid number: expected a digit")),
        }
        if self.peek() == Some(b'.') {
            self.advance();
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.err("invalid number: expected a digit after '.'"));
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.advance();
            }
        }
        if matches!(self.peek(), Some(b'e') | Some(b'E')) {
            self.advance();
            if matches!(self.peek(), Some(b'+') | Some(b'-')) {
                self.advance();
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.err("invalid number: expected a digit in exponent"));
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.advance();
            }
        }
        let lexeme = std::str::from_utf8(&self.bytes[start..self.pos]).expect("ascii number lexeme is valid utf-8").to_string();
        Ok(JsonValue::Number { lexeme })
    }
}

/// 🔒️ Compact (no extraneous whitespace) RFC8259 serialization — used for the `pack` (binary)
/// representation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_json_text(value: &JsonValue) -> String {
    let mut out = String::new();
    write_value_compact(value, &mut out);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_value_compact(value: &JsonValue, out: &mut String) { write_value_iterative(value, out, false, 0); }

pub(crate) enum JsonWrite<'a> { Value(&'a JsonValue, usize), String(&'a str), Literal(&'static str), Indent(usize) }

pub(crate) fn write_value_iterative(value: &JsonValue, out: &mut String, pretty: bool, depth: usize) {
    let mut stack = vec![JsonWrite::Value(value, depth)];
    while let Some(action) = stack.pop() {
        match action {
            JsonWrite::String(value) => write_string_escaped(value, out),
            JsonWrite::Literal(value) => out.push_str(value),
            JsonWrite::Indent(depth) => push_indent(out, depth),
            JsonWrite::Value(value, depth) => match value {
                JsonValue::Null => out.push_str("null"),
                JsonValue::Bool { value } => out.push_str(if *value { "true" } else { "false" }),
                JsonValue::Number { lexeme } => out.push_str(lexeme),
                JsonValue::String { value } => write_string_escaped(value, out),
                JsonValue::Array { items } => {
                    if items.is_empty() { out.push_str("[]"); continue; }
                    out.push_str(if pretty { "[\n" } else { "[" }); stack.push(JsonWrite::Literal("]"));
                    if pretty { stack.push(JsonWrite::Indent(depth)); }
                    for (index, item) in items.iter().enumerate().rev() {
                        if pretty { stack.push(JsonWrite::Literal("\n")); }
                        if index + 1 < items.len() { stack.push(JsonWrite::Literal(",")); }
                        stack.push(JsonWrite::Value(item, depth + 1));
                        if pretty { stack.push(JsonWrite::Indent(depth + 1)); }
                    }
                },
                JsonValue::Object { members } => {
                    if members.is_empty() { out.push_str("{}"); continue; }
                    out.push_str(if pretty { "{\n" } else { "{" }); stack.push(JsonWrite::Literal("}"));
                    if pretty { stack.push(JsonWrite::Indent(depth)); }
                    for (index, member) in members.iter().enumerate().rev() {
                        if pretty { stack.push(JsonWrite::Literal("\n")); }
                        if index + 1 < members.len() { stack.push(JsonWrite::Literal(",")); }
                        stack.push(JsonWrite::Value(&member.value, depth + 1));
                        stack.push(JsonWrite::Literal(if pretty { ": " } else { ":" })); stack.push(JsonWrite::String(&member.key));
                        if pretty { stack.push(JsonWrite::Indent(depth + 1)); }
                    }
                },
            },
        }
    }
}

/// 🎀️ 2-space-indented pretty print — used for the `dsl` (text-on-disk) representation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_json_pretty(value: &JsonValue) -> String {
    let mut out = String::new();
    write_value_pretty(value, &mut out, 0);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn push_indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_value_pretty(value: &JsonValue, out: &mut String, depth: usize) { write_value_iterative(value, out, true, depth); }

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_string_escaped(s: &str, out: &mut String) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}
}
pub use snapshot_wire_codec::*;
