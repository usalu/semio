//! Direct binary identity for `insert-media-annotation`.

pub const TAG: u8 = 6;
pub const BINARY_TAG: u8 = TAG;

use super::InsertMediaAnnotation;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &InsertMediaAnnotation) -> Result<Vec<u8>, String> {
    serde_json::to_vec(payload).map_err(|error| error.to_string())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<InsertMediaAnnotation, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}
