//! 📦️ `trinity.graph` artifact — binary document surface + laws (constitutional: pack).
//!
//! 📌️ The `ArtifactPack` impl itself lives in `🗣️dsl/🦀️component.rs`, next to the private
//! `JackSnapshotDsl` mirror it delegates through (same reason the DSL impl lives there too) — this
//! file only holds the public encode/decode entry points, matching the old bundle crate's shape.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::jack::JackSnapshot;
use store::{ArtifactPack, PackError};

/// 📦️ Encodes a `JackSnapshot` to its binary pack form.
pub async fn encode(document: &JackSnapshot) -> Vec<u8> {
    ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `JackSnapshot` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<JackSnapshot, PackError> {
    <JackSnapshot as ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::dsl::{parse_dsl, NAKAGIN_EXAMPLE_TEXT};

    #[test]
    async fn nakagin_example_pack_round_trips_and_agrees_with_dsl() {
        let document = parse_dsl(NAKAGIN_EXAMPLE_TEXT).expect("parse nakagin example");
        ::store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
