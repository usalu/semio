import { interactivityPuzzleFillRunJobFailures, type InteractivityPuzzleFillSources } from "../../../../../📜️script.ts";

/** 🧫️ A contract-conforming miniature of puzzle 3d fill as a framework tool run (`📋️tool-run-contract.md` §2.4, §3.7, §5 wave 1); shared with the trace self-test. */
export const PUZZLE_FILL_TOOL_RUN_FIXTURE: InteractivityPuzzleFillSources = {
  precompute: `//! ⏳️ Placement session: geometry caches only, the ledger owns the fill run.
pub struct Puzzle3dPrecomputeSession {
    meshes: Arc<HashMap<String, CollisionBody>>,
}

impl Puzzle3dPrecomputeSession {
    pub fn fill_run_job(&self, scene: Arc<SceneConfig>, operation: FillOperation) -> FillRunJob {
        FillRunJob::new(FillBuilder::begin_preparation(FillPreparationRoots::new(scene, self.meshes.clone()), operation))
    }
}
`,
  fill: `//! 🪣️ Fill run job.
pub(crate) struct FillRunJob {
    builder: FillBuilder,
    writer: ToolRunTickWriter,
    retirement: FillBuilderRetirementCursor,
}

impl semio_framework_job::InteractiveJob for FillRunJob {
    fn step(&mut self, cx: &mut StepContext) -> StepOutcome {
        while cx.consume_fuel(1) {
            let key = self.builder.next_key();
            self.writer.upsert(key, ToolRunVerdict::Testing, 0, ToolRunTraceSubject::Instance3d { mesh: 0, position: [0.0; 3], rotation: [0.0, 0.0, 0.0, 1.0], scale: 1.0 });
            match self.builder.test_one() {
                FillVerdict::Collision(subject) => self.writer.upsert(key, ToolRunVerdict::Danger, 1, subject),
                FillVerdict::Rule(subject) => self.writer.upsert(key, ToolRunVerdict::Warning, 2, subject),
                FillVerdict::Fits(subject, ops) => {
                    self.writer.upsert(key, ToolRunVerdict::Success, 3, subject);
                    for op in ops {
                        let _ = self.writer.append_op(op);
                    }
                    self.writer.append_entity(key);
                }
                FillVerdict::Stalled => {
                    let _ = self.writer.step(ToolRunStepKind::Warning, 0, 4, None, &[]);
                    return StepOutcome::Complete(self.builder.commit());
                }
            }
            if let Some(len) = self.builder.lowered_len() {
                self.writer.retract_to(len);
            }
        }
        if let Some(checkpoint) = self.builder.checkpoint() {
            return StepOutcome::CheckpointReady(checkpoint);
        }
        self.writer.finish().map_or(StepOutcome::Yield, |tick| StepOutcome::PreviewReady(self.builder.page(tick)))
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self) -> InteractiveJobCloseStep {
        self.retirement.retire_one()
    }

    fn terminal_is_empty(&self) -> bool {
        self.retirement.is_empty()
    }
}
`,
  geometry: `//! 📐️ Geometry.
pub(crate) struct FixedOwnerVec<T, const N: usize>;
`,
  editor: `//! ✏️ Puzzle 3d editor.
pub fn puzzle3d_dispatch(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    match action {
        "setFillCount" => set_fill_count::apply(ctx, args),
        _ => {}
    }
}

pub fn coalesce(action: &str) -> Option<String> {
    let coalesce_key = match action {
        "translateSelection" => Some("gumball-translate".to_string()),
        _ => None,
    };
    coalesce_key
}
`,
  tool: `//! 🪣️ Fill tool.
pub const TOOL_ID: &str = "fill";

pub fn definition(label: LocalizedLabel, labels: &Puzzle3dLabels) -> ToolDefinition {
    ToolDefinition { run: Some(fill_run_definition(labels)), ..semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, label, "paint-bucket")) }
}

pub fn fill_run_definition(labels: &Puzzle3dLabels) -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: true,
        rebase: ToolRunRebasePolicy::Revalidate,
        reconfigure: ToolRunReconfigurePolicy::Resume,
        unit: LocalizedLabel::native("candidate", "Kandidat"),
        stages: vec![ToolRunStageDefinition { id: "search".into(), label: labels.fill_stage_search.clone() }],
        counters: vec![ToolRunCounterDefinition { id: "tested".into(), label: labels.fill_tested.clone() }],
        reasons: vec![
            ToolRunReasonDefinition { code: 1, id: "collision".into(), verdict: ToolRunVerdict::Danger, template: labels.fill_collision.clone() },
            ToolRunReasonDefinition { code: 2, id: "rule".into(), verdict: ToolRunVerdict::Warning, template: labels.fill_rule.clone() },
            ToolRunReasonDefinition { code: 3, id: "fits".into(), verdict: ToolRunVerdict::Success, template: labels.fill_fits.clone() },
        ],
        trace: ToolRunTraceKind::Instance3d,
        run_job: JobKindId::new(FILL_RUN_JOB_KIND),
        revalidate_job: Some(JobKindId::new(FILL_REVALIDATE_JOB_KIND)),
    }
}

pub fn count_measure(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Number { id: "puzzle3d-fill-count".into(), label: Some(labels.count.into()), value: envelope.runtime.fill_count as f64, min: Some(0.0), max: None, step: Some(1.0), ready: None, loading: None, waiting: None, disabled: None, on_change: puzzle3d_action("setFillCount", None) }
}
`,
  terminology: `//! 🗣️ Terminology.
fill_tested: native_en "Tested", native_de "Getestet", reuse_en "Tested", reuse_de "Getestet",
`,
  setFillCount: `//! 🧮️ \`set-fill-count\`: a config change the tool-run driver observes as reconfigure.
pub(crate) fn parse_count(args: Option<&Value>) -> u32 {
    args.and_then(|value| value.get("value")).and_then(Value::as_f64).map_or(0, |value| value.round().clamp(0.0, f64::from(u32::MAX)) as u32)
}
`,
  fillBuildTick: "",
  editorTests: `mod tests {
    #[test]
    fn fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin() {}
    #[test]
    fn fill_run_finalize_publishes_one_edit_with_every_provisional_placement() {}
}
`,
  runFixture: JSON.stringify({
    laws: { opsPerPlacement: 2, parryOracle: { document: "concrete-forest", seed: 7, requested: 400 }, delivery: { document: "concrete-forest", seed: 7, requested: 1000000, candidates: 5000, deltaBudgetBytes: 65536 }, interactive: { document: "nakagin", seed: 1, requested: 100, turns: 771, budgetUs: 2000, coldRuns: 5 } },
    cases: [{ document: "nakagin", seed: 1, requested: 12, expected: { verdictPrefix: ["testing:candidate", "danger:collision", "success:fits"] } }],
  }),
  previewFixture: "",
  schema: `//! 🧬️ Schema.
pub struct FillRunCheckpoint {
    pub requested: u32,
}
`,
  transport: `//! 🧊️ Main window.
pub fn world_scene_body(precompute: &Puzzle3dPrecomputeSession) -> WorldScene {
    WorldScene::from_session(precompute)
}
`,
  renderer: `/** 🌐️ World host. */
export function WorldTraceMount({ lane }: { readonly lane: string }) {
  return <ToolRunTraceLayer lane={lane} />;
}
`,
  puzzle5dPrecompute: `//! 🧠️ Puzzle 5d precompute reuses the 3d fill run job.
pub fn fill_run_job(inner: &Puzzle3dPrecomputeSession, scene: Arc<SceneConfig>, operation: FillOperation) -> FillRunJob {
    inner.fill_run_job(scene, operation)
}
`,
  puzzle5dWindow: `//! 🧊️ Puzzle 5d 3d window.
pub fn world_scene_body(precompute: &Puzzle5dPrecomputeSession) -> WorldScene {
    WorldScene::from_session(precompute)
}
`,
};

