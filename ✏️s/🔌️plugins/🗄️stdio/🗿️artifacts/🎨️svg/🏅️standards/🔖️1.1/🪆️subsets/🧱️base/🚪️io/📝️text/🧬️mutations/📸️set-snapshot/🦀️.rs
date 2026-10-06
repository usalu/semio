//! 📝️ Direct set-snapshot text codec.

use crate::standards::v1_1::subsets::base::io::text::mutations::*;

pub const TEXT_OPCODE: &str = "set-snapshot";
fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|byte| format!("{byte:02x}")).collect() }
fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) { return Err("snapshot payload has odd hexadecimal length".into()); }
    (0..value.len()).step_by(2).map(|index| u8::from_str_radix(&value[index..index + 2], 16).map_err(|error| error.to_string())).collect()
}
pub fn print(value: &SvgMutation) -> Option<String> {
    let SvgMutation::SetSnapshot(SetSnapshot { snapshot }) = value else { return None };
    Some(format!("{TEXT_OPCODE} snapshot={}", hex_encode(semio_framework_pack_json::to_json_string(snapshot).as_bytes())))
}
pub fn parse(line: &str) -> Result<SvgMutation, String> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE { return Err(format!("expected {TEXT_OPCODE}")); }
    let encoded = arguments.strip_prefix("snapshot=").ok_or_else(|| "missing snapshot".to_string())?;
    let bytes = hex_decode(encoded)?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let snapshot = <SvgSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())?;
    Ok(SvgMutation::SetSnapshot(SetSnapshot { snapshot }))
}
