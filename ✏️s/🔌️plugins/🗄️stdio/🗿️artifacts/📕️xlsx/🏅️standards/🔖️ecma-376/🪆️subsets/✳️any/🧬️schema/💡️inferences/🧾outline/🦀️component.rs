//! 🧾 `outline` — one named inference: this SpreadsheetML workbook's own sheet/cell structure.
//! `sheetNames` is every sheet's `name`, in document order (`sectionOutline`-equivalent for a
//! workbook — a spreadsheet's own top-level sections ARE its sheets); `sheetCount` is
//! `sheetNames.len()`; `cellCount` is the total non-empty cell count across every sheet (a real
//! spreadsheet is sparse — only cells actually present in `XlsxSheet::cells` count, never a dense
//! row/col grid).

use crate::artifacts::xlsx::XlsxSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ `Xlsx` document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxOutline {
    pub sheet_names: Vec<String>,
    pub sheet_count: u32,
    pub cell_count: u32,
}

impl XlsxOutline {
    pub async fn compute(snapshot: &XlsxSnapshot) -> Self {
        let sheet_names: Vec<String> = snapshot.workbook.sheets.iter().map(|s| s.name.clone()).collect();
        let sheet_count = sheet_names.len() as u32;
        let cell_count = snapshot.workbook.sheets.iter().map(|s| s.cells.len() as u32).sum();
        Self { sheet_names, sheet_count, cell_count }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet};

    #[semio_framework_async_macros::async_test]
    async fn counts_sheets_and_cells() {
        let snapshot = XlsxSnapshot {
            schema: "stdio.xlsx".into(),
            opc: Default::default(),
            workbook: crate::artifacts::xlsx::schema::snapshot::XlsxWorkbook {
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
}
//#endregion 🧪️Tests
