//! 🪣️ Fill utility — a whole-document generator declared as a framework tool run
//! (`📋️tool-run-contract.md` §2.4, §3.7). The manifest carries its [`ToolRunDefinition`]; the framework
//! injects start/pause/resume/step/abort/finalize/dismiss, owns the provisional placements in its ledger and
//! publishes them as one edit on finalize. The run job is the puzzle 3d planner behind the 5d translation of
//! `🧠️precompute/🪣️fill`.
//!
//! 🔁️ SHARED BY BOTH WINDOWS: the 2D board window and the 3D world window bind the identical `fill`
//! utility id, so the definition is declared ONCE here (under the 2D window, the first binder) and
//! `🪟️windows/🧊️3d`'s own `definition()` references this same module rather than duplicating it.
//! Its live options are the mode-level `🎭️modes/✏️edit/☑️options/🪣️fill` measure group, tagged with
//! this utility's id.

use crate::editor::puzzle5d::puzzle5d_action;
use crate::editor::puzzle5d::terminology::{puzzle5d_fill_run_counters, puzzle5d_fill_run_reasons, puzzle5d_fill_run_stages, puzzle5d_fill_run_unit};
use semio_framework_plugin::{ActionDescriptor, LocalizedLabel, ToolRunView, UtilityDefinition};
use semio_framework_tool_run::{JobKindId, ToolRunDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunTraceKind, TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID};

pub const UTILITY_ID: &str = "fill";
/// 🧵️ `ToolRunDefinition.runJob` of the fill utility.
pub const RUN_JOB_KIND: &str = "s.puzzle.puzzle5d.fill.run";
/// 🔍️ `ToolRunDefinition.revalidateJob` of the fill utility.
pub const REVALIDATE_JOB_KIND: &str = "s.puzzle.puzzle5d.fill.revalidate";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition { run: Some(run_definition()), ..UtilityDefinition::new(UTILITY_ID, label, "paint-bucket") }
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
        settings: ToolRunSettingsReads { config: ["/fillCount", "/overlapBudget", "/objectKindWeights", "/vortexKindWeights"].map(String::from).to_vec(), ..ToolRunSettingsReads::default() },
        windows: Vec::new(),
    }
}

/// 🏃️ The fill run of this document instance while it is not terminal.
pub fn live_fill_run(tool_run: Option<&ToolRunView>) -> Option<&ToolRunView> {
    tool_run.filter(|run| run.tool_id == UTILITY_ID && !run.state.is_terminal())
}

/// 🛑️ `toolRunAbort` bound to the live fill run's current identity, or `None` without one.
pub fn abort_action(tool_run: Option<&ToolRunView>) -> Option<ActionDescriptor> {
    let run = live_fill_run(tool_run)?;
    Some(puzzle5d_action(TOOL_RUN_ABORT_ACTION_ID, Some(dsl::os_pack::json::object([(TOOL_RUN_ARG_RUN_ID.to_string(), run.identity.id.run.to_string().into()), (TOOL_RUN_ARG_GENERATION.to_string(), u64::from(run.identity.generation).into())]))))
}
