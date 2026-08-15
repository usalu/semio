//! 📖️ Deterministic text I/O for `s.stdio.gltf.inference`.

use serde::Serialize;
use serde_json::Value;
use std::{cmp::Ordering, fmt};

use crate::artifacts::gltf::schema::inferences::GltfInference;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 📨️Envelope
pub const GLTF_INFERENCE_SCHEMA: &str = "s.stdio.gltf.inference";
pub const GLTF_INFERENCE_SCHEMA_VERSION: u32 = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GltfInferenceTextError {
    Serialization(String),
    CarriageReturn,
    MissingPayload,
    Header { line: u8, expected: String, actual: String },
    LengthSyntax(String),
    LengthMismatch { declared: usize, actual: usize },
    ChecksumSyntax(String),
    ChecksumMismatch { declared: u32, actual: u32 },
    Json(String),
    NonCanonical,
}

impl fmt::Display for GltfInferenceTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialization(error) => write!(formatter, "inference serialization failed: {error}"),
            Self::CarriageReturn => formatter.write_str("text envelope must be LF-only"),
            Self::MissingPayload => formatter.write_str("text envelope is missing its payload"),
            Self::Header { line, expected, actual } => write!(formatter, "invalid header line {line}: expected {expected:?}, got {actual:?}"),
            Self::LengthSyntax(value) => write!(formatter, "invalid canonical payload length {value:?}"),
            Self::LengthMismatch { declared, actual } => write!(formatter, "payload length mismatch: declared {declared}, got {actual}"),
            Self::ChecksumSyntax(value) => write!(formatter, "invalid lowercase CRC-32 {value:?}"),
            Self::ChecksumMismatch { declared, actual } => write!(formatter, "payload CRC-32 mismatch: declared {declared:08x}, got {actual:08x}"),
            Self::Json(error) => write!(formatter, "invalid inference JSON: {error}"),
            Self::NonCanonical => formatter.write_str("inference JSON is not RFC 8785 canonical"),
        }
    }
}

impl std::error::Error for GltfInferenceTextError {}

pub fn encode_gltf_inference_text(value: &GltfInference) -> Result<String, GltfInferenceTextError> {
    let payload = canonical_json_bytes(value)?;
    let payload = String::from_utf8(payload).map_err(|error| GltfInferenceTextError::Serialization(error.to_string()))?;
    Ok(format!("schema {GLTF_INFERENCE_SCHEMA}\nversion {GLTF_INFERENCE_SCHEMA_VERSION}\nlength {}\nchecksum {:08x}\n{payload}", payload.len(), crc32_iso_hdlc(payload.as_bytes())))
}

pub fn decode_gltf_inference_text(input: &str) -> Result<GltfInference, GltfInferenceTextError> {
    if input.contains('\r') {
        return Err(GltfInferenceTextError::CarriageReturn);
    }
    let mut parts = input.splitn(5, '\n');
    check_header(1, parts.next(), &format!("schema {GLTF_INFERENCE_SCHEMA}"))?;
    check_header(2, parts.next(), &format!("version {GLTF_INFERENCE_SCHEMA_VERSION}"))?;
    let length_line = parts.next().ok_or(GltfInferenceTextError::MissingPayload)?;
    let checksum_line = parts.next().ok_or(GltfInferenceTextError::MissingPayload)?;
    let payload = parts.next().ok_or(GltfInferenceTextError::MissingPayload)?.as_bytes();
    let length_text = length_line.strip_prefix("length ").ok_or_else(|| GltfInferenceTextError::Header { line: 3, expected: "length <canonical-decimal>".into(), actual: length_line.into() })?;
    let length = length_text.parse::<usize>().map_err(|_| GltfInferenceTextError::LengthSyntax(length_text.into()))?;
    if length_text != length.to_string() {
        return Err(GltfInferenceTextError::LengthSyntax(length_text.into()));
    }
    if length != payload.len() {
        return Err(GltfInferenceTextError::LengthMismatch { declared: length, actual: payload.len() });
    }
    let checksum_text = checksum_line.strip_prefix("checksum ").ok_or_else(|| GltfInferenceTextError::Header { line: 4, expected: "checksum <eight-lowercase-hex>".into(), actual: checksum_line.into() })?;
    if checksum_text.len() != 8 || !checksum_text.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err(GltfInferenceTextError::ChecksumSyntax(checksum_text.into()));
    }
    let checksum = u32::from_str_radix(checksum_text, 16).map_err(|_| GltfInferenceTextError::ChecksumSyntax(checksum_text.into()))?;
    let actual_checksum = crc32_iso_hdlc(payload);
    if checksum != actual_checksum {
        return Err(GltfInferenceTextError::ChecksumMismatch { declared: checksum, actual: actual_checksum });
    }
    decode_canonical_json(payload)
}

