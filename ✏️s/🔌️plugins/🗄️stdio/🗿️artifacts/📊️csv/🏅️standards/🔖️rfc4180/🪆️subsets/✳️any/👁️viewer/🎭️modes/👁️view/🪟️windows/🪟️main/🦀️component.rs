//! 📊️ Csv viewer — `main` window: a real, READ-ONLY table of every `CsvRecord`, built from the
//! framework `TableWindowKit` (contract §2.6). Independent render from the sibling mutation-capable
//! surface — same header-row convention read, no edit affordances (`window_kind()`, the read-only
//! variant, not the editable one).

use crate::artifacts::csv::CsvSnapshot;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::csv::create_csv_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Table", "Tabelle"), icon_id: "table-2".into(), ..TableWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `CsvSnapshot -> BuiltNode` read: header row (if any) supplies column labels, every
/// remaining record is one read-only row — no mutation, no selection state.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &CsvSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (columns, data_rows): (Vec<String>, &[crate::artifacts::csv::CsvRecord]) = if document.has_header && !document.records.is_empty() {
        (document.records[0].fields.iter().map(|field| field.value.clone()).collect(), &document.records[1..])
    } else {
        let width = document.records.iter().map(|record| record.fields.len()).max().unwrap_or(0);
        ((0..width).map(|index| format!("Column {}", index + 1)).collect(), &document.records[..])
    };
    let rows = data_rows.iter().map(|record| record.fields.iter().map(|field| field.value.clone()).collect()).collect();
    TableWindowKit::render(&TableView { columns, rows })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use semio_framework_plugin::Component;
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_table_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_splits_header_from_data_rows() {
        let document = CsvSnapshot {
            schema: "stdio.csv".into(),
            has_header: true,
            records: vec![
                crate::artifacts::csv::CsvRecord { fields: vec![crate::artifacts::csv::CsvField { value: "name".into(), quoted: false }] },
                crate::artifacts::csv::CsvRecord { fields: vec![crate::artifacts::csv::CsvField { value: "ada".into(), quoted: false }] },
            ],
        };
        let node = render(&document).expect("render");
        let Component::Surface(props) = node.component else { panic!("expected a retained table surface") };
        let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(&props).expect("decode table scene");
        let rows: Vec<Vec<String>> = serde_json::from_str(&scene.rows_json).expect("rows json");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], "ada");
    }
}
//#endregion 🧪️Tests
