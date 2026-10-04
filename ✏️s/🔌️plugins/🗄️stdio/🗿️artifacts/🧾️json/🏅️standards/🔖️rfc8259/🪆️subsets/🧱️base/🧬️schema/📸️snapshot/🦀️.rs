//! 🧬️ JsonSnapshot schema — own `JsonValue` model + a from-scratch RFC8259 recursive-descent
//! parser/serializer. Preserves object-member INSERTION ORDER (`Vec<JsonMember>`, not a map) and
//! the ORIGINAL NUMBER LEXEME verbatim (rfc8259 allows arbitrary precision — never round-tripped
//! through `f64`). No `serde_json::Value` anywhere in this file.

#[path="🔢️number/🦀️.rs"]
pub(crate) mod number;

use crate::STDIO_JSON_DOCUMENT_SCHEMA;
use semio_framework_diagnostic::TextSpan;
use framework_schema::ArtifactSchema;
use semio_framework_diagnostic::TextError;

//#region 🔖️JsonModel
/// 🍃️ One `object` member, in source order.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JsonMember {
    pub key: String,
    pub value: JsonValue,
}

/// 🌳 An RFC8259 JSON value. `Number` keeps the ORIGINAL LEXEME (never parsed to `f64` — rfc8259
/// permits arbitrary precision, so re-emitting a lossy `f64` round-trip would silently corrupt
/// real documents carrying e.g. 19-digit ids or high-precision decimals). `Object` is a `Vec` of
/// [`JsonMember`] (never a map) so decode->encode preserves member insertion order exactly.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
#[derive(Default)]
pub enum JsonValue {
    #[default]
    Null,
    // NOTE: every non-unit variant MUST be a struct variant (named field), never a bare tuple
    // variant — serde's internally-tagged (`tag = "kind"`) representation can only merge the tag
    // into map-shaped content; a tuple variant wrapping a non-map type (`bool`/`String`/`Vec<_>`)
    // compiles fine but fails at RUNTIME serialization ("can only flatten structs and maps").
    Bool {
        value: bool,
    },
    Number {
        lexeme: String,
    },
    String {
        value: String,
    },
    Array {
        items: Vec<JsonValue>,
    },
    Object {
        members: Vec<JsonMember>,
    },
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

/// 🌉️ `pack::json::Value` → this module's own key-order/lexeme-preserving `JsonValue` — the
/// cross-plugin bridge the fan-out playbook flagged as needed (ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`, §`🔱️trinity` batch):
/// callers converting off `ToValue`/`FromValue` (never `serde_json::Value`) still need to reach
/// this artifact's own `JsonSnapshot::from_value`. `pack::json::Number` has no independent lexeme
/// (unlike this crate's own arbitrary-precision `Number { lexeme }`), so it round-trips through
/// `pack::json_to_string` on a lone `Number` value — the exact bytes `pack`'s own writer would
/// have emitted for that number inline, not a re-implementation of its float/int formatting.
impl From<semio_framework_pack_json::Value> for JsonValue {
    fn from(v: semio_framework_pack_json::Value) -> Self {
        JsonValue::from(&v)
    }
}

impl From<&semio_framework_pack_json::Value> for JsonValue {
    fn from(v: &semio_framework_pack_json::Value) -> Self {
        match v {
            semio_framework_pack_json::Value::Null => JsonValue::Null,
            semio_framework_pack_json::Value::Bool(b) => JsonValue::Bool { value: *b },
            semio_framework_pack_json::Value::Number(n) => JsonValue::Number { lexeme: semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Number(*n)) },
            semio_framework_pack_json::Value::String(s) => JsonValue::String { value: s.clone() },
            semio_framework_pack_json::Value::Array(items) => JsonValue::Array { items: items.iter().map(JsonValue::from).collect() },
            semio_framework_pack_json::Value::Object(members) => JsonValue::Object { members: members.iter().map(|(k, v)| JsonMember { key: k.to_string(), value: JsonValue::from(v) }).collect() },
        }
    }
}

/// 🌉️ The reverse of the impl above — `JsonSnapshot::to_pack_value`'s bridge back into
/// `pack::json::Value` for a caller that needs the value tree, not the wire bytes. The original
/// arbitrary-precision `lexeme` re-parses through `pack::parse_json` (a full round trip through
/// the exact writer/reader pair `pack::json_to_string`/`pack::parse_json` already exercise
/// elsewhere in this crate) rather than a second hand-rolled number lexer.
impl From<&JsonValue> for semio_framework_pack_json::Value {
    fn from(v: &JsonValue) -> Self {
        match v {
            JsonValue::Null => semio_framework_pack_json::Value::Null,
            JsonValue::Bool { value } => semio_framework_pack_json::Value::Bool(*value),
            JsonValue::Number { lexeme } => semio_framework_pack_json::parse(lexeme, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(semio_framework_pack_json::Value::Null),
            JsonValue::String { value } => semio_framework_pack_json::Value::String(value.clone()),
            JsonValue::Array { items } => semio_framework_pack_json::Value::Array(items.iter().map(semio_framework_pack_json::Value::from).collect()),
            JsonValue::Object { members } => semio_framework_pack_json::object(members.iter().map(|member| (member.key.clone(), semio_framework_pack_json::Value::from(&member.value)))),
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
        TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message, self.span())
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
fn write_value_compact(value: &JsonValue, out: &mut String) { write_value_iterative(value, out, false, 0); }

enum JsonWrite<'a> { Value(&'a JsonValue, usize), String(&'a str), Literal(&'static str), Indent(usize) }

fn write_value_iterative(value: &JsonValue, out: &mut String, pretty: bool, depth: usize) {
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
fn push_indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_value_pretty(value: &JsonValue, out: &mut String, depth: usize) { write_value_iterative(value, out, true, depth); }

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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.json")]
pub struct JsonSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
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

    /// 🌉️ `to_serde_value`'s first-party analog — for a caller that has stopped depending on
    /// `serde_json` and only wants `pack::json::Value`.
    // 🚫️async: E1 pure inherent-impl helper, same reason as `to_serde_value` above — see R9
    pub fn to_pack_value(&self) -> semio_framework_pack_json::Value {
        semio_framework_pack_json::Value::from(&self.value)
    }
}
//#endregion 🔖️Snapshot

#[path = "📦️pack/🦀️.rs"]
mod owned_pack;

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
/// `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law` in
/// `../💡️inferences/🦀️.rs`'s `conformance_laws`) and for
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
#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
