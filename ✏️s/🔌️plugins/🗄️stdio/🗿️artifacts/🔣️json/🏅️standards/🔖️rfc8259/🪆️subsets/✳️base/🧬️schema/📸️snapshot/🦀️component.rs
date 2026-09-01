//! 🧬️ JsonSnapshot schema — own `JsonValue` model + a from-scratch RFC8259 recursive-descent
//! parser/serializer. Preserves object-member INSERTION ORDER (`Vec<JsonMember>`, not a map) and
//! the ORIGINAL NUMBER LEXEME verbatim (rfc8259 allows arbitrary precision — never round-tripped
//! through `f64`). No `serde_json::Value` anywhere in this file.

use crate::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;
use dsl::TextSpan;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use store::TextError;

//#region 🔖️JsonModel
/// 🍃️ One `object` member, in source order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonMember {
    pub key: String,
    pub value: JsonValue,
}

/// 🌳 An RFC8259 JSON value. `Number` keeps the ORIGINAL LEXEME (never parsed to `f64` — rfc8259
/// permits arbitrary precision, so re-emitting a lossy `f64` round-trip would silently corrupt
/// real documents carrying e.g. 19-digit ids or high-precision decimals). `Object` is a `Vec` of
/// [`JsonMember`] (never a map) so decode->encode preserves member insertion order exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JsonValue {
    Null,
    // NOTE: every non-unit variant MUST be a struct variant (named field), never a bare tuple
    // variant — serde's internally-tagged (`tag = "kind"`) representation can only merge the tag
    // into map-shaped content; a tuple variant wrapping a non-map type (`bool`/`String`/`Vec<_>`)
    // compiles fine but fails at RUNTIME serialization ("can only flatten structs and maps").
    Bool { value: bool },
    Number { lexeme: String },
    String { value: String },
    Array { items: Vec<JsonValue> },
    Object { members: Vec<JsonMember> },
}

impl Default for JsonValue {
    fn default() -> Self {
        JsonValue::Null
    }
}

impl From<serde_json::Value> for JsonValue {
    fn from(v: serde_json::Value) -> Self {
        match v {
            serde_json::Value::Null => JsonValue::Null,
            serde_json::Value::Bool(b) => JsonValue::Bool { value: b },
            serde_json::Value::Number(n) => JsonValue::Number { lexeme: n.to_string() },
            serde_json::Value::String(s) => JsonValue::String { value: s },
            serde_json::Value::Array(arr) => JsonValue::Array { items: arr.into_iter().map(JsonValue::from).collect() },
            serde_json::Value::Object(map) => JsonValue::Object { members: map.into_iter().map(|(k, v)| JsonMember { key: k, value: JsonValue::from(v) }).collect() },
        }
    }
}

impl From<&serde_json::Value> for JsonValue {
    fn from(v: &serde_json::Value) -> Self {
        match v {
            serde_json::Value::Null => JsonValue::Null,
            serde_json::Value::Bool(b) => JsonValue::Bool { value: *b },
            serde_json::Value::Number(n) => JsonValue::Number { lexeme: n.to_string() },
            serde_json::Value::String(s) => JsonValue::String { value: s.clone() },
            serde_json::Value::Array(arr) => JsonValue::Array { items: arr.iter().map(JsonValue::from).collect() },
            serde_json::Value::Object(map) => JsonValue::Object { members: map.iter().map(|(k, v)| JsonMember { key: k.clone(), value: JsonValue::from(v) }).collect() },
        }
    }
}

impl From<JsonValue> for serde_json::Value {
    fn from(v: JsonValue) -> Self {
        match v {
            JsonValue::Null => serde_json::Value::Null,
            JsonValue::Bool { value } => serde_json::Value::Bool(value),
            JsonValue::Number { lexeme } => {
                if let Ok(n) = lexeme.parse::<serde_json::Number>() {
                    serde_json::Value::Number(n)
                } else if let Ok(val) = serde_json::from_str::<serde_json::Value>(&lexeme) {
                    val
                } else {
                    serde_json::Value::String(lexeme)
                }
            }
            JsonValue::String { value } => serde_json::Value::String(value),
            JsonValue::Array { items } => serde_json::Value::Array(items.into_iter().map(serde_json::Value::from).collect()),
            JsonValue::Object { members } => {
                let mut map = serde_json::Map::with_capacity(members.len());
                for m in members {
                    map.insert(m.key, serde_json::Value::from(m.value));
                }
                serde_json::Value::Object(map)
            }
        }
    }
}

