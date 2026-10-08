//! 📝️ Direct set-gamma text codec.
use crate::standards::v1_2::subsets::any::io::text::mutations::*;
use crate::standards::v1_2::subsets::any::io::text::mutations::Entry;
pub const TEXT_OPCODE: &str = "set-gamma";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::SetGamma(payload) = value else { return None };
    let json = semio_framework_pack_json::to_json_string(payload);
    Some(format!("{TEXT_OPCODE} payload={}", json.as_bytes().iter().map(|byte| format!("{byte:02x}")).collect::<String>()))
}

pub fn parse(line: &str) -> Result<PngMutation, semio_framework_diagnostic::TextError> {
    let (_, encoded) = line.split_once(" payload=").ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing set-gamma payload", semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    if !encoded.len().is_multiple_of(2) { return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "odd payload hexadecimal length", semio_framework_diagnostic::TextSpan::at(1, 1))); }
    let bytes: Result<Vec<u8>, _> = (0..encoded.len()).step_by(2).map(|index| u8::from_str_radix(&encoded[index..index + 2], 16)).collect();
    let text = String::from_utf8(bytes.map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let payload = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    Ok(PngMutation::SetGamma(payload))
}
