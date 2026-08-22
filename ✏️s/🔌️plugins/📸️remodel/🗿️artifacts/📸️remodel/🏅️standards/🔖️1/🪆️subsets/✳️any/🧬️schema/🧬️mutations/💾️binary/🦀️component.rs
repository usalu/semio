//! ⚖️ Remodel artifact — the binary operation surface (`spr`) and its laws.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::remodel::schema::mutations::text::RemodelMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `RemodelMutation` to its binary command form.
pub fn encode_op(operation: &RemodelMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RemodelMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RemodelMutation, protocol::ProtocolError> {
    RemodelMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::remodel::default_remodel_scene;

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let scene = default_remodel_scene();
        let operation = crate::artifacts::remodel::mutations::update_feature_params(scene.params.feature);
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// 📄️ Full `print_document_text`/`parse_document_text` round trip through a live `ArtifactStore`
    /// with an applied edit, the ground-truth contract for replacing the JSON envelope with text files.
    #[semio_framework_async_macros::async_test]
    async fn store_roundtrips_through_document_text() {
        let initial = default_remodel_scene();
        let envelope = store::create_document_envelope("test/v1", "test", initial, None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        let mut feature_params = store.snapshot().expect("initial projection").params.feature;
        feature_params.target_count = 12345;
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::artifacts::remodel::mutations::update_feature_params(feature_params)], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
