//! 🖌️ Brush utility — point at a grip and watch every compatible part being tested for it, then place a free one,
//! in both projections at once. The candidate search is the puzzle 3d read-only brush suggestions run
//! (`📋️tool-run-contract.md` §2.4, §3.7) behind the planner bridge: the framework owns the run, its panel and its
//! trace, which the world window paints as instances and the board as placements.
//!
//! 🔁️ SHARED BY BOTH WINDOWS: the 2D board window and the 3D world window bind the identical `brush`
//! utility id, so the definition is declared ONCE here (under the 2D window, the first binder) and
//! `🪟️windows/🧊️3d`'s own `definition()` references this same module rather than duplicating it.
//! Its live options are the mode-level `🎭️modes/✏️edit/☑️options/🖌️brush` measure group, tagged with
//! this utility's id.

use crate::editor::puzzle5d::terminology::{puzzle5d_brush_suggestions_run_counters, puzzle5d_brush_suggestions_run_reasons, puzzle5d_brush_suggestions_run_stages, puzzle5d_brush_suggestions_run_unit};
use semio_framework_plugin::{LocalizedLabel, UtilityDefinition};
use semio_framework_tool_run::{JobKindId, ToolRunDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunTraceKind};

pub const UTILITY_ID: &str = "brush";
/// 🧵️ `ToolRunDefinition.runJob` of the brush suggestions run.
pub const RUN_JOB_KIND: &str = "s.puzzle.puzzle5d.brush.suggestions";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition { run: Some(run_definition()), ..UtilityDefinition::new(UTILITY_ID, label, "paintbrush") }
}

/// ⏯️ A read-only run with `instance3d` subjects and `placement2d` board twins. It authors no provisional edit, so a
/// document change or a change of the settings the search reads restarts it against what is now there.
pub fn run_definition() -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: false,
        rebase: ToolRunRebasePolicy::Restart,
        reconfigure: ToolRunReconfigurePolicy::Restart,
        unit: puzzle5d_brush_suggestions_run_unit(),
        stages: puzzle5d_brush_suggestions_run_stages(),
        counters: puzzle5d_brush_suggestions_run_counters(),
        reasons: puzzle5d_brush_suggestions_run_reasons(),
        trace: ToolRunTraceKind::Instance3d,
        run_job: JobKindId::new(RUN_JOB_KIND),
        revalidate_job: None,
        settings: ToolRunSettingsReads { config: ["/contactTolerance", "/objectKindWeights", "/vortexKindWeights"].map(String::from).to_vec(), ..ToolRunSettingsReads::default() },
        windows: Vec::new(),
    }
}
