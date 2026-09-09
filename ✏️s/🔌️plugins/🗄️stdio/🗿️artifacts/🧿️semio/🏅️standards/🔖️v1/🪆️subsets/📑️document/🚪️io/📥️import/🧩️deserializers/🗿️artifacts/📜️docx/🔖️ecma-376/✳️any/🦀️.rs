//! 📥️ Deserialize `s.stdio.semio/v1/document` from a real `s.stdio.docx` (ecma-376) snapshot —
//! maps `DocxSnapshot`'s typed `word/document.xml` body + `word/styles.xml` view onto
//! `SemioDocumentSnapshot`'s block tree. Zero codec reimplementation: `DocxSnapshot` is already
//! decoded (this leaf only maps Snapshot -> Snapshot).
//!
//! Honest, documented losses (never fabricated):
//! - `DocxRun::extra_run_properties`, `DocxParagraph::extra_paragraph_properties`,
//!   `DocxTable{Row,Cell}::extra_*_properties` — raw-retained XML this docx model doesn't
//!   interpret (color/font/size/alignment/numbering/borders/…) has no `RunStyle`/`DocBlock`
//!   field to land in; dropped.
//! - `images` is always empty on import: docx media (headers/footers, `word/media/*`, drawings)
//!   lives in unmodeled `opc` parts at the typed-`DocxDocument` level, not reachable without
//!   re-parsing the OPC package's raw bytes, which this leaf must not do (zero-codec-reimplementation
//!   rule — that parsing already happened once, upstream, to produce `DocxSnapshot`).
//! - docx's block model only knows `Paragraph`/`Table` (no `Heading`/`List`/`Code`/`Quote`/`Image`/
//!   `PageBreak` distinctions) — every docx paragraph becomes `DocBlock::Paragraph` here, never a
//!   guessed `Heading` (guessing heading level from a style NAME string would be fabrication, not
//!   honest extraction).

use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, DocStyle, DocTableCell, DocTableRow, RunStyle, SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_docx::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxTable};
use semio_s_artifact_stdio_docx::DocxSnapshot;

//#region 🔖️FieldMapping
/// ✍️ `DocxRun` -> `DocRun`: text + the 3 boolean flags both models share. `size`/`font`/`color`/
/// `link` have no `DocxRun` source field (docx keeps them, if present, inside
/// `extra_run_properties` raw XML) so they stay `None`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn map_run(run: &DocxRun) -> DocRun {
    DocRun { text: run.text.clone(), style: RunStyle { bold: run.bold, italic: run.italic, underline: run.underline, size: None, font: None, color: None, link: None } }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn map_paragraph(p: &DocxParagraph) -> DocBlock {
    DocBlock::Paragraph { style_id: p.style.clone(), runs: p.runs.iter().map(map_run).collect() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn map_table(t: &DocxTable) -> DocBlock {
    DocBlock::Table { rows: t.rows.iter().map(|row| DocTableRow { cells: row.cells.iter().map(|cell| DocTableCell { blocks: cell.blocks.iter().map(map_block).collect() }).collect() }).collect() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn map_block(block: &DocxBlock) -> DocBlock {
    match block {
        DocxBlock::Paragraph(p) => map_paragraph(p),
        DocxBlock::Table(t) => map_table(t),
    }
}
//#endregion 🔖️FieldMapping

//#region 🔖️Deserializer
pub struct SemioDocumentFromDocx;

impl ArtifactDeserializer for SemioDocumentFromDocx {
    type From = DocxSnapshot;
    type Into = SemioDocumentSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(SemioDocumentSnapshot {
            schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: from.document.styles.iter().map(|s| DocStyle { id: s.id.clone(), name: s.name.clone(), based_on: s.based_on.clone() }).collect(),
            images: Vec::new(),
            blocks: from.document.body.iter().map(map_block).collect(),
        })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
