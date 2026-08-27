//! Direct binary identity for `remove-javascript-action`.

pub const TAG: u8 = 3;
pub const BINARY_TAG: u8 = TAG;

use super::RemoveJavascriptAction;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &RemoveJavascriptAction) -> Result<Vec<u8>, String> {
    serde_json::to_vec(payload).map_err(|error| error.to_string())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<RemoveJavascriptAction, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}
