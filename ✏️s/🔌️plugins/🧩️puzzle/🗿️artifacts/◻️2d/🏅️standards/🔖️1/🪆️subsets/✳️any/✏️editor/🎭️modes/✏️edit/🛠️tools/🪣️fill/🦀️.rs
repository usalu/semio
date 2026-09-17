//! 🪣️ Edit-mode tool — Fill: a whole-document generator (not a window utility) declared as a tool run. Its
//! count is a mode-level tool measure over the shared config; progress, pause, single step, abort and finalize
//! are the framework ToolRun panel and actions (`📋️tool-run-contract.md` §2.4–§2.6), and tested candidates are
//! `placement2d` trace records.

use crate::editor::puzzle2d::modes::edit::options::brush::puzzle2d_distribution_measures;
use crate::editor::puzzle2d::precompute::fill::{FillRunCounter, FillRunReason, FillRunStage};
use crate::editor::puzzle2d::terminology::{puzzle2d_localized, Puzzle2dLabels};
use crate::editor::puzzle2d::{puzzle2d_action, Puzzle2dScene};
use semio_framework_plugin::{LocalizedLabel, ToolDefinition, WindowMeasure};
use semio_framework_tool_run::{JobKindId, ToolRunCounterDefinition, ToolRunDefinition, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunStageDefinition, ToolRunTraceKind};

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
/// 🎯️ The count a fresh 2d document offers: a batch large enough to be worth watching, small enough
/// that a first run finishes while the operator is still looking at it.
pub const PUZZLE2D_DEFAULT_FILL_COUNT: u32 = 100;
/// 🧵️ Job kind of the fill run job (`ToolRunDefinition.runJob`).
pub const PUZZLE2D_FILL_RUN_JOB: &str = "puzzle2d.fill.run";
/// 🔍️ Job kind of the fill revalidation job (`ToolRunDefinition.revalidateJob`).
pub const PUZZLE2D_FILL_REVALIDATE_JOB: &str = "puzzle2d.fill.revalidate";
/// 🎚️ `ToolRunDefinition.settings.config`: the `Puzzle2dConfig` fields a fill run reads — the count it
/// plans toward, the kind weights its candidate order digests, and the two placement-tuning measures
/// its collision test is widened by (`contactTolerance - brushPlacementOverlapBudget`).
pub const RUN_SETTINGS_CONFIG: [&str; 5] = ["/fillCount", "/nodeKindWeights", "/handleKindWeights", "/contactTolerance", "/brushPlacementOverlapBudget"];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle2d::create_puzzle2d_app`.
pub fn definition(label: LocalizedLabel) -> ToolDefinition {
    ToolDefinition { run: Some(run_definition()), ..semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, label, "paint-bucket")) }
}

/// ⏯️ The fill run declaration (`$defs.Puzzle2dFillRun`): a mutating run revalidated at finalize and resumed on a
/// count change, reporting `placement2d` trace records.
pub fn run_definition() -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: true,
        rebase: ToolRunRebasePolicy::Revalidate,
        reconfigure: ToolRunReconfigurePolicy::Resume,
        unit: puzzle2d_localized(|labels| labels.fill_unit),
        stages: FillRunStage::ALL.iter().map(|stage| ToolRunStageDefinition { id: stage.id().into(), label: puzzle2d_localized(stage.label()) }).collect(),
        counters: FillRunCounter::ALL.iter().map(|counter| ToolRunCounterDefinition { id: counter.id().into(), label: puzzle2d_localized(counter.label()) }).collect(),
        reasons: FillRunReason::ALL.iter().map(|reason| ToolRunReasonDefinition { code: reason.code(), id: reason.id().into(), verdict: reason.verdict(), template: puzzle2d_localized(reason.label()) }).collect(),
        trace: ToolRunTraceKind::Placement2d,
        run_job: JobKindId::new(PUZZLE2D_FILL_RUN_JOB),
        revalidate_job: Some(JobKindId::new(PUZZLE2D_FILL_REVALIDATE_JOB)),
        settings: ToolRunSettingsReads { config: RUN_SETTINGS_CONFIG.iter().map(|pointer| pointer.to_string()).collect(), ..ToolRunSettingsReads::default() },
        windows: Vec::new(),
    }
}

/// 🔢️ Fill-count entry — the fill tool's core parameter (`setFillCount` reads `count`-or-`value`). `max: None`
/// is the product decision: the run searches toward whatever the operator types, and a board that cannot hold
/// that many ends the run with a visible stall step rather than a silent clamp.
pub fn count_measure(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
    WindowMeasure::Number {
        id: "puzzle2d-fill-count".into(),
        label: Some(labels.count.into()),
        value: f64::from(envelope.runtime.fill_count),
        min: Some(0.0),
        max: None,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        on_change: puzzle2d_action("setFillCount", None),
    }
}

/// 🎚️ The fill tool's measure group, surfaced in the mode-level tool panel while the fill tool is active:
/// the count entry and the per-kind distribution trees the run's candidate order reads.
pub fn measures(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
    let mut children = vec![count_measure(envelope, labels)];
    children.extend(puzzle2d_distribution_measures(envelope, labels));
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
