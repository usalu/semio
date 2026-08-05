//! 🧮️ Remodel play app — the `DocumentApp::Config` view state and its operation vocabulary.
//!
//! Every former `RemodelPlayRuntime` field (camera/selection/layers/frame cursor/report table) lives
//! here, written through `RemodelConfigOperation`s with a real `backwards`, never ad hoc runtime
//! mutation. This is app-level, not artifact-level, precisely because it is view state: the artifact
//! must never depend on the app, so nothing under `🗿️artifacts/` may reference these types.

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🎥️ Ephemeral viewport orbit camera — never persisted as document content, mirrors the pre-B1
/// `RemodelPlayRuntime::camera`'s shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelWorldCamera {
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(coord)]
    pub target: [f64; 3],
    pub fov: f64,
}

impl Default for RemodelWorldCamera {
    fn default() -> Self {
        Self { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }
    }
}

/// 🖱️ Ephemeral face/vertex/object selection — was `RemodelPlayRuntime::selection`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelSelection {
    pub mode: String,
    pub ids: Vec<String>,
}

/// 👁️ Which `remodel-main` point-cloud/mesh layers are visible — was `RemodelPlayRuntime::layers`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelLayerVisibility {
    pub mesh: bool,
    pub dense: bool,
    pub sparse: bool,
    pub cameras: bool,
    pub gcps: bool,
}

impl Default for RemodelLayerVisibility {
    fn default() -> Self {
        Self { mesh: true, dense: true, sparse: true, cameras: true, gcps: true }
    }
}

/// 🎞️ Which frame `remodel-frames` currently shows — was `RemodelPlayRuntime::frame_cursor`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelFrameCursor {
    pub stream_id: Option<String>,
    pub frame_index: u32,
}

