
use super::*;
use crate::schema::{create_drawing_image_layer, create_drawing_shape_layer_rect, default_drawing_document, default_layer_base};
use crate::{DrawingImageAsset, DrawingLayerNode, DrawingTextBody, StrokeStyle};

/// 🌉️ Ported from the pre-migration `drawing_document_to_svg_renders_shape_text_image_and_gradient_nodes`
/// (same shape/text/image/gradient coverage) onto the new `SemioDrawingSnapshot`→`io_dispatch`
/// bridge — decodes the real bridged SVG back into stdio's own typed `SvgElement` tree instead
/// of substring-matching hand-rolled markup, since the markup is no longer hand-rolled.
#[semio_framework_async_macros::async_test]
async fn drawing_document_to_svg_bridges_shape_text_image_and_gradient_nodes_through_semio_drawing() {
    use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::{SvgElement, parse_svg_xml, svg_element_from_xml_node};

    let mut rect = create_drawing_shape_layer_rect("Rect");
    if let DrawingLayerNode::Shape(shape) = &mut rect {
        shape.base.attributes.fill = Some(FillStyle::Solid { color: [1.0, 0.0, 0.0, 0.5] });
        shape.base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: "round".into(), join: "round".into(), dash: None });
    }
    let mut gradient_rect = create_drawing_shape_layer_rect("Gradient");
    if let DrawingLayerNode::Shape(shape) = &mut gradient_rect {
        shape.base.attributes.fill = Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: Vec::new() });
    }
    let text = DrawingLayerNode::Text(DrawingTextBody { base: default_layer_base("T"), x: 0.0, y: 0.0, content: "<a & b>".into(), size: 12.0 });
    let mut assets = std::collections::BTreeMap::new();
    assets.insert("img".to_string(), DrawingImageAsset { mime: "image/png".into(), data: "aGVsbG8=".into(), width: Some(4), height: Some(4) });
    let image = create_drawing_image_layer("Image", "img");

    let mut doc = default_drawing_document("svg-test", None);
    doc.layers = vec![rect, gradient_rect, text, image];
    doc.assets = assets;
    doc.artboard = None;

    let (svg_text, width, height) = drawing_document_to_svg(&doc).expect("svg export via semio/drawing bridge");
    assert!(width >= 1 && height >= 1);

    let reparsed = parse_svg_xml(&svg_text).expect("bridged svg reparses");
    let root = svg_element_from_xml_node(reparsed.root.as_ref().expect("svg root")).expect("typed svg root");
    let layer_children = match &root {
        SvgElement::Svg { children, .. } => match &children[0] {
            SvgElement::Group { children, .. } => children,
            other => panic!("expected layer group, got {other:?}"),
        },
        other => panic!("expected <svg> root, got {other:?}"),
    };
    assert_eq!(layer_children.len(), 4, "rect, gradient rect, text, image");
    let leaf = |index: usize| match &layer_children[index] {
        SvgElement::Group { children, .. } => &children[0],
        other => panic!("expected node wrapper group, got {other:?}"),
    };

    match leaf(0) {
        SvgElement::Path { common, .. } => assert!(common.presentation.fill.as_deref().is_some_and(|fill| fill.starts_with("rgba(255,")), "{:?}", common.presentation.fill),
        other => panic!("expected filled rect path, got {other:?}"),
    }
    match leaf(1) {
        SvgElement::Path { common, .. } => assert!(common.presentation.fill.is_none(), "gradients have no semio/drawing equivalent — dropped, not fabricated"),
        other => panic!("expected gradient rect path, got {other:?}"),
    }
    match leaf(2) {
        SvgElement::Text { children, .. } => assert_eq!(children, &vec![SvgElement::TextNode("<a & b>".into())]),
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

    let json_error = drawing_document_json_to_svg(&dsl::DslValue::object([("bad".to_string(), dsl::DslValue::Bool(true))]));
    assert!(json_error.is_err());
}
