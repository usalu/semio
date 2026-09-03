//! 📦️ Raster artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::raster::RasterSnapshot;
use store::PackError;

/// 📦️ Encodes a `RasterSnapshot` to its binary pack form.
pub fn encode(document: &RasterSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `RasterSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<RasterSnapshot, PackError> {
    <RasterSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::raster::mutations::create_layer;
    use crate::artifacts::raster::op::RasterMutation;
    use crate::artifacts::raster::RasterOwnedMap;
    use crate::artifacts::raster::{RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trips_and_agrees_with_dsl() {
        let document = crate::artifacts::raster::schema::semio_fixture_snapshot();
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trips_representative_document() {
        let mut assets = RasterOwnedMap::new();
        assets.insert("asset-1".into(), crate::artifacts::raster::image_asset_child_handle("asset-1", &RasterImageAsset { mime: "image/png".into(), data: b"abc".to_vec() }));
        let mut params = RasterOwnedMap::new();
        params.insert("brightness".into(), dsl::DslValue::float(0.06));
        params.insert("label".into(), dsl::DslValue::String("Warm \"Curve\"".to_string()));
        params.insert("enabled".into(), dsl::DslValue::Bool(true));
        params.insert("fallback".into(), dsl::DslValue::Null);
        params.insert(
            "curves".into(),
            dsl::DslValue::Array(vec![
                dsl::DslValue::Array(vec![dsl::DslValue::float(0.0), dsl::DslValue::float(0.0)]),
                dsl::DslValue::Array(vec![dsl::DslValue::float(0.25), dsl::DslValue::float(0.2)]),
                dsl::DslValue::Array(vec![dsl::DslValue::float(1.0), dsl::DslValue::float(1.0)]),
            ]),
        );
        params.insert("nested".into(), dsl::DslValue::Object(vec![("inner".to_string(), dsl::DslValue::float(1.5))]));
        let document = RasterSnapshot {
            schema: RASTER_DOCUMENT_SCHEMA.into(),
            id: "doc-1".into(),
            title: Some("Representative \"Doc\"".into()),
            assets,
            layers: vec![
                RasterLayerNode::Pixel {
                    id: "pixel-1".into(),
                    name: "Pixel One".into(),
                    visible: true,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    mask: Some(RasterLayerMask { enabled: true, linked: false, invert: true, width: Some(64), height: None }),
                    width: Some(256),
                    height: Some(256),
                    image_key: Some("asset-1".into()),
                },
                RasterLayerNode::Group {
                    id: "group-1".into(),
                    name: "Group / Nested".into(),
                    visible: false,
                    opacity: 0.5,
                    blend_mode: "screen".into(),
                    transform: RasterTransform { x: 1.0, y: -2.0, scale_x: 1.5, scale_y: 0.5, rotation: 12.0 },
                    mask: None,
                    children: vec![
                        RasterLayerNode::Pixel {
                            id: "pixel-2".into(),
                            name: "Child Pixel".into(),
                            visible: true,
                            opacity: 0.75,
                            blend_mode: "multiply".into(),
                            transform: RasterTransform::default(),
                            mask: None,
                            width: None,
                            height: None,
                            image_key: None,
                        },
                        RasterLayerNode::Group { id: "group-2".into(), name: "Nested Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() },
                    ],
                },
                RasterLayerNode::Adjustment { id: "adjust-1".into(), name: "Curves & Co".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "curves".into(), params },
            ],
        };
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `RasterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing pack round-trip laws (same pattern as `mathematical_pack`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let envelope = create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster-command-envelope-demo", crate::artifacts::raster::schema::empty_raster_document(), None);
        let mut store = ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store
            .dispatch(ArtifactCommand::Apply {
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
