//! 🪣️ Edit-mode tool — Fill: a whole-document generator (not a window utility), so its count entry
//! and distribution tree are mode-level *tool* measures keyed by the tool id rather than a window
//! utility-options group. The count has no ceiling — the user asks for a number and the planner keeps
//! going until it reaches it or says why it cannot — and a live run offers a real stop button.

use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;
use crate::editor::puzzle3d::terminology::{puzzle3d_fill_stage_label, Puzzle3dLabels};
use crate::editor::puzzle3d::{puzzle3d_action, puzzle3d_distribution_group, Puzzle3dScene, PUZZLE3D_PLAY_CONTROLLER_ID};
use dsl::json;
use semio_framework_plugin::{LocalizedLabel, ToolDefinition, WindowMeasure};

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
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

/// 🛑️ The live background fill run's stop button — present only while a run exists, so the panel never
/// offers to cancel nothing. Its text carries the phase and the locked-of-requested count until the
/// framework ToolRun panel takes the run over (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS §3.6).
/// The job's own `(job, operation, generation)` identity travels in the cancel args, so a cancel that
/// arrives after the run it was rendered for was superseded is a no-op rather than a kill of the current plan.
pub fn cancel_measure(precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> Option<WindowMeasure> {
    let progress = precompute.fill_progress_summary();
    let (job, operation, generation) = precompute.fill_job_identity()?;
    if progress.done {
        return None;
    }
    let stage = puzzle3d_fill_stage_label(labels, progress.stage.as_str(), progress.stall_reason.as_deref());
    Some(WindowMeasure::Toggle {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-fill-cancel"),
        icon_id: "circle-stop".into(),
        label: Some(labels.fill_cancel.into()),
        pressed: false,
        text: Some(format!("{stage} · {} / {} {}", progress.applied_count, progress.max_count, labels.fill_locked.as_str())),
        on_change: puzzle3d_action("cancelFillBuild", Some(json!({ "job": job, "operation": operation, "generation": generation }))),
    })
}

/// 🛠️ Fill tool measures — count entry, the live run's cancel toggle, and the nested distribution tree.
pub fn measures(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
    let mut measures = vec![count_measure(envelope, precompute, labels)];
    measures.extend(cancel_measure(precompute, labels));
    measures.push(puzzle3d_distribution_group(envelope, labels, Some(true)));
    measures
}
//#endregion 🔖️Definition
