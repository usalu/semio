//! 📑️ Tsv editor — `main` window: a real, directly editable table of `TsvSnapshot.records`, built
//! from the framework `TableWindowKit` (contract §2.6). IANA TSV draws no header/data structural
//! distinction (unlike csv's optional convention) — every record renders as one editable row,
//! columns are synthesized positionally (`Column N`).

use crate::TsvSnapshot;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::tsv::create_tsv_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Table", "Tabelle"), icon_id: "table-2".into(), ..TableWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `TsvSnapshot -> BuiltNode`: one row per record, `set-cell`'s `row`/`column` index this
/// grid directly (a 1:1 mapping onto `records`, unlike csv's header-offset math).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &TsvSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let width = document.records.iter().map(|record| record.len()).max().unwrap_or(0);
    let columns = (0..width).map(|index| format!("Column {}", index + 1)).collect();
    let rows = document.records.clone();
    TableWindowKit::render(&TableView { columns, rows })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
