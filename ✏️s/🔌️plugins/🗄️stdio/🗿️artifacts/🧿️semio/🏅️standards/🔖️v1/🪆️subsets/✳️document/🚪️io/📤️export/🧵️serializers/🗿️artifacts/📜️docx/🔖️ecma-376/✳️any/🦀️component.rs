//! 📤️ Serialize `s.stdio.semio/v1/document` into a real `s.stdio.docx` (ecma-376) snapshot —
//! the mirror of this pair's deserializer. Zero codec reimplementation: only builds a
//! `DocxSnapshot` value (`opc` regenerated fresh by docx's own `engine::encode_docx` from
//! `document`, never touched here directly).
//!
//! Honest, documented losses (never fabricated):
//! - docx's block model only has `Paragraph`/`Table` — `List`/`Quote` FLATTEN (their nested
//!   blocks are emitted in document order, list/quote grouping is lost); `Code`'s `language` tag
//!   has no docx equivalent (only its `text` survives, as a plain paragraph); `Image` has no docx
//!   block type at this level (only `alt` text survives, as a plain paragraph — real image BYTES
//!   are never fabricated into a fake OPC media part); `PageBreak` has no docx block equivalent
//!   and is dropped.
//! - `Heading.level` is encoded into the paragraph's `style` (`"Heading{level}"`) only when no
//!   explicit `style_id` was already set — a real, standard-shaped convention (WordprocessingML's
//!   own default "HeadingN" style ids), not a fabrication.
//! - `RunStyle::{size,font,color,link}` have no `DocxRun` field and are dropped.

use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use crate::artifacts::docx::DocxSnapshot;
use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, SemioDocumentSnapshot};
use crate::artifacts::zip::opc::OpcPackage;

//#region 🔖️FieldMapping
fn map_semio_run(run: &DocRun) -> DocxRun {
    DocxRun { text: run.text.clone(), bold: run.style.bold, italic: run.style.italic, underline: run.style.underline, extra_run_properties: Vec::new() }
}

fn map_semio_runs(runs: &[DocRun]) -> Vec<DocxRun> {
    runs.iter().map(map_semio_run).collect()
}

/// 🧱 One `DocBlock` -> zero or more `DocxBlock`s: `List`/`Quote` flatten their children in place
/// (documented lossy: grouping is lost, content order is preserved); `PageBreak` drops to nothing
/// (docx has no page-break block).
pub(crate) fn map_semio_block(block: &DocBlock) -> Vec<DocxBlock> {
    match block {
        DocBlock::Paragraph { style_id, runs } => vec![DocxBlock::Paragraph(DocxParagraph { runs: map_semio_runs(runs), style: style_id.clone(), extra_paragraph_properties: Vec::new() })],
        DocBlock::Heading { level, style_id, runs } => {
            let style = style_id.clone().or_else(|| Some(format!("Heading{level}")));
            vec![DocxBlock::Paragraph(DocxParagraph { runs: map_semio_runs(runs), style, extra_paragraph_properties: Vec::new() })]
        }
        DocBlock::List { items, .. } => items.iter().flat_map(|item| item.blocks.iter().flat_map(map_semio_block)).collect(),
        DocBlock::Table { rows } => vec![DocxBlock::Table(DocxTable {
            rows: rows
                .iter()
                .map(|row| DocxTableRow {
                    cells: row.cells.iter().map(|cell| DocxTableCell { blocks: cell.blocks.iter().flat_map(map_semio_block).collect(), extra_cell_properties: Vec::new() }).collect(),
                    extra_row_properties: Vec::new(),
                })
                .collect(),
            extra_table_properties: Vec::new(),
        })],
        DocBlock::Code { text, .. } => vec![DocxBlock::paragraph(text.clone())],
        DocBlock::Quote { blocks } => blocks.iter().flat_map(map_semio_block).collect(),
        DocBlock::Image { alt, .. } => vec![DocxBlock::paragraph(alt.clone())],
        DocBlock::PageBreak => Vec::new(),
    }
}
//#endregion 🔖️FieldMapping

//#region 🔖️Serializer
pub struct SemioDocumentToDocx;

impl ArtifactSerializer for SemioDocumentToDocx {
    type From = SemioDocumentSnapshot;
    type Into = DocxSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let styles = from.styles.iter().map(|s| DocxStyle { id: s.id.clone(), name: s.name.clone(), based_on: s.based_on.clone() }).collect();
        let body = from.blocks.iter().flat_map(map_semio_block).collect();
        Ok(DocxSnapshot::from_parts(OpcPackage::default(), DocxDocument { body, styles }))
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocImage, DocStyle, DocTableCell, DocTableRow, RunStyle, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};

    fn sample_semio() -> SemioDocumentSnapshot {
        SemioDocumentSnapshot {
            schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: vec![DocStyle { id: "Heading1".into(), name: "Heading 1".into(), based_on: None }],
            images: Vec::new(),
            blocks: vec![
                DocBlock::Heading { level: 1, style_id: None, runs: vec![DocRun { text: "Title".into(), style: RunStyle { bold: true, ..Default::default() } }] },
                DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain("Body")] },
                DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] },
            ],
        }
    }

    #[test]
    fn maps_heading_paragraph_and_table() {
        let docx = SemioDocumentToDocx::serialize(&sample_semio()).expect("serialize");
        assert_eq!(docx.document.styles.len(), 1);
        assert_eq!(docx.document.body.len(), 3);
        assert!(matches!(&docx.document.body[0], DocxBlock::Paragraph(p) if p.style.as_deref() == Some("Heading1") && p.runs[0].bold));
        assert!(matches!(&docx.document.body[1], DocxBlock::Paragraph(p) if p.style.is_none()));
        assert!(matches!(&docx.document.body[2], DocxBlock::Table(t) if t.rows.len() == 1));
    }

    #[test]
    fn list_and_quote_flatten_image_and_pagebreak_drop() {
        let snap = SemioDocumentSnapshot {
            schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: Vec::new(),
            images: vec![DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] }],
            blocks: vec![
                DocBlock::List { ordered: true, items: vec![crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::DocListItem { blocks: vec![DocBlock::paragraph("item one")] }] },
                DocBlock::Quote { blocks: vec![DocBlock::paragraph("quoted")] },
                DocBlock::Image { image_id: "img1".into(), alt: "alt text".into(), width: None, height: None },
                DocBlock::PageBreak,
            ],
        };
        let docx = SemioDocumentToDocx::serialize(&snap).expect("serialize");
        // list item + quote paragraph + image-alt paragraph = 3 blocks; PageBreak drops entirely.
        assert_eq!(docx.document.body.len(), 3);
        assert!(matches!(&docx.document.body[2], DocxBlock::Paragraph(p) if p.runs[0].text == "alt text"));
    }
}
//#endregion 🔖️Tests
