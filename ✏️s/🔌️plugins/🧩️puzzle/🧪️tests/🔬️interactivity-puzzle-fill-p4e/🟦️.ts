import { policyReadFileSafe, INTERACTIVITY_AUDIT_PUZZLE_FILL_ENVELOPE_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_STATE_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_GEOMETRY_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_SCHEMA_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_TRANSPORT_FILE, INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_FILE, interactivityPuzzleFillP4eFailures } from "../../../../../📜️script.ts";

/** 🧪️ Executes interactivity puzzle fill p4e policy assertions. */
export function interactivityPuzzleFillP4eSelfTests(repoRoot: string): void {
  const precompute = policyReadFileSafe(repoRoot, INTERACTIVITY_AUDIT_PUZZLE_FILL_ENVELOPE_FILE);
  const fill = policyReadFileSafe(repoRoot, INTERACTIVITY_AUDIT_PUZZLE_FILL_STATE_FILE);
  const geometry = policyReadFileSafe(repoRoot, INTERACTIVITY_AUDIT_PUZZLE_FILL_GEOMETRY_FILE);
  const schema = policyReadFileSafe(repoRoot, INTERACTIVITY_AUDIT_PUZZLE_FILL_SCHEMA_FILE);
  const transport = policyReadFileSafe(repoRoot, INTERACTIVITY_AUDIT_PUZZLE_FILL_TRANSPORT_FILE);
  const renderer = policyReadFileSafe(repoRoot, INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_FILE);
  const mutations: [string, string, string, string, string, string, string][] = [
    ["whole-builder", precompute.replace("let fill = FillBuilder::begin_preparation(FillPreparationRoots::new(scene, self.meshes.clone()), operation);", "let fill = FillBuilder::new(scene.fixture.clone(), scene.seed, &self.meshes, &KindCatalogBundle::default());"), fill, geometry, schema, transport, renderer],
    ["direct-configure", precompute.replace("self.fill = Some(Arc::new(Mutex::new(fill)));", "fill.configure(operation); self.fill = Some(Arc::new(Mutex::new(fill)));"), fill, geometry, schema, transport, renderer],
    ["direct-rebuild", precompute.replace("fn soft_replan_fill_tail(&mut self) {\n        self.start_fill_preparation(false);", "fn soft_replan_fill_tail(&mut self) { self.fill.as_ref().unwrap().lock().unwrap().rebuild_collision_index();"), fill, geometry, schema, transport, renderer],
    ["whole-checkpoint", precompute, fill.replace("pub(crate) struct FillBuilder {", "struct FillJobCheckpoint;\npub(crate) struct FillBuilder {"), geometry, schema, transport, renderer],
    ["clone-helper", precompute, fill, geometry.replace("pub(crate) struct FixedOwnerSet", "fn cloned_btree() {}\npub(crate) struct FixedOwnerSet"), schema, transport, renderer],
    ["dynamic-bucket", precompute, fill, geometry.replace("cells: FixedOwnerMap<(i32, i32, i32), FixedOwnerSet<String>>", "cells: FixedOwnerMap<(i32, i32, i32), Vec<String>>"), schema, transport, renderer],
    ["materialized-coverage", precompute, fill, geometry.replace("impl CollisionCellSpan {", "fn covered_cells() -> Vec<(i32, i32, i32)> { Vec::new() }\nimpl CollisionCellSpan {"), schema, transport, renderer],
    ["decorative-query", precompute, fill.replace("self.spatial_index.step_query(query, owner)", "{ let _all = self.placed.iter(); CollisionQueryStep::Complete }"), geometry, schema, transport, renderer],
    ["direct-spatial-mutation", precompute, fill.replace("self.spatial_index.step_replacement(mutation, owner)", "self.spatial_index.upsert(String::new(), CollisionAabb { min: [0.0; 3], max: [0.0; 3] })"), geometry, schema, transport, renderer],
    ["producer-clone", precompute, fill.replace("appended_attractions: Vec::new(),\n            sequence: Vec::new(),\n            preview", "appended_attractions: Vec::new(),\n            sequence: self.sequence.clone(),\n            preview"), geometry, schema, transport, renderer],
    ["unbounded-diagnostic", precompute, fill, geometry, schema.replace("pub candidate_page: [Option<String>; 8]", "pub candidate_page: Vec<String>"), transport, renderer],
    ["ghost-gated", precompute, fill, geometry, schema, transport.replace("session.fill_preview_json_page(&color, labels.fill_progress.as_str())", "session.fill_preview_object_kind().and(None)"), renderer],
    ["ignored-overlay", precompute, fill, geometry, schema, transport, renderer.replace("{fillDiagnostic ? <FillDiagnosticOverlay diagnostic={fillDiagnostic} /> : null}", "null")],
    ["stale-render", precompute, fill, geometry, schema, transport, renderer.replace("identity[4] >= latest[4]", "true")],
    ["ignored-truncation", precompute, fill, geometry, schema, transport, renderer.replace("data-fill-truncated={diagnostic.truncated}", "data-fill-truncated={false}")],
    ["missing-fixture-object-preflight", precompute, fill.replace("(PreparationCapacityBranch::FixtureObjects, roots.scene.fixture.objects.len())", "(PreparationCapacityBranch::FixtureObjects, 0)"), geometry, schema, transport, renderer],
    ["missing-fixture-attraction-preflight", precompute, fill.replace("(PreparationCapacityBranch::FixtureAttractions, roots.scene.fixture.attractions.len())", "(PreparationCapacityBranch::FixtureAttractions, 0)"), geometry, schema, transport, renderer],
    ["missing-fixture-volume-preflight", precompute, fill.replace("(PreparationCapacityBranch::FixtureTargetVolumes, roots.scene.fixture.target_volumes.len())", "(PreparationCapacityBranch::FixtureTargetVolumes, 0)"), geometry, schema, transport, renderer],
    ["missing-mesh-preflight", precompute, fill.replace("(PreparationCapacityBranch::Meshes, roots.meshes.len())", "(PreparationCapacityBranch::Meshes, 0)"), geometry, schema, transport, renderer],
    ["missing-catalog-object-preflight", precompute, fill.replace("(PreparationCapacityBranch::CatalogObjects, catalogs.map_or(0, |value| value.objects.len()))", "(PreparationCapacityBranch::CatalogObjects, 0)"), geometry, schema, transport, renderer],
    ["missing-catalog-vortex-preflight", precompute, fill.replace("(PreparationCapacityBranch::CatalogVortices, catalogs.map_or(0, |value| value.vortices.len()))", "(PreparationCapacityBranch::CatalogVortices, 0)"), geometry, schema, transport, renderer],
    ["missing-catalog-cable-preflight", precompute, fill.replace("(PreparationCapacityBranch::CatalogCables, catalogs.map_or(0, |value| value.cables.len()))", "(PreparationCapacityBranch::CatalogCables, 0)"), geometry, schema, transport, renderer],
    ["missing-compatibility-preflight", precompute, fill.replace("(PreparationCapacityBranch::KindCompatibility, roots.scene.kind_compatibility.len())", "(PreparationCapacityBranch::KindCompatibility, 0)"), geometry, schema, transport, renderer],
    ["missing-object-weight-preflight", precompute, fill.replace("(PreparationCapacityBranch::ObjectWeights, roots.scene.weights.object_weights.len())", "(PreparationCapacityBranch::ObjectWeights, 0)"), geometry, schema, transport, renderer],
    ["missing-vortex-weight-preflight", precompute, fill.replace("(PreparationCapacityBranch::VortexWeights, roots.scene.weights.vortex_weights.len())", "(PreparationCapacityBranch::VortexWeights, 0)"), geometry, schema, transport, renderer],
    ["dynamic-fixture-object-owner", precompute, fill.replace("pub(crate) objects: FixedOwnerVec<FixtureObject>", "pub(crate) objects: Vec<FixtureObject>"), geometry, schema, transport, renderer],
    ["dynamic-fixture-attraction-owner", precompute, fill.replace("pub(crate) attractions: FixedOwnerVec<AttractionProps>", "pub(crate) attractions: Vec<AttractionProps>"), geometry, schema, transport, renderer],
    ["dynamic-fixture-volume-owner", precompute, fill.replace("pub(crate) target_volumes: FixedOwnerVec<WorldVolumeProps>", "pub(crate) target_volumes: Vec<WorldVolumeProps>"), geometry, schema, transport, renderer],
    ["dynamic-catalog-object-owner", precompute, fill.replace("objects: FixedOwnerVec<ObjectKind>", "objects: Vec<ObjectKind>"), geometry, schema, transport, renderer],
    ["dynamic-catalog-vortex-owner", precompute, fill.replace("vortices: FixedOwnerVec<VortexKindCatalog>", "vortices: Vec<VortexKindCatalog>"), geometry, schema, transport, renderer],
    ["dynamic-catalog-cable-owner", precompute, fill.replace("cables: FixedOwnerVec<CableKindCatalog>", "cables: Vec<CableKindCatalog>"), geometry, schema, transport, renderer],
    ["dynamic-compatibility-owner", precompute, fill.replace("kind_compatibility: FixedOwnerVec<KindCompatEntry>", "kind_compatibility: Vec<KindCompatEntry>"), geometry, schema, transport, renderer],
    ["dynamic-mesh-owner", precompute, fill.replace("meshes: FixedOwnerMap<String, CollisionBody>", "meshes: HashMap<String, CollisionBody>"), geometry, schema, transport, renderer],
    ["missing-catalog-cap-acceptance", precompute, fill.replace("(HostileRoot::CatalogObjects, \"catalog-objects\")", "(HostileRoot::FixtureObjects, \"catalog-objects\")"), geometry, schema, transport, renderer],
    ["fault-before-rejection-diagnostic", precompute, fill.replace("return self.publish_preview(context);", "return StepOutcome::Fault(JobFault { detail: b\"fill-preparation-capacity\".to_vec() });"), geometry, schema, transport, renderer],
    ["whole-fill-preview", precompute, fill.replace("pub(crate) struct FillPreviewJsonCursor", "fn whole_fill_preview() { let _ = serde_json::to_vec(&self.preview); }\npub(crate) struct FillPreviewJsonCursor"), geometry, schema, transport, renderer],
    ["missing-constructor-cap-fixture", precompute, fill.replace("constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently", "constructor_cap_smoke"), geometry, schema, transport, renderer],
    ["missing-stale-preparation-fixture", precompute, fill.replace("stale_generation_stops_preparation_before_installing_any_entry", "stale_preparation_smoke"), geometry, schema, transport, renderer],
    ["missing-sparse-query-fixture", precompute, fill, geometry.replace("spatial_resumable_query_narrows_sparse_cells_without_visiting_distant_population", "spatial_query_smoke"), schema, transport, renderer],
    ["missing-capacity-fixture", precompute, fill, geometry.replace("spatial_capacity_plus_one_refusal_preserves_exact_old_state", "spatial_capacity_smoke"), schema, transport, renderer],
    ["missing-stale-spatial-fixture", precompute, fill, geometry.replace("spatial_stale_owner_cannot_finish_partial_replacement", "spatial_stale_smoke"), schema, transport, renderer],
    ["missing-multicell-fixture", precompute, fill, geometry.replace("spatial_multi_cell_oversized_replacement_and_removal_make_bounded_progress", "spatial_multicell_smoke"), schema, transport, renderer],
  ];
  for (const [name, mutatedPrecompute, mutatedFill, mutatedGeometry, mutatedSchema, mutatedTransport, mutatedRenderer] of mutations) {
    if (interactivityPuzzleFillP4eFailures(mutatedPrecompute, mutatedFill, mutatedGeometry, mutatedSchema, mutatedTransport, mutatedRenderer).length === 0) throw new Error(`[verify interactivity] Puzzle fill P4e self-test ${name} was falsely accepted.`);
  }
  const failures = interactivityPuzzleFillP4eFailures(precompute, fill, geometry, schema, transport, renderer);
  if (failures.length !== 0) throw new Error(`[verify interactivity] Puzzle fill P4e baseline was falsely rejected: ${failures.join("; ")}`);
}
