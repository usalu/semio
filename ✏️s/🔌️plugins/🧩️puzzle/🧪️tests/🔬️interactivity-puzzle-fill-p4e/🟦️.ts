import { interactivityPuzzleFillP4eFailures } from "../../../../../📜️script.ts";

/** 🧫️ A FillBuilder miniature that satisfies every P4e law surviving the tool-run conversion: cooperative preparation, fixed owners with cap/+1 evidence, resumable spatial index, danger step before a capacity fault. */
const CLEAN = {
  precompute: `//! ⏳️ Placement session.
pub struct Puzzle3dPrecomputeSession {
    meshes: Arc<HashMap<String, CollisionBody>>,
}

impl Puzzle3dPrecomputeSession {
    pub fn fill_run_job(&self, scene: Arc<SceneConfig>, operation: FillOperation) -> FillRunJob {
        FillRunJob::new(FillBuilder::begin_preparation(FillPreparationRoots::new(scene, self.meshes.clone()), operation))
    }
}
`,
  fill: `//! 🪣️ FillBuilder.
pub(crate) struct FillPreparationRoots {
    scene: Arc<SceneConfig>,
    meshes: Arc<HashMap<String, CollisionBody>>,
}

pub(crate) struct FillPreparedBase {
    pub(crate) objects: FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS>,
    pub(crate) attractions: FixedOwnerVec<AttractionProps, DOCUMENT_ATTRACTION_SLOTS>,
    pub(crate) target_volumes: FixedOwnerVec<WorldVolumeProps, DOCUMENT_VOLUME_SLOTS>,
}

pub(crate) struct FillPreparedCatalogs {
    objects: FixedOwnerVec<ObjectKind, DOCUMENT_KIND_SLOTS>,
    vortices: FixedOwnerVec<VortexKindCatalog, DOCUMENT_KIND_SLOTS>,
    cables: FixedOwnerVec<CableKindCatalog, DOCUMENT_KIND_SLOTS>,
}

pub(crate) struct FillBuilder {
    kind_compatibility: FixedOwnerVec<KindCompatEntry, DOCUMENT_KIND_SLOTS>,
    meshes: FixedOwnerMap<String, CollisionBody, DOCUMENT_KIND_SLOTS>,
    object_weights: FixedOwnerMap<String, f64, DOCUMENT_KIND_SLOTS>,
    vortex_weights: FixedOwnerMap<String, f64, DOCUMENT_KIND_SLOTS>,
}

impl FillBuilder {
    fn prepare(&mut self) -> StepOutcome {
        match self.stage {
            FillJobStage::PrepareFixture | FillJobStage::PrepareCatalogs | FillJobStage::PrepareMeshes | FillJobStage::PrepareEntries | FillJobStage::PrepareSpatial | FillJobStage::PrepareLookup | FillJobStage::PrepareConfiguration => {
                self.prepare_one();
                StepOutcome::Yield
            }
        }
    }

    fn preparation_capacity_refusal(roots: &FillPreparationRoots) -> Option<(PreparationCapacityBranch, usize)> {
        let catalogs = roots.scene.catalogs.as_ref();
        [
            (PreparationCapacityBranch::FixtureObjects, roots.scene.fixture.objects.len(), DOCUMENT_OBJECT_SLOTS),
            (PreparationCapacityBranch::FixtureAttractions, roots.scene.fixture.attractions.len(), DOCUMENT_ATTRACTION_SLOTS),
            (PreparationCapacityBranch::FixtureTargetVolumes, roots.scene.fixture.target_volumes.len(), DOCUMENT_VOLUME_SLOTS),
            (PreparationCapacityBranch::Meshes, roots.meshes.len(), DOCUMENT_KIND_SLOTS),
            (PreparationCapacityBranch::CatalogObjects, catalogs.map_or(0, |value| value.objects.len()), DOCUMENT_KIND_SLOTS),
            (PreparationCapacityBranch::CatalogVortices, catalogs.map_or(0, |value| value.vortices.len()), DOCUMENT_KIND_SLOTS),
            (PreparationCapacityBranch::CatalogCables, catalogs.map_or(0, |value| value.cables.len()), DOCUMENT_KIND_SLOTS),
            (PreparationCapacityBranch::KindCompatibility, roots.scene.kind_compatibility.len(), DOCUMENT_KIND_SLOTS),
            (PreparationCapacityBranch::ObjectWeights, roots.scene.weights.object_weights.len(), DOCUMENT_KIND_SLOTS),
            (PreparationCapacityBranch::VortexWeights, roots.scene.weights.vortex_weights.len(), DOCUMENT_KIND_SLOTS),
        ]
        .into_iter()
        .find_map(|(branch, len, capacity)| (len > capacity).then_some((branch, capacity)))
    }

    fn refusal_label(&self) -> String {
        format!("preparation-capacity:{}:{}", self.branch.label(), self.omitted_index)
    }

    fn step_prepared(&mut self, query: &mut CollisionQueryCursor, mutation: &mut CollisionIndexMutation, owner: u64) -> StepOutcome {
        if let Some(refusal) = self.preparation_capacity_refusal.as_mut() {
            if !refusal.published {
                refusal.published = true;
                let _ = self.writer.step(ToolRunStepKind::Danger, 0, refusal.reason(), None, &[]);
                return StepOutcome::PreviewReady(self.page());
            }
            return StepOutcome::Fault(JobFault { detail: b"fill-preparation-capacity".to_vec() });
        }
        if self.collection_over_capacity {
            return StepOutcome::Fault(JobFault { detail: b"fill-collection-capacity".to_vec() });
        }
        let _ = self.spatial_index.step_query(query, owner);
        let _ = self.spatial_index.step_replacement(mutation, owner);
        StepOutcome::Yield
    }
}

#[cfg(test)]
mod tests {
    fn preparation_refusal_owner_for_test() {}

    #[test]
    fn every_preparation_branch_refuses_cap_plus_one() {
        for (branch, _label, cap) in [
            (HostileRoot::FixtureObjects, "fixture-objects", DOCUMENT_OBJECT_SLOTS),
            (HostileRoot::FixtureAttractions, "fixture-attractions", DOCUMENT_ATTRACTION_SLOTS),
            (HostileRoot::FixtureTargetVolumes, "fixture-target-volumes", DOCUMENT_VOLUME_SLOTS),
            (HostileRoot::Meshes, "meshes", DOCUMENT_KIND_SLOTS),
            (HostileRoot::CatalogObjects, "catalog-objects", DOCUMENT_KIND_SLOTS),
            (HostileRoot::CatalogVortices, "catalog-vortices", DOCUMENT_KIND_SLOTS),
            (HostileRoot::CatalogCables, "catalog-cables", DOCUMENT_KIND_SLOTS),
            (HostileRoot::KindCompatibility, "kind-compatibility", DOCUMENT_KIND_SLOTS),
            (HostileRoot::ObjectWeights, "object-weights", DOCUMENT_KIND_SLOTS),
            (HostileRoot::VortexWeights, "vortex-weights", DOCUMENT_KIND_SLOTS),
        ] {
            let _ = roots(branch, cap);
            let _ = roots(branch, cap + 1);
        }
    }

    #[test]
    fn nakagin_scale_fill_is_not_refused_and_places_at_least_one_object() {}

    #[test]
    fn document_capacities_match_the_language_neutral_capacity_law() {}

    #[test]
    fn constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently() {}

    #[test]
    fn stale_generation_stops_preparation_before_installing_any_entry() {}
}
`,
  geometry: `//! 📐️ Fixed owners and the spatial index.
pub(crate) struct FixedOwnerVec<T, const N: usize> {
    page: Option<Box<[Option<T>; N]>>,
    len: usize,
}

pub(crate) struct CollisionIndex {
    cells: FixedOwnerMap<(i32, i32, i32), FixedOwnerSet<String>, DOCUMENT_CELL_SLOTS>,
}

struct CollisionCellSpan {
    min: [i32; 3],
    max: [i32; 3],
}

pub(crate) struct CollisionIndexMutation {
    owner: u64,
    stage: CollisionMutationStage,
}

pub(crate) struct CollisionIndexRemoval {
    owner: u64,
}

pub(crate) struct CollisionQueryCursor {
    owner: u64,
    overlap: CollisionOverlapStage,
}

impl CollisionIndex {
    pub(crate) fn step_replacement(&mut self, mutation: &mut CollisionIndexMutation, current: u64) -> CollisionMutationStep {
        if mutation.owner != current {
            return CollisionMutationStep::Stale;
        }
        match mutation.stage {
            CollisionMutationStage::PreflightNew => CollisionMutationStep::Pending,
            _ => CollisionMutationStep::Complete,
        }
    }

    pub(crate) fn step_query(&self, query: &mut CollisionQueryCursor, current: u64) -> CollisionQueryStep {
        if query.owner != current {
            return CollisionQueryStep::Stale;
        }
        match query.overlap {
            CollisionOverlapStage::SamplingPointA | CollisionOverlapStage::SamplingPointB => CollisionQueryStep::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn spatial_resumable_query_narrows_sparse_cells_without_visiting_distant_population() {}

    #[test]
    fn spatial_capacity_plus_one_refusal_preserves_exact_old_state() {}

    #[test]
    fn spatial_stale_owner_cannot_finish_partial_replacement() {}

    #[test]
    fn spatial_multi_cell_oversized_replacement_and_removal_make_bounded_progress() {}
}
`,
};

