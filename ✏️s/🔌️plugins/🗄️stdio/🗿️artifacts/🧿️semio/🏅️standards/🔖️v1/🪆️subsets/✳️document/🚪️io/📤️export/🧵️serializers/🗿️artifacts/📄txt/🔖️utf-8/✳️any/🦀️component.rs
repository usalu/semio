//! 📤️ Serialize `s.stdio.semio/v1/document` into a real `s.stdio.txt` (utf-8) snapshot — honest
//! best-effort plain-text extraction: every block contributes zero or more lines of its own
//! visible text, real content, never fabricated layout. `trailing_newline`/`line_ending` are set
//! to reasonable fixed defaults (`true`/`Lf`) since `SemioDocumentSnapshot` has no field to source
//! them from.
//!
//! Honest, documented losses (never fabricated):
//! - ALL formatting is dropped: `RunStyle` (bold/italic/underline/size/font/color/link), heading
//!   `level`, code `language`, list `ordered` flag, and list/quote/table STRUCTURE all disappear —
//!   only the visible text survives, flattened to one line per leaf block (list items and quote
//!   paragraphs are recursively flattened in document order; table rows join their cells with a
//!   tab character, one row per line).
//! - `Image` contributes its `alt` text only (bytes/mime dropped — plain text cannot carry raster
//!   data).
//! - `PageBreak` contributes NOTHING (dropped) rather than an empty line, since an empty line
//!   would be indistinguishable from (and so falsely suggest equivalence with) a genuinely empty
//!   paragraph on the way back through this pair's deserializer.

use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun};
use crate::artifacts::txt::schema::snapshot::LineEnding;
use crate::artifacts::txt::TxtSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

//#region 🔖️FieldMapping
async fn join_runs(runs: &[DocRun]) -> String {
    runs.iter().map(|r| r.text.as_str()).collect::<Vec<_>>().join("")
}

/// 🧱 One `DocBlock` -> zero or more plain-text lines.
pub(crate) async fn block_to_lines(block: &DocBlock) -> Vec<String> {
    match block {
        DocBlock::Paragraph { runs, .. } => vec![join_runs(runs).await],
        DocBlock::Heading { runs, .. } => vec![join_runs(runs).await],
        DocBlock::List { items, .. } => items.iter().flat_map(|item| item.blocks.iter().flat_map(block_to_lines)).collect(),
        DocBlock::Table { rows } => rows.iter().map(|row| row.cells.iter().map(|cell| cell.blocks.iter().flat_map(block_to_lines).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join("\t")).collect(),
        DocBlock::Code { text, .. } => text.lines().map(str::to_string).collect(),
        DocBlock::Quote { blocks } => blocks.iter().flat_map(block_to_lines).collect(),
        DocBlock::Image { alt, .. } => vec![alt.clone()],
        DocBlock::PageBreak => Vec::new(),
    }
}
//#endregion 🔖️FieldMapping

//#region 🔖️Serializer
pub struct SemioDocumentToTxt;

impl ArtifactSerializer for SemioDocumentToTxt {
    type From = SemioDocumentSnapshot;
    type Into = TxtSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let lines = from.blocks.iter().flat_map(block_to_lines).collect();
        Ok(TxtSnapshot { schema: crate::artifacts::txt::STDIO_TXT_DOCUMENT_SCHEMA.into(), lines, trailing_newline: true, line_ending: LineEnding::Lf })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocListItem, RunStyle, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};

    async fn sample_semio() -> SemioDocumentSnapshot {
        SemioDocumentSnapshot {
            schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: Vec::new(),
            images: Vec::new(),
            blocks: vec![
                DocBlock::Heading { level: 1, style_id: None, runs: vec![DocRun { text: "Title".into(), style: RunStyle { bold: true, ..Default::default() } }] },
                DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain("Body text.")] },
                DocBlock::List { ordered: true, items: vec![DocListItem { blocks: vec![DocBlock::paragraph("item one")] }] },
                DocBlock::PageBreak,
            ],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn extracts_plain_text_lines_and_drops_pagebreak() {
        let txt = semio_framework_plugin::resolve_ready(SemioDocumentToTxt::serialize(&sample_semio())).expect("serialize");
        assert_eq!(txt.lines, vec!["Title".to_string(), "Body text.".to_string(), "item one".to_string()]);
        assert!(txt.trailing_newline);
    }
}
//#endregion 🔖️Tests
