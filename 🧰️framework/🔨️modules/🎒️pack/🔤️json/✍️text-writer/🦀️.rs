//! ✍️ Text writer: compact and pretty serialization of values, strings and numbers. See [`super`].

use super::*;

//#region 🔖️Writer
/// 📝️ Writes `value` as compact JSON.
// 🚫️async: R9 pure in-memory format, no I/O.
pub fn to_string(value: &Value) -> String {
    let mut out = String::new();
    write_value(value, &mut out);
    out
}

/// 🪐️ Writes `value` as 2-space-indented JSON, matching `serde_json::to_string_pretty`'s own
/// layout (`": "` after object keys, one array/object member per line, no trailing newline) — the
/// `serde_json::to_string_pretty` replacement every human-facing JSON view (an example-document
/// viewer, an exported fixture, a rule inspector) still needs even after its call site stops
/// depending on `serde_json` for everything else.
// 🚫️async: R9 pure in-memory format, no I/O.
pub fn to_string_pretty(value: &Value) -> String {
    let mut out = String::new();
    write_value_pretty(value, &mut out, 0);
    out
}

fn write_indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn write_value_pretty(value: &Value, out: &mut String, depth: usize) {
    match value {
        Value::Array(items) if !items.is_empty() => {
            out.push_str("[\n");
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push_str(",\n");
                }
                write_indent(out, depth + 1);
                write_value_pretty(item, out, depth + 1);
            }
            out.push('\n');
            write_indent(out, depth);
            out.push(']');
        }
        Value::Object(object) if !object.is_empty() => {
            out.push_str("{\n");
            for (index, (key, value)) in object.iter().enumerate() {
                if index > 0 {
                    out.push_str(",\n");
                }
                write_indent(out, depth + 1);
                write_string(key, out);
                out.push_str(": ");
                write_value_pretty(value, out, depth + 1);
            }
            out.push('\n');
            write_indent(out, depth);
            out.push('}');
        }
        other => write_value(other, out),
    }
}

/// 🪞️ `value.to_string()`/`format!("{value}")`, mirroring `serde_json::Value`'s own `Display` —
/// every plugin call site that used to write `serde_json::json!({...}).to_string()` keeps
/// compiling unchanged against `pack::json!({...}).to_string()`.
impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&to_string(self))
    }
}

fn write_value(value: &Value, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(number) => write_number(*number, out),
        Value::String(text) => write_string(text, out),
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_value(item, out);
            }
            out.push(']');
        }
        Value::Object(object) => {
            out.push('{');
            for (index, (key, value)) in object.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_string(key, out);
                out.push(':');
                write_value(value, out);
            }
            out.push('}');
        }
    }
}

/// 🎛️ Escapes `"`, `\`, and control characters (`\b \f \n \r \t` shorthands, `\u00XX` otherwise);
/// everything else — including non-ASCII — passes through unescaped, matching `serde_json`'s
/// default (`ensure_ascii`-off) writer.
fn write_string(text: &str, out: &mut String) {
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if (control as u32) < 0x20 => {
                use fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", control as u32);
            }
            other => out.push(other),
        }
    }
    out.push('"');
}

fn write_number(number: Number, out: &mut String) {
    match number {
        Number::UInt(value) => {
            use fmt::Write as _;
            let _ = write!(out, "{value}");
        }
        Number::Int(value) => {
            use fmt::Write as _;
            let _ = write!(out, "{value}");
        }
        Number::Float(value) => write_float(value, out),
    }
}

/// 🔧️ `serde_json`'s own (`zmij`-based) fixed/exponential split for `f64`: fixed notation for
/// `-5 <= exponent <= 15` on the leading-digit decimal exponent, exponential otherwise — traced
/// by hand from `zmij 1.0.21`'s `write<Float>` (`~/.cargo/registry/…/zmij-1.0.21/src/lib.rs`,
/// the `(-5..=15).contains(&dec_exp)` guard) and confirmed against the pinned resolved version in
/// this workspace's own `Cargo.lock`. Not ECMA-262's `-6 <= e < 21` (that was this writer's prior,
/// wrong rule — ties in the last significant digit alone hid the boundary-case fanout it caused).
/// A whole-number float always gets an explicit `.0` in fixed notation so it never collapses onto
/// its integer twin on the wire. See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/📓️float-format-parity.md`.
pub(super) fn write_float(value: f64, out: &mut String) {
    write_float_to(value, out).expect("String accepts formatted JSON floats");
}
//#endregion 🔖️Writer
