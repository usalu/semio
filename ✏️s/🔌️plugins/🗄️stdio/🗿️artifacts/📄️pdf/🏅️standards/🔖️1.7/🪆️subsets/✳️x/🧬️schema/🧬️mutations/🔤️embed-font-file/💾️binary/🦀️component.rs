//! 🔤️ Direct binary codec for `embed-font-file`.

use super::EmbedFontFile;

//#region 🔖️Identity
pub const TAG: u8 = 6;
pub const BINARY_TAG: u8 = TAG;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 📤️ Encodes the owned payload as canonical schema JSON bytes.
pub fn encode(payload: &EmbedFontFile) -> Result<Vec<u8>, String> {
    serde_json::to_vec(payload).map_err(|error| error.to_string())
}

/// 📥️ Decodes the owned payload from schema JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<EmbedFontFile, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_payload_round_trips() {
        let payload = EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program: crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::ObjRef { num: 1, gen: 0 } };
        assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
    }
}
//#endregion 🧪️Tests
