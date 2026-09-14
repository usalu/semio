//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🧾️ Repository YAML: a bounded YAML data decoder and a deterministic YAML-compatible encoder.
//! Behaviour twin of `github.com/usalu/semio/repo/yaml`. The decoder accepts the JSON superset
//! first, exactly like the Go implementation, and falls back to the indentation scanner.

//#endregion 🧲️Header

use serde_json::{Map, Number, Value};

//#region 📥️Decoding

/// 📏️ One significant input line with its indentation depth.
#[derive(Debug, Clone)]
struct Line {
    indent: usize,
    text: String,
}

/// 📥️ Decodes a YAML (or JSON) document into the shared value tree.
pub fn unmarshal(data: &str) -> Result<Value, String> {
    if let Ok(value) = serde_json::from_str::<Value>(data) {
        return Ok(value);
    }
    let lines = scan(data)?;
    if lines.is_empty() {
        return Ok(Value::Object(Map::new()));
    }
    let indent = lines[0].indent;
    let (node, next) = parse_block(&lines, 0, indent)?;
    if next != lines.len() {
        return Err(format!("yaml contains an unexpected block at line {}", next + 1));
    }
    Ok(node)
}

/// 🔍️ Drops blanks, comments and document markers, and records each remaining line's indent.
fn scan(data: &str) -> Result<Vec<Line>, String> {
    let mut lines = Vec::new();
    for raw_line in data.split('\n') {
        let raw = raw_line.trim_end_matches([' ', '\t', '\r']);
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed == "---" || trimmed == "..." {
            continue;
        }
        let indent = raw.len() - raw.trim_start_matches(' ').len();
        if raw[..indent].contains('\t') {
            return Err("yaml tabs are not allowed".to_string());
        }
        lines.push(Line { indent, text: strip_comment(trimmed).trim().to_string() });
    }
    Ok(lines)
}

/// ✂️ Removes a trailing ` #` comment that is not inside a quoted scalar.
fn strip_comment(value: &str) -> &str {
    let bytes = value.as_bytes();
    let mut quoted = 0u8;
    for index in 0..bytes.len() {
        let current = bytes[index];
        if current == b'\'' || current == b'"' {
            if quoted == 0 {
                quoted = current;
            } else if quoted == current {
                quoted = 0;
            }
        }
        if current == b'#' && quoted == 0 && (index == 0 || bytes[index - 1] == b' ') {
            return value[..index].trim_end();
        }
    }
    value
}

/// 🧱️ Parses either a mapping or a sequence block at the given indentation.
fn parse_block(lines: &[Line], start: usize, indent: usize) -> Result<(Value, usize), String> {
    if start >= lines.len() {
        return Ok((Value::Object(Map::new()), start));
    }
    if lines[start].text.starts_with('-') {
        return parse_sequence(lines, start, indent);
    }
    let mut result = Map::new();
    let mut index = start;
    while index < lines.len() && lines[index].indent == indent && !lines[index].text.starts_with('-') {
        let Some((raw_key, raw_value)) = lines[index].text.split_once(':') else {
            return Err(format!("invalid yaml mapping at line {}", index + 1));
        };
        let key = raw_key.trim().to_string();
        if key.is_empty() {
            return Err(format!("invalid yaml mapping at line {}", index + 1));
        }
        let raw = raw_value.trim().to_string();
        index += 1;
        if raw.is_empty() {
            if index < lines.len() && lines[index].indent > indent {
                let child_indent = lines[index].indent;
                let (child, next) = parse_block(lines, index, child_indent)?;
                result.insert(key, child);
                index = next;
            } else {
                result.insert(key, Value::Object(Map::new()));
            }
        } else {
            result.insert(key, parse_scalar(&raw));
        }
    }
    Ok((Value::Object(result), index))
}

/// 📃️ Parses a sequence block at the given indentation.
fn parse_sequence(lines: &[Line], start: usize, indent: usize) -> Result<(Value, usize), String> {
    let mut result: Vec<Value> = Vec::new();
    let mut index = start;
    while index < lines.len() && lines[index].indent == indent && lines[index].text.starts_with('-') {
        let raw = lines[index].text.trim_start_matches('-').trim().to_string();
        index += 1;
        if raw.is_empty() {
            if index < lines.len() && lines[index].indent > indent {
                let child_indent = lines[index].indent;
                let (child, next) = parse_block(lines, index, child_indent)?;
                result.push(child);
                index = next;
            } else {
                result.push(Value::Null);
            }
            continue;
        }
        match raw.split_once(':') {
            Some((key, value)) => {
                let mut object = Map::new();
                object.insert(key.trim().to_string(), parse_scalar(value.trim()));
                if index < lines.len() && lines[index].indent > indent {
                    let child_indent = lines[index].indent;
                    let (child, next) = parse_block(lines, index, child_indent)?;
                    if let Value::Object(fields) = child {
                        for (child_key, child_value) in fields {
                            object.insert(child_key, child_value);
                        }
                    }
                    index = next;
                }
                result.push(Value::Object(object));
            }
            None => result.push(parse_scalar(&raw)),
        }
    }
    Ok((Value::Array(result), index))
}

