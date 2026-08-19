//! 📦️ Draw artifact — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::draw::DrawSnapshot;
use store::PackError;

//#region 🔖️HandcraftedArtifactPack
/// ✉️ P6 handcrafted `ArtifactPack` (derive no longer emits this trait) — relocated from
/// `🧬️schema/📸️snapshot/🦀️component.rs` (design.md §1 CORRECTION: unsplit native codec lives at
/// `🚪️io/<facet>/<representation>/`; `🧬️schema` keeps only the `DrawSnapshot` struct + `Default`).
impl store::ArtifactPack for DrawSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedArtifactPack

/// 📦️ Encodes a `DrawSnapshot` to its binary pack form.
pub async fn encode(document: &DrawSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `DrawSnapshot` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<DrawSnapshot, PackError> {
    <DrawSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::draw::dsl;
    use crate::artifacts::draw::schema::{create_draw_boolean_layer, create_draw_image_layer, create_draw_path_layer, create_draw_shape_layer_rect, create_draw_trace_layer, default_draw_document, default_layer_base, layer_id};
    use crate::artifacts::draw::{DrawArtboard, DrawCircle, DrawEllipse, DrawGroupBody, DrawImageAsset, DrawLayerNode, DrawLine, DrawPolygon, DrawShapeBody, DrawTextBody, FillStyle, GradientStop, PathSegment, StrokeStyle, DRAW_DOCUMENT_SCHEMA};

    async fn representative_draw_document() -> DrawSnapshot {
        let mut assets = std::collections::BTreeMap::new();
        assets.insert("src-1".to_string(), DrawImageAsset { mime: "image/png".into(), data: "aGVsbG8=".into(), width: Some(8), height: Some(8) });

        let mut rect_shape = create_draw_shape_layer_rect("Rect");
        if let DrawLayerNode::Shape(shape) = &mut rect_shape {
            shape.base.attributes.fill = Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 10.0, y2: 10.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 1.0, 1.0] }] });
            shape.base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.5, cap: "round".into(), join: "round".into(), dash: Some(vec![2.0, 4.0]) });
        }
        let rect_id = layer_id(&rect_shape).to_string();

        let line_shape = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawLine { x1: 0.0, y1: 0.0, x2: 5.0, y2: 5.0 }), polygon: None });
        let line_id = layer_id(&line_shape).to_string();

        let polygon_shape = DrawLayerNode::Shape(DrawShapeBody {
            base: default_layer_base("Polygon"),
            shape_kind: "polygon".into(),
            rect: None,
            ellipse: None,
            circle: None,
            line: None,
            polygon: Some(DrawPolygon { points: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]] }),
        });

        let mut radial_circle = DrawShapeBody { base: default_layer_base("RadialCircle"), shape_kind: "circle".into(), rect: None, ellipse: None, circle: Some(DrawCircle { cx: 1.0, cy: 2.0, r: 3.0 }), line: None, polygon: None };
        radial_circle.base.attributes.fill = Some(FillStyle::RadialGradient { cx: 1.0, cy: 2.0, r: 3.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 0.0, 0.0] }] });
        let radial_circle = DrawLayerNode::Shape(radial_circle);

        let path_layer = create_draw_path_layer(
            "Path",
            vec![
                PathSegment::Move { to: [0.0, 0.0] },
                PathSegment::Line { to: [1.0, 0.0] },
                PathSegment::Quad { ctrl: [1.0, 1.0], to: [2.0, 1.0] },
                PathSegment::Cubic { ctrl1: [2.0, 2.0], ctrl2: [3.0, 2.0], to: [3.0, 3.0] },
                PathSegment::Arc { rx: 2.0, ry: 2.0, rotation: 0.0, large_arc: false, sweep: true, to: [1.0, -1.0] },
                PathSegment::Close,
            ],
        );

        let text_layer = DrawLayerNode::Text(DrawTextBody { base: default_layer_base("Label"), x: 4.0, y: 5.0, content: "semio \"draw\"\ndsl".into(), size: 12.0 });
        let image_layer = create_draw_image_layer("Image", "src-1");
        let trace_layer = create_draw_trace_layer("Trace", "src-1");
        let boolean_layer = create_draw_boolean_layer("Boolean", "xor", vec![rect_id, line_id]);

        let ellipse_shape =
            DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Ellipse"), shape_kind: "ellipse".into(), rect: None, ellipse: Some(DrawEllipse { cx: 1.0, cy: 2.0, rx: 3.0, ry: 4.0 }), circle: None, line: None, polygon: None });
        let group_layer = DrawLayerNode::Group(DrawGroupBody { base: default_layer_base("Group \"nested\""), children: vec![ellipse_shape, radial_circle] });

        DrawSnapshot {
            schema: DRAW_DOCUMENT_SCHEMA.into(),
            id: "dsl-fixture".into(),
            title: Some("DSL Fixture \"Quotes\" \\ backslash".into()),
            layers: vec![rect_shape, line_shape, polygon_shape, path_layer, text_layer, image_layer, trace_layer, boolean_layer, group_layer],
            assets: assets,
            artboard: Some(DrawArtboard { width: 640.0, height: 480.0 }),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trips_and_agrees_with_dsl() {
        let document = dsl::parse_dsl(dsl::SEMIO_DRAW_EXAMPLE_TEXT).expect("parse semio example");
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trips_representative_document() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&representative_draw_document());
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trips_document_without_assets_or_artboard() {
        let mut doc = default_draw_document("no-extras", None);
        doc.assets = Default::default();
        doc.artboard = None;
        doc.title = None;
        store::os_store::test_support::assert_dsl_pack_equivalence(&doc);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `DrawMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing pack round-trip laws.
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::draw::op::DrawMutation;
        use protocol::{ArtifactId, Edit, SchemaId};

        let initial = default_draw_document("doc-text-test", None);
        let envelope = store::create_document_envelope::<DrawSnapshot, DrawMutation>(DRAW_DOCUMENT_SCHEMA, "doc-text-test", initial, None);
        let mut doc_store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        let layer = create_draw_shape_layer_rect("Added Rect");
        let layer_id_value = layer_id(&layer).to_string();
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::artifacts::draw::mutations::create_layer(None, None, layer)], description: Some("add rect".into()) }).expect("apply add layer");
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::artifacts::draw::mutations::set_layer_opacity(layer_id_value, 0.5)], description: Some("set opacity".into()) }).expect("apply set opacity");
        let edit: &Edit<DrawMutation> = doc_store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<DrawSnapshot, DrawMutation>(edit, &ArtifactId(doc_store.envelope().id.clone()), &SchemaId(doc_store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
