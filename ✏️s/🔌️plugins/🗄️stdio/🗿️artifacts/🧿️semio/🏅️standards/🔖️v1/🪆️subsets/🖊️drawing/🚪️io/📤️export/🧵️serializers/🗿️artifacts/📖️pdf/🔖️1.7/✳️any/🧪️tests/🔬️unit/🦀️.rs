use super::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::png::v1_2::any::rasterize_drawing;
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer};
use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::{decode_pdf, encode_pdf};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn p(x: f64, y: f64) -> SemioPoint2 {
    SemioPoint2 { x, y }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn scene() -> SemioDrawingSnapshot {
    let styles = vec![
        DrawStyle { name: "red".into(), fill: Some(SemioRgba { r: 0.9, g: 0.1, b: 0.1, a: 1.0 }), stroke: None, stroke_width: None, opacity: None },
        DrawStyle { name: "outline".into(), fill: None, stroke: Some(SemioRgba { r: 0.1, g: 0.2, b: 0.9, a: 1.0 }), stroke_width: Some(6.0), opacity: None },
        DrawStyle { name: "ghost".into(), fill: Some(SemioRgba { r: 0.1, g: 0.8, b: 0.2, a: 1.0 }), stroke: None, stroke_width: None, opacity: Some(0.5) },
    ];
    let turned = SemioTransform { translation: SemioPoint3 { x: 150.0, y: 60.0, z: 0.0 }, rotation: SemioQuaternion { x: 0.0, y: 0.0, z: (0.3f64).sin(), w: (0.3f64).cos() }, scale: SemioPoint3 { x: 1.5, y: 0.75, z: 1.0 } };
    let square = |x0: f64, y0: f64, x1: f64, y1: f64, style: Option<&str>| DrawNode::Path { segments: vec![PathSegment::MoveTo { to: p(x0, y0) }, PathSegment::LineTo { to: p(x1, y0) }, PathSegment::LineTo { to: p(x1, y1) }, PathSegment::LineTo { to: p(x0, y1) }, PathSegment::Close], style: style.map(Into::into) };
    SemioDrawingSnapshot {
        canvas: DrawCanvas { width: 240.0, height: 160.0, background: None },
        styles,
        layers: vec![DrawLayer {
            id: "0".into(),
            name: "0".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    square(10.0, 10.0, 90.0, 70.0, Some("red")),
                    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: p(20.0, 140.0) }, PathSegment::CubicTo { c1: p(60.0, 80.0), c2: p(120.0, 200.0), to: p(200.0, 120.0) }], style: Some("outline".into()) },
                    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: p(100.0, 40.0) }, PathSegment::ArcTo { rx: 30.0, ry: 20.0, x_rotation: 15.0, large_arc: true, sweep: false, to: p(160.0, 40.0) }, PathSegment::QuadTo { c: p(130.0, 0.0), to: p(100.0, 40.0) }, PathSegment::Close], style: Some("ghost".into()) },
                    DrawNode::Group { transform: turned, children: vec![square(-20.0, -10.0, 20.0, 10.0, None)] },
                    DrawNode::Text { value: "hello".into(), at: p(10.0, 150.0), style: None },
                    DrawNode::Text { value: "semio".into(), at: p(60.0, 150.0), style: Some("red".into()) },
                ],
            },
        }],
        ..SemioDrawingSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn the_page_keeps_its_text_through_the_pdf_codec() {
    let pdf = SemioDrawingToPdf::serialize(&scene()).await.expect("serialize");
    let decoded = decode_pdf(&encode_pdf(&pdf).expect("encode")).expect("decode");
    assert_eq!(decoded.pages.len(), 1);
    assert_eq!(decoded.pages[0].media_box, [0.0, 0.0, 240.0, 160.0]);
    assert_eq!(decoded.pages[0].text(), "hello\nsemio");
    assert!(decoded.pages[0].content.iter().any(|op| matches!(op, PdfOp::CurveTo { .. })));
}

/// 🔮️ hayro (third-party PDF rasterizer, test-only) paints the pdf leaf's bytes; the geometry it
/// shows must match what the png leaf paints for the same drawing, text aside.
#[test]
fn hayro_paints_the_same_geometry_the_rasterizer_paints() {
    let mut drawing = scene();
    if let DrawNode::Group { children, .. } = &mut drawing.layers[0].root {
        children.retain(|node| !matches!(node, DrawNode::Text { .. }));
    }
    let pdf = drawing_to_pdf(&drawing).expect("pdf");
    let bytes = encode_pdf(&pdf).expect("encode");
    let document = hayro::Pdf::new(std::sync::Arc::new(bytes)).expect("hayro opens the pdf leaf output");
    let pages = document.pages();
    assert_eq!(pages.len(), 1);
    let pixmap = hayro::render(&pages[0], &hayro::InterpreterSettings::default(), &hayro::RenderSettings::default());
    assert_eq!((u32::from(pixmap.width()), u32::from(pixmap.height())), (240, 160));
    let theirs: Vec<f32> = pixmap.data_as_u8_slice().iter().map(|v| *v as f32 / 255.0).collect();
    let raster = rasterize_drawing(&drawing).expect("raster");
    let ours: Vec<f32> = raster
        .rgba8
        .chunks(4)
        .flat_map(|px| {
            let a = px[3] as f32 / 255.0;
            [px[0] as f32 / 255.0 * a + (1.0 - a), px[1] as f32 / 255.0 * a + (1.0 - a), px[2] as f32 / 255.0 * a + (1.0 - a), 1.0]
        })
        .collect();
    assert_eq!(ours.len(), theirs.len());
    let differences: Vec<f32> = ours.iter().zip(&theirs).map(|(a, b)| (a - b).abs()).collect();
    let mean = differences.iter().sum::<f32>() / differences.len() as f32;
    let far = differences.iter().filter(|d| **d > 0.25).count() as f32 / differences.len() as f32;
    assert!(mean < 0.01, "mean channel difference {mean}");
    assert!(far < 0.006, "fraction of channels differing by more than a quarter {far}");
}

#[test]
fn a_canvas_without_size_is_refused() {
    let mut drawing = scene();
    drawing.canvas.width = 0.0;
    assert!(drawing_to_pdf(&drawing).is_err());
}
