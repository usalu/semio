//! 🪣️ Edit-mode tool — Fill: a whole-document generator declared as a framework tool run
//! (`📋️tool-run-contract.md` §2.4, §3.7), not a window utility. The manifest carries its
//! [`ToolRunDefinition`]; the framework injects start/pause/resume/step/abort/finalize/dismiss, owns the
//! provisional placements in its ledger and publishes them as ONE edit on finalize. The run job itself is the
//! puzzle 3d planner behind the 5d translation of `🧠️precompute/🪣️fill`, so both projections see every
//! placement: the planner's `instance3d` records paint the world pane and the bridge's `placement2d` twins the
//! board pane.
//!
//! 🎚️ Its measures are the mode-level tool options rail (`ArtifactEditor::tool_measures`): the count the run
//! plans toward and the part/grip distribution trees its candidate order reads. Progress, status and every run
//! control belong to the framework ToolRun panel.

use crate::editor::puzzle5d::modes::edit::options::brush::distribution_children;
use crate::editor::puzzle5d::terminology::{puzzle5d_fill_run_counters, puzzle5d_fill_run_reasons, puzzle5d_fill_run_stages, puzzle5d_fill_run_unit, Puzzle5dLabels};
use crate::editor::puzzle5d::{puzzle5d_action, Puzzle5dScene, PUZZLE5D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{ActionDescriptor, LocalizedLabel, ToolDefinition, ToolRunView, WindowMeasure};
use semio_framework_tool_run::{JobKindId, ToolRunDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunTraceKind, TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID};

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
/// 🧵️ `ToolRunDefinition.runJob` of the fill tool.
pub const RUN_JOB_KIND: &str = "s.puzzle.puzzle5d.fill.run";
/// 🔍️ `ToolRunDefinition.revalidateJob` of the fill tool.
pub const REVALIDATE_JOB_KIND: &str = "s.puzzle.puzzle5d.fill.revalidate";
/// 🎚️ `ToolRunDefinition.settings.config`: the `Puzzle5dConfig` fields a fill job reads — the count it plans
/// toward and the contact tolerance and kind weights its inputs digest. A camera move or any other publication
/// leaves them unchanged and never reconfigures a run.
pub const RUN_SETTINGS_CONFIG: [&str; 4] = ["/fillCount", "/contactTolerance", "/objectKindWeights", "/vortexKindWeights"];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn definition(label: LocalizedLabel) -> ToolDefinition {
    ToolDefinition { run: Some(run_definition()), ..semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, label, "paint-bucket")) }
}

/// ⏯️ A mutating run whose trace carries `instance3d` subjects for the world window and `placement2d` twins for
/// the board: provisional placements rebase by revalidation at finalize, and a count change resumes the
/// deterministic sequence (a raise continues it, a lower retracts its tail). Only the configuration the planner
/// reads reconfigures it, never a camera or window option.
pub fn run_definition() -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: true,
        rebase: ToolRunRebasePolicy::Revalidate,
        reconfigure: ToolRunReconfigurePolicy::Resume,
        unit: puzzle5d_fill_run_unit(),
        stages: puzzle5d_fill_run_stages(),
        counters: puzzle5d_fill_run_counters(),
        reasons: puzzle5d_fill_run_reasons(),
        trace: ToolRunTraceKind::Instance3d,
        run_job: JobKindId::new(RUN_JOB_KIND),
        revalidate_job: Some(JobKindId::new(REVALIDATE_JOB_KIND)),
        settings: ToolRunSettingsReads { config: RUN_SETTINGS_CONFIG.iter().map(|pointer| pointer.to_string()).collect(), ..ToolRunSettingsReads::default() },
        windows: Vec::new(),
    }
}

/// 🔢️ Fill-count entry — the fill tool's core parameter (`setFillCount` reads `count`-or-`value`, so a numeric
/// measure's `{value}` payload preserves the action semantics). `max: None` is the product decision, not an
/// oversight: the planner plans toward whatever the operator types and reports a capacity it cannot reach as a
/// visible stall. Changing it during a run reconfigures that run.
pub fn count_measure(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels, tool_run: Option<&ToolRunView>) -> WindowMeasure {
    WindowMeasure::Number {
        id: "puzzle5d-fill-count".into(),
        label: Some(labels.count.into()),
        value: envelope.runtime.fill_count as f64,
        min: Some(0.0),
        max: None,
        step: Some(1.0),
        ready: None,
        loading: live_fill_run(tool_run).map(|_| true),
        waiting: None,
        disabled: None,
        on_change: puzzle5d_action("setFillCount", None),
    }
}

/// ⚖️ The part/grip distribution trees the fill run's candidate order reads — the same kind rows the brush
/// suggestions options expose, over the document's own kinds when it authors no catalogs.
pub fn distribution_group(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-fill-distribution"),
        label: labels.placement.into(),
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
        children: distribution_children(envelope, labels),
    }
}

/// 🛠️ The fill tool's measures — the count entry and the distribution trees. Progress, status and every run
/// control are the framework ToolRun panel's.
pub fn measures(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels, tool_run: Option<&ToolRunView>) -> Vec<WindowMeasure> {
    vec![count_measure(envelope, labels, tool_run), distribution_group(envelope, labels)]
}

/// 🏃️ The fill run of this document instance while it is not terminal.
pub fn live_fill_run(tool_run: Option<&ToolRunView>) -> Option<&ToolRunView> {
    tool_run.filter(|run| run.tool_id == TOOL_ID && !run.state.is_terminal())
}

/// 🛑️ `toolRunAbort` bound to the live fill run's current identity, or `None` without one.
pub fn abort_action(tool_run: Option<&ToolRunView>) -> Option<ActionDescriptor> {
    let run = live_fill_run(tool_run)?;
    Some(puzzle5d_action(TOOL_RUN_ABORT_ACTION_ID, Some(dsl::os_pack::json::object([(TOOL_RUN_ARG_RUN_ID.to_string(), run.identity.id.run.to_string().into()), (TOOL_RUN_ARG_GENERATION.to_string(), u64::from(run.identity.generation).into())]))))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
