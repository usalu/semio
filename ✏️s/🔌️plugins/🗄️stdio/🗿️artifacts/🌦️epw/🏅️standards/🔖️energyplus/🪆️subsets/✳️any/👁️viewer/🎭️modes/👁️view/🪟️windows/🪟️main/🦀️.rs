//! 🌡️ EPW viewer — the `main` window: every hourly weather record as a real, READ-ONLY table, built
//! from the framework `TableWindowKit` (contract §2.6). Independent render from the sibling
//! mutation-capable surface — the same `EpwSnapshot.records` read, no edit affordances
//! (`window_kind()`, the read-only variant, not the editable one).

use crate::EpwSnapshot;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Weather Records", "Wetterdatensätze"), icon_id: "table-2".into(), ..TableWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `EpwSnapshot -> BuiltNode` read: one row per hourly record, all 35 spec columns, no edit
/// affordances.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &EpwSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let columns = EPW_TABLE_COLUMNS.iter().map(|column| column.to_string()).collect();
    let rows = document.records.iter().map(|record| record.fields().iter().map(|field| field.to_string()).collect()).collect();
    TableWindowKit::render(&TableView { columns, rows })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
