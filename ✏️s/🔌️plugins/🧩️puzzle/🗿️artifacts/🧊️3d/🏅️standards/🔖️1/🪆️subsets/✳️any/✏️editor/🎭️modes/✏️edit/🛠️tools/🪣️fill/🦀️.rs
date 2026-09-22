//! 🪣️ Edit-mode tool — Fill: a whole-document generator declared as a framework tool run
//! (`📋️tool-run-contract.md` §2.4, §3.7). The manifest carries its [`ToolRunDefinition`]; the framework
//! injects start/pause/resume/step/abort/finalize/dismiss, owns the provisional placements in its ledger and
//! publishes them as one edit on finalize. This module declares the definition, the count measure (shared
//! configuration the driver watches) and builds the run and revalidate jobs.

use crate::editor::puzzle3d::config::{Puzzle3dConfig, Puzzle3dRuntime};
use crate::editor::puzzle3d::modes::edit::windows::main;
use crate::editor::puzzle3d::precompute::fill::{fill_run_placements, FillPreparationRoots, FillRevalidateJob, FillRunJob, FillRunPlacement, FILL_RUN_OPS_PER_PLACEMENT};
use crate::editor::puzzle3d::precompute::geometry::{collision_body_from_buffers, CollisionBody};
use crate::editor::puzzle3d::precompute::{brush_mesh_digest, shared_brush_mesh};
use crate::editor::puzzle3d::terminology::{puzzle3d_fill_run_counters, puzzle3d_fill_run_reasons, puzzle3d_fill_run_stages, puzzle3d_fill_run_unit, Puzzle3dLabels};
use crate::editor::puzzle3d::{puzzle3d_action, puzzle3d_distribution_group, puzzle3d_fallback_mesh_buffers, puzzle3d_fixture_from_snapshot, scene_config, Puzzle3dPlayApp, Puzzle3dScene, PUZZLE3D_FALLBACK_MESH_KIND};
use crate::standards::v1::subsets::any::schema::{FillRunCheckpoint, SceneConfig};
use semio_framework_job::{allocate_operation_id, Generation, InteractiveJob, InteractiveJobCloseStep, Operation, RevisionId, StepContext, StepOutcome};
use semio_framework_plugin::{ActionDescriptor, EditorApp, Fault, LocalizedLabel, ToolDefinition, ToolRunJobPurpose, ToolRunJobRequest, ToolRunRetargetableJob, ToolRunView, WindowMeasure};
use semio_framework_tool_run::{JobKindId, ToolRunDefinition, ToolRunIdentity, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunTraceKind, TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID};
use std::collections::HashMap;
use std::sync::Arc;

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
/// 🧵️ `ToolRunDefinition.runJob` of the fill tool.
pub const RUN_JOB_KIND: &str = "s.puzzle.puzzle3d.fill.run";
/// 🔍️ `ToolRunDefinition.revalidateJob` of the fill tool.
pub const REVALIDATE_JOB_KIND: &str = "s.puzzle.puzzle3d.fill.revalidate";
/// 🎚️ `ToolRunDefinition.settings.config`: the `Puzzle3dConfig` fields a fill job reads — the count it plans
/// toward and the contact tolerance and kind weights its inputs digest. A camera move or any other publication
/// leaves them unchanged and never reconfigures a run.
pub const RUN_SETTINGS_CONFIG: [&str; 4] = ["/fillCount", "/contactTolerance", "/objectKindWeights", "/vortexKindWeights"];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub fn definition(label: LocalizedLabel) -> ToolDefinition {
    ToolDefinition { run: Some(run_definition()), ..semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, label, "paint-bucket")) }
}

/// ⏯️ A mutating `instance3d` run: provisional placements rebase by revalidation at finalize, and a count
/// change retargets the resident run in place (a raise continues its sequence, a lower retracts its tail).
pub fn run_definition() -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: true,
        rebase: ToolRunRebasePolicy::Revalidate,
        reconfigure: ToolRunReconfigurePolicy::Resume,
        unit: puzzle3d_fill_run_unit(),
        stages: puzzle3d_fill_run_stages(),
        counters: puzzle3d_fill_run_counters(),
        reasons: puzzle3d_fill_run_reasons(),
        trace: ToolRunTraceKind::Instance3d,
        run_job: JobKindId::new(RUN_JOB_KIND),
        revalidate_job: Some(JobKindId::new(REVALIDATE_JOB_KIND)),
        settings: ToolRunSettingsReads { config: RUN_SETTINGS_CONFIG.iter().map(|pointer| pointer.to_string()).collect(), ..ToolRunSettingsReads::default() },
        windows: Vec::new(),
    }
}

