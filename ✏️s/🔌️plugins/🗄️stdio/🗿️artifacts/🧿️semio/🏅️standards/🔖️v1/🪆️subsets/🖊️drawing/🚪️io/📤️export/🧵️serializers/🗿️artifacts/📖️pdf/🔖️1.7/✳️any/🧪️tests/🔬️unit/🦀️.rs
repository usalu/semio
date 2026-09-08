
use super::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, PathSegment};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_drawing() -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        canvas: DrawCanvas { width: 200.0, height: 100.0, background: None },
        layers: vec![DrawLayer {
            id: "0".into(),
            name: "0".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    DrawNode::Text { value: "hello".into(), at: SemioPoint2::default(), style: None },
                    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2::default() }, PathSegment::Close], style: None },
                    DrawNode::Text { value: "semio".into(), at: SemioPoint2::default(), style: None },
                ],
            },
        }],
        ..SemioDrawingSnapshot::default()
    }
}

/// 🧪️ Real round trip through pdf's own real writer/reader — `encode_pdf` regenerates a
/// content stream from `text` and `decode_pdf` re-extracts it, so this proves genuinely
/// working PDF bytes, not just a plausible struct.
#[semio_framework_async_macros::async_test]
async fn real_byte_round_trip_through_pdf_codec() {
    let drawing = sample_drawing();
    let pdf = semio_framework_plugin::resolve_ready(SemioDrawingToPdf::serialize(&drawing)).expect("serialize");
    assert_eq!(pdf.pages.len(), 1);
    assert_eq!(pdf.pages[0].text, "hello\nsemio");

    let bytes = semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::encode_pdf(&pdf).expect("encode real pdf bytes");
    let decoded = semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::decode_pdf(&bytes).expect("decode real pdf bytes");
    assert_eq!(decoded.pages.len(), 1);
    assert_eq!(decoded.pages[0].text, "hello\nsemio");
}
