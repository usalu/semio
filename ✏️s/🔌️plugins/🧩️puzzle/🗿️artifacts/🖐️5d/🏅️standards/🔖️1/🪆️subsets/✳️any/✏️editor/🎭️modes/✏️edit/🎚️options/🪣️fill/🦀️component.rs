//! 🪣️ Edit-mode window option — the Fill utility's Utility Options group (the fill-count slider),
//! tagged `Some("fill")` so `partition_window_measures` surfaces it in the Utility Options rail only
//! while the Fill utility is active.
//!
//! 🎚️ SHARED at MODE level, not per window (TEMPLATE.md §12.2): BOTH the 2D board window and the 3D
//! world window bind the `fill` utility and expose the identical group, so this measure is declared
//! once here and each window's `window_measures()` collects from it.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, Puzzle5dScene, PUZZLE5D_FILL_COUNT_MAX, PUZZLE5D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::WindowMeasure;

/// 🪣️ Fill-count slider measure — the fill-utility's core parameter (`setFillCount` reads
/// `count`-or-`value`, so the slider's `{value}` payload preserves the action semantics).
async fn fill_count_measure(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "puzzle5d-fill-count".into(),
        label: Some(labels.count.into()),
        value: envelope.runtime.fill_count as f64,
        min: 0.0,
        max: PUZZLE5D_FILL_COUNT_MAX as f64,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: puzzle5d_action("setFillCount", None),
    }
}

/// 🪣️ The Fill utility's Utility Options group, collected by both windows' `window_measures()`.
pub async fn measure(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowMeasure {
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
        children: vec![fill_count_measure(envelope, labels)],
    }
}
