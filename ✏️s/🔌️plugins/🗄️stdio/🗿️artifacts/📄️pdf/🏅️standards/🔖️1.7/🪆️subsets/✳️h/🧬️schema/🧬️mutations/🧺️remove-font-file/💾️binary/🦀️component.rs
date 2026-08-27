//! Direct binary identity for `remove-font-file`.

pub const TAG: u8 = 9;
pub const BINARY_TAG: u8 = TAG;

use super::RemoveFontFile;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &RemoveFontFile) -> Result<Vec<u8>, String> {
    serde_json::to_vec(payload).map_err(|error| error.to_string())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<RemoveFontFile, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}
