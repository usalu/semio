use super::*;
use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet};

#[semio_framework_async_macros::async_test]
async fn counts_sheets_and_cells() {
    let snapshot = XlsxSnapshot {
        schema: "stdio.xlsx".into(),
        opc: Default::default(),
        workbook: crate::schema::snapshot::XlsxWorkbook {
            sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }, XlsxSheet { name: "Sheet2".into(), cells: vec![] }],
            shared_strings: vec![],
        },
    };
    let outline = XlsxOutline::compute(&snapshot);
    assert_eq!(outline.sheet_names, vec!["Sheet1".to_string(), "Sheet2".to_string()]);
    assert_eq!(outline.sheet_count, 2);
    assert_eq!(outline.cell_count, 1);
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = XlsxSnapshot::default();
    assert_eq!(XlsxOutline::compute(&snapshot), XlsxOutline::compute(&snapshot));
}
