//! 🌡️ EPW viewer — the `main` window: every hourly weather record as a real, READ-ONLY table, built
//! from the framework `TableWindowKit` (contract §2.6). Independent render from the sibling
//! mutation-capable surface — the same `EpwSnapshot.records` read, no edit affordances
//! (`window_kind()`, the read-only variant, not the editable one).

use crate::artifacts::epw::EpwSnapshot;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;

/// 🔢️ The 35 EPW record columns, in `EpwRecord::field_at`'s canonical wire order — mirrors the
/// sibling authoring surface's own constant (the substring the sibling module would be found under
/// must never be imported here, so this is its own independent copy, not a re-export).
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
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::epw::create_epw_viewer`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Weather Records", "Wetterdatensätze"), icon_id: "table-2".into(), ..TableWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `EpwSnapshot -> UiNode` read: one row per hourly record, all 35 spec columns, no edit
/// affordances.
pub async fn render(document: &EpwSnapshot) -> UiNode {
    let columns = EPW_TABLE_COLUMNS.iter().map(|column| column.to_string()).collect();
    let rows = document.records.iter().map(|record| record.fields().iter().map(|field| field.to_string()).collect()).collect();
    TableWindowKit::render(&TableView { columns, rows })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn definition_declares_a_read_only_table_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
        assert!(def.actions.is_empty(), "a viewer window kind declares no mutation-shaped actions");
    }

    #[test]
    async fn render_lists_one_row_per_record_with_35_columns() {
        let mut document = EpwSnapshot::default();
        document.records.push(Default::default());
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.table.expect("table scene");
        let columns: Vec<String> = serde_json::from_str(&scene.columns_json).expect("columns json");
        assert_eq!(columns.len(), 35);
        let rows: Vec<Vec<String>> = serde_json::from_str(&scene.rows_json).expect("rows json");
        assert_eq!(rows.len(), 1);
    }
}
//#endregion 🧪️Tests
