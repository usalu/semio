//! 🌡️ EPW editor — the `main` window: every hourly weather record as a directly editable table,
//! built from the framework `TableWindowKit` (contract §2.6). One row per `EpwRecord`, one column
//! per its 35 EnergyPlus Weather spec fields, in `EpwRecord::field_at`'s canonical wire order. The 8
//! header lines (LOCATION, DESIGN CONDITIONS, …, DATA PERIODS) are NOT surfaced here — a flat table
//! has no natural slot for scalar header fields, so editing them is out of this first pass's scope
//! (a documented limitation, not a silent drop; a future header-focused window could add them).

use crate::artifacts::epw::EpwSnapshot;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;

/// 🔢️ The 35 EPW record columns, in `EpwRecord::field_at`'s canonical wire order — shared by
/// `render` (column headers + row cells) and the surface root's `EpwEditorCommand::SetCell` (column
/// name -> wire index lookup for `EpwMutation::SetRecordField`).
pub const EPW_TABLE_COLUMNS: [&str; 35] = [
    "year",
    "month",
    "day",
    "hour",
    "minute",
    "dataSourceUncertainty",
    "dryBulbTemp",
    "dewPointTemp",
    "relativeHumidity",
    "atmosphericPressure",
    "extraterrestrialHorizontalRadiation",
    "extraterrestrialDirectNormalRadiation",
    "horizontalInfraredRadiation",
    "globalHorizontalRadiation",
    "directNormalRadiation",
    "diffuseHorizontalRadiation",
    "globalHorizontalIlluminance",
    "directNormalIlluminance",
    "diffuseHorizontalIlluminance",
    "zenithLuminance",
    "windDirection",
    "windSpeed",
    "totalSkyCover",
    "opaqueSkyCover",
    "visibility",
    "ceilingHeight",
    "presentWeatherObservation",
    "presentWeatherCodes",
    "precipitableWater",
    "aerosolOpticalDepth",
    "snowDepth",
    "daysSinceLastSnowfall",
    "albedo",
    "liquidPrecipDepth",
    "liquidPrecipQuantity",
];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::epw::create_epw_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Weather Records", "Wetterdatensätze"), icon_id: "table-2".into(), ..TableWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `EpwSnapshot -> BuiltNode`: one row per hourly record, all 35 spec columns — every column is
/// a real `set-cell` edit target (`EpwEditorCommand::SetCell`, keyed by row index + column name).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &EpwSnapshot) -> BuiltNode {
    let columns = EPW_TABLE_COLUMNS.iter().map(|column| column.to_string()).collect();
    let rows = document.records.iter().map(|record| record.fields().iter().map(|field| field.to_string()).collect()).collect();
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
    async fn render_lists_one_row_per_record_with_35_columns() {
        let mut document = EpwSnapshot::default();
        document.records.push(Default::default());
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.table.expect("table scene");
        let columns: Vec<String> = serde_json::from_str(&scene.columns_json).expect("columns json");
        assert_eq!(columns.len(), 35);
        let rows: Vec<Vec<String>> = serde_json::from_str(&scene.rows_json).expect("rows json");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].len(), 35);
    }
}
//#endregion 🧪️Tests
