//! 📦️ S Studio app — binary document surface + laws (constitutional: pack).
//!
//! 🕳️ Wraps `semio_framework_os::WorkflowDocument` (the dissolved `OsProjection`'s successor, see `##
//! The inversion`) — see `space_op`'s doc comment for why this app owns no document/operation type.

use semio_framework_os::WorkflowDocument;
use store::PackError;

/// 📦️ Encodes a `WorkflowDocument` to its binary pack form.
pub fn encode(projection: &WorkflowDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(projection)
}

/// 📖️ Decodes a `WorkflowDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<WorkflowDocument, PackError> {
    <WorkflowDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use space_shared::demo_space_projection;

    #[test]
    fn demo_document_dsl_pack_equivalence() {
        let projection = demo_space_projection();
        store::test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn pack_round_trips_demo_projection() {
        let projection = demo_space_projection();
        let bytes = encode(&projection);
        assert_eq!(decode(&bytes).expect("decode"), projection);
    }

    /// 🧮️ Per-app recipe item 3: `SpaceConfig` round-trips dsl<->pack independently of the
    /// `OsProjection` document grammar above.
    #[test]
    fn space_config_dsl_pack_equivalence() {
        store::test_support::assert_dsl_pack_equivalence(&space_engine::SpaceConfig::default());
    }
}
//#endregion 🧪️Tests