type Role = keyof typeof CLEAN;

const CASES: readonly (readonly [name: string, expect: "report" | "silent", role: Role, from: string, to: string])[] = [
  ["whole-builder", "report", "precompute", "FillBuilder::begin_preparation(FillPreparationRoots::new(scene, self.meshes.clone()), operation)", "FillBuilder::new(scene.fixture.clone(), &self.meshes)"],
  ["direct-configure", "report", "precompute", "        FillRunJob::new(", "        let _ = self.fill.configure(operation);\n        FillRunJob::new("],
  ["missing-stage", "report", "fill", "FillJobStage::PrepareLookup | ", ""],
  ["missing-cooperative-unit", "report", "fill", "self.prepare_one();", "self.prepare_all();"],
  ["missing-fixture-object-preflight", "report", "fill", "roots.scene.fixture.objects.len()", "0"],
  ["missing-vortex-weight-preflight", "report", "fill", "roots.scene.weights.vortex_weights.len()", "0"],
  ["dynamic-fixture-object-owner", "report", "fill", "pub(crate) objects: FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS>", "pub(crate) objects: Vec<FixtureObject>"],
  ["dynamic-mesh-owner", "report", "fill", "meshes: FixedOwnerMap<String, CollisionBody, DOCUMENT_KIND_SLOTS>", "meshes: HashMap<String, CollisionBody>"],
  ["direct-push", "report", "fill", "        self.prepare_one();", "        self.base.objects.push(object);\n        self.prepare_one();"],
  ["missing-hostile-branch", "report", "fill", '(HostileRoot::CatalogObjects, "catalog-objects", DOCUMENT_KIND_SLOTS)', '(HostileRoot::FixtureObjects, "catalog-objects", DOCUMENT_KIND_SLOTS)'],
  ["missing-plus-one-law", "report", "fill", "            let _ = roots(branch, cap + 1);\n", ""],
  ["missing-exact-refusal", "report", "fill", "(len > capacity).then_some((branch, capacity))", "(len >= capacity).then_some((branch, capacity))"],
  ["silent-refusal-fault", "report", "fill", "ToolRunStepKind::Danger", "ToolRunStepKind::Info"],
  ["fault-before-danger-step", "report", "fill", "                let _ = self.writer.step(ToolRunStepKind::Danger, 0, refusal.reason(), None, &[]);\n                return StepOutcome::PreviewReady(self.page());\n            }\n            return StepOutcome::Fault(JobFault { detail: b\"fill-preparation-capacity\".to_vec() });", "                return StepOutcome::Fault(JobFault { detail: b\"fill-preparation-capacity\".to_vec() });\n            }\n            let _ = self.writer.step(ToolRunStepKind::Danger, 0, refusal.reason(), None, &[]);\n            return StepOutcome::PreviewReady(self.page());"],
  ["ghost-in-refusal", "report", "fill", "                refusal.published = true;", "                refusal.published = true;\n                self.candidate_ghost = None;"],
  ["dynamic-bucket", "report", "geometry", "cells: FixedOwnerMap<(i32, i32, i32), FixedOwnerSet<String>, DOCUMENT_CELL_SLOTS>", "cells: FixedOwnerMap<(i32, i32, i32), Vec<String>>"],
  ["materialized-coverage", "report", "geometry", "struct CollisionCellSpan {", "fn covered_cells() -> Vec<(i32, i32, i32)> { Vec::new() }\n\nstruct CollisionCellSpan {"],
  ["stale-owner-accepted", "report", "geometry", "if mutation.owner != current {", "if false {"],
  ["direct-spatial-mutation", "report", "fill", "self.spatial_index.step_replacement(mutation, owner)", "self.spatial_index.upsert(String::new(), CollisionAabb::default())"],
  ["decorative-query", "report", "fill", "self.spatial_index.step_query(query, owner)", "self.placed.iter().count()"],
  ["whole-checkpoint", "report", "fill", "pub(crate) struct FillBuilder {", "struct FillJobCheckpoint;\n\npub(crate) struct FillBuilder {"],
  ["clone-helper", "report", "geometry", "pub(crate) struct CollisionIndex {", "fn cloned_btree() {}\n\npub(crate) struct CollisionIndex {"],
  ["sequence-clone", "report", "fill", "        StepOutcome::Yield\n    }\n}\n", "        let _ = self.sequence.clone();\n        StepOutcome::Yield\n    }\n}\n"],
  ["missing-constructor-fixture", "report", "fill", "constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently", "constructor_cap_smoke"],
  ["missing-sparse-query-fixture", "report", "geometry", "spatial_resumable_query_narrows_sparse_cells_without_visiting_distant_population", "spatial_query_smoke"],
  ["commented-constructor-is-silent", "silent", "precompute", "pub struct Puzzle3dPrecomputeSession {", "// FillBuilder::new( and .configure( were retired.\npub struct Puzzle3dPrecomputeSession {"],
];

