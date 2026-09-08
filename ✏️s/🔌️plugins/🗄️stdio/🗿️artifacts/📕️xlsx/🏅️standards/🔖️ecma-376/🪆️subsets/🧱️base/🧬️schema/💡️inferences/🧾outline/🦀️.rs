//! 🧾 `outline` — one named inference: this SpreadsheetML workbook's own sheet/cell structure.
//! `sheetNames` is every sheet's `name`, in document order (`sectionOutline`-equivalent for a
//! workbook — a spreadsheet's own top-level sections ARE its sheets); `sheetCount` is
//! `sheetNames.len()`; `cellCount` is the total non-empty cell count across every sheet (a real
//! spreadsheet is sparse — only cells actually present in `XlsxSheet::cells` count, never a dense
//! row/col grid).

use crate::XlsxSnapshot;

//#region 🔖️Outline
/// 🧾️ `Xlsx` document outline.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XlsxOutline {
    pub sheet_names: Vec<String>,
    pub sheet_count: u32,
    pub cell_count: u32,
}

impl XlsxOutline {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compute(snapshot: &XlsxSnapshot) -> Self {
        let sheet_names: Vec<String> = snapshot.workbook.sheets.iter().map(|s| s.name.clone()).collect();
        let sheet_count = sheet_names.len() as u32;
        let cell_count = snapshot.workbook.sheets.iter().map(|s| s.cells.len() as u32).sum();
        Self { sheet_names, sheet_count, cell_count }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
