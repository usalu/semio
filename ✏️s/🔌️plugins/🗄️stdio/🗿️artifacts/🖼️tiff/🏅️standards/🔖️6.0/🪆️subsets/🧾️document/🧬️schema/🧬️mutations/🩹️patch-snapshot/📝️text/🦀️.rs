//! 📝️ Direct compact TIFF snapshot-patch text codec.

use super::*;
use crate::schema::mutations::text::Entry;
use protocol::OpText;

pub const TEXT_OPCODE: &str = "patch-snapshot";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };
fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|byte| format!("{byte:02x}")).collect() }
fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
    let bytes = value.as_bytes();
    if !bytes.len().is_multiple_of(2) { return Err("snapshot patch has odd hexadecimal length".into()); }
    let nibble = |byte| match byte { b'0'..=b'9' => Some(byte - b'0'), b'a'..=b'f' => Some(byte - b'a' + 10), b'A'..=b'F' => Some(byte - b'A' + 10), _ => None };
    bytes.chunks_exact(2).map(|pair| Ok(nibble(pair[0]).ok_or_else(|| "snapshot patch contains non-hexadecimal bytes".to_string())? << 4 | nibble(pair[1]).ok_or_else(|| "snapshot patch contains non-hexadecimal bytes".to_string())?)).collect()
}
pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(format!("{TEXT_OPCODE} patch={}", hex_encode(patch.print_op().as_bytes())))
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE { return Err(format!("expected {TEXT_OPCODE}")); }
    let encoded = arguments.strip_prefix("patch=").ok_or_else(|| "missing patch".to_string())?;
    let bytes = hex_decode(encoded)?;
    let source = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    let patch = editing::SnapshotPatch::parse_op(source).map_err(|error| error.to_string())?;
    Ok(TiffMutation::PatchSnapshot(PatchSnapshot { patch }))
}
