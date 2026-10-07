//! 📝️ Direct set-snapshot text codec.
use crate::standards::v6_0::subsets::document::schema::mutations::*;
use crate::standards::v6_0::subsets::document::schema::mutations::set_snapshot::SetSnapshot;
use crate::standards::v6_0::subsets::document::schema::snapshot::*;
use crate::standards::v6_0::subsets::document::io::text::diff::*;

use crate::standards::v6_0::subsets::document::io::text::mutations::*;
use crate::standards::v6_0::subsets::document::io::text::mutations::Entry;

pub const TEXT_OPCODE: &str = "set-snapshot";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };
fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|byte| format!("{byte:02x}")).collect() }
fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) { return Err("snapshot payload has odd hexadecimal length".into()); }
    (0..value.len()).step_by(2).map(|index| u8::from_str_radix(&value[index..index + 2], 16).map_err(|error| error.to_string())).collect()
}
pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::SetSnapshot(SetSnapshot { snapshot }) = value else { return None };
    Some(format!("{TEXT_OPCODE} snapshot={}", hex_encode(semio_framework_pack_json::to_json_string(snapshot).as_bytes())))
}
pub fn parse(line: &str) -> Result<TiffMutation, semio_framework_diagnostic::TextError> {
    let (opcode, arguments) = line.split_once(' ').unwrap_or((line, Default::default()));
    if opcode != TEXT_OPCODE { return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected {TEXT_OPCODE}"),semio_framework_diagnostic::TextSpan::at(1,1))); }
    let encoded = arguments.strip_prefix("snapshot=").ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"missing snapshot",semio_framework_diagnostic::TextSpan::at(1,1)))?;
    let bytes = hex_decode(encoded).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message,semio_framework_diagnostic::TextSpan::at(1,1)))?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error.into_value_error(),semio_framework_diagnostic::TextSpan::at(1,1)))?;
    let snapshot = <TiffSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))?;
    Ok(TiffMutation::SetSnapshot(SetSnapshot { snapshot }))
}
