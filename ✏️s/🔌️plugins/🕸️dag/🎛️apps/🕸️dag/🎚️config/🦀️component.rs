//! 🧮️ DAG play app — view state (`DagConfig`) and its operation enum (`DagConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.dag` document. It absorbs everything that used to live in the old
//! ui crate's `DagPlayRuntime` (an app-struct `RefCell`) AND the two fields the dag UI actually read off
//! the deleted host-pushed `ViewState` (`locale`, via `dag_play_labels`/`app_labels`/`context_menu`): the
//! selected node ids, the free/live node-graph viewport camera, and the BCP-47 locale tag — session-only
//! view state round-trips through the config `DocumentStore` exactly like document content, with a real
//! `backwards` per `DagConfigOperation` instead of never being VCS'd at all.

use infinite_board_port_directed_dag::DagCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `DagPlayApp::Config` — the pure-trait `DocumentApp::Config` for the dag app.
///
/// The camera is flattened to its three scalar fields (`camera_x`/`camera_y`/`camera_zoom`) rather than
/// embedding `infinite_board_port_directed_dag::DagCamera` as a `#[dsl(block)]`: that kernel type is
/// explicitly out of scope for this crate and doesn't derive `dsl::DslRecord` (only
/// `Clone`/`Debug`/`PartialEq`/`Serialize`/`Deserialize`), so it can't satisfy a nested-block field —
/// three plain `f64` fields need no such support at all. See `dag_config_camera` below for the seam back
/// to the real `DagCamera` type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "dagcfg")]
#[dsl(layout = "lines")]
pub struct DagConfig {
    /// 👁️ Selected node ids — was `DagPlayRuntime::selected_node_ids`.
    pub selected_node_ids: Vec<String>,
    /// 🎥️ Viewport camera x — was `DagPlayRuntime::camera.x`.
    pub camera_x: f64,
    /// 🎥️ Viewport camera y — was `DagPlayRuntime::camera.y`.
    pub camera_y: f64,
    /// 🎥️ Viewport camera zoom — was `DagPlayRuntime::camera.zoom`.
    pub camera_zoom: f64,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for DagConfig {
    fn default() -> Self {
        // 🎥️ Matches `DagCamera`'s own implicit default (`x: 0.0, y: 0.0, zoom: 1.0`, see `DagFixture`'s
        // `Default` impl in the kernel crate) without needing to parse the bundled demo document just to
        // read a trivial camera default.
        Self { selected_node_ids: Vec::new(), camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0, locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(DagConfig);

/// 🎥️ Reassembles the kernel's `DagCamera` from `DagConfig`'s flattened scalar fields — the seam
/// `crate::apps::dag` uses wherever the old `DagPlayRuntime::camera` field was read.
pub fn dag_config_camera(config: &DagConfig) -> DagCamera {
    DagCamera { x: config.camera_x, y: config.camera_y, zoom: config.camera_zoom }
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `DagConfig`'s operation enum — one variant per settled interaction (mirrors the pre-migration
/// `DagPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()` returns: since
/// a config-only "View" dispatch is a plain `Apply` (not an `AmendLast`), each tick is its own distinct,
/// real config edit, and "undo this tick" is exactly "restore the whole-config snapshot from just before
/// it" — the simplest correct inverse, needing no per-field reverse-patch bookkeeping. Mirrors
/// `shooting_op::ShootingConfigOperation` exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum DagConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: DagConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { node_ids: Vec<String> },
    #[dsl(key = "camera")]
    SetCamera { x: f64, y: f64, zoom: f64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<DagConfig> for DagConfigOperation {
    type Diff = DagConfig;

    fn diff(&self, base: &DagConfig) -> DagConfig {
        let mut next = base.clone();
        match self {
            DagConfigOperation::Snapshot { config } => return config.clone(),
            DagConfigOperation::SetSelection { node_ids } => next.selected_node_ids = node_ids.clone(),
            DagConfigOperation::SetCamera { x, y, zoom } => {
                next.camera_x = *x;
                next.camera_y = *y;
                next.camera_zoom = *zoom;
            }
            DagConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &DagConfig) -> Vec<Self> {
        vec![DagConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dag_config_default_matches_dag_camera_implicit_default() {
        let config = DagConfig::default();
        assert!(config.selected_node_ids.is_empty());
        assert_eq!((config.camera_x, config.camera_y, config.camera_zoom), (0.0, 0.0, 1.0));
        assert_eq!(dag_config_camera(&config), DagCamera { x: 0.0, y: 0.0, zoom: 1.0 });
        assert_eq!(config.locale, "en-US");
    }

    /// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `DagConfig`.
    #[test]
    fn dag_config_dsl_pack_round_trip() {
        let config = DagConfig { selected_node_ids: vec!["n1".into(), "n2".into()], camera_x: 12.5, camera_y: -3.0, camera_zoom: 2.25, locale: "de-DE".into() };
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn dag_config_operation_text_binary_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&DagConfigOperation::Snapshot { config: DagConfig { selected_node_ids: vec!["n1".into()], camera_x: 1.0, camera_y: 2.0, camera_zoom: 3.0, locale: "de-DE".into() } });
        store::test_support::assert_op_line_round_trip(&DagConfigOperation::SetSelection { node_ids: vec!["n1".into(), "n2".into()] });
        store::test_support::assert_op_line_round_trip(&DagConfigOperation::SetSelection { node_ids: Vec::new() });
        store::test_support::assert_op_line_round_trip(&DagConfigOperation::SetCamera { x: 12.5, y: -3.0, zoom: 2.25 });
        store::test_support::assert_op_line_round_trip(&DagConfigOperation::SetLocale { value: "de-DE".into() });
    }

    #[test]
    fn dag_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = DagConfig { selected_node_ids: vec!["n1".into()], camera_x: 1.0, camera_y: 2.0, camera_zoom: 3.0, locale: "en-US".into() };
        let operation = DagConfigOperation::SetSelection { node_ids: vec!["n2".into()] };
        let forward = operation.diff(&base);
        assert_eq!(forward.selected_node_ids, vec!["n2".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![DagConfigOperation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
