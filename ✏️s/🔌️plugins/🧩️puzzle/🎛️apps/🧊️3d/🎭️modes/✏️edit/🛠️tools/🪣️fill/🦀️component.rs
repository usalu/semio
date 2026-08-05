//! 🪣️ Edit-mode tool — Fill: a whole-document generator (not a window utility), so its count slider
//! and distribution tree are mode-level *tool* measures keyed by the tool id rather than a window
//! utility-options group. The slider's `ready` extent tracks how far background planning has got;
//! its `reveal` key lets the viewport show/hide already-planned pieces client-side per drag value
//! with zero WASM round trips.

use crate::apps::puzzle3d::terminology::Puzzle3dLabels;
use crate::apps::puzzle3d::{puzzle3d_action, puzzle3d_distribution_group, Puzzle3dScene, PUZZLE3D_FILL_COUNT_MAX};
use crate::artifacts::puzzle3d::engine::Puzzle3dPrecomputeSession;
use semio_framework_plugin::{LocalizedLabel, ToolDefinition, WindowMeasure};

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::puzzle3d::create_puzzle3d_app`.
pub fn definition(label: LocalizedLabel) -> ToolDefinition {
    ToolDefinition::new(TOOL_ID, label, "paint-bucket")
}

/// 🪣️ Fill-count slider measure — the fill tool's core parameter (`setFillCount` reads `count`-or-
/// `value`, so a slider's `{value}` payload preserves the action semantics). The label stays fixed;
/// preload progress is the `ready` extent plus the loading ring, and the range stays pinned at
/// [`PUZZLE3D_FILL_COUNT_MAX`].
pub fn count_measure(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> WindowMeasure {
    let progress = precompute.fill_progress_summary();
    let done = progress.done;
    let available_count = progress.count;
    WindowMeasure::Slider {
        id: "puzzle3d-fill-count".into(),
        label: Some(labels.count.into()),
        value: envelope.runtime.fill_count.min(PUZZLE3D_FILL_COUNT_MAX) as f64,
        min: 0.0,
        max: PUZZLE3D_FILL_COUNT_MAX as f64,
        step: Some(1.0),
        ready: Some(available_count as f64),
        loading: if done { None } else { Some(true) },
        waiting: None,
        disabled: None,
        // 🪣️ Live drag reveals/hides already-planned pieces client-side (see `WorldInstancesLayer`'s
        // reveal cutoff store); only the committed value on gesture release round-trips through here.
        reveal: Some("puzzle3d-fill".into()),
        on_change: puzzle3d_action("setFillCount", None),
    }
}

/// 🛠️ Fill tool measures — count slider and nested distribution tree.
pub fn measures(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
    vec![count_measure(envelope, precompute, labels), puzzle3d_distribution_group(envelope, labels, Some(true))]
}
//#endregion 🔖️Definition
