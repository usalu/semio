/** ⏳️ Interactivity audit: puzzle-fill tool-run laws, tool-run policy and history closure.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts
 */
import { type InteractivityToolRunSource, type InteractivityToolRunRequirement, INTERACTIVITY_TOOL_RUN_TRIGGER, interactivityToolRunWithin, interactivityToolRunScopePath, interactivityToolRunCode, interactivityToolRunEnclosing } from "../../../🔍️discovery/📋️policy/⏯️tool-run/🟦️.ts";
import { type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { POLICY_SKIP_DIRS, policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyWalkRelFiles } from "../../../🔍️discovery/🚶️file-walk/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { interactivityIsRuntimeSource } from "../🔎️scan/🟦️.ts";
import { POLICY_RUST_TEST_EVIDENCE_BOUNDARY, policyReadRustPolicySource } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

//#region 🪣️PuzzleFillToolRun
/** 🪣️ Puzzle fill sources the run-job, trace and P4e predicates read, keyed by role (`📋️tool-run-contract.md` §5 wave 1). */
export const INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES = {
  precompute: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs",
  fill: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs",
  geometry: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs",
  editor: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  tool: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs",
  terminology: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs",
  setFillCount: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️set-fill-count/🦀️.rs",
  fillBuildTick: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs",
  editorTests: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
  runFixture: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🧫️fixtures/🎞️fill-run.json",
  previewFixture: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🧫️fixtures/🔣️.json",
  schema: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs",
  transport: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs",
  renderer: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx",
  puzzle5dPrecompute: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs",
  puzzle5dWindow: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️.rs",
} as const;

/** 🪣️ Raw text per [[INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES]] role; a missing file is the empty string. */
export type InteractivityPuzzleFillSources = Readonly<Record<keyof typeof INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES, string>>;

/** 🪣️ Reads every puzzle fill role, Rust files with their declared canonical test evidence appended. */
export function interactivityPuzzleFillSources(repoRoot: string): InteractivityPuzzleFillSources {
  return Object.fromEntries(Object.entries(INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES).map(([key, path]) => [key, policyReadRustPolicySource(repoRoot, path)])) as InteractivityPuzzleFillSources;
}

/** ✂️ Production code per role: test evidence, `#[cfg(test)]` items and comments removed, string literals kept. */
function interactivityPuzzleFillCode(sources: InteractivityPuzzleFillSources): InteractivityPuzzleFillSources {
  return Object.fromEntries(Object.entries(sources).map(([key, text]) => {
    const path = INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES[key as keyof InteractivityPuzzleFillSources];
    return [key, path.endsWith(".json") ? text : interactivityToolRunCode({ path, text: text.split(POLICY_RUST_TEST_EVIDENCE_BOUNDARY, 1)[0]! }).join("\n")];
  })) as InteractivityPuzzleFillSources;
}

/**
 * 🪣️ Puzzle 3d fill is a framework tool run: the fill tool declares a mutating `instance3d` `ToolRunDefinition`, its
 * `FillBuilder` run job spends one fuel unit per candidate and streams every verdict as a trace upsert, accepted
 * placements stay provisional `appendOps`, and the plugin owns no lifecycle (no tick command, lock-is-commit, plugin
 * spawn/cancel or history coalescing). See `📋️tool-run-contract.md` §2.4, §2.7, §3.6, §3.7 and §5 W1-A/W1-B/W1-F.
 */
export function interactivityPuzzleFillRunJobFailures(sources: InteractivityPuzzleFillSources): string[] {
  const code = interactivityPuzzleFillCode(sources);
  const failures: string[] = [];
  const missing = (source: string, needles: readonly string[]) => needles.filter((needle) => !source.includes(needle));
  const definition = [code.tool, code.editor, code.terminology, code.precompute, code.fill].join("\n");
  const toolLiteral = interactivityToolRunEnclosing(code.tool, code.tool.search(/\bToolDefinition::new\(\s*TOOL_ID\b/), /\bToolDefinition\s*\{/g) ?? "";
  if (!/\brun\s*:\s*Some\s*\(/.test(toolLiteral)) failures.push("Puzzle fill ToolDefinition(TOOL_ID) declares no run: Some(ToolRunDefinition)");
  const policy = missing(definition, ["mutating: true", "ToolRunRebasePolicy::Revalidate", "ToolRunReconfigurePolicy::Resume", "ToolRunTraceKind::Instance3d", "run_job: JobKindId::new(", "revalidate_job: Some(JobKindId::new("]);
  if (policy.length > 0) failures.push(`Puzzle fill run definition is not a mutating instance3d run with revalidate rebase, resume reconfigure, run and revalidate jobs (missing ${policy.join(", ")})`);
  const vocabulary = missing(definition, ["ToolRunStageDefinition {", "ToolRunCounterDefinition {", "ToolRunReasonDefinition {", "verdict: ToolRunVerdict::Danger", "verdict: ToolRunVerdict::Warning", "verdict: ToolRunVerdict::Success", "LocalizedLabel::native("]);
  if (vocabulary.length > 0) failures.push(`Puzzle fill run definition lacks localized stages, counters or verdict-typed reasons (missing ${vocabulary.join(", ")})`);
  const stepping = missing(code.fill, ["ToolRunTickWriter", "consume_fuel(1)", "JobTurn::Preview(", "JobTurn::Checkpoint {", "JobTurn::Complete"]);
  if (!/\bimpl\s+(?:[\w:]+::)?InteractiveJob\s+for\s+Fill\w*/.test(code.fill) || stepping.length > 0) failures.push(`Puzzle fill run job is not an InteractiveJob spending one fuel unit per candidate and publishing ticks, checkpoints and completion (missing ${stepping.join(", ") || "impl InteractiveJob for Fill…"})`);
  const verdicts = missing(code.fill, [".upsert(", "ToolRunVerdict::Testing", "ToolRunVerdict::Danger", "ToolRunVerdict::Warning", "ToolRunVerdict::Success", "ToolRunTraceSubject::Instance3d"]);
  if (verdicts.length > 0) failures.push(`Puzzle fill run job does not stream every tested candidate as a testing, danger, warning or success trace upsert (missing ${verdicts.join(", ")})`);
  const provisional = missing(code.fill, [".append_op(", ".append_entity(", ".retract_to("]);
  if (provisional.length > 0) failures.push(`Puzzle fill accepted placements are not provisional appendOps and entities with retraction on a lowered count (missing ${provisional.join(", ")})`);
  if (!code.fill.includes("ToolRunStepKind::Warning")) failures.push("Puzzle fill stall is not reported as a warning step");
  const retirement = missing(code.fill, ["fn begin_close", "fn close_step", "fn terminal_is_empty"]);
  if (retirement.length > 0) failures.push(`Puzzle fill run job lacks incremental begin_close/close_step retirement (missing ${retirement.join(", ")})`);
  for (const token of ["enqueue_fill_job", "cancel_fill_job", "fill_job_identity", "poll_fill_job", "take_fill_locked_chunk", "take_locked_into_fixture", "take_locked_mutations", "FILL_LOCK_PLACEMENTS_PER_TICK", "kind: FILL_JOB_KIND", '"fillBuildTick"', '"cancelFillBuild"', '"fill-count"']) {
    if ([code.precompute, code.editor, code.tool, code.setFillCount].some((source) => source.includes(token))) failures.push(`Puzzle fill plugin still owns the run lifecycle (tick, lock-is-commit, plugin spawn or cancel, history coalescing): ${token}`);
  }
  if (sources.fillBuildTick.length > 0) failures.push("Puzzle fill fill-build-tick command still exists");
  const evidence = [sources.editorTests, sources.fill, sources.precompute].join("\n");
  for (const fixture of ["fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin", "fill_run_finalize_publishes_one_edit_with_every_provisional_placement"]) {
    if (!evidence.includes(`fn ${fixture}`)) failures.push(`Puzzle fill red→green successor fixture missing: ${fixture}`);
  }
  for (const fixture of ["fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin", "fill_build_tick_locks_planned_placements_into_the_document_in_bounded_chunks"]) {
    if (evidence.includes(`fn ${fixture}`)) failures.push(`Puzzle fill lock-is-commit fixture survives: ${fixture}`);
  }
  try {
    const fixture = JSON.parse(sources.runFixture) as { laws?: { opsPerPlacement?: unknown; parryOracle?: unknown; delivery?: { candidates?: unknown }; interactive?: { document?: unknown; turns?: unknown; budgetWork?: unknown; stepWorkCeiling?: unknown; appendOpsCeiling?: unknown } }; cases?: { expected?: { verdictPrefix?: unknown } }[] };
    const laws = fixture.laws;
    const interactive = laws?.interactive;
    if (!laws || !(Number(laws.opsPerPlacement) >= 1) || typeof laws.parryOracle !== "object" || laws.parryOracle === null || !(Number(laws.delivery?.candidates) >= 5000) || interactive?.document !== "nakagin" || !(Number(interactive?.turns) >= 1) || interactive?.budgetWork !== 1000 || !(Number(interactive?.stepWorkCeiling) > 0 && Number(interactive?.stepWorkCeiling) <= 100000) || !(Number(interactive?.appendOpsCeiling) > 0)) failures.push("Puzzle fill run fixture does not pin the parry3d oracle, the 5 000-candidate delivery law and the deterministic Nakagin work slice (budgetWork 1000, stepWorkCeiling at most 100 000)");
    if (!Array.isArray(fixture.cases) || fixture.cases.length === 0 || !fixture.cases.every((entry) => Array.isArray(entry.expected?.verdictPrefix) && (entry.expected.verdictPrefix as unknown[]).every((verdict) => typeof verdict === "string" && /^(?:testing|success|warning|danger):[\w.-]+$/.test(verdict)))) failures.push("Puzzle fill run fixture cases do not carry verdict:reason prefixes");
  } catch {
    failures.push("Puzzle fill run fixture is missing or malformed");
  }
  return failures;
}

/**
 * 🪣️ Puzzle fill shows its process through tool-run trace pages only: no diagnostic page, candidate ghost or reveal
 * cutoff survives in the 3d/5d producers or World3dHost, the legacy preview JSON fixture is gone, and World3dHost mounts
 * the framework `ToolRunTraceLayer` (`📋️tool-run-contract.md` §3.2, §3.6, §4.1, §5 W1-C).
 */
export function interactivityPuzzleFillTraceFailures(sources: InteractivityPuzzleFillSources): string[] {
  const code = interactivityPuzzleFillCode(sources);
  const failures: string[] = [];
  const producers = { fill: code.fill, precompute: code.precompute, schema: code.schema, transport: code.transport, puzzle5dPrecompute: code.puzzle5dPrecompute, puzzle5dWindow: code.puzzle5dWindow };
  for (const [role, source] of Object.entries(producers)) {
    for (const token of ["candidate_page", "candidate_ghost", "FillDiagnostic", "world_fill_preview_json"]) if (source.includes(token)) failures.push(`Puzzle fill ${role} still publishes the retired diagnostic page: ${token}`);
  }
  for (const token of ["WorldFillDiagnosticRecord", "FillDiagnosticOverlay", "latestFillIdentityRef", "data-fill-", "WORLD_FILL_"]) if (code.renderer.includes(token)) failures.push(`World3dHost still parses or paints the retired fill diagnostic: ${token}`);
  for (const [role, source] of Object.entries({ ...producers, renderer: code.renderer })) {
    const reveal = source.match(/\b(?:reveal_index|revealIndex|revealCutoffs?|reveal_cutoffs?)\b/);
    if (reveal) failures.push(`Puzzle fill ${role} still carries a reveal cutoff: ${reveal[0]}`);
  }
  if (!code.renderer.includes("<ToolRunTraceLayer")) failures.push("World3dHost does not mount ToolRunTraceLayer for tool-run trace pages");
  if (sources.previewFixture.length > 0) failures.push("Puzzle fill preview JSON fixture still exists");
  return failures;
}

/** 🧮️ P4e FillBuilder-intrinsic laws that survive the tool-run conversion: cooperative preparation, fixed owners with exact cap/+1 handback, resumable spatial index, no whole-state checkpoint or clone, danger step before a capacity fault. */
export function interactivityPuzzleFillP4eFailures(precomputeSource: string, fillSource: string, geometrySource: string): string[] {
  const [precompute, fill, geometry] = [precomputeSource, fillSource, geometrySource].map((text) => interactivityToolRunCode({ path: "🦀️.rs", text: text.split(POLICY_RUST_TEST_EVIDENCE_BOUNDARY, 1)[0]! }).join("\n")) as [string, string, string];
  const failures: string[] = [];
  const preparationStages = ["PrepareScene", "PrepareCatalogs", "PrepareMeshes", "PrepareEntries", "PrepareSpatial", "PrepareLookup", "PrepareConfiguration"];
  if (
    precompute.includes("FillBuilder::new(") ||
    fill.includes("FillBuilder::new(") ||
    precompute.includes(".configure(") ||
    precompute.includes(".refresh_meshes(") ||
    precompute.includes(".rebuild_collision_index(") ||
    ![precompute, fill].some((source) => source.includes("FillBuilder::begin_preparation(")) ||
    !fill.includes("pub(crate) struct FillPreparationRoots") ||
    !preparationStages.every((stage) => fill.includes(`FillJobStage::${stage}`)) ||
    !fill.includes("self.prepare_one()")
  ) failures.push("P4e fill construction/reconfiguration is not one generation-owned cooperative preparation path");
  const preparationBranches = ["FixtureObjects", "FixtureAttractions", "FixtureTargetVolumes", "Meshes", "CatalogObjects", "CatalogVortices", "CatalogCables", "KindCompatibility", "ObjectWeights", "VortexWeights"];
  const preparationLengths = [
    "roots.scene.fixture.objects.len()",
    "roots.scene.fixture.attractions.len()",
    "roots.scene.fixture.target_volumes.len()",
    "roots.meshes.len()",
    "catalogs.map_or(0, |value| value.objects.len())",
    "catalogs.map_or(0, |value| value.vortices.len())",
    "catalogs.map_or(0, |value| value.cables.len())",
    "roots.scene.kind_compatibility.len()",
    "roots.scene.weights.object_weights.len()",
    "roots.scene.weights.vortex_weights.len()",
  ];
  const fixedPreparationOwners = [
    "pub(crate) objects: FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS>",
    "pub(crate) attractions: FixedOwnerVec<AttractionProps, DOCUMENT_ATTRACTION_SLOTS>",
    "pub(crate) target_volumes: FixedOwnerVec<WorldVolumeProps, DOCUMENT_VOLUME_SLOTS>",
    "objects: FixedOwnerVec<ObjectKind, DOCUMENT_KIND_SLOTS>",
    "vortices: FixedOwnerVec<VortexKindCatalog, DOCUMENT_KIND_SLOTS>",
    "cables: FixedOwnerVec<CableKindCatalog, DOCUMENT_KIND_SLOTS>",
    "kind_compatibility: FixedOwnerVec<KindCompatEntry, DOCUMENT_KIND_SLOTS>",
    "meshes: FixedOwnerMap<String, CollisionBody, DOCUMENT_KIND_SLOTS>",
    "object_weights: FixedOwnerMap<String, f64, DOCUMENT_KIND_SLOTS>",
    "vortex_weights: FixedOwnerMap<String, f64, DOCUMENT_KIND_SLOTS>",
  ];
  const hostilePreparationBranches = [
    '(HostileRoot::FixtureObjects, "fixture-objects", DOCUMENT_OBJECT_SLOTS)',
    '(HostileRoot::FixtureAttractions, "fixture-attractions", DOCUMENT_ATTRACTION_SLOTS)',
    '(HostileRoot::FixtureTargetVolumes, "fixture-target-volumes", DOCUMENT_VOLUME_SLOTS)',
    '(HostileRoot::Meshes, "meshes", DOCUMENT_KIND_SLOTS)',
    '(HostileRoot::CatalogObjects, "catalog-objects", DOCUMENT_KIND_SLOTS)',
    '(HostileRoot::CatalogVortices, "catalog-vortices", DOCUMENT_KIND_SLOTS)',
    '(HostileRoot::CatalogCables, "catalog-cables", DOCUMENT_KIND_SLOTS)',
    '(HostileRoot::KindCompatibility, "kind-compatibility", DOCUMENT_KIND_SLOTS)',
    '(HostileRoot::ObjectWeights, "object-weights", DOCUMENT_KIND_SLOTS)',
    '(HostileRoot::VortexWeights, "vortex-weights", DOCUMENT_KIND_SLOTS)',
  ];
  if (
    !geometry.includes("pub(crate) struct FixedOwnerVec") ||
    !fill.includes("fn preparation_capacity_refusal") ||
    !preparationBranches.every((branch) => fill.includes(`PreparationCapacityBranch::${branch}`)) ||
    !preparationLengths.every((length) => fill.includes(length)) ||
    !fixedPreparationOwners.every((owner) => fill.includes(owner)) ||
    ["self.base.objects.push(", "self.base.attractions.push(", "self.base.target_volumes.push(", "self.catalogs.objects.push(", "self.catalogs.vortices.push(", "self.catalogs.cables.push(", "self.kind_compatibility.push("].some((push) => fill.includes(push)) ||
    !fillSource.includes("preparation_refusal_owner_for_test") ||
    !hostilePreparationBranches.every((branch) => fillSource.includes(branch)) ||
    !fillSource.includes("roots(branch, cap)") ||
    !fillSource.includes("roots(branch, cap + 1)") ||
    !fillSource.includes("fn nakagin_scale_fill_is_not_refused_and_places_at_least_one_object") ||
    !fillSource.includes("fn document_capacities_match_the_language_neutral_capacity_law") ||
    !fill.includes("(len > capacity).then_some((branch, capacity))") ||
    !fill.includes('format!("preparation-capacity:{}:{}", self.branch.label(), self.omitted_index)')
  ) failures.push("P4e preparation preflight/storage omits a fixture, mesh, catalog, or compatibility fixed owner or lacks exact cap/+1 handback evidence");
  const refusalStart = fill.indexOf("if let Some(refusal) = self.preparation_capacity_refusal.as_mut()");
  const refusal = fill.slice(refusalStart, fill.indexOf("if self.collection_over_capacity", refusalStart));
  if (refusalStart < 0 || !refusal.includes("ToolRunStepKind::Danger") || !refusal.includes("JobTurn::Fault") || refusal.indexOf("ToolRunStepKind::Danger") > refusal.indexOf("JobTurn::Fault") || refusal.includes("candidate_ghost")) failures.push("P4e capacity refusal does not publish a danger step before the terminal fault");
  if (
    !geometry.includes("cells: FixedOwnerMap<(i32, i32, i32), CollisionCellMembers, DOCUMENT_CELL_SLOTS>") ||
    !geometry.includes("struct CollisionCellSpan") ||
    !geometry.includes("pub(crate) struct CollisionIndexMutation") ||
    !geometry.includes("pub(crate) struct CollisionIndexRemoval") ||
    !geometry.includes("pub(crate) struct CollisionQueryCursor") ||
    !geometry.includes("mutation.owner != current") ||
    !geometry.includes("query.owner != current") ||
    !geometry.includes("CollisionMutationStage::PreflightNew") ||
    geometry.includes("fn covered_cells") ||
    geometry.includes("pub(crate) fn upsert") ||
    geometry.includes("pub(crate) fn remove(") ||
    geometry.includes("pub(crate) fn query(") ||
    geometry.includes("entry_intersects") ||
    fill.includes("spatial_index.upsert(") ||
    fill.includes("self.placed.iter()") ||
    !fill.includes("self.spatial_index.step_query(query, owner)") ||
    !fill.includes("self.spatial_index.step_replacement(mutation, owner)") ||
    !geometry.includes("CollisionOverlapStage::SamplingPointA") ||
    !geometry.includes("CollisionOverlapStage::SamplingPointB")
  ) failures.push("P4e spatial owner is not fixed, resumable, generation-bound, and used by the production broad phase");
  if (["FillJobCheckpoint", "checkpoint_bytes", "restore_checkpoint", "normalized_checkpoint", "cloned_btree", "try_from_btree", "clear_for_rebuild_residual"].some((token) => fillSource.includes(token)) || ["cloned_btree", "try_from_btree", "clear_for_rebuild_residual"].some((token) => geometrySource.includes(token))) failures.push("P4e retained a dormant whole-state checkpoint, clone, conversion, or rebuild-clear escape");
  if (["self.sequence.clone()", "self.appended_objects.clone()", "self.appended_attractions.clone()"].some((clone) => fill.includes(clone)) || fillSource.includes("accepted_prefix")) failures.push("P4e progress or tick publication clones the accepted sequence or appended owners");
  for (const fixture of ["constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently", "stale_generation_stops_preparation_before_installing_any_entry"]) {
    if (!fillSource.includes(`fn ${fixture}`)) failures.push(`P4e preparation fixture missing ${fixture}`);
  }
  for (const fixture of ["spatial_resumable_query_narrows_sparse_cells_without_visiting_distant_population", "spatial_capacity_plus_one_refusal_preserves_exact_old_state", "spatial_stale_owner_cannot_finish_partial_replacement", "spatial_multi_cell_oversized_replacement_and_removal_make_bounded_progress"]) {
    if (!geometrySource.includes(`fn ${fixture}`)) failures.push(`P4e spatial fixture missing ${fixture}`);
  }
  return failures;
}
//#endregion 🪣️PuzzleFillToolRun

//#region ⏯️ToolRunPolicy












const INTERACTIVITY_TOOL_RUN_ANY = "🏅️standards/🔖️1/🪆️subsets/✳️any";

/**
 * ⏯️ Every `algorithmic-mutating` tool of the phase-4 inventory with its plugin path; each must declare a
 * `ToolRunDefinition` (`📋️tool-run-contract.md` §2.4). Read-only runs (layout/shooting exports, architect
 * report/validation/search, puzzle 3d brush suggestions), the param-only draw trace, generation3d `importDocument` (a
 * host-driven chunk transfer closed by one bounded parse that already publishes one staged edit) and the inner
 * `WfcJob` capability (covered by the assembly row) are deliberately absent; the read-only generation3d and generation2d
 * `previewEval` runs are listed because they converted. A converting lane edits its row.
 */
export const INTERACTIVITY_TOOL_RUN_REQUIREMENTS: readonly InteractivityToolRunRequirement[] = [
  { toolId: "fill", root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill", "✏️editor/🎮️commands/🪣️fill-build-tick", "✏️editor/🎮️commands/🧮️set-fill-count", "✏️editor/⏳️precompute/🪣️fill"], verbs: ["fillBuildTick", "fill_build_tick", "cancelFillBuild", "cancel_fill_build", "take_locked_into_fixture", "FILL_LOCK_PLACEMENTS_PER_TICK", "enqueue_fill_job", "fill_job_identity"], measures: ["cancel_measure", "progress_measure"], lane: "W1-B", inventory: "§1.1" },
  { toolId: "importSnapshot", root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/📥️import-snapshot"], verbs: [], measures: [], lane: "unassigned", inventory: "§1.1" },
  { toolId: "fill", root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill", "✏️editor/🎮️commands/🧮️set-fill-count", "✏️editor/⏳️precompute/🪣️fill"], verbs: ["brushFillSessionBegin", "brushFillSessionStep", "brushFillSessionCancel", "brushFillSessionDiscard", "brushFillSessionAdopt", "fill_session_begin", "fill_session_step", "fill_session_clear", "Puzzle2dFillLifecycle"], measures: ["cancel_measure", "progress_measure"], lane: "W2-A", inventory: "§1.2" },
  { toolId: "fill", root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎭️modes/✏️edit/☑️options/🪣️fill", "✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🪛️utilities/🪣️fill", "✏️editor/🎮️commands/🧮️set-fill-count", "✏️editor/🎮️commands/🛑️cancel-fill-build", "✏️editor/🧠️precompute"], verbs: ["cancelFillBuild", "cancel_fill_build"], measures: ["fill_cancel_measure", "fill_progress_measure"], lane: "W2-B", inventory: "§1.3" },
  { toolId: "previewEval", root: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["🧵️preview-eval/⏯️tool-run"], verbs: ["cancelPreviewEval", "cancel_preview_eval", "CancelPreviewEval", "PREVIEW_CANCEL_ACTION_ID", "rearm_attached_previews"], measures: [], lane: "W3-2 procedural preview eval", inventory: "§2.1" },
  { toolId: "reorganize", root: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/🗺️reorganize"], verbs: [], measures: [], lane: "unassigned", inventory: "§2.1" },
  { toolId: "previewEval", root: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["🧵️preview-eval/⏯️tool-run"], verbs: ["rearm", "Generation2dPreviewCommandWork", "generation2d-preview-evaluation"], measures: [], lane: "W3-2b generation2d preview eval", inventory: "§2.2" },
  { toolId: "reorganize", root: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/🗺️reorganize"], verbs: [], measures: [], lane: "unassigned", inventory: "§2.2" },
  { toolId: "energySimulation", root: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["🧵️simulation-session", "✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation"], verbs: ["start-energy-simulation", "cancel-energy-simulation", "retry-energy-simulation", "discard-energy-simulation", "adopt-energy-simulation", "EnergySimulationStatus"], measures: [], lane: "W3 (1) energy", inventory: "§3.1" },
  { toolId: "reconstruction", root: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🧵️reconstruction-session", "✏️editor/🎭️modes/🧊️model/🛠️tools/🏗️reconstruction", "✏️editor/📌️panels/🗿️artifact"], verbs: ["runReconstruction", "advanceReconstruction", "cancelReconstruction", "retryStage", "runStage"], measures: [], lane: "W3 (4) remodel", inventory: "§3.3" },
  { toolId: "decimate", root: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/🔷️mesh-edit", "🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs"], verbs: [], measures: [], lane: "W3 (15) lowpoly", inventory: "§3.4" },
  { toolId: "formatDocument", root: `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/🧹️format-document"], verbs: [], measures: [], lane: "unassigned", inventory: "§4.1" },
  { toolId: "reorganize", root: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/🗺️reorganize"], verbs: [], measures: [], lane: "W3-F layout-run", inventory: "§4.3" },
  { toolId: "reorganize", root: `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/🔄️layout"], verbs: [], measures: [], lane: "W3-F layout-run", inventory: "§4.7" },
  { toolId: "runAnalysis", root: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/🔬️analysis"], verbs: [], measures: [], lane: "W3 (12) architect", inventory: "§4.8" },
  { toolId: "importProgram", root: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/📤️exchange"], verbs: [], measures: [], lane: "W3 (12) architect", inventory: "§4.8" },
  { toolId: "reorganize", root: `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize"], verbs: ["forceLayout", "force-layout"], measures: [], lane: "W3-F1 layout-run consumers", inventory: "§5.1" },
  { toolId: "importCadFile", root: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/📥️io"], verbs: [], measures: [], lane: "unassigned", inventory: "§5.4" },
  { toolId: "run", root: `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/🏃️run"], verbs: [], measures: [], lane: "W3 (10) imperative", inventory: "§5.7" },
  { toolId: "reorganize", root: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize"], verbs: [], measures: [], lane: "W3-F1 layout-run consumers", inventory: "§5.8" },
  { toolId: "reorganize", root: `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize"], verbs: [], measures: [], lane: "W3-F1 layout-run consumers", inventory: "§5.9" },
  { toolId: "combineBoolean", root: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/🔀️combine-boolean"], verbs: [], measures: [], lane: "unassigned", inventory: "§6.1" },
  { toolId: "inkApplyEvents", root: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/${INTERACTIVITY_TOOL_RUN_ANY}`, scope: ["✏️editor/🎮️commands/🖊️ink-apply-events"], verbs: [], measures: [], lane: "W3 (14) note", inventory: "§6.2" },
  { toolId: "exportStudioPack", root: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space", scope: ["🎮️commands/📦️export-studio-pack"], verbs: [], measures: [], lane: "W3 (13) space", inventory: "§6.4" },
  { toolId: "exportMedia", root: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space", scope: ["🎮️commands/📤️export-media"], verbs: [], measures: [], lane: "W3 (13) space", inventory: "§6.4" },
  { toolId: "reorganizeWorkflow", root: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space", scope: ["🎮️commands/🗂️reorganize-workflow"], verbs: [], measures: [], lane: "W3 (13) space", inventory: "§6.4" },
];



























/** 📚️ Every repo runtime Rust/TS source a tool-run predicate can hit: under a requirement root or scope, or carrying one of their trigger tokens. */
export function interactivityToolRunPolicySources(repoRoot: string, requirements: readonly InteractivityToolRunRequirement[]): InteractivityToolRunSource[] {
  const anchors = requirements.flatMap((row) => [row.root, ...row.scope.map((entry) => interactivityToolRunScopePath(row.root, entry))]);
  return policyWalkRelFiles(repoRoot, ["✏️s", "🧰️framework"], (_path, name) => /\.(?:rs|tsx?)$/.test(name))
    .filter(interactivityIsRuntimeSource)
    .flatMap((path) => {
      const text = policyReadFileSafe(repoRoot, path);
      return anchors.some((anchor) => interactivityToolRunWithin(path, anchor)) || INTERACTIVITY_TOOL_RUN_TRIGGER.test(text) ? [{ path, text }] : [];
    });
}








//#endregion ⏯️ToolRunPolicy

//#region 🚫️HistoryClosure
/**
 * 🚫️ One closure rule of the non-destructive history contract (ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`,
 * `📋️design.md` §13.2, §17.5, §17.7, §20): a token that must not survive in the scanned trees. `anchors` are literal substrings a file must
 * contain before the rule's `pattern` runs, `applies` narrows the files the rule reads, `allow` names the files whose hit is a deliberate negative case, `functions` the enclosing Rust functions
 * whose hit is a declared whole-document intent.
 */
export type HistoryClosureRule = { readonly id: string; readonly anchors: readonly string[]; readonly pattern: RegExp; readonly applies: (path: string) => boolean; readonly allow: readonly string[]; readonly functions?: readonly string[]; readonly within?: RegExp; readonly unless?: RegExp; readonly item?: { readonly header: RegExp; readonly unless: RegExp }; readonly reason: string };

/** 🚫️ One closure violation anchored to its rule, a repo-relative file and a 1-based line. */
export type HistoryClosureFinding = { readonly rule: string; readonly file: string; readonly line: number; readonly text: string };

const HISTORY_CLOSURE_ROOTS = ["✏️s", "🧰️framework", "🌎️hub"] as const;
/** 🪧️ The module that defines the rules: it names the vocabulary it bans, so it is not one of the closure sources. */
const HISTORY_CLOSURE_RULE_SOURCE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history/🟦️.ts";
const HISTORY_CLOSURE_SOURCE = /\.(?:rs|ts|tsx|js|py|feature|json)$/;
const HISTORY_CLOSURE_TESTS = /\/🧪️tests\/|\/🧪️testing\//;
const HISTORY_CLOSURE_STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/";
const HISTORY_CLOSURE_PLUGIN_TREES = ["✏️s/", "🌎️hub/🧩️compositions/"] as const;

/** 🧩️ True for production (non-test) code of a plugin tree. */
function historyClosurePluginProduction(path: string): boolean {
  return HISTORY_CLOSURE_PLUGIN_TREES.some((tree) => path.startsWith(tree)) && !HISTORY_CLOSURE_TESTS.test(path) && path.endsWith(".rs");
}

/**
 * 🚫️ The closure rules. Every lane commits through a tool transaction (artifact) or one plain config edit at gesture end
 * (§20.1): no amend constructor, no `AmendLast*` command, no coalesce key on any emission, edit, wire frame or fixture, no
 * gesture bracket verb, no host-snapshot bracket. Plugins never hand-build a `protocol::Edit` (the Store authority's
 * `next_edit` mints it) and never hand-declare a fold footprint (derived from the leaf's `x-semio-inverse-rows`, §20.5).
 */
export const HISTORY_CLOSURE_RULES: readonly HistoryClosureRule[] = [
  { id: "amend-emit", anchors: ["Emit::amend", "amend_config"], pattern: /\b(?:Action)?Emit::amend\b|\bamend_config\b/, applies: () => true, allow: [], reason: "a gesture is a tool transaction (artifact lane) or one config edit at gesture end (config lane); no emission amends the previous edit" },
  { id: "amend-last", anchors: ["AmendLast", "amendLast"], pattern: /\bAmendLast\w*|\bamendLast\w*/, applies: () => true, allow: [], reason: "the Store has no amend command on any lane; a transaction-scoped open edit (§15) is the only growing edit" },
  { id: "coalesce-key", anchors: ["coalesce_key", "coalesceKey"], pattern: /\bcoalesce_key\b|\bcoalesceKey\b|\bset_coalesce_key\b/, applies: () => true, allow: ["🌎️hub/🧩️compositions/🪐️space/🧪️testing/🧪️tests/🧫️fixtures/🔮️ownership/🔣️.json"], reason: "no emission, edit, digest, wire frame, persisted record or fixture carries a coalesce key; history rows group by TransactionRef" },
  { id: "preview-contract", anchors: ["UtilityPreviewContract"], pattern: /\bUtilityPreviewContract\b/, applies: () => true, allow: [], reason: "the actions-vs-tools contract is `🔖️ToolContract`; there is no preview pattern beside tool machines" },
  { id: "bracket-verb", anchors: ["transformBegin", "transformEnd", "paintStrokeBegin", "paintStrokeEnd"], pattern: /\b(?:transform|paintStroke)(?:Begin|End)\b/, applies: () => true, allow: ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧩️app/🖐️gumball-audience/🧪️tests/🧫️fixtures/🖐️gumball-verb-audience.json", "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"], reason: "a gesture is one tool-machine press with stream/commit/abort phases; no bracket verb exists" },
  { id: "host-snapshot-bracket", anchors: ["setHostSnapshot", "SetHostSnapshot", "set_host_snapshot"], pattern: /\bsetHostSnapshot\b|\bSetHostSnapshot\b|\bset_host_snapshot\b/, applies: (path) => !HISTORY_CLOSURE_TESTS.test(path), allow: ["🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json"], reason: "node-graph hosts journal id-keyed rows (move/connect/delete); a whole host snapshot is never a history mutation (refusal laws and the shared refusal fixture name it)" },
  { id: "edit-literal", anchors: ["Edit"], pattern: /\bprotocol::Edit\s*\{|(?<![\w:])Edit\s*\{\s*line\s*:/, applies: historyClosurePluginProduction, allow: [], reason: "plugins never hand-build an edit; `ArtifactStoreOneItemLiveAuthority::next_edit` mints it from the Store authority" },
  { id: "release-plain-commit", anchors: ["artifact_mutations", "Emit::mutations(", "Emit::commit("], pattern: /\bartifact_mutations\s*:|\bEmit::(?:mutations|commit)\(/, applies: historyClosurePluginProduction, allow: [], within: /pointer_up|_release$|^release_/, unless: /\b(?:commit|stream|abort)_transaction\b|\bnode_drag\w*|\bToolStep::/, reason: "a pointer/drag sequence commits through its tool machine (`commit_transaction`); a release handler never publishes an absolute final-state leaf (L6)" },
  { id: "footprint-hand", anchors: ["for_one_", "ArtifactStoreOneItemFootprint"], pattern: /\bfor_one_invertible_item\b|\bfor_one_item\b|\bArtifactStoreOneItemFootprint\s*\{\s*work_items\b/, applies: (path) => path.endsWith(".rs") && !path.startsWith(HISTORY_CLOSURE_STORE) && !HISTORY_CLOSURE_TESTS.test(path), allow: [], reason: "a fold footprint is derived from the leaf's `x-semio-inverse-rows` through `ArtifactStoreOneItemFootprint::for_leaf`; a hand count under-declares multi-row inverses" },
  { id: "footprint-default", anchors: ["INPUT_SCHEMAS"], pattern: /\bconst\s+INPUT_SCHEMAS\b/, applies: (path) => path.endsWith(".rs") && !HISTORY_CLOSURE_TESTS.test(path), allow: [], item: { header: /^\s*impl\b.*(?<!Semantic)\bMutation</, unless: /\bfn\s+inverse_rows\b/ }, reason: "a hand-written aggregate that forwards leaf payload schemas forwards their `x-semio-inverse-rows` too (`fn inverse_rows`); the default single row under-declares every multi-row leaf (§20.5, CLOSURE-4)" },
];

/** 🪧️ The header line and brace-balanced body of the Rust `impl`/`trait` item enclosing `index` in `lines`, or `undefined`. */
function historyClosureEnclosingItem(lines: readonly string[], index: number): { readonly header: string; readonly body: string } | undefined {
  for (let cursor = index; cursor >= 0; cursor--) if (/^\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:impl|trait)\b/.test(lines[cursor]!)) return { header: lines[cursor]!, body: historyClosureFunctionBody(lines, cursor) };
  return undefined;
}

/** 🧱️ The brace-balanced body text of the Rust function whose `fn` line is `start` in `lines`. */
function historyClosureFunctionBody(lines: readonly string[], start: number): string {
  let depth = 0;
  let opened = false;
  const body: string[] = [];
  for (let cursor = start; cursor < lines.length; cursor++) {
    body.push(lines[cursor]!);
    for (const character of lines[cursor]!) {
      if (character === "{") [depth, opened] = [depth + 1, true];
      else if (character === "}") depth--;
    }
    if (opened && depth <= 0) break;
  }
  return body.join("\n");
}

/** 🔎️ The line index of the Rust `fn` enclosing `index` in `lines` (nearest preceding `fn name`), or `-1`. */
function historyClosureEnclosingFunctionLine(lines: readonly string[], index: number): number {
  for (let cursor = index; cursor >= 0; cursor--) if (/\bfn\s+\w+/.test(lines[cursor]!)) return cursor;
  return -1;
}

/** 🔎️ The Rust function enclosing `index` in `lines` (nearest preceding `fn name`), or `undefined`. */
function historyClosureEnclosingFunction(lines: readonly string[], index: number): string | undefined {
  for (let cursor = index; cursor >= 0; cursor--) {
    const name = lines[cursor]!.match(/\bfn\s+(\w+)/)?.[1];
    if (name) return name;
  }
  return undefined;
}

/** 🚫️ Every closure violation in `sources` under `rules` (default [[HISTORY_CLOSURE_RULES]]); pure, so planted-violation tests drive it directly. */
export function historyClosureFindings(sources: readonly { readonly path: string; readonly text: string }[], rules: readonly HistoryClosureRule[] = HISTORY_CLOSURE_RULES): HistoryClosureFinding[] {
  const findings: HistoryClosureFinding[] = [];
  for (const { path, text } of sources) {
    const active = rules.filter((rule) => rule.anchors.some((anchor) => text.includes(anchor)) && rule.applies(path) && !rule.allow.includes(path));
    if (active.length === 0) continue;
    const lines = text.split("\n");
    lines.forEach((line, index) => {
      for (const rule of active) {
        if (!rule.anchors.some((anchor) => line.includes(anchor))) continue;
        const hit = line.match(rule.pattern)?.[0];
        if (hit === undefined) continue;
        if (rule.functions && path.endsWith(".rs") && rule.functions.includes(historyClosureEnclosingFunction(lines, index) ?? "")) continue;
        if (rule.within) {
          const start = historyClosureEnclosingFunctionLine(lines, index);
          const name = start < 0 ? "" : (lines[start]!.match(/\bfn\s+(\w+)/)?.[1] ?? "");
          if (!rule.within.test(name) || (rule.unless && rule.unless.test(historyClosureFunctionBody(lines, start)))) continue;
        }
        if (rule.item) {
          const item = historyClosureEnclosingItem(lines, index);
          if (item === undefined || !rule.item.header.test(item.header) || rule.item.unless.test(item.body)) continue;
        }
        findings.push({ rule: rule.id, file: path, line: index + 1, text: `[🚫️ ${rule.id}] ${hit}: ${rule.reason}` });
      }
    });
  }
  return findings;
}

/** 🗂️ The git-visible (tracked + unignored) source files of the closure roots, read once. */
export function historyClosureSources(repoRoot: string): { readonly path: string; readonly text: string }[] {
  const listed = Bun.spawnSync(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", ...HISTORY_CLOSURE_ROOTS], { cwd: repoRoot });
  if (listed.exitCode !== 0) throw new Error(`[verify history-closure] git-visible inventory failed: ${listed.stderr.toString()}`);
  const paths = [...new Set(listed.stdout.toString().split("\0").map((path) => path.normalize("NFC")))].filter((path) => HISTORY_CLOSURE_SOURCE.test(path) && path !== HISTORY_CLOSURE_RULE_SOURCE && !path.split("/").some((segment) => POLICY_SKIP_DIRS.has(segment)));
  return paths.flatMap((path) => {
    try {
      return [{ path, text: readFileSync(join(repoRoot, path), "utf8") }];
    } catch {
      return [];
    }
  });
}

/** 🚫️ The closure gate as breach records: one high-priority breach per finding. */
export function policyHistoryClosureBreaches(repoRoot: string): BreachRecord[] {
  return historyClosureFindings(historyClosureSources(repoRoot)).map((finding) => ({
    id: `history-closure-${finding.rule}-${finding.file}:${finding.line}`,
    summary: `${finding.file}:${finding.line} ${finding.text}`,
    kind: `history-closure/${finding.rule}`,
    scope: finding.file,
    priority: "high",
    reason: HISTORY_CLOSURE_RULES.find((rule) => rule.id === finding.rule)?.reason ?? finding.rule,
    solution: "Delete the legacy path: commit gestures through a ToolTransaction or one config edit at gesture end, mint edits with `next_edit`, derive footprints with `for_leaf` (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING §20).",
  }));
}
//#endregion 🚫️HistoryClosure
