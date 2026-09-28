//! 🌡️ EPW editor — the `main` window: every hourly weather record as a directly editable table,
//! built from the framework `TableWindowKit` (contract §2.6). One row per `EpwRecord`, one column
//! per its 35 EnergyPlus Weather spec fields, in `EpwRecord::field_at`'s canonical wire order. The 8
//! header lines (LOCATION, DESIGN CONDITIONS, …, DATA PERIODS) are NOT surfaced here — a flat table
//! has no natural slot for scalar header fields, so editing them is out of this first pass's scope
//! (a documented limitation, not a silent drop; a future header-focused window could add them).

use crate::editor::epw::epw_row_revision;
use crate::EpwSnapshot;
use semio_framework_plugin::app::{EditableTableCell, TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, UiMapBuilder, UiText, UiValue, WindowKindDefinition};

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
    WindowKindDefinition { label: LocalizedLabel::native("Weather Records", "Wetterdatensätze"), icon_id: "table-2".into(), ..semio_s_artifact_stdio_contract::revision_addressed_table_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `EpwSnapshot -> BuiltNode`: one row per hourly record, all 35 spec columns — every column is
/// a real `set-cell` edit target (`EpwEditorCommand::SetCell`, keyed by row index + column name).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &EpwSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let columns = EPW_TABLE_COLUMNS.iter().map(|column| column.to_string()).collect::<Vec<_>>();
    let rows = document.records.iter().map(|record| record.fields().iter().map(|field| field.to_string()).collect()).collect();
    let editable = document
        .records
        .iter()
        .enumerate()
        .flat_map(|(row, record)| {
            let revision = epw_row_revision(record);
            (0..columns.len()).map(move |column| (row, column, revision.clone()))
        })
        .map(|(row, column, revision)| {
            let mut arguments = UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("epw.cell-arguments", "cell argument map capacity"))?;
            arguments.try_insert("row".into(), UiValue::Number(row as f64)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("epw.cell-arguments", "row argument capacity"))?;
            arguments.try_insert("column".into(), UiValue::Number(column as f64)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("epw.cell-arguments", "column argument capacity"))?;
            arguments
                .try_insert("revision".into(), UiValue::Text(UiText::try_from_string(revision).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("epw.cell-revision", "row revision exceeds the UI text bound"))?))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("epw.cell-arguments", "revision argument capacity"))?;
            Ok(EditableTableCell::new(row, column, "set-cell", UiValue::Map(arguments.finish())))
        })
        .collect::<semio_framework_plugin::UiAssemblyResult<Vec<_>>>()?;
    TableWindowKit::render_editable_cells(&TableView { columns, rows }, "s.stdio.epw@energyplus/*#editor", &editable)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
