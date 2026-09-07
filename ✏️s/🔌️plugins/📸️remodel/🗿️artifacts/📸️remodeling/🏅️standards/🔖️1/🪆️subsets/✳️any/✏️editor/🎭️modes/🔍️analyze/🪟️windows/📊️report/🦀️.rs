//! 📊️ Remodeling play app — the Report window: a Table surface over whichever reconstruction dataset the
//! config's `report_table` selects.

use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::editor::remodeling::config::RemodelingConfig;
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, TableScene, UiAssemblyResult, WindowEngagementSlot, WindowKindDefinition, WindowOptions};
// 🧬️ Two `SurfaceKind` enums coexist: `WindowKindDefinition` carries the retained `ui_wgpu` one
// (re-exported by the SDK root), while `scene_surface` takes the semantic contract's — same spelling,
// different types, so both are imported explicitly.
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;
use pack::JsonValue;

//#region 🔖️Constants
pub const REMODELING_PLAY_WINDOW_REPORT: &str = "remodeling-report";
pub const REMODELING_PLAY_BODY_REPORT: &str = "remodeling.play.report";
const REMODELING_PLAY_SURFACE_REPORT: &str = "remodeling.play.report";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: REMODELING_PLAY_WINDOW_REPORT.into(),
        label: LocalizedLabel::native("Report", "Bericht"),
        body_key: REMODELING_PLAY_BODY_REPORT.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "document-report".into(),
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::None },
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Scene
/// 🏷️ One table column. `sortable` is always emitted: `TableHost` (`🧰️framework/…/📊️Table/🟦️.tsx`)
/// reads `column.sortable` off the record and leaves the header inert when it is absent, so a column
/// set without the flag has a dead sort interaction. Every remodeling column carries scalar cells, so
/// every one of them sorts.
fn column(id: &str, label: &str) -> JsonValue {
    pack::json_object([("id".to_string(), JsonValue::from(id)), ("label".to_string(), JsonValue::from(label)), ("sortable".to_string(), JsonValue::from(true))])
}

/// 🧱️ One table row from its `(column id, cell)` pairs.
fn row(cells: impl IntoIterator<Item = (&'static str, JsonValue)>) -> JsonValue {
    pack::json_object(cells.into_iter().map(|(id, cell)| (id.to_string(), cell)))
}

/// 🔢️ An optional float cell — `null` where the document carries no measurement.
fn optional_number(value: Option<f32>) -> JsonValue {
    value.map_or(JsonValue::Null, |number| JsonValue::from(f64::from(number)))
}

/// 📊️ The `(columns_json, rows_json)` pair for one dataset name; any unknown name falls back to the
/// frame list.
fn report_table_json(scene: &RemodelingSnapshot, table: &str) -> (String, String) {
    let (columns, rows): (Vec<JsonValue>, Vec<JsonValue>) = match table {
        "cameras" => (
            vec![column("id", "Id"), column("model", "Model"), column("fx", "fx"), column("fy", "fy"), column("rms", "RMS (px)")],
            scene
                .calibration
                .cameras
                .iter()
                .map(|camera| {
                    row([
                        ("id", JsonValue::from(camera.id.as_str())),
                        ("model", JsonValue::from(camera.model.as_str())),
                        ("fx", JsonValue::from(camera.fx)),
                        ("fy", JsonValue::from(camera.fy)),
                        ("rms", optional_number(camera.rms_reprojection_px)),
                    ])
                })
                .collect(),
        ),
        "tracks" => (
            vec![column("id", "Id"), column("length", "Length"), column("class", "Class"), column("speed", "Mean Speed (m/s)")],
            scene
                .results
                .tracks
                .iter()
                .map(|track| {
                    row([
                        ("id", JsonValue::from(track.id.as_str())),
                        ("length", JsonValue::from(u64::from(track.length))),
                        ("class", JsonValue::from(format!("{:?}", track.class))),
                        ("speed", JsonValue::from(f64::from(track.mean_speed_m_s))),
                    ])
                })
                .collect(),
        ),
        "gcps" => (
            vec![column("id", "Id"), column("name", "Name"), column("x", "X"), column("y", "Y"), column("z", "Z"), column("observations", "Observations")],
            scene
                .gcps
                .iter()
                .map(|gcp| {
                    row([
                        ("id", JsonValue::from(gcp.id.as_str())),
                        ("name", JsonValue::from(gcp.name.as_str())),
                        ("x", JsonValue::from(gcp.world_position[0])),
                        ("y", JsonValue::from(gcp.world_position[1])),
                        ("z", JsonValue::from(gcp.world_position[2])),
                        ("observations", JsonValue::from(gcp.observations.len() as u64)),
                    ])
                })
                .collect(),
        ),
        "qcStages" => (
            vec![column("stage", "Stage"), column("status", "Status")],
            vec![row([("stage", JsonValue::from(format!("{:?}", scene.job.stage))), ("status", JsonValue::from(if scene.job.error.is_some() { "error" } else { "ok" }))])],
        ),
        "matches" => (
            vec![column("note", "Note")],
            vec![row([("note", JsonValue::from("Pairwise match data is reconstruction-runtime scratch, never distilled into durable document state."))])],
        ),
        _ => (
            vec![column("streamId", "Stream"), column("index", "Index"), column("timestampMs", "Timestamp (ms)"), column("assetId", "Asset")],
            scene
                .streams
                .iter()
                .flat_map(|stream| {
                    stream.frames.iter().map(move |frame| {
                        row([
                            ("streamId", JsonValue::from(stream.id.as_str())),
                            ("index", JsonValue::from(u64::from(frame.index))),
                            ("timestampMs", JsonValue::from(frame.timestamp_ms)),
                            ("assetId", JsonValue::from(frame.asset_id.as_str())),
                        ])
                    })
                })
                .collect(),
        ),
    };
    (pack::json_to_string(&pack::json_array(columns)), pack::json_to_string(&pack::json_array(rows)))
}

pub fn render(scene: &RemodelingSnapshot, config: &RemodelingConfig) -> UiAssemblyResult<BuiltNode> {
    let (columns_json, rows_json) = report_table_json(scene, &config.report_table);
    semio_framework_plugin::scene_surface(REMODELING_PLAY_SURFACE_REPORT, ContractSurfaceKind::Table, &TableScene::base(columns_json, rows_json))
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::remodeling::default_remodeling_scene;
    use crate::editor::remodeling::commands::set_report_table::SetReportTable;
    use crate::editor::remodeling::testkit::{app, dispatch, render as render_body};
    use crate::editor::remodeling::RemodelingCommand;

    #[semio_framework_async_macros::async_test]
    async fn every_dataset_name_yields_its_own_column_set_and_unknown_falls_back_to_frames() {
        let scene = default_remodeling_scene();
        for (table, marker) in [("cameras", "RMS (px)"), ("tracks", "Mean Speed (m/s)"), ("gcps", "Observations"), ("qcStages", "Status"), ("matches", "Note"), ("nonsense", "Timestamp (ms)")] {
            let (columns, _rows) = report_table_json(&scene, table);
            assert!(columns.contains(marker), "table {table} must expose {marker}: {columns}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn switching_the_selected_table_changes_the_rendered_columns() {
        let mut app = app().await;
        dispatch(&mut app, RemodelingCommand::SetReportTable(SetReportTable { table: "gcps".into() })).await;
        assert!(render_body(&mut app, REMODELING_PLAY_BODY_REPORT).await.contains("Observations"));
    }
}
//#endregion 🧪️Tests
