//! 📤️ `s.stdio.semio/v1/drawing` → `pdf` (1.7) — mirrors the import leaf's honest text-only
//! boundary: one `PdfPage` per `DrawLayer`, `media_box` from the drawing canvas, `text` built by
//! concatenating every `DrawNode::Text.value` found anywhere in that layer's tree (real recursive
//! walk). `Path`/`Group`(-transform)/`Image` nodes have no vector-graphics writer on this codec's
//! side (`encode_pdf` only regenerates a content stream FROM `PdfPage.text`, per that snapshot's
//! own module doc — it has no path-painting operator emission at all) and are dropped, documented,
//! not fabricated.

use crate::artifacts::pdf::{schema::snapshot::PdfPage, PdfSnapshot};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId::ANY };

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn collect_text(node: &DrawNode, out: &mut String) {
    match node {
        DrawNode::Text { value, .. } => {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(value);
        }
        DrawNode::Group { children, .. } => {
            for c in children {
                collect_text(c, out);
            }
        }
        DrawNode::Path { .. } | DrawNode::Image { .. } => {}
    }
}

//#region 🔖️Serializer
pub struct SemioDrawingToPdf;

impl ArtifactSerializer for SemioDrawingToPdf {
    type From = SemioDrawingSnapshot;
    type Into = PdfSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        if from.layers.is_empty() {
            return Err(store::PackError::Schema("semio/drawing→pdf: no layers to export".into()));
        }
        let pages = from
            .layers
            .iter()
            .map(|layer| {
                let mut text = String::new();
                collect_text(&layer.root, &mut text);
                PdfPage { text, ..PdfPage::new(from.canvas.width.max(1.0), from.canvas.height.max(1.0)) }
            })
            .collect();
        Ok(PdfSnapshot { schema: crate::artifacts::pdf::schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA.into(), declared_version: "1.7".into(), pages, ..PdfSnapshot::default() })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint2, SemioTransform};
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, PathSegment};

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

        let bytes = crate::artifacts::pdf::standards::v1_7::subsets::any::io::encode_pdf(&pdf).await.expect("encode real pdf bytes");
        let decoded = crate::artifacts::pdf::standards::v1_7::subsets::any::io::decode_pdf(&bytes).await.expect("decode real pdf bytes");
        assert_eq!(decoded.pages.len(), 1);
        assert_eq!(decoded.pages[0].text, "hello\nsemio");
    }
}
//#endregion 🔖️Tests
