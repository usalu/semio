use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::document::schema::snapshot::RunStyle;
use crate::standards::v1::subsets::presentation::schema::snapshot::{Slide, SlidePictureImage, STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio() -> SemioPresentationSnapshot {
    SemioPresentationSnapshot {
        schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(),
        masters: Vec::new(),
        layouts: Vec::new(),
        slides: vec![Slide {
            id: "slide0".into(),
            layout_id: None,
            shapes: vec![
                SlideShape::TextBox {
                    frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 100.0, height: 20.0 },
                    blocks: vec![DocBlock::Paragraph { style_id: None, runs: vec![DocRun { text: "Hi".into(), style: RunStyle { bold: true, ..Default::default() } }] }],
                },
                SlideShape::Picture { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 30.0 }, width: 50.0, height: 50.0 }, image: SlidePictureImage { asset_id: "rId2".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] } },
                SlideShape::Placeholder { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 200.0, height: 40.0 }, kind: PlaceholderKind::Title },
            ],
            notes: Vec::new(),
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn maps_shapes_positions_and_placeholder_kind() {
    let pptx = semio_framework_plugin::resolve_ready(SemioPresentationToPptx::serialize(&sample_semio())).expect("serialize");
    assert_eq!(pptx.presentation.slides.len(), 1);
    let shapes = &pptx.presentation.slides[0].shapes;
    assert_eq!(shapes.len(), 3);
    assert!(matches!(&shapes[0], PptxShape::TextBox { text_frame, position } if text_frame[0].runs[0].text == "Hi" && text_frame[0].runs[0].bold && position.cx == 100));
    assert!(matches!(&shapes[1], PptxShape::Picture { blip_rel_id, .. } if blip_rel_id == "rId2"));
    assert!(matches!(&shapes[2], PptxShape::Placeholder { kind, .. } if kind == "title"));
}
