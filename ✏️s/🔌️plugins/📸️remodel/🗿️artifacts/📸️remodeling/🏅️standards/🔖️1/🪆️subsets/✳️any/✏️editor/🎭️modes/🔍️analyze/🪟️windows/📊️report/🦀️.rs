//! 📊️ Remodeling play app — the Report window: a Table surface over whichever reconstruction dataset the
//! config's `report_table` selects.

use crate::editor::remodeling::modes::analyze::windows::report::config::RemodelingReportWindowConfig;
use crate::RemodelingSnapshot;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::TableScene;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::WindowEngagementSlot;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
// 🧬️ Two `SurfaceKind` enums coexist: `WindowKindDefinition` carries the retained `ui_wgpu` one
// (re-exported by the SDK root), while `scene_surface` takes the semantic contract's — same spelling,
// different types, so both are imported explicitly.
use semio_framework_pack_json::Value;
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;

//#region 🔖️Constants
pub const REMODELING_PLAY_WINDOW_REPORT: &str = "remodeling-report";
pub const REMODELING_PLAY_BODY_REPORT: &str = "remodeling.play.report";
const REMODELING_PLAY_SURFACE_REPORT: &str = "remodeling.play.report";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
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
fn column(id: &str, label: &str) -> Value {
    semio_framework_pack_json::object([("id".to_string(), Value::from(id)), ("label".to_string(), Value::from(label)), ("sortable".to_string(), Value::from(true))])
}

/// 🧱️ One table row from its `(column id, cell)` pairs.
fn row(cells: impl IntoIterator<Item = (&'static str, Value)>) -> Value {
    semio_framework_pack_json::object(cells.into_iter().map(|(id, cell)| (id.to_string(), cell)))
}

/// 🔢️ An optional float cell — `null` where the document carries no measurement.
fn optional_number(value: Option<f32>) -> Value {
    value.map_or(Value::Null, |number| Value::from(f64::from(number)))
}

/// 📊️ The `(columns_json, rows_json)` pair for one dataset name; any unknown name falls back to the
/// frame list.
fn report_table_json(scene: &RemodelingSnapshot, table: &str) -> (String, String) {
    let (columns, rows): (Vec<Value>, Vec<Value>) = match table {
        "cameras" => (
            vec![column("id", "Id"), column("model", "Model"), column("fx", "fx"), column("fy", "fy"), column("rms", "RMS (px)")],
            scene
                .calibration
                .cameras
                .iter()
                .map(|camera| {
                    row([("id", Value::from(camera.id.as_str())), ("model", Value::from(camera.model.as_str())), ("fx", Value::from(camera.fx)), ("fy", Value::from(camera.fy)), ("rms", optional_number(camera.rms_reprojection_px))])
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
                    row([("id", Value::from(track.id.as_str())), ("length", Value::from(u64::from(track.length))), ("class", Value::from(format!("{:?}", track.class))), ("speed", Value::from(f64::from(track.mean_speed_m_s)))])
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
                        ("id", Value::from(gcp.id.as_str())),
                        ("name", Value::from(gcp.name.as_str())),
                        ("x", Value::from(gcp.world_position[0])),
                        ("y", Value::from(gcp.world_position[1])),
                        ("z", Value::from(gcp.world_position[2])),
                        ("observations", Value::from(gcp.observations.len() as u64)),
                    ])
                })
                .collect(),
        ),
        "qcStages" => (
            vec![column("check", "Check"), column("value", "Value")],
            scene.results.qc.as_ref().map_or_else(Vec::new, |qc| {
                let mut rows = vec![row([("check", Value::from("reprojectionRmsPx")), ("value", Value::from(qc.reprojection_rms_px))]), row([("check", Value::from("registeredFrameRatio")), ("value", Value::from(f64::from(qc.registered_frame_ratio)))])];
                rows.extend(qc.warnings.iter().map(|warning| row([("check", Value::from("warning")), ("value", Value::from(warning.as_str()))])));
                rows
            }),
        ),
        "matches" => (vec![column("note", "Note")], vec![row([("note", Value::from("Pairwise match data is reconstruction-runtime scratch, never distilled into durable document state."))])]),
        _ => (
            vec![column("streamId", "Stream"), column("index", "Index"), column("timestampMs", "Timestamp (ms)"), column("assetId", "Asset")],
            scene
                .streams
                .iter()
                .flat_map(|stream| {
                    stream.frames.iter().map(move |frame| {
                        row([("streamId", Value::from(stream.id.as_str())), ("index", Value::from(u64::from(frame.index))), ("timestampMs", Value::from(frame.timestamp_ms)), ("assetId", Value::from(frame.asset_id.as_str()))])
                    })
                })
                .collect(),
        ),
    };
    (semio_framework_pack_json::to_string(&semio_framework_pack_json::array(columns)), semio_framework_pack_json::to_string(&semio_framework_pack_json::array(rows)))
}

pub fn render(scene: &RemodelingSnapshot, config: &RemodelingReportWindowConfig) -> UiAssemblyResult<BuiltNode> {
    let (columns_json, rows_json) = report_table_json(scene, &config.report_table);
    semio_framework_plugin::scene_surface(REMODELING_PLAY_SURFACE_REPORT, ContractSurfaceKind::Table, &TableScene::base(columns_json, rows_json))
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