/// 🔤️ Resolves one scalar token to a quoted string, flow sequence, boolean, null or number.
fn parse_scalar(raw: &str) -> Value {
    if raw.is_empty() {
        return Value::String(String::new());
    }
    let bytes = raw.as_bytes();
    let quoted = bytes.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''));
    if quoted {
        if bytes[0] == b'\'' {
            return Value::String(raw[1..raw.len() - 1].replace("''", "'"));
        }
        if let Ok(Value::String(decoded)) = serde_json::from_str::<Value>(raw) {
            return Value::String(decoded);
        }
    }
    if raw.starts_with('[') && raw.ends_with(']') {
        let body = raw[1..raw.len() - 1].trim();
        if body.is_empty() {
            return Value::Array(Vec::new());
        }
        return Value::Array(body.split(',').map(|part| parse_scalar(part.trim())).collect());
    }
    match raw.to_lowercase().as_str() {
        "true" => return Value::Bool(true),
        "false" => return Value::Bool(false),
        "null" | "~" => return Value::Null,
        _ => {}
    }
    if let Ok(value) = raw.parse::<i64>() {
        return Value::Number(Number::from(value));
    }
    if let Ok(value) = raw.parse::<f64>() {
        if let Some(number) = Number::from_f64(value) {
            return Value::Number(number);
        }
    }
    Value::String(raw.to_string())
}

//#endregion 📥️Decoding

//#region 📤️Encoding

/// 📤️ Encodes a value tree as a deterministic, key-sorted YAML document.
pub fn marshal(value: &Value) -> Result<String, String> {
    let mut output = String::new();
    emit(&mut output, value, 0);
    Ok(output)
}

/// ✍️ Writes one node at the given indentation.
fn emit(output: &mut String, value: &Value, indent: usize) {
    let prefix = " ".repeat(indent);
    match value {
        Value::Object(fields) => {
            let mut keys: Vec<&String> = fields.keys().collect();
            keys.sort();
            for key in keys {
                let child = &fields[key];
                if is_container(child) {
                    output.push_str(&format!("{prefix}{key}:\n"));
                    emit(output, child, indent + 2);
                } else {
                    output.push_str(&format!("{prefix}{key}: {}\n", scalar_string(child)));
                }
            }
        }
        Value::Array(items) => {
            for child in items {
                if is_container(child) {
                    output.push_str(&format!("{prefix}-\n"));
                    emit(output, child, indent + 2);
                } else {
                    output.push_str(&format!("{prefix}- {}\n", scalar_string(child)));
                }
            }
        }
        other => output.push_str(&format!("{prefix}{}\n", scalar_string(other))),
    }
}

/// 📦️ Reports whether a node is emitted as an indented block.
fn is_container(value: &Value) -> bool {
    matches!(value, Value::Object(_) | Value::Array(_))
}

/// 🔤️ Renders one scalar, quoting it whenever a bare form would be ambiguous.
fn scalar_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(text) => {
            if text.is_empty() {
                return "\"\"".to_string();
            }
            let ambiguous = text.chars().any(|current| ":#{}[],&*!|>'\"%@`\n\r\t".contains(current));
            if ambiguous || text.trim() != text {
                return serde_json::to_string(text).unwrap_or_else(|_| text.clone());
            }
            text.clone()
        }
        other => other.to_string(),
    }
}

//#endregion 📤️Encoding

//#region 🎯️Canonical

/// 🎯️ Renders a value as canonical JSON: members sorted by key, integers without a fractional part,
/// no insignificant whitespace. This is the shape the language-agnostic cases compare, so no host
/// language contributes number or ordering artefacts of its own.
pub fn canonical_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|error| format!("!{error}"))
}

/// 📥️ Decodes a document and renders the result as canonical JSON.
pub fn decode_to_canonical_json(source: &str) -> Result<String, String> {
    Ok(canonical_json(&unmarshal(source)?))
}

/// 🔁️ Decodes, re-encodes with the owned encoder, decodes again, and renders the result.
pub fn round_trip_to_canonical_json(source: &str) -> Result<String, String> {
    let decoded = unmarshal(source)?;
    let encoded = marshal(&decoded)?;
    Ok(canonical_json(&unmarshal(&encoded)?))
}

//#endregion 🎯️Canonical

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_mapping_and_sequence_decode() {
        let value = unmarshal("name: semio\npaths:\n  - a\n  - b\nenabled: true\n").unwrap();
        assert_eq!(value["name"], Value::String("semio".to_string()));
        assert_eq!(value["paths"], serde_json::json!(["a", "b"]));
        assert_eq!(value["enabled"], Value::Bool(true));
    }

    #[test]
    fn json_is_accepted_as_a_superset() {
        let value = unmarshal("{\"a\": 1}").unwrap();
        assert_eq!(value["a"], serde_json::json!(1));
    }

    #[test]
    fn encoding_sorts_keys_and_quotes_ambiguous_scalars() {
        let value = serde_json::json!({"b": 1, "a": "x: y"});
        assert_eq!(marshal(&value).unwrap(), "a: \"x: y\"\nb: 1\n");
    }

    #[test]
    fn tabs_are_not_indentation_and_produce_siblings() {
        let value = unmarshal("a:\n\tb: 1\n").unwrap();
        assert_eq!(value["b"], serde_json::json!(1));
    }

    #[test]
    fn round_trip_is_stable() {
        let source = "enabled: true\nname: semio\npaths:\n  - a\n  - b\n";
        let decoded = unmarshal(source).unwrap();
        assert_eq!(marshal(&decoded).unwrap(), source);
    }
}

//#endregion 🧪️Tests
