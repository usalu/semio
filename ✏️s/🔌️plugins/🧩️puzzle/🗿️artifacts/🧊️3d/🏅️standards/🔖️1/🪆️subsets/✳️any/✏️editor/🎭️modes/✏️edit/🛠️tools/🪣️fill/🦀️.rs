//! 🪣️ Edit-mode tool — Fill: a whole-document generator (not a window utility), so its count entry
//! and distribution tree are mode-level *tool* measures keyed by the tool id rather than a window
//! utility-options group. The count has no ceiling — the user asks for a number and the planner keeps
//! going until it reaches it or says why it cannot — and the run itself is a first-class progress
//! measure: phase, locked-of-requested, the last verdicts and a real stop button.

use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;
use crate::editor::puzzle3d::terminology::{puzzle3d_fill_stage_label, Puzzle3dLabels};
use crate::editor::puzzle3d::{puzzle3d_action, puzzle3d_distribution_group, Puzzle3dScene, PUZZLE3D_PLAY_CONTROLLER_ID};
use crate::standards::v1::subsets::any::schema::FillProgressSummary;
use dsl::json;
use semio_framework_plugin::{LocalizedLabel, MeasureProgressStep, MeasureProgressStepKind, ToolDefinition, WindowMeasure};

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
/// 🪜️ How many verdict lines the progress measure carries. Four is what a person reads at a glance on
/// a 120 ms refresh; more would turn a status row into a log nobody follows.
pub const FILL_PROGRESS_STEP_PAGE: usize = 4;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub fn definition(label: LocalizedLabel) -> ToolDefinition {
    semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, label, "paint-bucket"))
}

/// 🔢️ Fill-count entry — the fill tool's core parameter (`setFillCount` reads `count`-or-`value`, so a
/// numeric measure's `{value}` payload preserves the action semantics). `max: None` is the product
/// decision, not an oversight: the planner plans toward whatever the user types, and a document that
/// cannot hold that many says so as a visible stall reason rather than as a silently lowered ceiling.
/// `ready` is the LOCKED count — what the document already holds — so the soft extent behind the entry
/// never promises a piece the user cannot yet select.
pub fn count_measure(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> WindowMeasure {
    let progress = precompute.fill_progress_summary();
    WindowMeasure::Number {
        id: "puzzle3d-fill-count".into(),
        label: Some(labels.count.into()),
        value: envelope.runtime.fill_count as f64,
        min: Some(0.0),
        max: None,
        step: Some(1.0),
        ready: Some(progress.applied_count as f64),
        loading: if progress.done { None } else { Some(true) },
        waiting: None,
        disabled: None,
        on_change: puzzle3d_action("setFillCount", None),
    }
}

/// 🪜️ The last verdicts of the live run, newest first — what the search just did, in the reader's own
/// language. Built from the summary's own counters (the only per-tick truth the session publishes), so
/// the row can never disagree with the HUD it sits beside, and a counter still at zero contributes no
/// line rather than a misleading "0".
fn progress_steps(progress: &FillProgressSummary, labels: &Puzzle3dLabels) -> Vec<MeasureProgressStep> {
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

/// ⏳️ The live background fill run made visible: which phase it is in, how many pieces it has actually
/// locked into the document out of the requested count, the last verdicts, and a real stop button —
/// present only while a run exists, so the panel never offers to cancel nothing. The job's own
/// `(job, operation, generation)` identity travels in the cancel args, so a cancel that arrives after the
/// run it was rendered for was superseded is a no-op rather than a kill of the current plan.
pub fn progress_measure(precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> Option<WindowMeasure> {
    let progress = precompute.fill_progress_summary();
    let (job, operation, generation) = precompute.fill_job_identity()?;
    if progress.done {
        return None;
    }
    Some(WindowMeasure::Progress {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-fill-cancel"),
        label: Some(labels.fill_progress.into()),
        stage: Some(puzzle3d_fill_stage_label(labels, progress.stage.as_str(), progress.stall_reason.as_deref())),
        completed: progress.applied_count as f64,
        total: Some(progress.max_count as f64),
        steps: progress_steps(&progress, labels),
        cancel: Some(puzzle3d_action("cancelFillBuild", Some(json!({ "job": job, "operation": operation, "generation": generation })))),
        loading: Some(true),
    })
}

/// 🛠️ Fill tool measures — count entry, the live run's progress/cancel row, and the nested
/// distribution tree.
pub fn measures(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
    let mut measures = vec![count_measure(envelope, precompute, labels)];
    measures.extend(progress_measure(precompute, labels));
    measures.push(puzzle3d_distribution_group(envelope, labels, Some(true)));
    measures
}
//#endregion 🔖️Definition
