//! 📦️ Imperative artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::imperative::ImperativeSnapshot;
use store::PackError;

/// 📦️ Encodes an `ImperativeSnapshot` to its binary pack form.
pub fn encode(snapshot: &ImperativeSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(snapshot)
}

/// 📖️ Decodes an `ImperativeSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<ImperativeSnapshot, PackError> {
    <ImperativeSnapshot as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::imperative::dsl;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let snapshot = dsl::parse_dsl(dsl::IMPERATIVE_EXAMPLE_TEXT).expect("parse 📜️default.imperative");
        store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
        let bytes = encode(&snapshot);
        assert_eq!(decode(&bytes).expect("decode"), snapshot);
    }
}
//#endregion 🧪️Tests
