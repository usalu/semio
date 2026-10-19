//! 📤️ JSON writer and UTF-8 of the sampling vocabulary: the JSON writer and the UTF-8 sequence length and status. See [`super`].

use super::*;

// #region 🔖️Text
// #region 🔖️Json
/// 📜️ Writes a [`JsonValue`] in compact form with correct string escaping and round-trippable
/// float formatting (integral floats print without a trailing `.0`-free exponent surprise).
pub async fn write_json(value: &JsonValue) -> String {
    let mut out = String::new();
    write_json_into(value, &mut out).await;
    out
}

async fn write_json_into(value: &JsonValue, out: &mut String) {
    match value {
        JsonValue::Null => out.push_str("null"),
        JsonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        JsonValue::Num(n) => write_json_number(*n, out).await,
        JsonValue::Str(s) => write_json_string(s, out).await,
        JsonValue::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                Box::pin(write_json_into(item, out)).await;
            }
            out.push(']');
        }
        JsonValue::Object(entries) => {
            out.push('{');
            for (i, (key, item)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_json_string(key, out).await;
                out.push(':');
                Box::pin(write_json_into(item, out)).await;
            }
            out.push('}');
        }
    }
}

async fn write_json_number(n: f64, out: &mut String) {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        out.push_str(&format!("{}", n as i64));
    } else {
        out.push_str(&format!("{n}"));
    }
}

async fn write_json_string(s: &str, out: &mut String) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}
// #endregion 🔖️Json

// #region 🔖️Utf8
/// 📜️ How much of a byte sequence forms valid UTF-8, for incremental decoding of streamed token
/// bytes that may end mid-codepoint.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Utf8Status {
    /// 📜️ The whole slice is valid, complete UTF-8.
    Complete,
    /// 📜️ The slice ends with a valid-so-far partial codepoint needing `more` additional bytes.
    Partial { more: usize },
    /// 📜️ The slice contains a byte sequence that can never become valid UTF-8.
    Invalid,
}

/// 📜️ Length of the UTF-8 sequence a leading byte starts, or `None` if `byte` cannot lead one.
pub async fn utf8_sequence_len(byte: u8) -> Option<usize> {
    match byte {
        0x00..=0x7F => Some(1),
        0xC2..=0xDF => Some(2),
        0xE0..=0xEF => Some(3),
        0xF0..=0xF4 => Some(4),
        _ => None,
    }
}

/// 📜️ Classifies the tail of `bytes` as complete, valid-partial, or invalid UTF-8. Used to decide
/// how many trailing bytes of a just-emitted token must be held back until more bytes arrive.
pub async fn utf8_status(bytes: &[u8]) -> Utf8Status {
    if bytes.is_empty() {
        return Utf8Status::Complete;
    }
    let mut start = bytes.len() - 1;
    let mut lead_len = None;
    for _ in 0..4 {
        if let Some(len) = utf8_sequence_len(bytes[start]).await {
            lead_len = Some(len);
            break;
        }
        if start == 0 {
            return Utf8Status::Invalid;
        }
        start -= 1;
    }
    let Some(len) = lead_len else {
        return Utf8Status::Invalid;
    };
    let have = bytes.len() - start;
    if have > len {
        return Utf8Status::Invalid;
    }
    if have == len {
        return match core::str::from_utf8(&bytes[start..]) {
            Ok(_) => Utf8Status::Complete,
            Err(_) => Utf8Status::Invalid,
        };
    }
    Utf8Status::Partial { more: len - have }
}
// #endregion 🔖️Utf8
// #endregion 🔖️Text
