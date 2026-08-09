//! 📦️ S Home launcher artifact — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::home::SHomeSnapshot;
use store::PackError;

/// 📦️ Encodes an `SHomeSnapshot` to its binary pack form.
pub fn encode(document: &SHomeSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes an `SHomeSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<SHomeSnapshot, PackError> {
    <SHomeSnapshot as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::home::config::HomeConfig;

    #[test]
    fn home_pack_round_trips_default_and_populated_documents() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 });
        store::os_store::test_support::assert_dsl_pack_equivalence(&SHomeSnapshot { schema: "s.home".into(), catalog_generation: 42 });
    }

    #[test]
    fn pack_round_trips_populated_document() {
        let document = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 42 };
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    /// 🧮️ `HomeConfig` round-trips dsl<->pack independently of the `SHomeSnapshot` document grammar above.
    #[test]
    fn home_config_dsl_pack_equivalence() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&HomeConfig::default());
    }
}
//#endregion 🧪️Tests
