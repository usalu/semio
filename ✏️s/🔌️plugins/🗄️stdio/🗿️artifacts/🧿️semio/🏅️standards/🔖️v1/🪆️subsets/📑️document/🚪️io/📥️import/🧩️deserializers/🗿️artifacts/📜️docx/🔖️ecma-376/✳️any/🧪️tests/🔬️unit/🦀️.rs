use super::*;
use semio_s_artifact_stdio_docx::schema::snapshot::{DocxDocument, DocxStyle, DocxTableCell, DocxTableRow};
use semio_s_artifact_stdio_zip::opc::OpcPackage;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn sample_docx() -> DocxSnapshot {
    DocxSnapshot::from_parts(
        OpcPackage::default(),
        DocxDocument {
            styles: vec![DocxStyle { id: "Heading1".into(), name: "Heading 1".into(), based_on: None }, DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: Some("Heading1".into()) }],
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

#[semio_framework_async_macros::async_test]
async fn maps_styles_paragraphs_and_tables() {
    let semio = semio_framework_plugin::resolve_ready(SemioDocumentFromDocx::deserialize(&sample_docx())).expect("deserialize");
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
