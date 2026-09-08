
use super::*;
use semio_s_artifact_stdio_svg::schema::snapshot::{CommonAttrs, PresentationAttrs, TransformOp};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_svg() -> SvgSnapshot {
    let svg_el = SvgElement::Svg {
        common: CommonAttrs::default(),
        view_box: Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 100.0, height: 50.0 }),
        width: None,
        height: None,
        xmlns: Some("http://www.w3.org/2000/svg".into()),
        children: vec![
            SvgElement::Circle { common: CommonAttrs { presentation: PresentationAttrs { fill: Some("#ff0000".into()), ..Default::default() }, ..Default::default() }, cx: 10.0, cy: 10.0, r: 5.0 },
            SvgElement::Group {
                common: CommonAttrs { transform: Some(vec![TransformOp::Translate { x: 3.0, y: Some(4.0) }]), ..Default::default() },
                children: vec![SvgElement::Path { common: CommonAttrs::default(), d: vec![PathCommand::MoveTo { x: 0.0, y: 0.0, relative: false }, PathCommand::LineTo { x: 5.0, y: 0.0, relative: true }, PathCommand::ClosePath] }],
            },
            SvgElement::Text { common: CommonAttrs::default(), x: Some(1.0), y: Some(2.0), children: vec![SvgElement::TextNode("hi".into())] },
        ],
    };
    SvgSnapshot { doc: XmlDocument { root: Some(semio_s_artifact_stdio_svg::schema::snapshot::svg_element_to_xml_node(&svg_el)), doctype: None, declaration: None, prolog: Vec::new() }, ..SvgSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn maps_canvas_shapes_group_transform_and_text() {
    let drawing = semio_framework_plugin::resolve_ready(SemioDrawingFromSvg::deserialize(&sample_svg())).expect("deserialize");
    assert_eq!(drawing.canvas.width, 100.0);
    assert_eq!(drawing.canvas.height, 50.0);
    assert_eq!(drawing.layers.len(), 1);
    let children = match &drawing.layers[0].root {
        DrawNode::Group { children, .. } => children,
        other => panic!("expected root Group, got {other:?}"),
    };
    assert_eq!(children.len(), 3);
    match &children[0] {
        DrawNode::Path { segments, style } => {
            assert!(matches!(segments[0], PathSegment::MoveTo { .. }));
            let style_name = style.as_ref().expect("circle should have an interned style");
            let s = drawing.styles.iter().find(|s| &s.name == style_name).unwrap();
            assert_eq!(s.fill, Some(SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }));
        }
        other => panic!("expected circle→Path, got {other:?}"),
    }
    match &children[1] {
        DrawNode::Group { transform, children } => {
            assert!((transform.translation.x - 3.0).abs() < 1e-9);
            assert!((transform.translation.y - 4.0).abs() < 1e-9);
            match &children[0] {
                DrawNode::Path { segments, .. } => {
                    assert_eq!(segments.len(), 3);
                    assert!(matches!(segments[1], PathSegment::LineTo { to } if (to.x - 5.0).abs() < 1e-9));
                }
                other => panic!("expected Path, got {other:?}"),
            }
        }
        other => panic!("expected translated Group, got {other:?}"),
    }
    match &children[2] {
        DrawNode::Text { value, at, .. } => {
            assert_eq!(value, "hi");
            assert_eq!(*at, SemioPoint2 { x: 1.0, y: 2.0 });
        }
        other => panic!("expected Text, got {other:?}"),
    }
}