/** 🧪️ Mutation self-test of the surviving P4e FillBuilder laws: the clean miniature passes, every planted regression is reported, comments stay silent. Returns the executed case count. */
export function interactivityPuzzleFillP4eSelfTests(): number {
  const clean = interactivityPuzzleFillP4eFailures(CLEAN.precompute, CLEAN.fill, CLEAN.geometry);
  if (clean.length > 0) throw new Error(`[verify interactivity] Puzzle fill P4e clean fixture was falsely rejected: ${clean.join("; ")}`);
  for (const [name, expect, role, from, to] of CASES) {
    if (CLEAN[role].split(from).length !== 2) throw new Error(`[verify interactivity] Puzzle fill P4e self-test ${name} anchor is missing or ambiguous in ${role}: ${from}`);
    const sources = { ...CLEAN, [role]: CLEAN[role].replace(from, to) };
    const failures = interactivityPuzzleFillP4eFailures(sources.precompute, sources.fill, sources.geometry);
    if (expect === "report" && failures.length === 0) throw new Error(`[verify interactivity] Puzzle fill P4e self-test ${name} was falsely accepted.`);
    if (expect === "silent" && failures.length > 0) throw new Error(`[verify interactivity] Puzzle fill P4e self-test ${name} was falsely reported: ${failures.join("; ")}`);
  }
  return CASES.length + 1;
}
