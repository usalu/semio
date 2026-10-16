//! 🔏️ Digest and JSON: the hand-rolled SHA-256 and the ordered JSON value with Go-compatible encoding.
//!
//! Holds the `🔏️Digest`, `🧾️Json` regions of the crate root; the root re-exports its items.

//#region 🔏️Digest

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6,
    0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb,
    0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee,
    0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// 🔏️ Hand-rolled SHA-256 (FIPS 180-4). The event chain must not depend on an external crate, so the
/// digest is owned here and cross-checked against Node's `crypto` in the language-agnostic tests.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut state: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut padded = data.to_vec();
    let bits = (data.len() as u64).wrapping_mul(8);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bits.to_be_bytes());
    let mut schedule = [0u32; 64];
    for block in padded.as_chunks::<64>().0 {
        for index in 0..16 {
            schedule[index] = u32::from_be_bytes([block[index * 4], block[index * 4 + 1], block[index * 4 + 2], block[index * 4 + 3]]);
        }
        for index in 16..64 {
            let previous = schedule[index - 15];
            let ahead = schedule[index - 2];
            let sigma0 = previous.rotate_right(7) ^ previous.rotate_right(18) ^ (previous >> 3);
            let sigma1 = ahead.rotate_right(17) ^ ahead.rotate_right(19) ^ (ahead >> 10);
            schedule[index] = schedule[index - 16].wrapping_add(sigma0).wrapping_add(schedule[index - 7]).wrapping_add(sigma1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for index in 0..64 {
            let big1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(big1).wrapping_add(choose).wrapping_add(SHA256_K[index]).wrapping_add(schedule[index]);
            let big0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = big0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
    let mut digest = [0u8; 32];
    for (index, word) in state.iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

/// 🔤️ Lowercase hex rendering of [`sha256`].
pub fn sha256_hex(data: &[u8]) -> String {
    let mut text = String::with_capacity(64);
    for byte in sha256(data) {
        text.push(char::from_digit((byte >> 4) as u32, 16).unwrap_or('0'));
        text.push(char::from_digit((byte & 0x0f) as u32, 16).unwrap_or('0'));
    }
    text
}

//#endregion 🔏️Digest

//#region 🧾️Json

/// 🧾️ An ordered JSON value written exactly the way Go's `encoding/json` writes it, so both
/// implementations produce byte-identical wire frames and byte-identical event hashes.
#[derive(Clone, Debug, PartialEq)]
pub enum J {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
    Raw(String),
}

impl J {
    /// 🏷️ Builds an object from already-ordered members, dropping the ones a Go `omitempty` tag would omit.
    pub fn object(members: Vec<(&str, Option<J>)>) -> J {
        J::Obj(members.into_iter().filter_map(|(name, value)| value.map(|value| (name.to_string(), value))).collect())
    }

    /// ✍️ Renders the value into Go's exact output form.
    pub fn write(&self, out: &mut String) {
        match self {
            J::Null => out.push_str("null"),
            J::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
            J::Int(value) => out.push_str(&value.to_string()),
            J::UInt(value) => out.push_str(&value.to_string()),
            J::Float(value) => out.push_str(&go_float(*value)),
            J::Str(value) => go_string(value, out),
            J::Arr(items) => {
                out.push('[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    item.write(out);
                }
                out.push(']');
            }
            J::Obj(members) => {
                out.push('{');
                for (index, (name, value)) in members.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    go_string(name, out);
                    out.push(':');
                    value.write(out);
                }
                out.push('}');
            }
            J::Raw(text) => out.push_str(text),
        }
    }

    /// 📤️ Renders the value into a fresh string.
    pub fn text(&self) -> String {
        let mut out = String::new();
        self.write(&mut out);
        out
    }

    /// 🔁️ Adopts an arbitrary parsed value, sorting object keys the way Go sorts map keys.
    pub fn from_value(value: &serde_json::Value) -> J {
        match value {
            serde_json::Value::Null => J::Null,
            serde_json::Value::Bool(inner) => J::Bool(*inner),
            serde_json::Value::Number(number) => J::Raw(number.to_string()),
            serde_json::Value::String(inner) => J::Str(inner.clone()),
            serde_json::Value::Array(items) => J::Arr(items.iter().map(J::from_value).collect()),
            serde_json::Value::Object(members) => J::Obj(members.iter().map(|(name, value)| (name.clone(), J::from_value(value))).collect()),
        }
    }
}

/// 🔢️ Formats a float the way Go's encoder does for the magnitudes this protocol carries.
fn go_float(value: f64) -> String {
    if value.is_finite() && value.fract() == 0.0 && value.abs() < 1e21 {
        return format!("{}", value as i64);
    }
    let text = format!("{value}");
    if text.contains('e') {
        return text.replace('e', "e+").replace("e+-", "e-");
    }
    text
}

/// 🔤️ Escapes a string exactly as Go's `encoding/json` does, HTML escaping included.
pub(super) fn go_string(value: &str, out: &mut String) {
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            other if (other as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", other as u32)),
            other => out.push(other),
        }
    }
    out.push('"');
}

/// 🗜️ Strips insignificant whitespace and applies Go's HTML escaping to an already-valid JSON document.
pub fn compact(raw: &[u8]) -> String {
    let text = String::from_utf8_lossy(raw);
    let mut out = String::with_capacity(text.len());
    let mut in_string = false;
    let mut escaped = false;
    for character in text.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
        } else if character == '"' {
            in_string = true;
        } else if character == ' ' || character == '\t' || character == '\n' || character == '\r' {
            continue;
        }
        match character {
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            other => out.push(other),
        }
    }
    out
}

//#endregion 🧾️Json
