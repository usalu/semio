//! 📝️ Direct indexed-region text codec.
use crate::standards::v_v3::subsets::any::io::text::mutations::*;
use crate::standards::v_v3::subsets::any::io::text::mutations::Entry;
pub const TEXT_OPCODE: &str = "paint-indexed-region";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };
pub fn print(value: &BmpMutation) -> Option<String> {
    let BmpMutation::PaintIndexedRegion(payload) = value else { return None };
    Some(format!("{TEXT_OPCODE} payload={}", semio_framework_pack_json::to_json_string(payload).as_bytes().iter().map(|byte| format!("{byte:02x}")).collect::<String>()))
}
pub fn parse(line: &str) -> Result<BmpMutation, semio_framework_diagnostic::TextError> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, ""));
    if opcode != TEXT_OPCODE {
        return Err(error(format!("expected {TEXT_OPCODE}")));
    }
    let encoded = arguments.strip_prefix("payload=").ok_or_else(|| error("missing payload"))?;
    if !encoded.len().is_multiple_of(2) {
        return Err(error("payload has odd hexadecimal length"));
    }
    let bytes = (0..encoded.len()).step_by(2).map(|index| u8::from_str_radix(&encoded[index..index + 2], 16).map_err(|failure| failure.to_string())).collect::<Result<Vec<_>, _>>().map_err(error)?;
    crate::standards::v_v3::subsets::any::io::text::mutations::binary::decode(&bytes).map_err(|failure| error(failure.to_string()))
}
fn error(message: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message, semio_framework_diagnostic::TextSpan::at(1, 1))
}