impl From<&JsonValue> for serde_json::Value {
    fn from(v: &JsonValue) -> Self {
        match v {
            JsonValue::Null => serde_json::Value::Null,
            JsonValue::Bool { value } => serde_json::Value::Bool(*value),
            JsonValue::Number { lexeme } => {
                if let Ok(n) = lexeme.parse::<serde_json::Number>() {
                    serde_json::Value::Number(n)
                } else if let Ok(val) = serde_json::from_str::<serde_json::Value>(lexeme) {
                    val
                } else {
                    serde_json::Value::String(lexeme.clone())
                }
            }
            JsonValue::String { value } => serde_json::Value::String(value.clone()),
            JsonValue::Array { items } => serde_json::Value::Array(items.iter().map(serde_json::Value::from).collect()),
            JsonValue::Object { members } => {
                let mut map = serde_json::Map::with_capacity(members.len());
                for m in members {
                    map.insert(m.key.clone(), serde_json::Value::from(&m.value));
                }
                serde_json::Value::Object(map)
            }
        }
    }
}
//#endregion 🔖️JsonModel

//#region 🔖️Parser
/// 🚶️ Byte-cursor recursive-descent RFC8259 parser with 1-based line/column tracking for
/// `TextError` spans. Operates on the UTF-8 byte slice of a valid `&str` — multi-byte characters
/// inside string literals are re-assembled from their continuation bytes in [`Self::parse_string`].
struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
    line: u32,
    col: u32,
}