fn check_header(line: u8, actual: Option<&str>, expected: &str) -> Result<(), GltfInferenceTextError> {
    let actual = actual.ok_or(GltfInferenceTextError::MissingPayload)?;
    if actual != expected {
        return Err(GltfInferenceTextError::Header { line, expected: expected.into(), actual: actual.into() });
    }
    Ok(())
}

pub(crate) fn decode_canonical_json(payload: &[u8]) -> Result<GltfInference, GltfInferenceTextError> {
    let value: GltfInference = serde_json::from_slice(payload).map_err(|error| GltfInferenceTextError::Json(error.to_string()))?;
    if canonical_json_bytes(&value)? != payload {
        return Err(GltfInferenceTextError::NonCanonical);
    }
    Ok(value)
}

pub(crate) fn canonical_json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, GltfInferenceTextError> {
    let value = serde_json::to_value(value).map_err(|error| GltfInferenceTextError::Serialization(error.to_string()))?;
    let mut output = String::new();
    write_canonical_json(&value, &mut output)?;
    Ok(output.into_bytes())
}

fn write_canonical_json(value: &Value, output: &mut String) -> Result<(), GltfInferenceTextError> {
    match value {
        Value::Null => output.push_str("null"),
        Value::Bool(value) => output.push_str(if *value { "true" } else { "false" }),
        Value::Number(value) => output.push_str(&canonical_number(value)?),
        Value::String(value) => output.push_str(&serde_json::to_string(value).map_err(|error| GltfInferenceTextError::Serialization(error.to_string()))?),
        Value::Array(values) => {
            output.push('[');
            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                write_canonical_json(value, output)?;
            }
            output.push(']');
        }
        Value::Object(values) => {
            let mut entries: Vec<_> = values.iter().collect();
            entries.sort_by(|(left, _), (right, _)| utf16_cmp(left, right));
            output.push('{');
            for (index, (key, value)) in entries.into_iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                output.push_str(&serde_json::to_string(key).map_err(|error| GltfInferenceTextError::Serialization(error.to_string()))?);
                output.push(':');
                write_canonical_json(value, output)?;
            }
            output.push('}');
        }
    }
    Ok(())
}

fn utf16_cmp(left: &str, right: &str) -> Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}

