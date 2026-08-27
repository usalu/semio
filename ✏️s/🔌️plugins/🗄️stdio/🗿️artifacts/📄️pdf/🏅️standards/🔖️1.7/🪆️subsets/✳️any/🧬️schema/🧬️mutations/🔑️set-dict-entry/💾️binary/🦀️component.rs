//! 🔑️ Direct binary identity for `set-dict-entry`.

pub const TAG: u8 = 9;
pub const BINARY_TAG: u8 = TAG;

use super::SetDictEntry;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &SetDictEntry) -> Result<Vec<u8>, String> {
    serde_json::to_vec(payload).map_err(|error| error.to_string())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<SetDictEntry, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}
