//! 📤️ `s.stdio.semio/v1/drawing` → `pdf` (1.7) — mirrors the import leaf's honest text-only
//! boundary: one `PdfPage` per `DrawLayer`, `media_box` from the drawing canvas, `text` built by
//! concatenating every `DrawNode::Text.value` found anywhere in that layer's tree (real recursive
//! walk). `Path`/`Group`(-transform)/`Image` nodes have no vector-graphics writer on this codec's
//! side (`encode_pdf` only regenerates a content stream FROM `PdfPage.text`, per that snapshot's
//! own module doc — it has no path-painting operator emission at all) and are dropped, documented,
//! not fabricated.

use semio_s_artifact_stdio_pdf::{schema::snapshot::PdfPage, PdfSnapshot};
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
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
        Ok(PdfSnapshot { schema: semio_s_artifact_stdio_pdf::schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA.into(), declared_version: "1.7".into(), pages, ..PdfSnapshot::default() })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