/// 🔢️ Fill-count entry — the fill tool's core parameter (`setFillCount` reads `count`-or-`value`, so a
/// numeric measure's `{value}` payload preserves the action semantics). `max: None` is the product
/// decision: the planner plans toward whatever the user types, and a document that cannot hold that many
/// says so as a visible stall step. Changing it during a run reconfigures the run.
pub fn count_measure(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels, tool_run: Option<&ToolRunView>) -> WindowMeasure {
    WindowMeasure::Number {
        id: "puzzle3d-fill-count".into(),
        label: Some(labels.count.into()),
        value: envelope.runtime.fill_count as f64,
        min: Some(0.0),
        max: None,
        step: Some(1.0),
        ready: None,
        loading: live_fill_run(tool_run).map(|_| true),
        waiting: None,
        disabled: None,
        on_change: puzzle3d_action("setFillCount", None),
    }
}

/// 🛠️ Fill tool measures — count entry and the nested distribution tree. Progress, status and every run
/// control are the framework ToolRun panel's.
pub fn measures(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels, tool_run: Option<&ToolRunView>) -> Vec<WindowMeasure> {
    vec![count_measure(envelope, labels, tool_run), puzzle3d_distribution_group(envelope, labels, Some(true))]
}

/// 🏃️ The fill run of this document instance while it is not terminal.
pub(crate) fn live_fill_run(tool_run: Option<&ToolRunView>) -> Option<&ToolRunView> {
    tool_run.filter(|run| run.tool_id == TOOL_ID && !run.state.is_terminal())
}

/// 🛑️ `toolRunAbort` bound to the live fill run's current identity, or `None` without one.
pub fn abort_action(tool_run: Option<&ToolRunView>) -> Option<ActionDescriptor> {
    let run = live_fill_run(tool_run)?;
    Some(puzzle3d_action(TOOL_RUN_ABORT_ACTION_ID, Some(dsl::os_pack::json::object([(TOOL_RUN_ARG_RUN_ID.to_string(), run.identity.id.run.to_string().into()), (TOOL_RUN_ARG_GENERATION.to_string(), u64::from(run.identity.generation).into())]))))
}
//#endregion 🔖️Definition

//#region 🔖️Jobs
/// 🧵️ Builds the fill run (`Run`) or finalize revalidation (`Revalidate`) job for the framework ledger as a
/// retargetable job; `None` for any other tool. Revalidated placements take their trace keys from the run's
/// allocator.
pub fn build_run_job(request: ToolRunJobRequest<'_, EditorApp<Puzzle3dPlayApp>>) -> Result<Option<Box<dyn ToolRunRetargetableJob<Puzzle3dConfig>>>, Fault> {
    if request.tool_id != TOOL_ID {
        return Ok(None);
    }
    let config = request.config.as_ref();
    let runtime = Puzzle3dRuntime { fill_count: config.fill_count, contact_tolerance: config.contact_tolerance, object_kind_weights: config.object_kind_weights.clone(), vortex_kind_weights: config.vortex_kind_weights.clone(), ..Puzzle3dRuntime::default() };
    let envelope = Puzzle3dScene { fixture: puzzle3d_fixture_from_snapshot(request.snapshot.typed()), runtime, active_utility: TOOL_ID.into() };
    let scene = scene_config(&envelope).ok_or_else(|| Fault::from("puzzle3d-fill-run-scene"))?;
    let lane = main::mesh_lane(&envelope.fixture);
    let target = match request.purpose {
        ToolRunJobPurpose::Run => FillToolRunTarget::Run { requested: config.fill_count as usize, checkpoint: request.checkpoint.and_then(FillRunCheckpoint::decode), provisional: request.provisional.len() as u32 },
        ToolRunJobPurpose::Revalidate => {
            let keys = request.trace_keys.allocate(request.provisional.len() as u64 / u64::from(FILL_RUN_OPS_PER_PLACEMENT));
            FillToolRunTarget::Revalidate(fill_run_placements(request.provisional, &lane, keys.start).ok_or_else(|| Fault::from("puzzle3d-fill-run-provisional"))?)
        }
    };
    Ok(Some(Box::new(Puzzle3dFillToolRunJob::new(request.identity, scene, lane, target, FillRunInputs::of(config)))))
}

