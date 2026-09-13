//! 🪣️ Edit-mode window option — the Fill utility's Utility Options group (the fill-count entry and
//! the live run's progress row), tagged `Some("fill")` so `partition_window_measures` surfaces it in
//! the Utility Options rail only while the Fill utility is active.
//!
//! 🎚️ SHARED at MODE level, not per window (TEMPLATE.md §12.2): BOTH the 2D board window and the 3D
//! world window bind the `fill` utility and expose the identical group, so this measure is declared
//! once here and each window's `window_measures()` collects from it.
//!
//! 🧠️ The counters come from the wrapped 3d planner (`Puzzle5dPrecomputeSession::fill_progress`) —
//! 5d's fill IS that planner, so publishing an empty control beside a solver that knows exactly
//! where it is was the gap this measure closes.

use crate::editor::puzzle5d::precompute::Puzzle5dPrecomputeSession;
use crate::editor::puzzle5d::terminology::{puzzle5d_fill_stage_label, Puzzle5dLabels};
use crate::editor::puzzle5d::{puzzle5d_action, Puzzle5dScene, PUZZLE5D_PLAY_CONTROLLER_ID};
use dsl::json;
use semio_framework_plugin::{MeasureProgressStep, MeasureProgressStepKind, WindowMeasure};

//#region 🔖️Constants
/// 🪜️ How many counter lines the progress measure carries — what a person reads at a glance.
pub const FILL_PROGRESS_STEP_PAGE: usize = 4;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🔢️ Fill-count entry — the fill utility's core parameter (`setFillCount` reads `count`-or-`value`,
/// so a numeric measure's `{value}` payload preserves the action semantics). `max: None` is the
/// product decision, not an oversight: the planner plans toward whatever the operator types, and a
/// document that cannot hold that many says so as a visible stall reason rather than as a silently
/// lowered ceiling. `ready` is the locked count — what the document already holds.
fn fill_count_measure(envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> WindowMeasure {
    let progress = precompute.fill_progress();
    WindowMeasure::Number {
        id: "puzzle5d-fill-count".into(),
        label: Some(labels.count.into()),
        value: envelope.runtime.fill_count as f64,
        min: Some(0.0),
        max: None,
        step: Some(1.0),
        ready: Some(progress.applied_count as f64),
        loading: if progress.done { None } else { Some(true) },
        waiting: None,
        disabled: None,
        on_change: puzzle5d_action("setFillCount", None),
    }
}

/// 🪜️ The counters of the live run, strongest verdict first. A counter still at zero contributes no
/// line rather than a misleading "0".
fn progress_steps(progress: &crate::editor::puzzle5d::precompute::Puzzle5dFillProgress, labels: &Puzzle5dLabels) -> Vec<MeasureProgressStep> {
    let mut steps = Vec::with_capacity(FILL_PROGRESS_STEP_PAGE);
    let mut push = |kind: MeasureProgressStepKind, word: &str, count: u64| {
        if count > 0 && steps.len() < FILL_PROGRESS_STEP_PAGE {
            steps.push(MeasureProgressStep { kind, text: format!("{word} · {count}") });
        }
    };
    push(MeasureProgressStepKind::Success, labels.fill_locked.as_str(), progress.applied_count as u64);
    push(MeasureProgressStepKind::Danger, labels.fill_collision.as_str(), progress.collisions);
    push(MeasureProgressStepKind::Warning, labels.fill_rejected.as_str(), progress.rejected);
    push(MeasureProgressStepKind::Info, labels.fill_tested.as_str(), progress.tested);
    steps
}

/// ⏳️ The live fill run made visible: which phase it is in, how many parts it has locked into the
/// document out of the requested count, the last verdicts, and a real stop button — present only
/// while the planner still has work, so the panel never offers to cancel nothing. When a background
/// job is carrying the run, its own `(job, operation, generation)` identity travels in the cancel
/// args exactly as the 3d tool's does, so a cancel that arrives after the run it was rendered for was
/// superseded is a no-op rather than a kill of the current plan.
pub fn fill_progress_measure(precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> Option<WindowMeasure> {
    let progress = precompute.fill_progress();
    if progress.done {
        return None;
    }
    let identity = precompute.fill_job_identity().map_or_else(|| json!({}), |(job, operation, generation)| json!({ "job": job, "operation": operation, "generation": generation }));
    Some(WindowMeasure::Progress {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-fill-progress"),
        label: Some(labels.fill_progress.into()),
        stage: Some(puzzle5d_fill_stage_label(labels, progress.stage.as_str(), progress.stall_reason.as_deref())),
        completed: progress.applied_count as f64,
        total: Some(progress.requested_count as f64),
        steps: progress_steps(&progress, labels),
        cancel: Some(puzzle5d_action("cancelFillBuild", Some(identity))),
        loading: Some(true),
    })
}

/// 🪣️ The Fill utility's Utility Options group, collected by both windows' `window_measures()`.
pub fn measure(envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> WindowMeasure {
    let mut children = vec![fill_count_measure(envelope, precompute, labels)];
    children.extend(fill_progress_measure(precompute, labels));
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
