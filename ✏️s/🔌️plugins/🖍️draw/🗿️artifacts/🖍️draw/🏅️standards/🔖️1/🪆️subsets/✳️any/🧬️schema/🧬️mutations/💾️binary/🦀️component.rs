//! 📡️ Draw artifact — wire codec (encode_op/decode_op), renamed from the old `protocol` half
//! (constitutional: spr — state patch representation).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::draw::op::DrawMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `DrawMutation` to its binary command form.
pub fn encode_op(operation: &DrawMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `DrawMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<DrawMutation, protocol::ProtocolError> {
    DrawMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::draw::engine::{create_draw_shape_layer_rect, default_draw_document, layer_id};
    use crate::artifacts::draw::{DrawSnapshot, DRAW_DOCUMENT_SCHEMA};

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let document = default_draw_document("doc-text-test", None);
        let operation = DrawMutation::AddLayer { parent_id: None, index: Some(document.layers.len()), layer: Box::new(create_draw_shape_layer_rect("Op Binary Test")) };
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trips_a_store_with_an_applied_operation() {
        let initial = default_draw_document("doc-text-test", None);
        let envelope = store::create_document_envelope::<DrawSnapshot, DrawMutation>(DRAW_DOCUMENT_SCHEMA, "doc-text-test", initial, None);
        let mut doc_store = store::ArtifactStore::new(envelope);
        let layer = create_draw_shape_layer_rect("Added Rect");
        let layer_id_value = layer_id(&layer).to_string();
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![DrawMutation::AddLayer { parent_id: None, index: None, layer: Box::new(layer) }], description: Some("add rect".into()) }).expect("apply add layer");
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![DrawMutation::SetLayerOpacity { layer_id: layer_id_value, opacity: 0.5 }], description: Some("set opacity".into()) }).expect("apply set opacity");
        store::os_store::test_support::assert_document_text_round_trip(&doc_store);
        store::os_store::test_support::assert_document_pack_round_trip(&doc_store);
        store::os_store::test_support::assert_live_equals_replay(&doc_store);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `DrawMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing pack round-trip law.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{ArtifactId, Edit, SchemaId};

        let initial = default_draw_document("doc-text-test", None);
        let envelope = store::create_document_envelope::<DrawSnapshot, DrawMutation>(DRAW_DOCUMENT_SCHEMA, "doc-text-test", initial, None);
        let mut doc_store = store::ArtifactStore::new(envelope);
        let layer = create_draw_shape_layer_rect("Added Rect");
        let layer_id_value = layer_id(&layer).to_string();
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![DrawMutation::AddLayer { parent_id: None, index: None, layer: Box::new(layer) }], description: Some("add rect".into()) }).expect("apply add layer");
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![DrawMutation::SetLayerOpacity { layer_id: layer_id_value, opacity: 0.5 }], description: Some("set opacity".into()) }).expect("apply set opacity");
        let edit: &Edit<DrawMutation> = doc_store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<DrawSnapshot, DrawMutation>(edit, &ArtifactId(doc_store.envelope().id.clone()), &SchemaId(doc_store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
