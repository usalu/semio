//! ✏️ Direct binary identity for `set-page-content`.

pub const TAG: u8 = 14;
pub const BINARY_TAG: u8 = TAG;

use super::SetPageContent;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &SetPageContent) -> Result<Vec<u8>, String> {
    serde_json::to_vec(payload).map_err(|error| error.to_string())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<SetPageContent, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}