fn canonical_number(value: &serde_json::Number) -> Result<String, GltfInferenceTextError> {
    if let Some(value) = value.as_i64() {
        return Ok(value.to_string());
    }
    if let Some(value) = value.as_u64() {
        return Ok(value.to_string());
    }
    let value = value.as_f64().ok_or_else(|| GltfInferenceTextError::Serialization("non-finite JSON number".into()))?;
    if value == 0.0 {
        return Ok("0".into());
    }
    let negative = value.is_sign_negative();
    let raw = serde_json::to_string(&value.abs()).map_err(|error| GltfInferenceTextError::Serialization(error.to_string()))?;
    let (coefficient, exponent) = raw.split_once('e').or_else(|| raw.split_once('E')).map_or((raw.as_str(), 0), |(coefficient, exponent)| (coefficient, exponent.parse::<i32>().unwrap_or(0)));
    let integer_digits = coefficient.find('.').unwrap_or(coefficient.len()) as i32;
    let mut digits = coefficient.bytes().filter(|byte| *byte != b'.').map(char::from).collect::<String>();
    while digits.len() > 1 && digits.ends_with('0') {
        digits.pop();
    }
    let decimal_position = integer_digits + exponent;
    let mut result = if value.abs() >= 1e-6 && value.abs() < 1e21 {
        if decimal_position <= 0 {
            format!("0.{}{}", "0".repeat((-decimal_position) as usize), digits)
        } else if decimal_position as usize >= digits.len() {
            format!("{}{}", digits, "0".repeat(decimal_position as usize - digits.len()))
        } else {
            format!("{}.{}", &digits[..decimal_position as usize], &digits[decimal_position as usize..])
        }
    } else {
        let coefficient = if digits.len() == 1 { digits } else { format!("{}.{}", &digits[..1], &digits[1..]) };
        let exponent = decimal_position - 1;
        format!("{coefficient}e{}{exponent}", if exponent >= 0 { "+" } else { "" })
    };
    if negative {
        result.insert(0, '-');
    }
    Ok(result)
}

pub(crate) fn crc32_iso_hdlc(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & (0u32.wrapping_sub(crc & 1)));
        }
    }
    !crc
}
//#endregion 📨️Envelope

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn crc_and_canonicalization_vectors() {
        assert_eq!(crc32_iso_hdlc(b"123456789"), 0xcbf4_3926);
        assert_eq!(crc32_iso_hdlc(GLTF_INFERENCE_SCHEMA.as_bytes()), 0x6b25_7ae0);
        assert_eq!(String::from_utf8(canonical_json_bytes(&json!({"z":-0.0,"d":1e21,"c":1e20,"b":1e-7,"a":1e-6})).unwrap()).unwrap(), r#"{"a":0.000001,"b":1e-7,"c":100000000000000000000,"d":1e+21,"z":0}"#);
    }

    #[test]
    fn deterministic_roundtrip_has_no_trailing_lf() {
        let value = GltfInference::default();
        let encoded = encode_gltf_inference_text(&value).unwrap();
        assert_eq!(encoded, encode_gltf_inference_text(&value).unwrap());
        assert!(!encoded.ends_with('\n'));
        assert_eq!(decode_gltf_inference_text(&encoded).unwrap(), value);
    }

    #[test]
    fn rejects_header_length_checksum_and_noncanonical_payload() {
        let encoded = encode_gltf_inference_text(&GltfInference::default()).unwrap();
        assert!(matches!(decode_gltf_inference_text(&encoded.replacen("schema ", "schema x", 1)), Err(GltfInferenceTextError::Header { .. })));
        assert!(matches!(decode_gltf_inference_text(&encoded.replacen("length ", "length 0", 1)), Err(GltfInferenceTextError::LengthSyntax(_))));
        let checksum_index = encoded.find("checksum ").unwrap() + "checksum ".len();
        let mut corrupt = encoded.clone().into_bytes();
        corrupt[checksum_index] = if corrupt[checksum_index] == b'0' { b'1' } else { b'0' };
        assert!(matches!(decode_gltf_inference_text(std::str::from_utf8(&corrupt).unwrap()), Err(GltfInferenceTextError::ChecksumMismatch { .. })));
        let payload = canonical_json_bytes(&GltfInference::default()).unwrap();
        let mut noncanonical = vec![b' '];
        noncanonical.extend_from_slice(&payload);
        let envelope = format!("schema {GLTF_INFERENCE_SCHEMA}\nversion 2\nlength {}\nchecksum {:08x}\n{}", noncanonical.len(), crc32_iso_hdlc(&noncanonical), String::from_utf8(noncanonical).unwrap());
        assert_eq!(decode_gltf_inference_text(&envelope), Err(GltfInferenceTextError::NonCanonical));
    }
}
