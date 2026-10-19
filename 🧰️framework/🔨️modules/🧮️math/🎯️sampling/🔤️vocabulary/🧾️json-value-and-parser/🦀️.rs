//! 🧾️ JSON value and parser of the sampling vocabulary: the JSON value and the strict JSON parser. See [`super`].

use super::*;

// #region 🔖️Text
// #region 🔖️Json
/// 📜️ A parsed JSON value. Objects preserve insertion order via `Vec<(String, JsonValue)>` rather
/// than a `HashMap` — configs are small, order-preserving round-trips are nicer to diff, and this
/// keeps the whole module dependency-free (no hashing DoS surface to reason about either).
#[derive(Clone, PartialEq, Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    /// 📜️ Looks up a key in an object value; `None` for any other variant or a missing key.
    pub async fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            Self::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub async fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    pub async fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Num(n) => Some(*n),
            _ => None,
        }
    }

    pub async fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub async fn as_array(&self) -> Option<&[JsonValue]> {
        match self {
            Self::Array(items) => Some(items),
            _ => None,
        }
    }
}

/// 📜️ Recursive-descent JSON parser over `&str`, depth-capped by `max_depth` so a maliciously
/// nested document (`[[[[...`) cannot blow the call stack — this is the same nesting bound that
/// protects the JSON-Schema/grammar constraint machinery later in the file.
pub async fn parse_json(text: &str, max_depth: usize) -> Result<JsonValue, SamplingError> {
    let bytes = text.as_bytes();
    let mut pos = 0usize;
    let value = parse_json_value(bytes, &mut pos, 0, max_depth).await?;
    skip_json_whitespace(bytes, &mut pos).await;
    if pos != bytes.len() {
        return Err(SamplingError::Corrupted { reason: "trailing data after JSON value" });
    }
    Ok(value)
}

async fn skip_json_whitespace(bytes: &[u8], pos: &mut usize) {
    while *pos < bytes.len() && matches!(bytes[*pos], b' ' | b'\t' | b'\n' | b'\r') {
        *pos += 1;
    }
}

async fn parse_json_value(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    if depth > max_depth {
        return Err(SamplingError::LimitExceeded { limit: "max_json_depth" });
    }
    skip_json_whitespace(bytes, pos).await;
    let Some(&byte) = bytes.get(*pos) else {
        return Err(SamplingError::Corrupted { reason: "unexpected end of JSON input" });
    };
    match byte {
        b'n' => parse_json_literal(bytes, pos, "null", JsonValue::Null).await,
        b't' => parse_json_literal(bytes, pos, "true", JsonValue::Bool(true)).await,
        b'f' => parse_json_literal(bytes, pos, "false", JsonValue::Bool(false)).await,
        b'"' => parse_json_string(bytes, pos).await.map(JsonValue::Str),
        b'[' => parse_json_array(bytes, pos, depth, max_depth).await,
        b'{' => parse_json_object(bytes, pos, depth, max_depth).await,
        b'-' | b'0'..=b'9' => parse_json_number(bytes, pos).await,
        _ => Err(SamplingError::Corrupted { reason: "unexpected byte in JSON input" }),
    }
}

async fn parse_json_literal(bytes: &[u8], pos: &mut usize, literal: &str, value: JsonValue) -> Result<JsonValue, SamplingError> {
    let lit = literal.as_bytes();
    if bytes[*pos..].starts_with(lit) {
        *pos += lit.len();
        Ok(value)
    } else {
        Err(SamplingError::Corrupted { reason: "invalid JSON literal" })
    }
}

async fn parse_json_string(bytes: &[u8], pos: &mut usize) -> Result<String, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'"');
    *pos += 1;
    let mut out = String::new();
    loop {
        let &byte = bytes.get(*pos).ok_or(SamplingError::Corrupted { reason: "unterminated JSON string" })?;
        *pos += 1;
        match byte {
            b'"' => return Ok(out),
            b'\\' => {
                let &esc = bytes.get(*pos).ok_or(SamplingError::Corrupted { reason: "unterminated JSON escape" })?;
                *pos += 1;
                match esc {
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => out.push('/'),
                    b'n' => out.push('\n'),
                    b't' => out.push('\t'),
                    b'r' => out.push('\r'),
                    b'b' => out.push('\u{8}'),
                    b'f' => out.push('\u{c}'),
                    b'u' => {
                        let code = parse_json_unicode_escape(bytes, pos).await?;
                        out.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                    }
                    _ => return Err(SamplingError::Corrupted { reason: "invalid JSON escape" }),
                }
            }
            _ => {
                let start = *pos - 1;
                let ch_len = utf8_sequence_len(byte).await.ok_or(SamplingError::Corrupted { reason: "invalid UTF-8 in JSON string" })?;
                let end = start + ch_len;
                let slice = bytes.get(start..end).ok_or(SamplingError::Corrupted { reason: "truncated UTF-8 in JSON string" })?;
                let s = core::str::from_utf8(slice).map_err(|_| SamplingError::Corrupted { reason: "invalid UTF-8 in JSON string" })?;
                out.push_str(s);
                *pos = end;
            }
        }
    }
}

