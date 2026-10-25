use semio_s_artifact_draw_drawing::io;
use semio_s_artifact_draw_drawing_write::{drawing_document_json_to_svg, drawing_document_to_svg};
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

    let (svg_text, width, height) = drawing_document_to_svg(&doc).expect("direct SVG export");
    assert!(width >= 1 && height >= 1);

    parse_svg_xml(&svg_text).expect("bridged svg reparses");
    assert_eq!(svg_text.matches("<g").count(), 4, "rect, gradient rect, text, image wrappers");
    assert!(svg_text.contains("fill=\"rgb(255,"), "{svg_text}");
    assert!(svg_text.contains("url(#draw-gradient-"), "gradient references its exported definition");
    assert!(svg_text.contains("font-size=\"12\""));
    assert!(svg_text.contains("&lt;a &amp; b&gt;"));
    assert!(svg_text.contains("<image") && svg_text.contains("data:image/png;base64,"));

    let json_error = drawing_document_json_to_svg(&semio_framework_value::DslValue::object([("bad".to_string(), semio_framework_value::DslValue::Bool(true))]));
    assert!(json_error.is_err());
}

/// 📖️ The declared `s.draw.drawing → s.stdio.pdf@1.4` io row is a real hop now: fed the document's
/// native pack it answers PDF bytes, not the old "not yet implemented" refusal.
#[semio_framework_async_macros::async_test]
async fn declared_pdf_serializer_entry_answers_a_pdf_for_the_native_pack() {
    use semio_framework::io_schema::{IoPayload, IoResult};
    let entry = io().entries.iter().find(|entry| entry.into.artifact_kind == "s.stdio.pdf").expect("the pdf serializer row is declared");
    let mut doc = default_drawing_document("hop", None);
    doc.layers.push(create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Rect")).to_string().into()).expect("nonempty authored identity"), "Rect"));
    let result: IoResult<IoPayload> = (entry.run)(&IoPayload::Binary(<DrawingSnapshot as semio_framework_pack::snapshot::ArtifactPack>::encode_pack(&doc)));
    let outcome = result.expect("the pdf hop runs");
    let IoPayload::Binary(bytes) = &outcome.value else { panic!("pdf is a binary payload") };
    assert!(bytes.starts_with(b"%PDF-1.4\n"), "{:?}", &bytes[..16]);
    assert!(semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::io::decode_pdf(bytes).is_ok());
}
