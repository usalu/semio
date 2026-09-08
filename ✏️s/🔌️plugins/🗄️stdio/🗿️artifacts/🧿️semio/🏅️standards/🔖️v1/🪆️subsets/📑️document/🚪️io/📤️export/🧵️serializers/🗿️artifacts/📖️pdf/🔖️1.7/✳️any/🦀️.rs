//! 📤️ Serialize `s.stdio.semio/v1/document` into a real `s.stdio.pdf` (1.7) snapshot — the mirror
//! of this pair's deserializer: blocks are grouped into pages by splitting on `DocBlock::PageBreak`,
//! and each page's text is the plain-text extraction of its group's blocks (real content, not
//! fabricated layout — `pdf`'s own `engine::encode_pdf` regenerates a fresh minimal PDF from
//! `pages`+`info` alone, so no byte-level PDF writing happens here, per the zero-codec-
//! reimplementation rule).
//!
//! Honest, documented losses (never fabricated):
//! - ALL formatting/structure inside a page collapses to plain joined lines (same scope as this
//!   subset's `document`<->`txt` pair): `RunStyle`, heading level, code language, list/table
//!   structure are all dropped — only visible text survives.
//! - `PdfInfo` is left at its default (empty) — `SemioDocumentSnapshot` has no metadata fields to
//!   source `title`/`author`/… from.
//! - `media_box` is fixed at US Letter (612x792pt) — `SemioDocumentSnapshot` has no page-size
//!   concept to draw a real value from.

use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};
use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, SemioDocumentSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn make_page(lines: &[String]) -> PdfPage {
    let mut page = PdfPage::new(612.0, 792.0);
    page.text = lines.join("\n");
    page
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
        let mut pages = Vec::new();
        if !from.blocks.is_empty() {
            let mut current: Vec<String> = Vec::new();
            for block in &from.blocks {
                if matches!(block, DocBlock::PageBreak) {
                    pages.push(make_page(&current));
                    current.clear();
                } else {
                    current.extend(block_to_lines(block));
                }
            }
            pages.push(make_page(&current));
        }
        Ok(PdfSnapshot { schema: semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA.into(), declared_version: "1.7".into(), pages, info: PdfInfo::default(), objects: Vec::new(), trailer: Vec::new() })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
