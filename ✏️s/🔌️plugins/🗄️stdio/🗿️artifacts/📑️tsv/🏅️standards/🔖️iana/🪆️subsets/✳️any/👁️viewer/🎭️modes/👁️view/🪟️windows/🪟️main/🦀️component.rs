//! 📑️ Tsv viewer — `main` window: a real, READ-ONLY table of `TsvSnapshot.records`, built from
//! the framework `TableWindowKit` (contract §2.6). Independent render from the sibling
//! mutation-capable surface — no edit affordances (`window_kind()`, the read-only variant).

use crate::artifacts::tsv::TsvSnapshot;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::tsv::create_tsv_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Table", "Tabelle"), icon_id: "table-2".into(), ..TableWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `TsvSnapshot -> BuiltNode` read: one row per record, no mutation, no selection state.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &TsvSnapshot) -> BuiltNode {
    let width = document.records.iter().map(|record| record.len()).max().unwrap_or(0);
    let columns = (0..width).map(|index| format!("Column {}", index + 1)).collect();
    let rows = document.records.clone();
    TableWindowKit::render(&TableView { columns, rows })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_table_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_lists_one_row_per_record() {
        let document = TsvSnapshot { schema: "stdio.tsv".into(), records: vec![vec!["a".into(), "b".into()]], trailing_newline: false, line_ending: Default::default() };
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.table.expect("table scene");
        let rows: Vec<Vec<String>> = serde_json::from_str(&scene.rows_json).expect("rows json");
        assert_eq!(rows.len(), 1);
    }
}
//#endregion 🧪️Tests
