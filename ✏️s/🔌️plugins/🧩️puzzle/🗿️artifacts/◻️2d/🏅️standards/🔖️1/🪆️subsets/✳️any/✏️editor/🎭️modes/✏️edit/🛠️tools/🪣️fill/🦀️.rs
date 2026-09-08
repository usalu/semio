//! 🪣️ Edit-mode tool — Fill: a whole-document generator (not a window utility), so its count slider
//! is a mode-level *tool* measure keyed by the tool id rather than a window utility-options group.

use crate::editor::puzzle2d::config::Puzzle2dFillLifecycle;
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{puzzle2d_action, Puzzle2dScene};
use semio_framework_plugin::{LocalizedLabel, ToolDefinition, WindowMeasure};

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
pub const PUZZLE2D_FILL_COUNT_MAX: u32 = 1000;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle2d::create_puzzle2d_app`.
pub fn definition(label: LocalizedLabel) -> ToolDefinition {
    semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, label, "paint-bucket"))
}

/// 🎚️ The fill-count slider, surfaced in the mode-level tool panel while the fill tool is active.
pub fn measures(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
    let lifecycle = envelope.runtime.fill_job_lifecycle;
    let accepted = envelope.runtime.fill_job_accepted_count as f64;
    let running = matches!(
        lifecycle,
        Puzzle2dFillLifecycle::Capturing
            | Puzzle2dFillLifecycle::Queued
            | Puzzle2dFillLifecycle::Running
            | Puzzle2dFillLifecycle::CheckpointReady
            | Puzzle2dFillLifecycle::Applying
            | Puzzle2dFillLifecycle::AwaitingAdoption
            | Puzzle2dFillLifecycle::Closing
    );
    let waiting = matches!(lifecycle, Puzzle2dFillLifecycle::Capturing | Puzzle2dFillLifecycle::Queued | Puzzle2dFillLifecycle::CheckpointReady | Puzzle2dFillLifecycle::AwaitingAdoption | Puzzle2dFillLifecycle::Closing);
    let status = match lifecycle {
        Puzzle2dFillLifecycle::Faulted => match envelope.runtime.fill_job_fault_code.as_deref() {
            Some(code) => format!("{}: {}", labels.fill_fault.as_str(), code),
            None => labels.fill_fault.into(),
        },
        Puzzle2dFillLifecycle::Completed => format!("{}: {}", labels.fill_result.as_str(), envelope.runtime.fill_job_accepted_count),
        _ => format!("{}: {}/{}", labels.fill_progress.as_str(), envelope.runtime.fill_job_accepted_count, envelope.runtime.fill_count),
    };
    let mut children = vec![WindowMeasure::Slider {
        id: "puzzle2d-fill-count".into(),
        label: Some(status),
        value: envelope.runtime.fill_count as f64,
        min: 0.0,
        max: PUZZLE2D_FILL_COUNT_MAX as f64,
        step: Some(1.0),
        ready: running.then_some(accepted),
        loading: running.then_some(!waiting),
        waiting: running.then_some(waiting),
        disabled: None,
        reveal: None,
        on_change: puzzle2d_action("setFillCount", None),
    }];
    if running {
        children.push(WindowMeasure::Toggle {
            id: "puzzle2d-fill-cancel".into(),
            icon_id: "x".into(),
            label: Some(labels.fill_cancel.into()),
            pressed: false,
            text: None,
            on_change: puzzle2d_action("brushFillSessionCancel", Some(serde_json::json!({ "generation": envelope.runtime.fill_job_generation }))),
        });
    }
    if matches!(lifecycle, Puzzle2dFillLifecycle::Faulted | Puzzle2dFillLifecycle::Cancelled) {
        children.push(WindowMeasure::Toggle { id: "puzzle2d-fill-retry".into(), icon_id: "refresh-cw".into(), label: Some(labels.fill_retry.into()), pressed: false, text: None, on_change: puzzle2d_action("brushFillSessionRetry", None) });
    }
    WindowMeasure::Group {
        id: "puzzle2d-tool-options-fill".into(),
        label: labels.fill.into(),
        default_open: Some(true),
        active_utility_id: None,
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
