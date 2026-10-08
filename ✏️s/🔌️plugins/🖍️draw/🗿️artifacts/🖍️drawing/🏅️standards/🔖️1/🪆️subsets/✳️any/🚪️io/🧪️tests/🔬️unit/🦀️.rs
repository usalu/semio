use super::*;
use crate::schema::{create_drawing_image_layer, create_drawing_shape_layer_rect, default_layer_base};
use crate::standards::v1::subsets::any::schema::{default_drawing_document};
use crate::{DrawingImageAsset, DrawingLayerNode, DrawingTextBody, StrokeStyle};

/// 🎨️ The direct typed SVG route retains authored paint and positioned text.
#[semio_framework_async_macros::async_test]
async fn drawing_document_to_svg_preserves_shape_text_image_and_gradient_nodes() {
    use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::svg_element_from_xml_node;
    use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::SvgElement;
    use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::{parse_svg_xml};

    let mut rect = create_drawing_shape_layer_rect("Rect");
    if let DrawingLayerNode::Shape(shape) = &mut rect {
        shape.base.attributes.fill = Some(FillStyle::Solid { color: [1.0, 0.0, 0.0, 0.5] });
        shape.base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: crate::StrokeCap::Round, join: crate::StrokeJoin::Round, dash: None });
    }
    let mut gradient_rect = create_drawing_shape_layer_rect("Gradient");
    if let DrawingLayerNode::Shape(shape) = &mut gradient_rect {
        shape.base.attributes.fill = Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: vec![crate::GradientStop {offset:0.0,color:[1.0,0.0,0.0,1.0]},crate::GradientStop {offset:1.0,color:[0.0,0.0,1.0,0.5]}].into() });
    }
    let text = DrawingLayerNode::Text(DrawingTextBody { base: default_layer_base("T"), x: 0.0, y: 0.0, content: "<a & b>".into(), size: 12.0 });
    let mut assets = semio_framework_value::paged::PagedMap::default();
    assets.insert("img".into(), DrawingImageAsset { mime: "image/png".into(), data: "aGVsbG8=".into(), width: Some(4), height: Some(4) });
    let image = create_drawing_image_layer("Image", "img");

    let mut doc = default_drawing_document("svg-test", None);
    doc.layers = vec![rect, gradient_rect, text, image].into();
    doc.assets = assets;
    doc.artboard = None;

    let (svg_text, width, height) = drawing_document_to_svg(&doc).expect("direct SVG export");
    assert!(width >= 1 && height >= 1);

    let reparsed = parse_svg_xml(&svg_text).expect("bridged svg reparses");
    let root = svg_element_from_xml_node(reparsed.root.as_ref().expect("svg root")).expect("typed svg root");
    let layer_children: Vec<&SvgElement> = match &root {
        SvgElement::Svg { children, .. } => children.iter().filter(|child| matches!(child, SvgElement::Group { .. })).collect(),
        other => panic!("expected <svg> root, got {other:?}"),
    };
    assert_eq!(layer_children.len(), 4, "rect, gradient rect, text, image");
    let leaf = |index: usize| match &layer_children[index] {
        SvgElement::Group { children, .. } => &children[0],
        other => panic!("expected node wrapper group, got {other:?}"),
    };

    match leaf(0) {
        SvgElement::Path { common, .. } => assert!(common.presentation.fill.as_deref().is_some_and(|fill| fill.starts_with("rgb(255,")), "{:?}", common.presentation.fill),
        other => panic!("expected filled rect path, got {other:?}"),
    }
    match leaf(1) {
        SvgElement::Path { common, .. } => assert!(common.presentation.fill.as_deref().is_some_and(|fill| fill.starts_with("url(#draw-gradient-")), "gradient references its exported definition"),
        other => panic!("expected gradient rect path, got {other:?}"),
    }
    match leaf(2) {
        SvgElement::Text { common, children, .. } => {
            assert_eq!(common.presentation.font_size.as_deref(), Some("12"));
            assert!(matches!(&children[0], SvgElement::Tspan { children, .. } if children == &vec![SvgElement::TextNode("<a & b>".into())]));
        },
        other => panic!("expected text node, got {other:?}"),
    }
    match leaf(3) {
        SvgElement::Unknown { name, attrs, .. } => {
            assert_eq!(name, "image");
            let href = attrs.iter().find(|attr| attr.name == "href").expect("image href attr");
            assert!(href.value.starts_with("data:image/png;base64,"));
        }
        other => panic!("expected image node, got {other:?}"),
    }

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
    doc.layers.push(create_drawing_shape_layer_rect("Rect"));
    let result: IoResult<IoPayload> = (entry.run)(&IoPayload::Binary(<DrawingSnapshot as store::ArtifactPack>::encode_pack(&doc)));
    let outcome = result.expect("the pdf hop runs");
    let IoPayload::Binary(bytes) = &outcome.value else { panic!("pdf is a binary payload") };
    assert!(bytes.starts_with(b"%PDF-1.4\n"), "{:?}", &bytes[..16]);
    assert!(semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::io::decode_pdf(bytes).is_ok());
}
