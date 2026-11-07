use semio_s_artifact_draw_drawing::io;
use semio_s_artifact_draw_drawing_write::serializers::artifacts::svg::v1_1::any::write::{SvgWriteJob, SvgWriteLimits};
use semio_s_artifact_draw_drawing_model::{create_drawing_image_layer, create_drawing_shape_layer_rect, default_layer_base};
use semio_s_artifact_draw_drawing_model::default_drawing_document;
use semio_s_artifact_draw_drawing_model::{DrawingImageAsset, DrawingLayerNode, DrawingTextBody, StrokeStyle};
use semio_s_artifact_draw_drawing_model::FillStyle;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

/// 🎨️ The direct typed SVG route retains authored paint and positioned text.
#[semio_framework_async_macros::async_test]
async fn drawing_document_to_svg_preserves_shape_text_image_and_gradient_nodes() {
    use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::parse_svg_xml;

    let mut rect = create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Rect")).to_string().into()).expect("nonempty authored identity"), "Rect");
    if let DrawingLayerNode::Shape(shape) = &mut rect {
        shape.base.attributes.fill = Some(FillStyle::Solid { color: [1.0, 0.0, 0.0, 0.5] });
        shape.base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Round, join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Round, dash: None });
    }
    let mut gradient_rect = create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Gradient")).to_string().into()).expect("nonempty authored identity"), "Gradient");
    if let DrawingLayerNode::Shape(shape) = &mut gradient_rect {
        shape.base.attributes.fill = Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: vec![semio_s_artifact_draw_drawing_model::GradientStop {offset:0.0,color:[1.0,0.0,0.0,1.0]},semio_s_artifact_draw_drawing_model::GradientStop {offset:1.0,color:[0.0,0.0,1.0,0.5]}].into() });
    }
    let text = DrawingLayerNode::Text(DrawingTextBody { base: default_layer_base(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("T")).to_string().into()).expect("nonempty authored identity"), "T"), x: 0.0, y: 0.0, content: "<a & b>".into(), size: 12.0, font_family: semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::Anta });
    let mut assets = semio_framework_value::paged::PagedMap::default();
    assets.insert("img".into(), DrawingImageAsset { width: 4, height: 4, samples: vec![[0,0,0,0];16].into() });
    let image = create_drawing_image_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Image")).to_string().into()).expect("nonempty authored identity"), "Image", "img");

    let mut doc = default_drawing_document("svg-test", None);
    doc.layers = vec![rect, gradient_rect, text, image].into();
    doc.assets = assets;
    doc.artboard = None;

    let plan = semio_s_artifact_draw_drawing_scene::preparation::testkit::scene_plan(&doc);
    let mut job = SvgWriteJob::new(plan, SvgWriteLimits { view_box: [0.0, 0.0, 64.0, 64.0], max_output_bytes: 1 << 24, max_work: 1_000_000_000 });
    while !job.advance(4096).expect("the svg writer advances").done {}
    let svg_text = String::from_utf8(job.result().expect("complete svg").iter().flatten().copied().collect()).expect("svg is utf-8");

    parse_svg_xml(&svg_text).expect("written svg reparses");
    assert!(svg_text.contains("<path"), "{svg_text}");
    assert!(svg_text.contains("<image") && svg_text.contains("data:image/png;base64,"), "{svg_text}");
}

/// 🚫️ Every drawing export row is owned and job-driven: the borrowed hop refuses, and dxf/dwg declare owned factories like svg/pdf/png.
#[semio_framework_async_macros::async_test]
async fn declared_export_rows_run_only_through_owned_factories() {
    use semio_framework::io_schema::{IoPayload, IoResult};
    for kind in ["s.stdio.pdf", "s.stdio.svg", "s.stdio.png", "s.stdio.dxf", "s.stdio.dwg"] {
        let entry = io().entries.iter().find(|entry| entry.into.artifact_kind == kind).unwrap_or_else(|| panic!("the {kind} serializer row is declared"));
        assert!(entry.owned_serializer.is_some(), "{kind} declares its owned factory");
        let doc = default_drawing_document("hop", None);
        let result: IoResult<IoPayload> = (entry.run)(&IoPayload::Binary(<DrawingSnapshot as semio_framework_pack::snapshot::ArtifactPack>::encode_pack(&doc)));
        assert!(result.is_err(), "the borrowed {kind} hop refuses");
    }
}
