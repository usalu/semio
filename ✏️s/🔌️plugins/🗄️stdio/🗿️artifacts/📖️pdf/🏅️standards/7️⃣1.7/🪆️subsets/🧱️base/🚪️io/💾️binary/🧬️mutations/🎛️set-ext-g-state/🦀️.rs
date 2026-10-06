//! 🎛️ Direct binary identity for `set-ext-g-state`.

pub const TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "set-ext-g-state");
pub const BINARY_TAG: u8 = TAG;

use crate::standards::v1_7::subsets::base::schema::mutations::SetExtGState;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &SetExtGState) -> Result<Vec<u8>, String> {
    Ok(semio_framework_pack_json::to_json_string(payload).into_bytes())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<SetExtGState, String> {
    let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())
}
