//! ⚖️ Sourcing curate artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! The app's typed `SourcingCurateCommand` enum — which used to share the old `📡️protocol` crate with
//! this codec — is an APP concern, not an artifact one: it now lives in `🎛️apps/🗂️curate/🦀️component.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::curate::schema::mutations::SourcingMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `SourcingMutation` to its binary state-patch form.
pub async fn encode_op(operation: &SourcingMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `SourcingMutation` from its binary state-patch form.
pub async fn decode_op(bytes: &[u8]) -> Result<SourcingMutation, protocol::ProtocolError> {
    SourcingMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = crate::artifacts::curate::schema::mutations::create_curated_item(crate::artifacts::curate::CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 3 });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn curate_document_text_round_trips_through_a_vcs_store() {
        let document = crate::artifacts::curate::curate_snapshot_from_stock(crate::artifacts::curate::schema::demo_stock(), Vec::new());
        let envelope = store::create_document_envelope(crate::artifacts::curate::SOURCING_CURATE_SCHEMA, "sourcing-curate-test", document, None);
        let mut doc_store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        let object_id = crate::artifacts::curate::stock_of(&doc_store.snapshot().expect("snapshot"))[0].id.clone();
        let mutation = crate::artifacts::curate::schema::mutations::create_curated_item(crate::artifacts::curate::CuratedItem { object_id, count: 3 });
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&doc_store);
        store::os_store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
//#endregion 🧪️Tests
