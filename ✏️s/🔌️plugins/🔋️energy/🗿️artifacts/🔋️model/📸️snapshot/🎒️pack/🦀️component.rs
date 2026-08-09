//! 📦️ EnergyModel artifact — binary document surface + laws.

use crate::artifacts::model::EnergyModelSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes an `EnergyModelSnapshot` to its binary pack form.
pub fn encode(document: &EnergyModelSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes an `EnergyModelSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<EnergyModelSnapshot, PackError> {
    <EnergyModelSnapshot as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = crate::artifacts::model::dsl::parse_dsl(
            crate::artifacts::model::dsl::SEMIO_ENERGY_MODEL_EXAMPLE_TEXT,
        )
        .expect("parse semio example");
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
