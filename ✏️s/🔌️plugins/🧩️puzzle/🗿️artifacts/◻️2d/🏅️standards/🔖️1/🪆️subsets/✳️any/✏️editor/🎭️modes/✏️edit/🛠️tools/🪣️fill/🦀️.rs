//! 🪣️ Edit-mode tool — Fill: a whole-document generator (not a window utility), so its count entry
//! is a mode-level *tool* measure keyed by the tool id rather than a window utility-options group.
//! The count has no ceiling — the operator asks for a number and the session searches toward it —
//! and the run itself is a first-class progress measure: phase, accepted-of-requested, the last
//! counters and a real stop button.

use crate::editor::puzzle2d::config::Puzzle2dFillLifecycle;
use crate::editor::puzzle2d::terminology::{puzzle2d_fill_stage_label, Puzzle2dLabels};
use crate::editor::puzzle2d::{puzzle2d_action, Puzzle2dScene};
use semio_framework_plugin::{LocalizedLabel, MeasureProgressStep, MeasureProgressStepKind, ToolDefinition, WindowMeasure};

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
/// 🎯️ The count a fresh 2d document offers: a batch large enough to be worth watching, small enough
/// that a first run finishes while the operator is still looking at it.
pub const PUZZLE2D_DEFAULT_FILL_COUNT: u32 = 100;
/// 🪜️ How many counter lines the progress measure carries — what a person reads at a glance.
pub const FILL_PROGRESS_STEP_PAGE: usize = 3;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle2d::create_puzzle2d_app`.
pub fn definition(label: LocalizedLabel) -> ToolDefinition {
    semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, label, "paint-bucket"))
}

/// 🔎️ Whether the session is still doing work the operator can cancel.
fn is_running(lifecycle: Puzzle2dFillLifecycle) -> bool {
    matches!(
        lifecycle,
        Puzzle2dFillLifecycle::Capturing
            | Puzzle2dFillLifecycle::Queued
            | Puzzle2dFillLifecycle::Running
            | Puzzle2dFillLifecycle::CheckpointReady
            | Puzzle2dFillLifecycle::Applying
            | Puzzle2dFillLifecycle::AwaitingAdoption
            | Puzzle2dFillLifecycle::Closing
    )
}

/// ⌛️ Stages where the session is parked on someone else — the leaf shows the dashed waiting ring.
fn is_waiting(lifecycle: Puzzle2dFillLifecycle) -> bool {
    matches!(
        lifecycle,
        Puzzle2dFillLifecycle::Capturing | Puzzle2dFillLifecycle::Queued | Puzzle2dFillLifecycle::CheckpointReady | Puzzle2dFillLifecycle::AwaitingAdoption | Puzzle2dFillLifecycle::Closing
    )
}

/// 🔢️ Fill-count entry — the fill tool's core parameter (`setFillCount` reads `count`-or-`value`, so a
/// numeric measure's `{value}` payload preserves the action semantics). `max: None` is the product
/// decision, not an oversight: the session searches toward whatever the operator types, and a board
/// that cannot hold that many reports it as a visible outcome rather than as a silent clamp. `ready`
/// is the accepted count — what the document already holds.
pub fn count_measure(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
    let lifecycle = envelope.runtime.fill_job_lifecycle;
    let running = is_running(lifecycle);
    WindowMeasure::Number {
        id: "puzzle2d-fill-count".into(),
        label: Some(labels.count.into()),
        value: envelope.runtime.fill_count as f64,
        min: Some(0.0),
        max: None,
        step: Some(1.0),
        ready: running.then_some(envelope.runtime.fill_job_accepted_count as f64),
        loading: running.then_some(!is_waiting(lifecycle)),
        waiting: running.then_some(is_waiting(lifecycle)),
        disabled: None,
        on_change: puzzle2d_action("setFillCount", None),
    }
}

/// 🪜️ The counters of the live run, strongest verdict first. A counter still at zero contributes no
/// line rather than a misleading "0", and a fault is its own danger line so the reason is never
/// carried by the stage caption alone.
fn progress_steps(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> Vec<MeasureProgressStep> {
    let mut steps = Vec::with_capacity(FILL_PROGRESS_STEP_PAGE);
    let mut push = |kind: MeasureProgressStepKind, word: &str, count: u64| {
        if count > 0 && steps.len() < FILL_PROGRESS_STEP_PAGE {
            steps.push(MeasureProgressStep { kind, text: format!("{word} · {count}") });
        }
    };
    push(MeasureProgressStepKind::Success, labels.fill_accepted.as_str(), envelope.runtime.fill_job_accepted_count);
    push(MeasureProgressStepKind::Info, labels.fill_tested.as_str(), envelope.runtime.fill_job_search_count);
    if let Some(code) = envelope.runtime.fill_job_fault_code.as_ref() {
        steps.push(MeasureProgressStep { kind: MeasureProgressStepKind::Danger, text: code.as_str().to_string() });
    }
    steps
}

/// ⏳️ The fill session made visible: which phase it is in, how many placements it has accepted out of
/// the requested count, the counters behind that number, and a real stop button while a run exists.
/// The run's own generation travels in the cancel args, so a cancel that arrives after the run it was
/// rendered for was superseded is a no-op rather than a kill of the current session.
pub fn progress_measure(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> Option<WindowMeasure> {
    let lifecycle = envelope.runtime.fill_job_lifecycle;
    if matches!(lifecycle, Puzzle2dFillLifecycle::Idle | Puzzle2dFillLifecycle::Discarded) {
        return None;
    }
    let running = is_running(lifecycle);
    Some(WindowMeasure::Progress {
        id: "puzzle2d-fill-progress".into(),
        label: Some(labels.fill_progress.into()),
        stage: Some(puzzle2d_fill_stage_label(labels, lifecycle, envelope.runtime.fill_job_fault_code.as_ref().map(|code| code.as_str()))),
        completed: envelope.runtime.fill_job_accepted_count as f64,
        total: Some(envelope.runtime.fill_count as f64),
        steps: progress_steps(envelope, labels),
        cancel: running.then(|| puzzle2d_action("brushFillSessionCancel", Some(serde_json::json!({ "generation": envelope.runtime.fill_job_generation })))),
        loading: running.then_some(true),
    })
}

/// 🎚️ The fill tool's measure group, surfaced in the mode-level tool panel while the fill tool is active.
pub fn measures(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
    let mut children = vec![count_measure(envelope, labels)];
    children.extend(progress_measure(envelope, labels));
    if matches!(envelope.runtime.fill_job_lifecycle, Puzzle2dFillLifecycle::Faulted | Puzzle2dFillLifecycle::Cancelled) {
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
