//! 🪣️ Edit-mode window option — the Fill utility's Utility Options group (the fill-count entry),
//! tagged `Some("fill")` so `partition_window_measures` surfaces it in the Utility Options rail only
//! while the Fill utility is active.
//!
//! 🎚️ SHARED at MODE level, not per window (TEMPLATE.md §12.2): BOTH the 2D board window and the 3D
//! world window bind the `fill` utility and expose the identical group, so this measure is declared
//! once here and each window's `window_measures()` collects from it.
//!
//! ⏯️ Progress, status and every run control are the framework ToolRun panel's; the count is configuration the
//! run's declared settings watch, so changing it during a run reconfigures that run.

use crate::editor::puzzle5d::modes::edit::windows::board2d::utilities::fill;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, Puzzle5dScene, PUZZLE5D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{ToolRunView, WindowMeasure};

//#region 🔖️Definition
/// 🔢️ Fill-count entry — the fill utility's core parameter (`setFillCount` reads `count`-or-`value`,
/// so a numeric measure's `{value}` payload preserves the action semantics). `max: None` is the
/// product decision, not an oversight: the planner plans toward whatever the operator types.
fn fill_count_measure(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels, tool_run: Option<&ToolRunView>) -> WindowMeasure {
    WindowMeasure::Number {
        id: "puzzle5d-fill-count".into(),
        label: Some(labels.count.into()),
        value: envelope.runtime.fill_count as f64,
        min: Some(0.0),
        max: None,
        step: Some(1.0),
        ready: None,
        loading: fill::live_fill_run(tool_run).map(|_| true),
        waiting: None,
        disabled: None,
        on_change: puzzle5d_action("setFillCount", None),
    }
}

/// 🪣️ The Fill utility's Utility Options group, collected by both windows' `window_measures()`.
pub fn measure(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels, tool_run: Option<&ToolRunView>) -> WindowMeasure {
    let children = vec![fill_count_measure(envelope, labels, tool_run)];
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-utility-options-fill"),
        label: labels.fill.into(),
        default_open: Some(true),
        active_utility_id: Some("fill".into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children,
    }
}
//#endregion 🔖️Definition
