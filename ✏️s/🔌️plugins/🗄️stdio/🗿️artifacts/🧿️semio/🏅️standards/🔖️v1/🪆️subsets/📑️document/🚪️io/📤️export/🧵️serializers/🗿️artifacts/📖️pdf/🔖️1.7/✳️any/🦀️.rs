//! 📤️ Serialize `s.stdio.semio/v1/document` into a real `s.stdio.pdf` (1.7) snapshot — the mirror
//! of this pair's deserializer: blocks are grouped into pages by splitting on `DocBlock::PageBreak`,
//! and each group's plain-text extraction is handed to `pdf`'s own `io::text_document`, which paints
//! it as real content-stream operators (Helvetica 12 pt, margins and line breaking from the font's
//! own metrics) and registers the font on the snapshot — no byte-level PDF writing and no
//! operator synthesis happens here, per the zero-codec-reimplementation rule.
//!
//! Honest, documented losses (never fabricated):
//! - ALL formatting/structure inside a page collapses to plain joined lines (same scope as this
//!   subset's `document`<->`txt` pair): `RunStyle`, heading level, code language, list/table
//!   structure are all dropped — only visible text survives.
//! - `PdfInfo` is left at its default (empty) — `SemioDocumentSnapshot` has no metadata fields to
//!   source `title`/`author`/… from.
//! - `media_box` is fixed at US Letter (612x792pt) — `SemioDocumentSnapshot` has no page-size
//!   concept to draw a real value from.
//! - Text longer than one page overflows: `io::text_document` lays a group out on a SINGLE page and
//!   drops the lines that do not fit, because `DocBlock::PageBreak` is the only pagination signal
//!   `SemioDocumentSnapshot` actually carries — inventing extra page breaks would fabricate
//!   structure the source document never declared.

use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, SemioDocumentSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_pdf::{io::text_document, PdfSnapshot};

/// 📐️ US Letter, the only page size this subset can honestly claim.
const PAGE_WIDTH: f64 = 612.0;
const PAGE_HEIGHT: f64 = 792.0;

//#region 🔖️FieldMapping
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn join_runs(runs: &[crate::standards::v1::subsets::document::schema::snapshot::DocRun]) -> String {
    runs.iter().map(|r| r.text.as_str()).collect::<Vec<_>>().join("")
}

/// 🧱 One `DocBlock` -> zero or more plain-text lines (same honest flattening this group's
/// `document`<->`txt` pair uses).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn block_to_lines(block: &DocBlock) -> Vec<String> {
    match block {
        DocBlock::Paragraph { runs, .. } => vec![join_runs(runs)],
        DocBlock::Heading { runs, .. } => vec![join_runs(runs)],
        DocBlock::List { items, .. } => items.iter().flat_map(|item| item.blocks.iter().flat_map(block_to_lines)).collect(),
        DocBlock::Table { rows } => rows.iter().map(|row| row.cells.iter().map(|cell| cell.blocks.iter().flat_map(block_to_lines).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join("\t")).collect(),
        DocBlock::Code { text, .. } => text.lines().map(str::to_string).collect(),
        DocBlock::Quote { blocks } => blocks.iter().flat_map(block_to_lines).collect(),
        DocBlock::Image { alt, .. } => vec![alt.clone()],
        DocBlock::PageBreak => Vec::new(),
    }
}

/// 📄️ Blocks split on `DocBlock::PageBreak`, each group flattened to one page's plain text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_texts(blocks: &[DocBlock]) -> Vec<String> {
    if blocks.is_empty() {
        return Vec::new();
    }
    let mut pages = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for block in blocks {
        if matches!(block, DocBlock::PageBreak) {
            pages.push(current.join("\n"));
            current.clear();
        } else {
            current.extend(block_to_lines(block));
        }
    }
    pages.push(current.join("\n"));
    pages
}
//#endregion 🔖️FieldMapping

//#region 🔖️Serializer
pub struct SemioDocumentToPdf;

impl ArtifactSerializer for SemioDocumentToPdf {
    type From = SemioDocumentSnapshot;
    type Into = PdfSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId::ANY };

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let texts = page_texts(&from.blocks);
        let pages: Vec<(f64, f64, &str)> = texts.iter().map(|text| (PAGE_WIDTH, PAGE_HEIGHT, text.as_str())).collect();
        Ok(text_document(&pages))
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