/// 🧮️ The config a fill run's `inputs` digest reads besides the count: a reconfigure that changes any of them
/// cannot continue the same sequence.
#[derive(Clone, Debug, PartialEq)]
struct FillRunInputs {
    contact_tolerance: u64,
    object_kind_weights: Vec<(String, u64)>,
    vortex_kind_weights: Vec<(String, u64)>,
}

impl FillRunInputs {
    fn of(config: &Puzzle3dConfig) -> Self {
        let weights = |weights: &HashMap<String, f64>| {
            let mut sorted: Vec<(String, u64)> = weights.iter().map(|(kind, weight)| (kind.clone(), weight.to_bits())).collect();
            sorted.sort_unstable();
            sorted
        };
        Self { contact_tolerance: config.contact_tolerance.to_bits(), object_kind_weights: weights(&config.object_kind_weights), vortex_kind_weights: weights(&config.vortex_kind_weights) }
    }
}

/// 🎯️ What a prepared fill tool run job becomes.
enum FillToolRunTarget {
    Run { requested: usize, checkpoint: Option<FillRunCheckpoint>, provisional: u32 },
    Revalidate(Vec<FillRunPlacement>),
}

/// 🥽️ The bounded collision mesh preparation of a fill tool run job: one mesh identity per unit, real
/// geometry from the process-wide derived mesh store or the built-in box's own geometry, digested together with the
/// base revision, contact tolerance, seed and weights into the run's `inputs`.
struct FillToolRunPreparation {
    identity: ToolRunIdentity,
    scene: SceneConfig,
    lane: Vec<String>,
    cursor: usize,
    meshes: HashMap<String, CollisionBody>,
    fallback: Option<CollisionBody>,
    digest: Vec<u8>,
    target: FillToolRunTarget,
}

enum FillToolRunPhase {
    Preparing(Box<FillToolRunPreparation>),
    Run(Box<FillRunJob>),
    Revalidate(Box<FillRevalidateJob>),
    Closed,
}

/// ⏯️ The fill tool's run and revalidate job as the framework ledger steps it: bounded mesh preparation,
/// then the planner run (fresh, replaying to its checkpoint, or restarting over the ledger's provisional ops)
/// or the finalize revalidation.
pub(crate) struct Puzzle3dFillToolRunJob {
    phase: FillToolRunPhase,
    inputs: FillRunInputs,
}

impl Puzzle3dFillToolRunJob {
    fn new(identity: ToolRunIdentity, scene: SceneConfig, lane: Vec<String>, target: FillToolRunTarget, inputs: FillRunInputs) -> Self {
        let mut digest = Vec::with_capacity(128);
        digest.extend_from_slice(&identity.base_revision);
        digest.extend_from_slice(&scene.contact_tolerance.to_bits().to_le_bytes());
        digest.extend_from_slice(&scene.seed.to_le_bytes());
        for (kind, weight) in scene.weights.object_weights.iter().chain(&scene.weights.vortex_weights) {
            digest.extend_from_slice(kind.as_bytes());
            digest.extend_from_slice(&weight.to_bits().to_le_bytes());
        }
        Self { phase: FillToolRunPhase::Preparing(Box::new(FillToolRunPreparation { identity, scene, lane, cursor: 0, meshes: HashMap::new(), fallback: None, digest, target })), inputs }
    }

    /// 🥽️ One preparation unit; `true` once every mesh identity is installed.
    fn prepare_one(preparation: &mut FillToolRunPreparation) -> bool {
        let Some(url) = preparation.lane.get(preparation.cursor).cloned() else { return true };
        preparation.cursor += 1;
        if url != main::VORTEX_MARKER_MESH_KIND {
            let (body, source) = match shared_brush_mesh(&url).and_then(|(positions, indices)| Some((collision_body_from_buffers(&positions, &indices)?, brush_mesh_digest(&positions, &indices)))) {
                Some(real) => real,
                None if url == PUZZLE3D_FALLBACK_MESH_KIND => {
                    let fallback = preparation.fallback.get_or_insert_with(|| {
                        let (positions, indices) = puzzle3d_fallback_mesh_buffers();
                        collision_body_from_buffers(&positions, &indices).expect("the scaled box fallback is a valid collision body")
                    });
                    (fallback.clone(), String::new())
                }
                None => return false,
            };
            preparation.digest.extend_from_slice(url.as_bytes());
            preparation.digest.extend_from_slice(source.as_bytes());
            preparation.meshes.insert(url, body);
        }
        false
    }

