//! 📑️ Tsv editor — `main` window: a real, directly editable table of `TsvSnapshot.records`, built
//! from the framework `TableWindowKit` (contract §2.6). IANA TSV draws no header/data structural
//! distinction (unlike csv's optional convention) — every record renders as one editable row,
//! columns are synthesized positionally (`Column N`).

use crate::artifacts::tsv::TsvSnapshot;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::tsv::create_tsv_editor`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Table", "Tabelle"), icon_id: "table-2".into(), ..TableWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `TsvSnapshot -> UiNode`: one row per record, `set-cell`'s `row`/`column` index this
/// grid directly (a 1:1 mapping onto `records`, unlike csv's header-offset math).
pub async fn render(document: &TsvSnapshot) -> UiNode {
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

    #[test]
    async fn definition_declares_a_table_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[test]
    async fn render_lists_one_row_per_record() {
        let document = TsvSnapshot { schema: "stdio.tsv".into(), records: vec![vec!["a".into(), "b".into()]], trailing_newline: false, line_ending: Default::default() };
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.table.expect("table scene");
        let rows: Vec<Vec<String>> = serde_json::from_str(&scene.rows_json).expect("rows json");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][1], "b");
    }
}
//#endregion 🧪️Tests
