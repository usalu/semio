//! 📊️ Remodel play app — the Report window: a Table surface over whichever reconstruction dataset the
//! config's `report_table` selects.

use crate::apps::remodel::config::RemodelConfig;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{build_table_scene, LocalizedLabel, SurfaceKind, TableScene, UiNode, WindowEngagementSlot, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const REMODEL_PLAY_WINDOW_REPORT: &str = "remodel-report";
pub const REMODEL_PLAY_BODY_REPORT: &str = "remodel.play.report";
const REMODEL_PLAY_SURFACE_REPORT: &str = "remodel.play.report";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: REMODEL_PLAY_WINDOW_REPORT.into(),
        label: LocalizedLabel::native("Report", "Bericht"),
        body_key: REMODEL_PLAY_BODY_REPORT.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "document-report".into(),
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::None },
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Scene
/// 📊️ The `(columns_json, rows_json)` pair for one dataset name; any unknown name falls back to the
/// frame list.
fn report_table_json(scene: &RemodelSnapshot, table: &str) -> (String, String) {
    let (columns, rows): (Vec<Value>, Vec<Value>) = match table {
        "cameras" => (
            vec![json!({ "id": "id", "label": "Id" }), json!({ "id": "model", "label": "Model" }), json!({ "id": "fx", "label": "fx" }), json!({ "id": "fy", "label": "fy" }), json!({ "id": "rms", "label": "RMS (px)" })],
            scene.calibration.cameras.iter().map(|camera| json!({ "id": camera.id, "model": camera.model, "fx": camera.fx, "fy": camera.fy, "rms": camera.rms_reprojection_px })).collect(),
        ),
        "tracks" => (
            vec![json!({ "id": "id", "label": "Id" }), json!({ "id": "length", "label": "Length" }), json!({ "id": "class", "label": "Class" }), json!({ "id": "speed", "label": "Mean Speed (m/s)" })],
            scene.results.tracks.iter().map(|track| json!({ "id": track.id, "length": track.length, "class": format!("{:?}", track.class), "speed": track.mean_speed_m_s })).collect(),
        ),
        "gcps" => (
            vec![
                json!({ "id": "id", "label": "Id" }),
                json!({ "id": "name", "label": "Name" }),
                json!({ "id": "x", "label": "X" }),
                json!({ "id": "y", "label": "Y" }),
                json!({ "id": "z", "label": "Z" }),
                json!({ "id": "observations", "label": "Observations" }),
            ],
            scene.gcps.iter().map(|gcp| json!({ "id": gcp.id, "name": gcp.name, "x": gcp.world_position[0], "y": gcp.world_position[1], "z": gcp.world_position[2], "observations": gcp.observations.len() })).collect(),
        ),
        "qcStages" => (vec![json!({ "id": "stage", "label": "Stage" }), json!({ "id": "status", "label": "Status" })], vec![json!({ "stage": format!("{:?}", scene.job.stage), "status": if scene.job.error.is_some() { "error" } else { "ok" } })]),
        "matches" => (vec![json!({ "id": "note", "label": "Note" })], vec![json!({ "note": "Pairwise match data is reconstruction-runtime scratch, never distilled into durable document state." })]),
        _ => (
            vec![json!({ "id": "streamId", "label": "Stream" }), json!({ "id": "index", "label": "Index" }), json!({ "id": "timestampMs", "label": "Timestamp (ms)" }), json!({ "id": "assetId", "label": "Asset" })],
            scene.streams.iter().flat_map(|stream| stream.frames.iter().map(move |frame| json!({ "streamId": stream.id, "index": frame.index, "timestampMs": frame.timestamp_ms, "assetId": frame.asset_id }))).collect(),
        ),
    };
    (serde_json::to_string(&columns).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()))
}

pub fn render(scene: &RemodelSnapshot, config: &RemodelConfig) -> UiNode {
    let (columns_json, rows_json) = report_table_json(scene, &config.report_table);
    build_table_scene(REMODEL_PLAY_SURFACE_REPORT, crate::apps::remodel::REMODEL_PLAY_APP_ID, TableScene::base(columns_json, rows_json))
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::commands::view::set_report_table::SetReportTable;
    use crate::apps::remodel::testkit::{app, dispatch, render as render_body};
    use crate::apps::remodel::RemodelCommand;
    use crate::artifacts::remodel::default_remodel_scene;

    #[test]
    fn every_dataset_name_yields_its_own_column_set_and_unknown_falls_back_to_frames() {
        let scene = default_remodel_scene();
        for (table, marker) in [("cameras", "RMS (px)"), ("tracks", "Mean Speed (m/s)"), ("gcps", "Observations"), ("qcStages", "Status"), ("matches", "Note"), ("nonsense", "Timestamp (ms)")] {
            let (columns, _rows) = report_table_json(&scene, table);
            assert!(columns.contains(marker), "table {table} must expose {marker}: {columns}");
        }
    }

    #[test]
    fn switching_the_selected_table_changes_the_rendered_columns() {
        let mut app = app();
        dispatch(&mut app, RemodelCommand::SetReportTable(SetReportTable { table: "gcps".into() }));
        assert!(render_body(&mut app, REMODEL_PLAY_BODY_REPORT).contains("Observations"));
    }
}
//#endregion 🧪️Tests