/// 🧮️ B1: remodel's real `DocumentApp::Config` — absorbs every former `RemodelPlayRuntime` view/session
/// field (camera/selection/layers/frame cursor/report table selection) plus the two `ViewState`-sourced
/// fields the UI actually reads (`active_utility_id`/`locale`, mirroring `shooting_engine::ShootingConfig`).
/// The live `engine::reconstruction::ReconstructionEngine` and the video-import blur-gate rolling window are
/// deliberately NOT here: neither is `Clone + Serialize + Deserialize` in a way that round-trips through
/// a pure `&self` `handle()` (see `remodel_ui::RemodelPlayApp::handle`'s `RunReconstruction`/
/// `ImportVideoFramePayload` docs for how both are rebuilt from already-persisted document state instead
/// of carried as hidden interior-mutable scratch).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "remodelcfg")]
#[dsl(layout = "lines")]
pub struct RemodelConfig {
    #[dsl(block)]
    pub camera: RemodelWorldCamera,
    #[dsl(block)]
    pub selection: RemodelSelection,
    #[dsl(block)]
    pub layers: RemodelLayerVisibility,
    #[dsl(block)]
    pub frame_cursor: RemodelFrameCursor,
    /// 📊️ Which `remodel-report` dataset is selected (`"frames"`/`"cameras"`/`"tracks"`/`"gcps"`/…).
    pub report_table: String,
    /// 🧰️ The active utility for `remodel-main`/`remodel-frames` — was read off `view_state.active_utility_id`.
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for RemodelConfig {
    fn default() -> Self {
        Self {
            camera: RemodelWorldCamera::default(),
            selection: RemodelSelection::default(),
            layers: RemodelLayerVisibility::default(),
            frame_cursor: RemodelFrameCursor::default(),
            report_table: "frames".into(),
            active_utility_id: "select".into(),
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(RemodelConfig);

//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `RemodelConfig`'s operation enum — one variant per settled interaction
/// (mirrors the pre-B1 `RemodelPlayRuntime` field writes), plus a generic `Snapshot` every variant's
/// `backwards()` returns. Mirrors `shooting_op::ShootingConfigOperation` exactly: a config-only "View"
/// dispatch is a plain `Apply` (never `AmendLast`), so each tick is its own distinct, real config edit
/// and "undo this tick" is exactly "restore the whole-config snapshot from just before it" — no
/// per-field reverse-patch bookkeeping needed. `Operation::Diff` is the WHOLE `RemodelConfig`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum RemodelConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: RemodelConfig,
    },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: RemodelWorldCamera,
    },
    #[dsl(key = "selection")]
    SetSelection { mode: String, ids: Vec<String> },
    #[dsl(key = "layer-visibility")]
    SetLayerVisibility { layer: String, visible: bool },
    #[dsl(key = "frame-cursor")]
    SetFrameCursor {
        #[serde(default)]
        stream_id: Option<String>,
        frame_index: u32,
    },
    #[dsl(key = "report-table")]
    SetReportTable { table: String },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<RemodelConfig> for RemodelConfigOperation {
    type Diff = RemodelConfig;

    fn diff(&self, base: &RemodelConfig) -> RemodelConfig {
        let mut next = base.clone();
        match self {
            RemodelConfigOperation::Snapshot { config } => return config.clone(),
            RemodelConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            RemodelConfigOperation::SetSelection { mode, ids } => {
                next.selection.mode = mode.clone();
                next.selection.ids = ids.clone();
            }
            RemodelConfigOperation::SetLayerVisibility { layer, visible } => match layer.as_str() {
                "mesh" => next.layers.mesh = *visible,
                "dense" => next.layers.dense = *visible,
                "sparse" => next.layers.sparse = *visible,
                "cameras" => next.layers.cameras = *visible,
                "gcps" => next.layers.gcps = *visible,
                _ => {}
            },
            RemodelConfigOperation::SetFrameCursor { stream_id, frame_index } => {
                if stream_id.is_some() {
                    next.frame_cursor.stream_id = stream_id.clone();
                }
                next.frame_cursor.frame_index = *frame_index;
            }
            RemodelConfigOperation::SetReportTable { table } => next.report_table = table.clone(),
            RemodelConfigOperation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            RemodelConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &RemodelConfig) -> Vec<Self> {
        vec![RemodelConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remodel_config_default_matches_the_former_runtime_defaults() {
        let config = RemodelConfig::default();
        assert_eq!(config.camera, RemodelWorldCamera { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 });
        assert_eq!(config.selection, RemodelSelection::default());
        assert!(config.layers.mesh && config.layers.dense && config.layers.sparse && config.layers.cameras && config.layers.gcps);
        assert_eq!(config.frame_cursor, RemodelFrameCursor::default());
        assert_eq!(config.report_table, "frames");
        assert_eq!(config.active_utility_id, "select");
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn remodel_config_operation_diff_is_whole_record_replace() {
        let base = RemodelConfig::default();
        let mut next = base.clone();
        next.report_table = "gcps".into();
        assert_eq!(protocol::OperationDiff::apply(&next, &base), next, "apply ignores base entirely, like ShootingConfig");
    }

    #[test]
    fn config_operations_apply_and_backwards_restore_the_pre_edit_snapshot() {
        let base = RemodelConfig::default();

        let camera = RemodelWorldCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 };
        let op = RemodelConfigOperation::SetCamera { camera: camera.clone() };
        let next = op.diff(&base);
        assert_eq!(next.camera, camera);
        assert_eq!(op.backwards(&base), vec![RemodelConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(op.backwards(&base)[0].diff(&next), base, "backwards restores the exact pre-edit config");

        let op = RemodelConfigOperation::SetSelection { mode: "rectangle".into(), ids: vec!["a".into()] };
        let next = op.diff(&base);
        assert_eq!(next.selection.mode, "rectangle");
        assert_eq!(next.selection.ids, vec!["a".to_string()]);

        let op = RemodelConfigOperation::SetLayerVisibility { layer: "dense".into(), visible: false };
        let next = op.diff(&base);
        assert!(!next.layers.dense);
        assert!(next.layers.mesh, "only the named layer flips");

        let op = RemodelConfigOperation::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 4 };
        let next = op.diff(&base);
        assert_eq!(next.frame_cursor.stream_id.as_deref(), Some("stream-1"));
        assert_eq!(next.frame_cursor.frame_index, 4);

        let op = RemodelConfigOperation::SetReportTable { table: "gcps".into() };
        assert_eq!(op.diff(&base).report_table, "gcps");

        let op = RemodelConfigOperation::SetActiveUtility { utility_id: "measure".into() };
        assert_eq!(op.diff(&base).active_utility_id, "measure");

        let op = RemodelConfigOperation::SetLocale { value: "de-DE".into() };
        assert_eq!(op.diff(&base).locale, "de-DE");
    }

    #[test]
    fn config_operations_roundtrip_through_op_text() {
        let config = RemodelConfig::default();
        store::test_support::assert_op_line_round_trip(&RemodelConfigOperation::Snapshot { config });
        store::test_support::assert_op_line_round_trip(&RemodelConfigOperation::SetCamera { camera: RemodelWorldCamera::default() });
        store::test_support::assert_op_line_round_trip(&RemodelConfigOperation::SetSelection { mode: "rectangle".into(), ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_line_round_trip(&RemodelConfigOperation::SetLayerVisibility { layer: "gcps".into(), visible: false });
        store::test_support::assert_op_line_round_trip(&RemodelConfigOperation::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 });
        store::test_support::assert_op_line_round_trip(&RemodelConfigOperation::SetFrameCursor { stream_id: None, frame_index: 0 });
        store::test_support::assert_op_line_round_trip(&RemodelConfigOperation::SetReportTable { table: "tracks".into() });
        store::test_support::assert_op_line_round_trip(&RemodelConfigOperation::SetActiveUtility { utility_id: "gcpPlace".into() });
        store::test_support::assert_op_line_round_trip(&RemodelConfigOperation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
