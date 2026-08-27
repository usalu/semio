//! Direct binary identity for `insert-javascript-action`.

pub const TAG: u8 = 2;
pub const BINARY_TAG: u8 = TAG;

use super::InsertJavascriptAction;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &InsertJavascriptAction) -> Result<Vec<u8>, String> {
    serde_json::to_vec(payload).map_err(|error| error.to_string())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<InsertJavascriptAction, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}
