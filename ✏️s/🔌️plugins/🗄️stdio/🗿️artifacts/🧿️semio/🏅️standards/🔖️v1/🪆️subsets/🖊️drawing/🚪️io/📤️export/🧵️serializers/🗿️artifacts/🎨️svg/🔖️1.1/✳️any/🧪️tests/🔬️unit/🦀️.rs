use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_drawing() -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        canvas: DrawCanvas { width: 100.0, height: 50.0, background: None },
        styles: vec![DrawStyle { name: "s0".into(), fill: Some(SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: None, opacity: None }],
        layers: vec![DrawLayer {
            id: "0".into(),
            name: "root".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }, PathSegment::LineTo { to: SemioPoint2 { x: 10.0, y: 10.0 } }, PathSegment::Close], style: Some("s0".into()) },
                    DrawNode::Image { at: SemioPoint2 { x: 1.0, y: 2.0 }, width: 3.0, height: 4.0, mime: "image/png".into(), bytes: vec![1, 2, 3, 4, 5] },
                ],
            },
        }],
        ..SemioDrawingSnapshot::default()
    }
}

/// 🧪️ Real round trip through svg's own real XML text codec.
#[semio_framework_async_macros::async_test]
async fn real_text_round_trip_through_svg_codec() {
    let drawing = sample_drawing();
    let svg = semio_framework_plugin::resolve_ready(SemioDrawingToSvg::serialize(&drawing)).expect("serialize");
    let text = <SvgSnapshot as store::ArtifactDsl>::print_dsl(&svg);
    let reparsed = <SvgSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("reparse real svg text");
    let root = semio_s_artifact_stdio_svg::schema::snapshot::svg_element_from_xml_node(reparsed.doc.root.as_ref().unwrap()).expect("typed view");
    match &root {
        SvgElement::Svg { view_box, .. } => assert_eq!(*view_box, Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 100.0, height: 50.0 })),
        other => panic!("expected <svg>, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn image_node_round_trips_through_data_uri_convention() {
    let drawing = sample_drawing();
    let svg = semio_framework_plugin::resolve_ready(SemioDrawingToSvg::serialize(&drawing)).expect("serialize");
    let root = semio_s_artifact_stdio_svg::schema::snapshot::svg_element_from_xml_node(svg.doc.root.as_ref().unwrap()).expect("typed view");
    let layer_group = match &root {
        SvgElement::Svg { children, .. } => &children[0],
        _ => panic!("expected svg root"),
    };
    let image_el = match layer_group {
        SvgElement::Group { children, .. } => &children[1],
        _ => panic!("expected layer group"),
    };
    match image_el {
        SvgElement::Unknown { name, attrs, .. } => {
            assert_eq!(name, "image");
            let href = attrs.iter().find(|a| a.name == "href").expect("href attr");
            assert!(href.value.starts_with("data:image/png;base64,"));
        }
        other => panic!("expected Unknown(image), got {other:?}"),
    }
}
