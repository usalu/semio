//! 📤️ `s.stdio.semio/v1/drawing` → `pdf` (1.7) — mirrors the import leaf's honest text-only
//! boundary: one `PdfPage` per `DrawLayer`, `media_box` from the drawing canvas, and the page's
//! text built by concatenating every `DrawNode::Text.value` found anywhere in that layer's tree
//! (real recursive walk). Painting it is `pdf`'s own `io::text_document`, which turns those lines
//! into real content-stream operators with the font's own metrics — this leaf emits no operators
//! itself, per the zero-codec-reimplementation rule.
//!
//! Honest, documented losses (never fabricated):
//! - `Path`/`Group`(-transform)/`Image` nodes are dropped: this pair's text lane has no
//!   path-painting emission, and inventing one here would be writing a vector backend inside a
//!   conversion leaf.
//! - A layer whose text overflows its canvas loses the lines that do not fit — `text_document`
//!   lays each layer out on exactly ONE page, because `DrawLayer` is this subset's only page
//!   signal.

use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_pdf::{io::text_document, PdfSnapshot};

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
        let width = from.canvas.width.max(1.0);
        let height = from.canvas.height.max(1.0);
        let texts: Vec<String> = from
            .layers
            .iter()
            .map(|layer| {
                let mut text = String::new();
                collect_text(&layer.root, &mut text);
                text
            })
            .collect();
        let pages: Vec<(f64, f64, &str)> = texts.iter().map(|text| (width, height, text.as_str())).collect();
        Ok(text_document(&pages))
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