async fn parse_json_unicode_escape(bytes: &[u8], pos: &mut usize) -> Result<u32, SamplingError> {
    let hex = bytes.get(*pos..*pos + 4).ok_or(SamplingError::Corrupted { reason: "truncated unicode escape" })?;
    let s = core::str::from_utf8(hex).map_err(|_| SamplingError::Corrupted { reason: "invalid unicode escape" })?;
    let code = u32::from_str_radix(s, 16).map_err(|_| SamplingError::Corrupted { reason: "invalid unicode escape" })?;
    *pos += 4;
    Ok(code)
}

async fn parse_json_number(bytes: &[u8], pos: &mut usize) -> Result<JsonValue, SamplingError> {
    let start = *pos;
    if bytes.get(*pos) == Some(&b'-') {
        *pos += 1;
    }
    while bytes.get(*pos).is_some_and(u8::is_ascii_digit) {
        *pos += 1;
    }
    if bytes.get(*pos) == Some(&b'.') {
        *pos += 1;
        while bytes.get(*pos).is_some_and(u8::is_ascii_digit) {
            *pos += 1;
        }
    }
    if matches!(bytes.get(*pos), Some(b'e' | b'E')) {
        *pos += 1;
        if matches!(bytes.get(*pos), Some(b'+' | b'-')) {
            *pos += 1;
        }
        while bytes.get(*pos).is_some_and(u8::is_ascii_digit) {
            *pos += 1;
        }
    }
    let text = core::str::from_utf8(&bytes[start..*pos]).expect("ASCII number slice is valid UTF-8");
    text.parse::<f64>().map(JsonValue::Num).map_err(|_| SamplingError::Corrupted { reason: "invalid JSON number" })
}

async fn parse_json_array(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'[');
    *pos += 1;
    let mut items = Vec::new();
    skip_json_whitespace(bytes, pos).await;
    if bytes.get(*pos) == Some(&b']') {
        *pos += 1;
        return Ok(JsonValue::Array(items));
    }
    loop {
        items.push(Box::pin(parse_json_value(bytes, pos, depth + 1, max_depth)).await?);
        skip_json_whitespace(bytes, pos).await;
        match bytes.get(*pos) {
            Some(b',') => {
                *pos += 1;
            }
            Some(b']') => {
                *pos += 1;
                return Ok(JsonValue::Array(items));
            }
            _ => return Err(SamplingError::Corrupted { reason: "expected ',' or ']' in JSON array" }),
        }
    }
}

async fn parse_json_object(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'{');
    *pos += 1;
    let mut entries = Vec::new();
    skip_json_whitespace(bytes, pos).await;
    if bytes.get(*pos) == Some(&b'}') {
        *pos += 1;
        return Ok(JsonValue::Object(entries));
    }
    loop {
        skip_json_whitespace(bytes, pos).await;
        if bytes.get(*pos) != Some(&b'"') {
            return Err(SamplingError::Corrupted { reason: "expected string key in JSON object" });
        }
        let key = parse_json_string(bytes, pos).await?;
        skip_json_whitespace(bytes, pos).await;
        if bytes.get(*pos) != Some(&b':') {
            return Err(SamplingError::Corrupted { reason: "expected ':' in JSON object" });
        }
        *pos += 1;
        let value = Box::pin(parse_json_value(bytes, pos, depth + 1, max_depth)).await?;
        entries.push((key, value));
        skip_json_whitespace(bytes, pos).await;
        match bytes.get(*pos) {
            Some(b',') => {
                *pos += 1;
            }
            Some(b'}') => {
                *pos += 1;
                return Ok(JsonValue::Object(entries));
            }
            _ => return Err(SamplingError::Corrupted { reason: "expected ',' or '}' in JSON object" }),
        }
    }
}
// #endregion 🔖️Json
// #endregion 🔖️Text
