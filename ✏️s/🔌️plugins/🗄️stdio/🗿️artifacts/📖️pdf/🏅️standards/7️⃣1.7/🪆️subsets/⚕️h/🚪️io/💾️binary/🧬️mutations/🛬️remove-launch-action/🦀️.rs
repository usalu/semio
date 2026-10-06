//! Direct binary identity for `remove-launch-action`.

pub const TAG: u8 = 5;
pub const BINARY_TAG: u8 = TAG;

use super::RemoveLaunchAction;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &RemoveLaunchAction) -> Result<Vec<u8>, String> {
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(payload))).into_bytes())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<RemoveLaunchAction, String> {
    let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())
}
