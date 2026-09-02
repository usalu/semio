//! 📜️ Drawing artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::drawing::DrawingSnapshot;

/// 🗄️ The Semio emblem example fixture, handcrafted in `drawing`'s DSL (`store::ArtifactDsl`).
pub const SEMIO_DRAW_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ P6 handcrafted `ArtifactDsl` (derive no longer emits this trait) — relocated from
/// `🧬️schema/📸️snapshot/🦀️.rs` (design.md §1 CORRECTION: the native codec is one
/// bidirectional thing and sits unsplit at `🚪️io/<facet>/<representation>/`; `🧬️schema` keeps only
/// the `DrawingSnapshot` struct and its `Default` impl).
impl store::ArtifactDsl for DrawingSnapshot {
    const EXTENSION: &'static str = "drawing";
    fn envelope_id() -> &'static str {
        "drawing.drawing"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

/// 📖️ Parses `.drawing` DSL text into a `DrawingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<DrawingSnapshot, store::TextError> {
    <DrawingSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `DrawingSnapshot` back to `.drawing` DSL text.
pub fn print_dsl(document: &DrawingSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::drawing::schema::{create_drawing_boolean_layer, create_drawing_image_layer, create_drawing_path_layer, create_drawing_shape_layer_rect, create_drawing_trace_layer, default_drawing_document, default_layer_base, layer_id};
    use crate::artifacts::drawing::{DrawingArtboard, DrawingCircle, DrawingEllipse, DrawingGroupBody, DrawingImageAsset, DrawingLayerNode, DrawingLine, DrawingPolygon, DrawingShapeBody, DrawingTextBody, FillStyle, GradientStop, PathSegment, StrokeStyle, DRAWING_DOCUMENT_SCHEMA};
    use store::ArtifactDsl;

    fn representative_drawing_document() -> DrawingSnapshot {
        let mut assets = std::collections::BTreeMap::new();
        assets.insert("src-1".to_string(), DrawingImageAsset { mime: "image/png".into(), data: "aGVsbG8=".into(), width: Some(8), height: Some(8) });

        let mut rect_shape = create_drawing_shape_layer_rect("Rect");
        if let DrawingLayerNode::Shape(shape) = &mut rect_shape {
            shape.base.attributes.fill = Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 10.0, y2: 10.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 1.0, 1.0] }] });
            shape.base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.5, cap: "round".into(), join: "round".into(), dash: Some(vec![2.0, 4.0]) });
        }
        let rect_id = layer_id(&rect_shape).to_string();

        let line_shape = DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base("Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawingLine { x1: 0.0, y1: 0.0, x2: 5.0, y2: 5.0 }), polygon: None });
        let line_id = layer_id(&line_shape).to_string();

        let polygon_shape = DrawingLayerNode::Shape(DrawingShapeBody {
            base: default_layer_base("Polygon"),
            shape_kind: "polygon".into(),
            rect: None,
            ellipse: None,
            circle: None,
            line: None,
            polygon: Some(DrawingPolygon { points: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]] }),
        });

        let mut radial_circle = DrawingShapeBody { base: default_layer_base("RadialCircle"), shape_kind: "circle".into(), rect: None, ellipse: None, circle: Some(DrawingCircle { cx: 1.0, cy: 2.0, r: 3.0 }), line: None, polygon: None };
        radial_circle.base.attributes.fill = Some(FillStyle::RadialGradient { cx: 1.0, cy: 2.0, r: 3.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 0.0, 0.0] }] });
        let radial_circle = DrawingLayerNode::Shape(radial_circle);

        let path_layer = create_drawing_path_layer(
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

        let text_layer = DrawingLayerNode::Text(DrawingTextBody { base: default_layer_base("Label"), x: 4.0, y: 5.0, content: "semio \"drawing\"\ndsl".into(), size: 12.0 });
        let image_layer = create_drawing_image_layer("Image", "src-1");
        let trace_layer = create_drawing_trace_layer("Trace", "src-1");
        let boolean_layer = create_drawing_boolean_layer("Boolean", "xor", vec![rect_id, line_id]);

        let ellipse_shape =
            DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base("Ellipse"), shape_kind: "ellipse".into(), rect: None, ellipse: Some(DrawingEllipse { cx: 1.0, cy: 2.0, rx: 3.0, ry: 4.0 }), circle: None, line: None, polygon: None });
        let group_layer = DrawingLayerNode::Group(DrawingGroupBody { base: default_layer_base("Group \"nested\""), children: vec![ellipse_shape, radial_circle] });

        DrawingSnapshot {
            schema: DRAWING_DOCUMENT_SCHEMA.into(),
            id: "dsl-fixture".into(),
            title: Some("DSL Fixture \"Quotes\" \\ backslash".into()),
            layers: vec![rect_shape, line_shape, polygon_shape, path_layer, text_layer, image_layer, trace_layer, boolean_layer, group_layer],
            assets,
            artboard: Some(DrawingArtboard { width: 640.0, height: 480.0 }),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_round_trips_representative_document() {
        store::os_store::test_support::assert_dsl_round_trip(&representative_drawing_document());
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_round_trips_document_without_assets_or_artboard() {
        let mut doc = default_drawing_document("no-extras", None);
        doc.assets = Default::default();
        doc.artboard = None;
        doc.title = None;
        store::os_store::test_support::assert_dsl_round_trip(&doc);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_round_trips_semio_example_fixture() {
        let doc = parse_dsl(SEMIO_DRAW_EXAMPLE_TEXT).expect("semio example fixture parses");
        assert_eq!(doc.id, "semio");
        assert_eq!(doc.title.as_deref(), Some("Semio Emblem"));
        assert_eq!(doc.layers.len(), 1);
        store::os_store::test_support::assert_dsl_round_trip(&doc);
    }

    #[semio_framework_async_macros::async_test]
    async fn drawing_document_parse_dsl_reports_error_for_unknown_layer_kind() {
        let unknown_layer = DrawingSnapshot::parse_dsl("schema=\"drawing.document\" id=\"test\"\nlayers {\n  weird id=\"layer-1\"\n}\n");
        assert!(unknown_layer.is_err(), "an unrecognized layer keyword must fail to parse");
    }
}
//#endregion 🧪️Tests
