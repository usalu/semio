//! 🎬️ Direct binary codec for `insert-media-annotation`.

use super::InsertMediaAnnotation;

//#region 🔖️Identity
pub const TAG: u8 = 12;
pub const BINARY_TAG: u8 = TAG;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 📤️ Encodes the owned payload as canonical schema JSON bytes.
pub fn encode(payload: &InsertMediaAnnotation) -> Result<Vec<u8>, String> {
    Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(payload))).into_bytes())
}

/// 📥️ Decodes the owned payload from schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<InsertMediaAnnotation, String> {
    let parsed = pack::parse_json_bytes(bytes).map_err(|error| error.to_string())?;
    <InsertMediaAnnotation as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_payload_round_trips() {
        let payload = InsertMediaAnnotation { subtype: "Movie".to_string(), title: "sample".to_string() };
        assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
    }
}
//#endregion 🧪️Tests