type Edit = readonly [role: keyof InteractivityPuzzleFillSources, from: string, to: string];

/** 🧪️ Applies anchored edits to the shared fixture, failing when an anchor is missing or ambiguous. */
export function puzzleFillToolRunFixtureWith(name: string, edits: readonly Edit[]): InteractivityPuzzleFillSources {
  const sources: Record<string, string> = { ...PUZZLE_FILL_TOOL_RUN_FIXTURE };
  for (const [role, from, to] of edits) {
    if (from === "") {
      sources[role] = to;
      continue;
    }
    if (sources[role]!.split(from).length !== 2) throw new Error(`[verify interactivity] puzzle fill self-test ${name} anchor is missing or ambiguous in ${role}: ${from}`);
    sources[role] = sources[role]!.replace(from, to);
  }
  return sources as InteractivityPuzzleFillSources;
}

const CASES: readonly (readonly [name: string, expect: "report" | "silent", edits: readonly Edit[]])[] = [
  ["tool-without-run", "report", [["tool", "ToolDefinition { run: Some(fill_run_definition(labels)), ..", "ToolDefinition { .."]]],
  ["readonly-run", "report", [["tool", "mutating: true", "mutating: false"]]],
  ["restart-on-rebase", "report", [["tool", "ToolRunRebasePolicy::Revalidate", "ToolRunRebasePolicy::Restart"]]],
  ["restart-on-reconfigure", "report", [["tool", "ToolRunReconfigurePolicy::Resume", "ToolRunReconfigurePolicy::Restart"]]],
  ["untraced-run", "report", [["tool", "ToolRunTraceKind::Instance3d", "ToolRunTraceKind::None"]]],
  ["missing-revalidate-job", "report", [["tool", "revalidate_job: Some(JobKindId::new(FILL_REVALIDATE_JOB_KIND)),", ""]]],
  ["no-danger-reason", "report", [["tool", "verdict: ToolRunVerdict::Danger", "verdict: ToolRunVerdict::Warning"]]],
  ["unlocalized-definition", "report", [["tool", 'LocalizedLabel::native("candidate", "Kandidat")', 'labels.fill_unit.clone()']]],
  ["not-an-interactive-job", "report", [["fill", "impl semio_framework_job::InteractiveJob for FillRunJob", "impl FillRunJob"]]],
  ["batched-fuel", "report", [["fill", "cx.consume_fuel(1)", "cx.consume_fuel(64)"]]],
  ["no-checkpoint", "report", [["fill", "return StepOutcome::CheckpointReady(checkpoint);", "let _ = checkpoint;"]]],
  ["hidden-collisions", "report", [["fill", "self.writer.upsert(key, ToolRunVerdict::Danger, 1, subject)", "()"]]],
  ["no-testing-record", "report", [["fill", "ToolRunVerdict::Testing", "ToolRunVerdict::Success"]]],
  ["committed-placements", "report", [["fill", "let _ = self.writer.append_op(op);", "self.document.apply(op);"]]],
  ["no-retraction", "report", [["fill", "self.writer.retract_to(len);", "let _ = len;"]]],
  ["silent-stall", "report", [["fill", "ToolRunStepKind::Warning", "ToolRunStepKind::Info"]]],
  ["bulk-close", "report", [["fill", "fn close_step(&mut self) -> InteractiveJobCloseStep {", "fn close_all(&mut self) -> InteractiveJobCloseStep {"]]],
  ["plugin-spawn", "report", [["precompute", "    pub fn fill_run_job", "    pub fn enqueue_fill_job(&mut self) {}\n\n    pub fn fill_run_job"]]],
  ["plugin-cancel-verb", "report", [["editor", '"setFillCount" => set_fill_count::apply(ctx, args),', '"setFillCount" => set_fill_count::apply(ctx, args),\n        "cancelFillBuild" => cancel(ctx, args),']]],
  ["lock-is-commit", "report", [["setFillCount", "pub(crate) fn parse_count", "pub(crate) fn take_locked_into_fixture() {}\n\npub(crate) fn parse_count"]]],
  ["history-coalesced-count", "report", [["editor", '"translateSelection" => Some("gumball-translate".to_string()),', '"translateSelection" => Some("gumball-translate".to_string()),\n        "setFillCount" => Some("fill-count".to_string()),']]],
  ["tick-command-survives", "report", [["fillBuildTick", "", "pub fn fill_build_tick() {}"]]],
  ["lock-is-commit-fixture-survives", "report", [["editorTests", "    #[test]\n    fn fill_run_finalize", "    #[test]\n    fn fill_build_tick_locks_planned_placements_into_the_document_in_bounded_chunks() {}\n    #[test]\n    fn fill_run_finalize"]]],
  ["missing-successor-fixture", "report", [["editorTests", "fill_run_finalize_publishes_one_edit_with_every_provisional_placement", "fill_finalize_smoke"]]],
  ["loosened-budget", "report", [["runFixture", '"budgetUs":2000', '"budgetUs":5090']]],
  ["partial-delivery-law", "report", [["runFixture", '"candidates":5000', '"candidates":4999']]],
  ["missing-oracle-law", "report", [["runFixture", '"parryOracle":{"document":"concrete-forest","seed":7,"requested":400},', ""]]],
  ["unqualified-verdict", "report", [["runFixture", '"danger:collision"', '"collision"']]],
  ["malformed-fixture", "report", [["runFixture", "", "{"]]],
  ["commented-legacy-is-silent", "silent", [["precompute", "pub struct Puzzle3dPrecomputeSession {", "// enqueue_fill_job and \"fill-count\" were retired with take_locked_into_fixture.\npub struct Puzzle3dPrecomputeSession {"]]],
  ["test-only-legacy-is-silent", "silent", [["editor", "pub fn coalesce", "#[cfg(test)]\nmod tests {\n    fn legacy() { let _ = \"fillBuildTick\"; }\n}\n\npub fn coalesce"]]],
];

/** 🧪️ Mutation self-test of the puzzle fill run-job law: the clean fixture passes, every planted regression is reported, and comment- or test-only mentions stay silent. Returns the executed case count. */
export function interactivityPuzzleFillRunJobSelfTests(): number {
  const clean = interactivityPuzzleFillRunJobFailures(PUZZLE_FILL_TOOL_RUN_FIXTURE);
  if (clean.length > 0) throw new Error(`[verify interactivity] Puzzle fill run-job clean fixture was falsely rejected: ${clean.join("; ")}`);
  for (const [name, expect, edits] of CASES) {
    const failures = interactivityPuzzleFillRunJobFailures(puzzleFillToolRunFixtureWith(name, edits));
    if (expect === "report" && failures.length === 0) throw new Error(`[verify interactivity] Puzzle fill run-job self-test ${name} was falsely accepted.`);
    if (expect === "silent" && failures.length > 0) throw new Error(`[verify interactivity] Puzzle fill run-job self-test ${name} was falsely reported: ${failures.join("; ")}`);
  }
  return CASES.length + 1;
}
