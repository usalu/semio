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

use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use crate::artifacts::docx::DocxSnapshot;
use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxTable};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{
    DocBlock, DocRun, DocStyle, DocTableCell, DocTableRow, RunStyle, SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA,
};

//#region 🔖️FieldMapping
/// ✍️ `DocxRun` -> `DocRun`: text + the 3 boolean flags both models share. `size`/`font`/`color`/
/// `link` have no `DocxRun` source field (docx keeps them, if present, inside
/// `extra_run_properties` raw XML) so they stay `None`.
pub(crate) fn map_run(run: &DocxRun) -> DocRun {
    DocRun { text: run.text.clone(), style: RunStyle { bold: run.bold, italic: run.italic, underline: run.underline, size: None, font: None, color: None, link: None } }
}

fn map_paragraph(p: &DocxParagraph) -> DocBlock {
    DocBlock::Paragraph { style_id: p.style.clone(), runs: p.runs.iter().map(map_run).collect() }
}

fn map_table(t: &DocxTable) -> DocBlock {
    DocBlock::Table {
        rows: t.rows.iter().map(|row| DocTableRow { cells: row.cells.iter().map(|cell| DocTableCell { blocks: cell.blocks.iter().map(map_block).collect() }).collect() }).collect(),
    }
}

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

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
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
mod tests {
    use super::*;
    use crate::artifacts::docx::schema::snapshot::{DocxDocument, DocxStyle, DocxTableCell, DocxTableRow};
    use crate::artifacts::zip::opc::OpcPackage;

    pub(crate) fn sample_docx() -> DocxSnapshot {
        DocxSnapshot::from_parts(
            OpcPackage::default(),
            DocxDocument {
                styles: vec![
                    DocxStyle { id: "Heading1".into(), name: "Heading 1".into(), based_on: None },
                    DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: Some("Heading1".into()) },
                ],
                body: vec![
                    DocxBlock::Paragraph(DocxParagraph {
                        runs: vec![DocxRun { text: "Title".into(), bold: true, italic: false, underline: false, extra_run_properties: Vec::new() }],
                        style: Some("Heading1".into()),
                        extra_paragraph_properties: Vec::new(),
                    }),
                    DocxBlock::Paragraph(DocxParagraph {
                        runs: vec![DocxRun { text: "Body text.".into(), bold: false, italic: true, underline: false, extra_run_properties: Vec::new() }],
                        style: Some("Normal".into()),
                        extra_paragraph_properties: Vec::new(),
                    }),
                    DocxBlock::Table(DocxTable {
                        rows: vec![DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("cell one")], extra_cell_properties: Vec::new() }], extra_row_properties: Vec::new() }],
                        extra_table_properties: Vec::new(),
                    }),
                ],
            },
        )
    }

    #[test]
    fn maps_styles_paragraphs_and_tables() {
        let semio = SemioDocumentFromDocx::deserialize(&sample_docx()).expect("deserialize");
        assert_eq!(semio.styles.len(), 2);
        assert_eq!(semio.styles[1].based_on.as_deref(), Some("Heading1"));
        assert_eq!(semio.blocks.len(), 3);
        assert!(matches!(&semio.blocks[0], DocBlock::Paragraph { style_id: Some(s), runs } if s == "Heading1" && runs[0].style.bold));
        assert!(matches!(&semio.blocks[1], DocBlock::Paragraph { style_id: Some(s), runs } if s == "Normal" && runs[0].style.italic));
        match &semio.blocks[2] {
            DocBlock::Table { rows } => {
                assert_eq!(rows.len(), 1);
                assert_eq!(rows[0].cells.len(), 1);
                assert!(matches!(&rows[0].cells[0].blocks[0], DocBlock::Paragraph { .. }));
            }
            other => panic!("expected Table, got {other:?}"),
        }
        assert!(semio.images.is_empty(), "docx typed model carries no media at this level — documented drop");
    }
}
//#endregion 🔖️Tests
