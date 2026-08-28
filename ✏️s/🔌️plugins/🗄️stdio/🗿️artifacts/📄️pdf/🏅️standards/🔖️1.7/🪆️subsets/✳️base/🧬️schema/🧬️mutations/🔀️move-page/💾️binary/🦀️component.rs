//! 🔀️ Direct binary identity for `move-page`.

pub const TAG: u8 = 13;
pub const BINARY_TAG: u8 = TAG;

use super::MovePage;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &MovePage) -> Result<Vec<u8>, String> {
    serde_json::to_vec(payload).map_err(|error| error.to_string())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<MovePage, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}