    fn start(preparation: FillToolRunPreparation) -> FillToolRunPhase {
        let FillToolRunPreparation { identity, scene, lane, meshes, digest, target, .. } = preparation;
        let inputs: [u8; 32] = semio_framework_hash::hash(&digest).as_bytes()[..32].try_into().expect("thirty-two digest bytes");
        let seed = u64::from(scene.seed);
        let roots = FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes));
        let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(u64::from(identity.generation) + 1), seed);
        match target {
            FillToolRunTarget::Revalidate(placements) => FillToolRunPhase::Revalidate(Box::new(FillRevalidateJob::new(operation, identity, roots, placements, 0))),
            FillToolRunTarget::Run { requested, checkpoint, provisional } => FillToolRunPhase::Run(Box::new(FillRunJob::start(roots, operation, identity, lane, inputs, requested, checkpoint, provisional))),
        }
    }
}

/// 🎯️ The fill job retargets in place: a rebind restamps whatever phase is live, and a reconfigure that only moves
/// the count retargets the planner (or the preparation's target) without a rebuild or a replay. A changed overlap
/// budget or weight declines, so the framework rebuilds the run over its new inputs.
impl ToolRunRetargetableJob<Puzzle3dConfig> for Puzzle3dFillToolRunJob {
    fn rebind(&mut self, identity: ToolRunIdentity) {
        match &mut self.phase {
            FillToolRunPhase::Preparing(preparation) => preparation.identity = identity,
            FillToolRunPhase::Run(job) => job.rebind(identity),
            FillToolRunPhase::Revalidate(job) => job.rebind(identity),
            FillToolRunPhase::Closed => {}
        }
    }

    fn reconfigure(&mut self, identity: ToolRunIdentity, config: Arc<Puzzle3dConfig>) -> bool {
        if FillRunInputs::of(&config) != self.inputs {
            return false;
        }
        let requested = config.fill_count as usize;
        match &mut self.phase {
            FillToolRunPhase::Preparing(preparation) => match &mut preparation.target {
                FillToolRunTarget::Run { requested: target, .. } => {
                    preparation.identity = identity;
                    *target = requested;
                    true
                }
                FillToolRunTarget::Revalidate(_) => false,
            },
            FillToolRunPhase::Run(job) => {
                job.retarget(identity, requested);
                true
            }
            FillToolRunPhase::Revalidate(_) | FillToolRunPhase::Closed => false,
        }
    }
}

impl InteractiveJob for Puzzle3dFillToolRunJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        loop {
            match &mut self.phase {
                FillToolRunPhase::Preparing(preparation) => {
                    if context.is_cancelled() {
                        return StepOutcome::Cancelled;
                    }
                    if context.deadline_exceeded() {
                        return StepOutcome::Yield;
                    }
                    if !Self::prepare_one(preparation) {
                        continue;
                    }
                    let FillToolRunPhase::Preparing(preparation) = std::mem::replace(&mut self.phase, FillToolRunPhase::Closed) else { unreachable!("preparing phase") };
                    self.phase = Self::start(*preparation);
                }
                FillToolRunPhase::Run(job) => return job.step(context),
                FillToolRunPhase::Revalidate(job) => return job.step(context),
                FillToolRunPhase::Closed => return StepOutcome::Cancelled,
            }
        }
    }

    fn begin_close(&mut self) {
        match &mut self.phase {
            FillToolRunPhase::Run(job) => job.begin_close(),
            FillToolRunPhase::Revalidate(job) => job.begin_close(),
            FillToolRunPhase::Preparing(_) => self.phase = FillToolRunPhase::Closed,
            FillToolRunPhase::Closed => {}
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        match &mut self.phase {
            FillToolRunPhase::Run(job) => job.close_step(maximum_items, maximum_bytes),
            FillToolRunPhase::Revalidate(job) => job.close_step(maximum_items, maximum_bytes),
            FillToolRunPhase::Preparing(_) => {
                self.phase = FillToolRunPhase::Closed;
                InteractiveJobCloseStep::Complete
            }
            FillToolRunPhase::Closed => InteractiveJobCloseStep::Complete,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        match &self.phase {
            FillToolRunPhase::Run(job) => job.terminal_is_empty(),
            FillToolRunPhase::Revalidate(job) => job.terminal_is_empty(),
            FillToolRunPhase::Preparing(_) => false,
            FillToolRunPhase::Closed => true,
        }
    }
}
//#endregion 🔖️Jobs

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
