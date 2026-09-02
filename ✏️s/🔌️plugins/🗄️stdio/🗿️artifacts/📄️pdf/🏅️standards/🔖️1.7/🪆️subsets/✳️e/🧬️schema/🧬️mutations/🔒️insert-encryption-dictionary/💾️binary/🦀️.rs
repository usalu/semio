//! Direct binary identity for `insert-encryption-dictionary`.

pub const TAG: u8 = 0;
pub const BINARY_TAG: u8 = TAG;

use super::InsertEncryptionDictionary;

/// 📤️ Encodes this direct payload as canonical schema JSON bytes.
pub fn encode(payload: &InsertEncryptionDictionary) -> Result<Vec<u8>, String> {
    Ok(pack::to_json_string(payload).into_bytes())
}

/// 📥️ Decodes this direct payload from canonical schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<InsertEncryptionDictionary, String> {
    let parsed = pack::parse_json_bytes(bytes).map_err(|error| error.to_string())?;
    dsl::FromValue::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())
}
