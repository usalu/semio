//! ⚖️ Raster artifact — binary command protocol surface + laws (constitutional: spr).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::raster::op::RasterMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `RasterMutation` to its binary command form.
pub async fn encode_op(operation: &RasterMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RasterMutation` from its binary command form.
pub async fn decode_op(bytes: &[u8]) -> Result<RasterMutation, protocol::ProtocolError> {
    RasterMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::raster::schema::empty_raster_document;
    use crate::artifacts::raster::mutations::create_layer;
    use crate::artifacts::raster::{RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let document = empty_raster_document();
        let operation = RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
            parent_id: None,
            index: document.layers.len(),
            layer: Box::new(RasterLayerNode::Pixel {
                id: "op-binary-test".into(),
                name: "Op Binary Test".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(64),
                height: Some(64),
                image_key: None,
            }),
        });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_document_text_round_trips_store_with_applied_operation() {
        use crate::artifacts::raster::RasterSnapshot;

        let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "doc-text-test", empty_raster_document(), None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store
            .dispatch(store::ArtifactCommand::Apply {
                mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
                    parent_id: None,
                    index: 1,
                    layer: Box::new(RasterLayerNode::Adjustment {
                        id: "adjust-text".into(),
                        name: "Levels".into(),
                        visible: true,
                        opacity: 1.0,
                        blend_mode: "normal".into(),
                        transform: RasterTransform::default(),
                        adjustment_kind: "levels".into(),
                        params: std::collections::BTreeMap::new(),
                    }),
                })],
                description: None,
            })
            .expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `RasterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing pack round-trip law (same pattern as `mathematical_protocol`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::raster::RasterSnapshot;
        use protocol::{ArtifactId, Edit, SchemaId};

        let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "command-envelope-demo", empty_raster_document(), None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store
            .dispatch(store::ArtifactCommand::Apply {
                mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
                    parent_id: None,
                    index: 0,
                    layer: Box::new(RasterLayerNode::Pixel {
                        id: "command-envelope-pixel".into(),
                        name: "Command Envelope Pixel".into(),
                        visible: true,
                        opacity: 1.0,
                        blend_mode: "normal".into(),
                        transform: RasterTransform::default(),
                        mask: None,
                        width: Some(32),
                        height: Some(32),
                        image_key: None,
                    }),
                })],
                description: None,
            })
            .expect("apply");
        let edit: &Edit<RasterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<RasterSnapshot, RasterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
