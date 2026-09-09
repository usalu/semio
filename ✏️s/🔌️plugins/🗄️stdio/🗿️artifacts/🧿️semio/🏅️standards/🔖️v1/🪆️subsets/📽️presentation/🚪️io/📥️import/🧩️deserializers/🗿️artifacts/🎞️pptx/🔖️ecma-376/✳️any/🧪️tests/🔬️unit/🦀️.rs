use super::*;
use semio_s_artifact_stdio_pptx::schema::snapshot::{PptxPresentation, PptxSlide};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use semio_s_artifact_stdio_zip::opc::OpcPackage;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn sample_pptx() -> PptxSnapshot {
    PptxSnapshot::from_parts(
        OpcPackage::default(),
        Vec::new(),
        PptxPresentation {
            slides: vec![PptxSlide {
                shapes: vec![
                    PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Hello".into(), bold: true, italic: false, font_size: Some(24) }] }], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 20 } },
                    PptxShape::Picture { blip_rel_id: "rId2".into(), position: PptxTransform { x: 0, y: 30, cx: 50, cy: 50 } },
                    PptxShape::Placeholder { kind: "title".into(), text_frame: vec![PptxParagraph::text("Title text")], position: PptxTransform { x: 0, y: 0, cx: 200, cy: 40 } },
                    PptxShape::Other { node: XmlNode::Element { name: "p:graphicFrame".into(), attrs: Vec::new(), children: Vec::new() } },
                ],
            }],
        },
    )
}

#[semio_framework_async_macros::async_test]
async fn maps_shapes_and_drops_other() {
    let semio = semio_framework_plugin::resolve_ready(SemioPresentationFromPptx::deserialize(&sample_pptx())).expect("deserialize");
    assert!(semio.masters.is_empty() && semio.layouts.is_empty());
    assert_eq!(semio.slides.len(), 1);
    let slide = &semio.slides[0];
    assert_eq!(slide.id, "slide0");
    assert_eq!(slide.shapes.len(), 3, "Other{{xml}} shape must be dropped");
    assert!(matches!(&slide.shapes[0], SlideShape::TextBox { blocks, .. } if matches!(&blocks[0], DocBlock::Paragraph { runs, .. } if runs[0].text == "Hello" && runs[0].style.bold)));
    assert!(matches!(&slide.shapes[1], SlideShape::Picture { image, .. } if image.asset_id == "rId2" && image.bytes.is_empty()));
    assert!(matches!(&slide.shapes[2], SlideShape::Placeholder { kind: PlaceholderKind::Title, .. }));
}