impl<'a> Parser<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(text: &'a str) -> Self {
        Self { bytes: text.as_bytes(), pos: 0, line: 1, col: 1 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn advance(&mut self) -> Option<u8> {
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
    fn span(&self) -> TextSpan {
        TextSpan::at(self.line, self.col)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn err(&self, message: impl Into<String>) -> TextError {
        TextError::new(message, self.span())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.advance();
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn expect(&mut self, byte: u8) -> Result<(), TextError> {
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
    fn parse_value(&mut self) -> Result<JsonValue, TextError> {
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
    fn parse_literal(&mut self, literal: &str, value: JsonValue) -> Result<JsonValue, TextError> {
        for expected in literal.bytes() {
            match self.advance() {
                Some(b) if b == expected => {}
                _ => return Err(self.err(format!("expected literal '{literal}'"))),
            }
        }
        Ok(value)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_object(&mut self) -> Result<JsonValue, TextError> {
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
    fn parse_array(&mut self) -> Result<JsonValue, TextError> {
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
    fn parse_string(&mut self) -> Result<String, TextError> {
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
    fn parse_unicode_escape(&mut self) -> Result<char, TextError> {
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
    fn parse_hex4(&mut self) -> Result<u32, TextError> {
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
    fn parse_number(&mut self) -> Result<JsonValue, TextError> {
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
//#endregion 🔖️Parser

//#region 🔖️Serializer
/// 🔒️ Compact (no extraneous whitespace) RFC8259 serialization — used for the `pack` (binary)
/// representation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_json_text(value: &JsonValue) -> String {
    let mut out = String::new();
    write_value_compact(value, &mut out);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_value_compact(value: &JsonValue, out: &mut String) {
    match value {
        JsonValue::Null => out.push_str("null"),
        JsonValue::Bool { value: true } => out.push_str("true"),
        JsonValue::Bool { value: false } => out.push_str("false"),
        JsonValue::Number { lexeme } => out.push_str(lexeme),
        JsonValue::String { value: s } => write_string_escaped(s, out),
        JsonValue::Array { items } => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_value_compact(item, out);
            }
            out.push(']');
        }
        JsonValue::Object { members } => {
            out.push('{');
            for (i, member) in members.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_string_escaped(&member.key, out);
                out.push(':');
                write_value_compact(&member.value, out);
            }
            out.push('}');
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
fn push_indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_value_pretty(value: &JsonValue, out: &mut String, depth: usize) {
    match value {
        JsonValue::Array { items } if !items.is_empty() => {
            out.push_str("[\n");
            for (i, item) in items.iter().enumerate() {
                push_indent(out, depth + 1);
                write_value_pretty(item, out, depth + 1);
                if i + 1 < items.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            push_indent(out, depth);
            out.push(']');
        }
        JsonValue::Array { items: _ } => out.push_str("[]"),
        JsonValue::Object { members } if !members.is_empty() => {
            out.push_str("{\n");
            for (i, member) in members.iter().enumerate() {
                push_indent(out, depth + 1);
                write_string_escaped(&member.key, out);
                out.push_str(": ");
                write_value_pretty(&member.value, out, depth + 1);
                if i + 1 < members.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            push_indent(out, depth);
            out.push('}');
        }
        JsonValue::Object { members: _ } => out.push_str("{}"),
        other => write_value_compact(other, out),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_string_escaped(s: &str, out: &mut String) {
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
//#endregion 🔖️Serializer

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.json` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.json")]
pub struct JsonSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub value: JsonValue,
}

impl Default for JsonSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: JsonValue::Null }
    }
}

impl JsonSnapshot {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_value(value: impl Into<JsonValue>) -> Self {
        Self { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: value.into() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_serde_value(&self) -> serde_json::Value {
        serde_json::Value::from(&self.value)
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for JsonSnapshot {
    const EXTENSION: &'static str = "json";
    fn envelope_id() -> &'static str {
        "stdio.json"
    }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let value = parse_json_text(body.trim())?;
        Ok(Self { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
    }
    fn print_dsl(&self) -> String {
        let body = write_json_pretty(&self.value);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for JsonSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_json_text(&self.value).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let text = std::str::from_utf8(&inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let value = parse_json_text(text).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — pure document helper, no engine needed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_json_snapshot() -> JsonSnapshot {
    JsonSnapshot::default()
}

/// 📄️ The demo `stdio.json` document — a genuinely 3-level-nested `JsonValue` (object → array,
/// object → object → object → array) exercising every `JsonValue` variant (`Null`/`Bool`/`Number`/
/// `String`/`Array`/`Object`) at least once. The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law` in
/// `../💡️inferences/🦀️component.rs`'s `conformance_laws`) and for
/// `nontrivial_nested_value_round_trip` below, which calls this instead of duplicating the literal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_json_snapshot() -> JsonSnapshot {
    let value = JsonValue::Object {
        members: vec![
            JsonMember { key: "name".into(), value: JsonValue::String { value: "semio".into() } },
            JsonMember { key: "count".into(), value: JsonValue::Number { lexeme: "42".into() } },
            JsonMember { key: "ratio".into(), value: JsonValue::Number { lexeme: "3.5".into() } },
            JsonMember { key: "active".into(), value: JsonValue::Bool { value: true } },
            JsonMember { key: "missing".into(), value: JsonValue::Null },
            JsonMember { key: "tags".into(), value: JsonValue::Array { items: vec![JsonValue::String { value: "a".into() }, JsonValue::String { value: "b".into() }, JsonValue::String { value: "c".into() }] } },
            JsonMember {
                key: "nested".into(),
                value: JsonValue::Object {
                    members: vec![JsonMember {
                        key: "deep".into(),
                        value: JsonValue::Object {
                            members: vec![JsonMember { key: "deeper".into(), value: JsonValue::Array { items: vec![JsonValue::Number { lexeme: "1".into() }, JsonValue::Number { lexeme: "2".into() }, JsonValue::Number { lexeme: "3".into() }] } }],
                        },
                    }],
                },
            },
        ],
    };
    JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn obj(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
        JsonValue::Object { members: pairs.into_iter().map(|(k, v)| JsonMember { key: k.into(), value: v }).collect() }
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_all_scalar_kinds() {
        assert_eq!(parse_json_text("null").unwrap(), JsonValue::Null);
        assert_eq!(parse_json_text("true").unwrap(), JsonValue::Bool { value: true });
        assert_eq!(parse_json_text("false").unwrap(), JsonValue::Bool { value: false });
        assert_eq!(parse_json_text("\"hi\"").unwrap(), JsonValue::String { value: "hi".into() });
        assert_eq!(parse_json_text("42").unwrap(), JsonValue::Number { lexeme: "42".into() });
    }

    #[semio_framework_async_macros::async_test]
    async fn preserves_number_lexeme_verbatim() {
        for lexeme in ["0", "-0", "3.140", "1e10", "1E+10", "-1.5e-3", "9007199254740993", "100000000000000000000000000000"] {
            let value = parse_json_text(lexeme).unwrap();
            assert_eq!(value, JsonValue::Number { lexeme: lexeme.into() });
            assert_eq!(write_json_text(&value), lexeme);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_leading_zero_number() {
        assert!(parse_json_text("01").is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn preserves_object_member_insertion_order() {
        let value = parse_json_text(r#"{"z": 1, "a": 2, "m": 3}"#).unwrap();
        match &value {
            JsonValue::Object { members } => {
                let keys: Vec<&str> = members.iter().map(|m| m.key.as_str()).collect();
                assert_eq!(keys, vec!["z", "a", "m"]);
            }
            _ => panic!("expected object"),
        }
        assert_eq!(write_json_text(&value), r#"{"z":1,"a":2,"m":3}"#);
    }

    #[semio_framework_async_macros::async_test]
    async fn decodes_string_escapes_incl_surrogate_pair() {
        let value = parse_json_text(r#""a\tb\nc\"\\ A 😀""#).unwrap();
        assert_eq!(value, JsonValue::String { value: "a\tb\nc\"\\ A 😀".into() });
    }

    #[semio_framework_async_macros::async_test]
    async fn nested_structure_round_trips() {
        let text = r#"{"name":"semio","count":42,"ratio":3.5,"active":true,"missing":null,"tags":["a","b","c"],"nested":{"deep":{"deeper":[1,2,3]}}}"#;
        let value = parse_json_text(text).unwrap();
        assert_eq!(write_json_text(&value), text);
        let pretty = write_json_pretty(&value);
        let reparsed = parse_json_text(&pretty).unwrap();
        assert_eq!(reparsed, value);
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_matches_schema() {
        let snapshot = JsonSnapshot::default();
        assert_eq!(snapshot.schema, STDIO_JSON_DOCUMENT_SCHEMA);
        assert_eq!(snapshot.value, JsonValue::Null);
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_dsl_and_pack_round_trip() {
        let snapshot = JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: obj(vec![("a", JsonValue::Number { lexeme: "1".into() }), ("b", JsonValue::Array { items: vec![JsonValue::Bool { value: true }, JsonValue::Null] })]) };
        let text = store::ArtifactDsl::print_dsl(&snapshot);
        let parsed = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed, snapshot);
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let decoded = <JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snapshot);
    }

    // 🦑 Dissolved out of the former `⚙️engine`'s own test region (ticket
    // 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). `empty_snapshot_matches_schema`'s
    // former assertion (`empty_json_snapshot().schema == STDIO_JSON_DOCUMENT_SCHEMA`) already
    // survives via `empty_snapshot_matches_schema` above (same fact, `JsonSnapshot::default()`).

    #[semio_framework_async_macros::async_test]
    async fn codec_round_trip() {
        let snap = empty_json_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    #[semio_framework_async_macros::async_test]
    async fn nontrivial_nested_value_round_trip() {
        let snap = demo_json_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.value, snap.value);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded.value, snap.value);
    }
}
//#endregion 🧪️Tests
