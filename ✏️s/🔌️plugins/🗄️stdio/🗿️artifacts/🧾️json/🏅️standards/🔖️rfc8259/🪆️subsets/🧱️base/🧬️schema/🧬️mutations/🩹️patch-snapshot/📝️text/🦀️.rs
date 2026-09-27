//! 📝️ Direct compact JSON snapshot-patch text codec.

use super::*;
use protocol::OpText;

pub const TEXT_OPCODE: &str = "patch-snapshot";

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
    fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            _ => None,
        }
    }
    let mut pairs = value.as_bytes().chunks_exact(2);
    let decoded = pairs
        .by_ref()
        .map(|pair| Ok((nibble(pair[0]).ok_or_else(|| "invalid lowercase hexadecimal".to_string())? << 4) | nibble(pair[1]).ok_or_else(|| "invalid lowercase hexadecimal".to_string())?))
        .collect::<Result<Vec<_>, String>>()?;
    if pairs.remainder().is_empty() { Ok(decoded) } else { Err("snapshot patch has odd hexadecimal length".into()) }
}

pub fn print(value: &JsonMutation) -> Option<String> {
    let JsonMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(format!("{TEXT_OPCODE} patch={}", hex_encode(patch.print_op().as_bytes())))
}

pub fn parse(line: &str) -> Result<JsonMutation, String> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let encoded = arguments.strip_prefix("patch=").ok_or_else(|| "missing patch".to_string())?;
    let bytes = hex_decode(encoded)?;
    let source = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    let patch = editing::SnapshotPatch::parse_op(source).map_err(|error| error.to_string())?;
    Ok(JsonMutation::PatchSnapshot(PatchSnapshot { patch }))
}

#[cfg(test)]
#[test]
fn malformed_unicode_hex_is_rejected_without_panicking() {
    assert!(parse("patch-snapshot patch=€0").is_err());
}
