//! 🪣️ Puzzle 3d play app — the precompute fill planner's own state: the running `FillBuilder` (base
//! scene, the growing plan sequence and its appended objects/attractions, the placed collision
//! entries the next step tests against, the per-session RNG stream) plus its progress readout. The
//! stepping itself lives in the sibling `⏳️precompute/🦀️component.rs`, which owns the two precompute
//! lanes. Rehomed from the former `⚙️engine/🪣️fill` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this is interactive fill-tool session state,
//! so it lives with the app, not the artifact.

use crate::artifacts::puzzle3d::schema::{
    puzzle3d_vortex_full_id, AttractionProps, BrushCompatibleCandidate, BrushHostRules, BrushKindWeights, BrushPlacePayload, BrushPreviewState, FillBuildPreview, FillBuildProgress, Fixture, FixtureObject, KindCatalogBundle, KindCompatEntry,
    VortexProps,
};
use crate::editor::puzzle3d::precompute::brush::{
    brush_candidate_suggestion_weight, brush_fill_candidate_at, brush_object_id, brush_preview_from_candidate, brush_stack_mate_pair, fill_candidate_diversity_score, fill_rng, fill_vortex_target_weight, resolve_object_kind_mesh_url,
    vortex_world_from_object, AttractionVortexContext, BrushFillVortexTarget, TargetVortexWorld,
};
use crate::editor::puzzle3d::precompute::geometry::{pose_isometry, world_bounds, world_volumes_contain_aabb, CollisionAabb, CollisionBody, CollisionOverlapState, CollisionSpatialIndex, CollisionStepResult, Pose3d};
use crate::editor::puzzle3d::precompute::FILL_COUNT_MAX;
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, Operation, StepContext, StepOutcome};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// 🧱️ One already-placed object's collision footprint, kept alongside the plan so each new fill step
/// only has to test the candidate against bodies it can actually hit.
#[derive(Clone)]
pub(crate) struct PlacedCollisionEntry {
    pub(crate) object_id: String,
    pub(crate) mesh_url: String,
    pub(crate) world: Pose3d,
}

fn fenwick_add(tree: &mut [f64], index: usize, delta: f64) {
    let mut cursor = index + 1;
    while cursor < tree.len() {
        tree[cursor] += delta;
        cursor += cursor & cursor.wrapping_neg();
    }
}

fn fenwick_total(tree: &[f64]) -> f64 {
    let mut cursor = tree.len().saturating_sub(1);
    let mut total = 0.0;
    while cursor > 0 {
        total += tree[cursor];
        cursor &= cursor - 1;
    }
    total
}

fn fenwick_pick(tree: &[f64], target: f64) -> usize {
    let mut index = 0;
    let mut prefix = 0.0;
    let mut bit = 1usize;
    while bit < tree.len() {
        bit <<= 1;
    }
    let mut step = bit >> 1;
    while step > 0 {
        let next = index + step;
        if next < tree.len() && prefix + tree[next] < target {
            prefix += tree[next];
            index = next;
        }
        step >>= 1;
    }
    index.min(tree.len().saturating_sub(2))
}

fn weighted_pick(weights: &mut [f64], tree: &mut [f64], remaining: usize, rng_state: &mut u32) -> Option<usize> {
    if remaining == 0 {
        return None;
    }
    let total = fenwick_total(tree);
    if total <= 0.0 {
        return None;
    }
    let target = if weights.len() == 1 { f64::MIN_POSITIVE } else { (fill_rng(rng_state) * total).max(f64::MIN_POSITIVE) };
    let index = fenwick_pick(tree, target);
    let weight = std::mem::replace(&mut weights[index], 0.0);
    fenwick_add(tree, index, -weight);
    Some(index)
}

fn fixture_fingerprint(fixture: &Fixture) -> u64 {
    serde_json::to_vec(fixture).expect("fill base fixture is serializable").into_iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum FillJobStage {
    PrepareTargets,
    SelectTarget,
    PrepareCandidates,
    SelectCandidate,
    ConstructPreview,
    QueryBroadPhase,
    TestCollision,
    AcceptCandidate,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum TargetPreparePhase {
    Blocked,
    Enumerate,
    BuildSeedWeights,
    BuildFrontierWeights,
    OrderSeed,
    OrderFrontier,
    Finish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum CandidatePreparePhase {
    Enumerate,
    Classify,
    DrainCross,
    DrainSame,
    BuildSameWeights,
    OrderSame,
    Finish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum AcceptPhase {
    Validate,
    CheckAttractions,
    BuildVortices,
    Commit,
}

#[derive(Clone, Serialize, Deserialize)]
struct FillJobCheckpoint {
    operation_id: u64,
    base_revision: u64,
    generation: u64,
    operation_preview_sequence: u64,
    operation_seed: u64,
    base_fingerprint: u64,
    applied_count: usize,
    sequence: Vec<BrushPlacePayload>,
    appended_objects: Vec<FixtureObject>,
    appended_attractions: Vec<AttractionProps>,
    candidate_cache: BTreeMap<String, Vec<BrushCompatibleCandidate>>,
    rng_state: u32,
    stalled: bool,
    stage: FillJobStage,
    targets: Vec<BrushFillVortexTarget>,
    target_cursor: usize,
    target_rotation: usize,
    target_prepare_phase: TargetPreparePhase,
    blocked_vortex_ids: BTreeSet<String>,
    target_attraction_cursor: usize,
    target_object_cursor: usize,
    target_vortex_cursor: usize,
    seed_targets: Vec<BrushFillVortexTarget>,
    frontier_targets: Vec<BrushFillVortexTarget>,
    seed_target_weights: Vec<f64>,
    frontier_target_weights: Vec<f64>,
    seed_target_tree: Vec<f64>,
    frontier_target_tree: Vec<f64>,
    target_prepare_cursor: usize,
    seed_target_remaining: usize,
    frontier_target_remaining: usize,
    current_target: Option<BrushFillVortexTarget>,
    candidates: Vec<BrushCompatibleCandidate>,
    candidate_cursor: usize,
    candidate_prepare_phase: CandidatePreparePhase,
    candidate_kind_cursor: usize,
    candidate_vortex_cursor: usize,
    candidate_prepare_cursor: usize,
    candidate_seen: BTreeSet<String>,
    candidate_raw: Vec<BrushCompatibleCandidate>,
    candidate_cross: BTreeMap<String, BrushCompatibleCandidate>,
    candidate_same: BTreeMap<String, BrushCompatibleCandidate>,
    candidate_same_sorted: Vec<BrushCompatibleCandidate>,
    candidate_same_weights: Vec<f64>,
    candidate_same_tree: Vec<f64>,
    candidate_same_remaining: usize,
    current_preview: Option<BrushPreviewState>,
    broad_phase_ids: Vec<String>,
    broad_phase_cursor: usize,
    broad_phase_bounds: Option<CollisionAabb>,
    collision: Option<CollisionOverlapState>,
    accept_phase: AcceptPhase,
    accept_attraction_cursor: usize,
    accept_vortex_cursor: usize,
    pending_payload: Option<BrushPlacePayload>,
    pending_object: Option<FixtureObject>,
    pending_attraction: Option<AttractionProps>,
    last_rejection: Option<String>,
    preview: FillBuildPreview,
    transition_count: u64,
    rejected_count: u64,
}

pub(crate) struct FillBuilder {
    pub(crate) base: Fixture,
    base_fingerprint: u64,
    pub(crate) fixture: Fixture,
    pub(crate) applied_count: usize,
    pub(crate) sequence: Vec<BrushPlacePayload>,
    pub(crate) appended_objects: Vec<FixtureObject>,
    pub(crate) appended_attractions: Vec<AttractionProps>,
    pub(crate) placed: Vec<PlacedCollisionEntry>,
    placed_lookup: BTreeMap<String, usize>,
    pub(crate) candidate_cache: BTreeMap<String, Vec<BrushCompatibleCandidate>>,
    pub(crate) seed_object_ids: std::collections::HashSet<String>,
    pub(crate) rng_state: u32,
    pub(crate) stalled: bool,
    pub(crate) max_count: usize,
    pub(crate) operation: Operation,
    pub(crate) stage: FillJobStage,
    pub(crate) preview: FillBuildPreview,
    catalogs: KindCatalogBundle,
    weights: BrushKindWeights,
    kind_compatibility: Vec<KindCompatEntry>,
    host_rules: BrushHostRules,
    target_volumes: Vec<crate::artifacts::puzzle3d::schema::WorldVolumeProps>,
    overlap_budget: f64,
    meshes: HashMap<String, CollisionBody>,
    spatial_index: CollisionSpatialIndex,
    targets: Vec<BrushFillVortexTarget>,
    target_cursor: usize,
    target_rotation: usize,
    target_prepare_phase: TargetPreparePhase,
    blocked_vortex_ids: BTreeSet<String>,
    target_attraction_cursor: usize,
    target_object_cursor: usize,
    target_vortex_cursor: usize,
    seed_targets: Vec<BrushFillVortexTarget>,
    frontier_targets: Vec<BrushFillVortexTarget>,
    seed_target_weights: Vec<f64>,
    frontier_target_weights: Vec<f64>,
    seed_target_tree: Vec<f64>,
    frontier_target_tree: Vec<f64>,
    target_prepare_cursor: usize,
    seed_target_remaining: usize,
    frontier_target_remaining: usize,
    current_target: Option<BrushFillVortexTarget>,
    candidates: Vec<BrushCompatibleCandidate>,
    candidate_cursor: usize,
    candidate_prepare_phase: CandidatePreparePhase,
    candidate_kind_cursor: usize,
    candidate_vortex_cursor: usize,
    candidate_prepare_cursor: usize,
    candidate_seen: BTreeSet<String>,
    candidate_raw: Vec<BrushCompatibleCandidate>,
    candidate_cross: BTreeMap<String, BrushCompatibleCandidate>,
    candidate_same: BTreeMap<String, BrushCompatibleCandidate>,
    candidate_same_sorted: Vec<BrushCompatibleCandidate>,
    candidate_same_weights: Vec<f64>,
    candidate_same_tree: Vec<f64>,
    candidate_same_remaining: usize,
    current_preview: Option<BrushPreviewState>,
    broad_phase_ids: Vec<String>,
    broad_phase_cursor: usize,
    broad_phase_bounds: Option<CollisionAabb>,
    collision: Option<CollisionOverlapState>,
    accept_phase: AcceptPhase,
    accept_attraction_cursor: usize,
    accept_vortex_cursor: usize,
    pending_payload: Option<BrushPlacePayload>,
    pending_object: Option<FixtureObject>,
    pending_attraction: Option<AttractionProps>,
    last_rejection: Option<String>,
    transition_count: u64,
    rejected_count: u64,
}

impl FillBuilder {
    pub(crate) fn new(base: Fixture, seed: u32, meshes: &HashMap<String, CollisionBody>, catalogs: &KindCatalogBundle) -> Self {
        let base_fingerprint = fixture_fingerprint(&base);
        let seed_object_ids: std::collections::HashSet<String> = base.objects.iter().map(|o| o.id.clone()).collect();
        let mut placed = Vec::new();
        for obj in &base.objects {
            if let Some(mesh_url) = resolve_object_kind_mesh_url(obj.object_kind.as_deref().unwrap_or(""), catalogs, &base) {
                if meshes.contains_key(&mesh_url) {
                    placed.push(PlacedCollisionEntry { object_id: obj.id.clone(), mesh_url, world: pose_isometry(obj.origin, obj.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &obj.scale) });
                }
            }
        }
        let mut spatial_index = CollisionSpatialIndex::new(8.0);
        for entry in &placed {
            if let Some(body) = meshes.get(&entry.mesh_url) {
                spatial_index.upsert(entry.object_id.clone(), CollisionAabb::from_body(body, &entry.world));
            }
        }
        let placed_lookup = placed.iter().enumerate().map(|(index, entry)| (entry.object_id.clone(), index)).collect();
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), seed as u64);
        Self {
            base: base.clone(),
            base_fingerprint,
            fixture: base,
            applied_count: 0,
            sequence: Vec::new(),
            appended_objects: Vec::new(),
            appended_attractions: Vec::new(),
            placed,
            placed_lookup,
            candidate_cache: BTreeMap::new(),
            seed_object_ids,
            rng_state: seed,
            stalled: false,
            max_count: FILL_COUNT_MAX,
            operation,
            stage: FillJobStage::PrepareTargets,
            preview: FillBuildPreview {
                operation: operation.operation.0,
                base_revision: operation.base_revision.0,
                sequence: 0,
                generation: 0,
                stage: "prepare-targets".into(),
                target_vortex_full_id: None,
                candidate_object_kind_id: None,
                candidate_ghost: None,
                broad_phase_object_ids: Vec::new(),
                current_pair_object_id: None,
                colliding_object_ids: Vec::new(),
                sample_cursor: 0,
                inside_both: 0,
                last_sample: None,
                collision_samples: Vec::new(),
                rejection_reason: None,
                target_cursor: 0,
                candidate_cursor: 0,
                accepted_count: 0,
                accepted_prefix: Vec::new(),
                search_count: 0,
                rejected_count: 0,
            },
            catalogs: catalogs.clone(),
            weights: BrushKindWeights::default(),
            kind_compatibility: Vec::new(),
            host_rules: BrushHostRules::default(),
            target_volumes: Vec::new(),
            overlap_budget: 0.0,
            meshes: meshes.clone(),
            spatial_index,
            targets: Vec::new(),
            target_cursor: 0,
            target_rotation: 0,
            target_prepare_phase: TargetPreparePhase::Blocked,
            blocked_vortex_ids: BTreeSet::new(),
            target_attraction_cursor: 0,
            target_object_cursor: 0,
            target_vortex_cursor: 0,
            seed_targets: Vec::new(),
            frontier_targets: Vec::new(),
            seed_target_weights: Vec::new(),
            frontier_target_weights: Vec::new(),
            seed_target_tree: vec![0.0],
            frontier_target_tree: vec![0.0],
            target_prepare_cursor: 0,
            seed_target_remaining: 0,
            frontier_target_remaining: 0,
            current_target: None,
            candidates: Vec::new(),
            candidate_cursor: 0,
            candidate_prepare_phase: CandidatePreparePhase::Enumerate,
            candidate_kind_cursor: 0,
            candidate_vortex_cursor: 0,
            candidate_prepare_cursor: 0,
            candidate_seen: BTreeSet::new(),
            candidate_raw: Vec::new(),
            candidate_cross: BTreeMap::new(),
            candidate_same: BTreeMap::new(),
            candidate_same_sorted: Vec::new(),
            candidate_same_weights: Vec::new(),
            candidate_same_tree: vec![0.0],
            candidate_same_remaining: 0,
            current_preview: None,
            broad_phase_ids: Vec::new(),
            broad_phase_cursor: 0,
            broad_phase_bounds: None,
            collision: None,
            accept_phase: AcceptPhase::Validate,
            accept_attraction_cursor: 0,
            accept_vortex_cursor: 0,
            pending_payload: None,
            pending_object: None,
            pending_attraction: None,
            last_rejection: None,
            transition_count: 0,
            rejected_count: 0,
        }
    }

    pub(crate) fn configure(
        &mut self,
        operation: Operation,
        weights: BrushKindWeights,
        kind_compatibility: Vec<KindCompatEntry>,
        host_rules: BrushHostRules,
        target_volumes: Vec<crate::artifacts::puzzle3d::schema::WorldVolumeProps>,
        overlap_budget: f64,
    ) {
        self.operation = operation;
        self.weights = weights;
        self.kind_compatibility = kind_compatibility;
        self.host_rules = host_rules;
        self.target_volumes = target_volumes;
        self.overlap_budget = overlap_budget;
        self.preview.generation = operation.generation.0;
        self.preview.operation = operation.operation.0;
        self.preview.base_revision = operation.base_revision.0;
    }

    pub(crate) fn refresh_meshes(&mut self, meshes: &HashMap<String, CollisionBody>) {
        self.meshes = meshes.clone();
        self.rebuild_collision_index();
        self.restart_search();
    }

    pub(crate) fn restart_search(&mut self) {
        self.reset_candidate();
        self.stage = FillJobStage::PrepareTargets;
        self.stalled = false;
    }

    pub(crate) fn progress(&self) -> FillBuildProgress {
        FillBuildProgress {
            count: self.sequence.len(),
            applied_count: self.applied_count,
            max_count: self.max_count,
            done: self.stalled || self.sequence.len() >= self.max_count,
            appended_objects: self.appended_objects.clone(),
            appended_attractions: self.appended_attractions.clone(),
            sequence: self.sequence.clone(),
            preview: Some(self.preview.clone()),
        }
    }
}

//#region 🧵️InteractiveFillJob
impl FillBuilder {
    pub(crate) fn checkpoint_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&FillJobCheckpoint {
            operation_id: self.operation.operation.0,
            base_revision: self.operation.base_revision.0,
            generation: self.operation.generation.0,
            operation_preview_sequence: self.operation.preview_sequence,
            operation_seed: self.operation.seed,
            base_fingerprint: self.base_fingerprint,
            applied_count: self.applied_count,
            sequence: self.sequence.clone(),
            appended_objects: self.appended_objects.clone(),
            appended_attractions: self.appended_attractions.clone(),
            candidate_cache: self.candidate_cache.clone(),
            rng_state: self.rng_state,
            stalled: self.stalled,
            stage: self.stage,
            targets: self.targets.clone(),
            target_cursor: self.target_cursor,
            target_rotation: self.target_rotation,
            target_prepare_phase: self.target_prepare_phase,
            blocked_vortex_ids: self.blocked_vortex_ids.clone(),
            target_attraction_cursor: self.target_attraction_cursor,
            target_object_cursor: self.target_object_cursor,
            target_vortex_cursor: self.target_vortex_cursor,
            seed_targets: self.seed_targets.clone(),
            frontier_targets: self.frontier_targets.clone(),
            seed_target_weights: self.seed_target_weights.clone(),
            frontier_target_weights: self.frontier_target_weights.clone(),
            seed_target_tree: self.seed_target_tree.clone(),
            frontier_target_tree: self.frontier_target_tree.clone(),
            target_prepare_cursor: self.target_prepare_cursor,
            seed_target_remaining: self.seed_target_remaining,
            frontier_target_remaining: self.frontier_target_remaining,
            current_target: self.current_target.clone(),
            candidates: self.candidates.clone(),
            candidate_cursor: self.candidate_cursor,
            candidate_prepare_phase: self.candidate_prepare_phase,
            candidate_kind_cursor: self.candidate_kind_cursor,
            candidate_vortex_cursor: self.candidate_vortex_cursor,
            candidate_prepare_cursor: self.candidate_prepare_cursor,
            candidate_seen: self.candidate_seen.clone(),
            candidate_raw: self.candidate_raw.clone(),
            candidate_cross: self.candidate_cross.clone(),
            candidate_same: self.candidate_same.clone(),
            candidate_same_sorted: self.candidate_same_sorted.clone(),
            candidate_same_weights: self.candidate_same_weights.clone(),
            candidate_same_tree: self.candidate_same_tree.clone(),
            candidate_same_remaining: self.candidate_same_remaining,
            current_preview: self.current_preview.clone(),
            broad_phase_ids: self.broad_phase_ids.clone(),
            broad_phase_cursor: self.broad_phase_cursor,
            broad_phase_bounds: self.broad_phase_bounds,
            collision: self.collision.clone(),
            accept_phase: self.accept_phase,
            accept_attraction_cursor: self.accept_attraction_cursor,
            accept_vortex_cursor: self.accept_vortex_cursor,
            pending_payload: self.pending_payload.clone(),
            pending_object: self.pending_object.clone(),
            pending_attraction: self.pending_attraction.clone(),
            last_rejection: self.last_rejection.clone(),
            preview: self.preview.clone(),
            transition_count: self.transition_count,
            rejected_count: self.rejected_count,
        })
        .expect("fill checkpoint state is serializable")
    }

    pub(crate) fn restore_checkpoint(&mut self, bytes: &[u8]) -> Result<(), serde_json::Error> {
        let checkpoint: FillJobCheckpoint = serde_json::from_slice(bytes)?;
        if checkpoint.base_fingerprint != self.base_fingerprint {
            return Err(<serde_json::Error as serde::de::Error>::custom("fill checkpoint base mismatch"));
        }
        self.operation = Operation::new(semio_framework_job::OperationId(checkpoint.operation_id), semio_framework_job::RevisionId(checkpoint.base_revision), semio_framework_job::Generation(checkpoint.generation), checkpoint.operation_seed);
        self.operation.preview_sequence = checkpoint.operation_preview_sequence;
        self.applied_count = checkpoint.applied_count;
        self.sequence = checkpoint.sequence;
        self.appended_objects = checkpoint.appended_objects;
        self.appended_attractions = checkpoint.appended_attractions;
        self.fixture = self.base.clone();
        self.fixture.objects.extend(self.appended_objects.iter().cloned().map(|mut object| {
            object.reveal_index = None;
            object
        }));
        self.fixture.attractions.extend(self.appended_attractions.iter().cloned());
        self.candidate_cache = checkpoint.candidate_cache;
        self.rng_state = checkpoint.rng_state;
        self.stalled = checkpoint.stalled;
        self.stage = checkpoint.stage;
        self.targets = checkpoint.targets;
        self.target_cursor = checkpoint.target_cursor;
        self.target_rotation = checkpoint.target_rotation;
        self.target_prepare_phase = checkpoint.target_prepare_phase;
        self.blocked_vortex_ids = checkpoint.blocked_vortex_ids;
        self.target_attraction_cursor = checkpoint.target_attraction_cursor;
        self.target_object_cursor = checkpoint.target_object_cursor;
        self.target_vortex_cursor = checkpoint.target_vortex_cursor;
        self.seed_targets = checkpoint.seed_targets;
        self.frontier_targets = checkpoint.frontier_targets;
        self.seed_target_weights = checkpoint.seed_target_weights;
        self.frontier_target_weights = checkpoint.frontier_target_weights;
        self.seed_target_tree = checkpoint.seed_target_tree;
        self.frontier_target_tree = checkpoint.frontier_target_tree;
        self.target_prepare_cursor = checkpoint.target_prepare_cursor;
        self.seed_target_remaining = checkpoint.seed_target_remaining;
        self.frontier_target_remaining = checkpoint.frontier_target_remaining;
        self.current_target = checkpoint.current_target;
        self.candidates = checkpoint.candidates;
        self.candidate_cursor = checkpoint.candidate_cursor;
        self.candidate_prepare_phase = checkpoint.candidate_prepare_phase;
        self.candidate_kind_cursor = checkpoint.candidate_kind_cursor;
        self.candidate_vortex_cursor = checkpoint.candidate_vortex_cursor;
        self.candidate_prepare_cursor = checkpoint.candidate_prepare_cursor;
        self.candidate_seen = checkpoint.candidate_seen;
        self.candidate_raw = checkpoint.candidate_raw;
        self.candidate_cross = checkpoint.candidate_cross;
        self.candidate_same = checkpoint.candidate_same;
        self.candidate_same_sorted = checkpoint.candidate_same_sorted;
        self.candidate_same_weights = checkpoint.candidate_same_weights;
        self.candidate_same_tree = checkpoint.candidate_same_tree;
        self.candidate_same_remaining = checkpoint.candidate_same_remaining;
        self.current_preview = checkpoint.current_preview;
        self.broad_phase_ids = checkpoint.broad_phase_ids;
        self.broad_phase_cursor = checkpoint.broad_phase_cursor;
        self.broad_phase_bounds = checkpoint.broad_phase_bounds;
        self.collision = checkpoint.collision;
        self.accept_phase = checkpoint.accept_phase;
        self.accept_attraction_cursor = checkpoint.accept_attraction_cursor;
        self.accept_vortex_cursor = checkpoint.accept_vortex_cursor;
        self.pending_payload = checkpoint.pending_payload;
        self.pending_object = checkpoint.pending_object;
        self.pending_attraction = checkpoint.pending_attraction;
        self.last_rejection = checkpoint.last_rejection;
        self.preview = checkpoint.preview;
        self.transition_count = checkpoint.transition_count;
        self.rejected_count = checkpoint.rejected_count;
        self.rebuild_collision_index();
        Ok(())
    }

    pub(crate) fn restore_checkpoint_for_fixture(&mut self, bytes: &[u8], fixture: &Fixture) -> Result<bool, serde_json::Error> {
        let checkpoint: FillJobCheckpoint = serde_json::from_slice(bytes)?;
        if checkpoint.base_fingerprint != self.base_fingerprint {
            return Ok(false);
        }
        let mut expected = self.base.clone();
        expected.objects.extend(checkpoint.appended_objects.iter().take(checkpoint.applied_count).cloned().map(|mut object| {
            object.reveal_index = None;
            object
        }));
        expected.attractions.extend(checkpoint.appended_attractions.iter().take(checkpoint.applied_count).cloned());
        if &expected != fixture {
            return Ok(false);
        }
        self.restore_checkpoint(bytes)?;
        Ok(true)
    }

    #[cfg(test)]
    pub(crate) fn normalized_checkpoint_bytes(bytes: &[u8]) -> Vec<u8> {
        let mut checkpoint: FillJobCheckpoint = serde_json::from_slice(bytes).expect("checkpoint");
        checkpoint.operation_id = 0;
        checkpoint.preview.operation = 0;
        serde_json::to_vec(&checkpoint).expect("checkpoint")
    }

    pub(crate) fn rebuild_collision_index(&mut self) {
        self.placed.clear();
        self.placed_lookup.clear();
        self.spatial_index = CollisionSpatialIndex::new(8.0);
        for object in &self.fixture.objects {
            let Some(mesh_url) = resolve_object_kind_mesh_url(object.object_kind.as_deref().unwrap_or(""), &self.catalogs, &self.fixture) else {
                continue;
            };
            let Some(body) = self.meshes.get(&mesh_url) else { continue };
            let world = pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale);
            self.spatial_index.upsert(object.id.clone(), CollisionAabb::from_body(body, &world));
            let index = self.placed.len();
            self.placed_lookup.insert(object.id.clone(), index);
            self.placed.push(PlacedCollisionEntry { object_id: object.id.clone(), mesh_url, world });
        }
    }

    fn prepare_targets(&mut self) {
        match self.target_prepare_phase {
            TargetPreparePhase::Blocked => {
                if let Some(attraction) = self.fixture.attractions.get(self.target_attraction_cursor) {
                    self.blocked_vortex_ids.insert(attraction.attracting.clone());
                    self.blocked_vortex_ids.insert(attraction.attracted.clone());
                    self.target_attraction_cursor += 1;
                } else {
                    self.target_prepare_phase = TargetPreparePhase::Enumerate;
                }
            }
            TargetPreparePhase::Enumerate => {
                let Some(object) = self.fixture.objects.get(self.target_object_cursor) else {
                    self.seed_target_tree = vec![0.0; self.seed_target_weights.len() + 1];
                    self.frontier_target_tree = vec![0.0; self.frontier_target_weights.len() + 1];
                    self.seed_target_remaining = self.seed_targets.len();
                    self.frontier_target_remaining = self.frontier_targets.len();
                    self.target_prepare_cursor = 0;
                    self.target_prepare_phase = TargetPreparePhase::BuildSeedWeights;
                    return;
                };
                let Some(vortex) = object.vortices.get(self.target_vortex_cursor) else {
                    self.target_object_cursor += 1;
                    self.target_vortex_cursor = 0;
                    return;
                };
                let vortex_index = self.target_vortex_cursor;
                self.target_vortex_cursor += 1;
                let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                if self.blocked_vortex_ids.contains(&full_id) {
                    return;
                }
                let target = BrushFillVortexTarget { full_id, object_id: object.id.clone(), object_kind: object.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone(), vortex_index };
                let weight = fill_vortex_target_weight(&target, &self.weights);
                if weight <= 0.0 {
                    return;
                }
                if self.seed_object_ids.contains(&target.object_id) {
                    self.seed_targets.push(target);
                    self.seed_target_weights.push(weight);
                } else {
                    self.frontier_targets.push(target);
                    self.frontier_target_weights.push(weight);
                }
            }
            TargetPreparePhase::BuildSeedWeights => {
                if let Some(weight) = self.seed_target_weights.get(self.target_prepare_cursor).copied() {
                    fenwick_add(&mut self.seed_target_tree, self.target_prepare_cursor, weight);
                    self.target_prepare_cursor += 1;
                } else {
                    self.target_prepare_cursor = 0;
                    self.target_prepare_phase = TargetPreparePhase::BuildFrontierWeights;
                }
            }
            TargetPreparePhase::BuildFrontierWeights => {
                if let Some(weight) = self.frontier_target_weights.get(self.target_prepare_cursor).copied() {
                    fenwick_add(&mut self.frontier_target_tree, self.target_prepare_cursor, weight);
                    self.target_prepare_cursor += 1;
                } else {
                    self.target_prepare_phase = TargetPreparePhase::OrderSeed;
                }
            }
            TargetPreparePhase::OrderSeed => {
                if let Some(index) = weighted_pick(&mut self.seed_target_weights, &mut self.seed_target_tree, self.seed_target_remaining, &mut self.rng_state) {
                    self.targets.push(self.seed_targets[index].clone());
                    self.seed_target_remaining -= 1;
                } else {
                    self.target_prepare_phase = TargetPreparePhase::OrderFrontier;
                }
            }
            TargetPreparePhase::OrderFrontier => {
                if let Some(index) = weighted_pick(&mut self.frontier_target_weights, &mut self.frontier_target_tree, self.frontier_target_remaining, &mut self.rng_state) {
                    self.targets.push(self.frontier_targets[index].clone());
                    self.frontier_target_remaining -= 1;
                } else {
                    self.target_prepare_phase = TargetPreparePhase::Finish;
                }
            }
            TargetPreparePhase::Finish => {
                if self.targets.is_empty() {
                    self.stalled = true;
                    self.stage = FillJobStage::Complete;
                    return;
                }
                self.target_rotation = self.sequence.len() % self.targets.len();
                self.target_cursor = 0;
                self.stage = FillJobStage::SelectTarget;
            }
        }
    }

    fn select_target(&mut self) {
        if self.target_cursor >= self.targets.len() {
            self.stalled = true;
            self.stage = FillJobStage::Complete;
            return;
        }
        let index = self.target_rotation.checked_add(self.target_cursor).map(|value| value % self.targets.len().max(1));
        let Some(target) = index.and_then(|index| self.targets.get(index)).cloned() else {
            self.stalled = true;
            self.stage = FillJobStage::Complete;
            return;
        };
        self.preview.target_vortex_full_id = Some(target.full_id.clone());
        self.preview.target_cursor = self.target_cursor;
        self.current_target = Some(target);
        self.reset_candidate_preparation();
        self.stage = FillJobStage::PrepareCandidates;
    }

    fn prepare_candidates(&mut self) {
        let Some(target) = self.current_target.clone() else {
            self.reject_target("missing-target");
            return;
        };
        let target_context = AttractionVortexContext { object_kind: target.object_kind.clone(), vortex_kind: target.vortex_kind.clone() };
        match self.candidate_prepare_phase {
            CandidatePreparePhase::Enumerate => {
                let Some(kind) = self.catalogs.objects.get(self.candidate_kind_cursor) else {
                    self.candidate_prepare_cursor = 0;
                    self.candidate_prepare_phase = CandidatePreparePhase::Classify;
                    return;
                };
                if self.candidate_vortex_cursor >= kind.vortices.len() {
                    self.candidate_kind_cursor += 1;
                    self.candidate_vortex_cursor = 0;
                    return;
                }
                let vortex_index = self.candidate_vortex_cursor;
                self.candidate_vortex_cursor += 1;
                let Some((candidate, _)) = brush_fill_candidate_at(&target_context, &self.catalogs, &self.kind_compatibility, &self.host_rules, self.candidate_kind_cursor, vortex_index) else { return };
                let key = format!("{}\u{1}{}", candidate.object_kind_id, candidate.source_vortex_index);
                if self.candidate_seen.insert(key) {
                    self.candidate_raw.push(candidate);
                }
            }
            CandidatePreparePhase::Classify => {
                let Some(candidate) = self.candidate_raw.get(self.candidate_prepare_cursor).cloned() else {
                    self.candidate_prepare_phase = CandidatePreparePhase::DrainCross;
                    return;
                };
                self.candidate_prepare_cursor += 1;
                if brush_candidate_suggestion_weight(&candidate, &self.weights, &self.catalogs) <= 0.0 {
                    return;
                }
                let source_vortex = self.catalogs.objects.iter().find(|kind| kind.id == candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|vortex| vortex.vortex_kind.as_deref()).unwrap_or("");
                let target_vortex = target.vortex_kind.as_deref().unwrap_or("");
                if source_vortex != target_vortex || brush_stack_mate_pair(source_vortex, target_vortex) {
                    let score = fill_candidate_diversity_score(&candidate, target.vortex_index, target.object_kind.as_deref()).max(0) as u64;
                    let key = format!("{:016x}\u{1}{}\u{1}{:016x}", u64::MAX - score, candidate.object_kind_id, candidate.source_vortex_index);
                    self.candidate_cross.insert(key, candidate);
                } else {
                    let key = format!("{}\u{1}{:016x}", candidate.object_kind_id, candidate.source_vortex_index);
                    self.candidate_same.insert(key, candidate);
                }
            }
            CandidatePreparePhase::DrainCross => {
                if let Some((_, candidate)) = self.candidate_cross.pop_first() {
                    self.candidates.push(candidate);
                } else {
                    self.candidate_prepare_phase = CandidatePreparePhase::DrainSame;
                }
            }
            CandidatePreparePhase::DrainSame => {
                if let Some((_, candidate)) = self.candidate_same.pop_first() {
                    self.candidate_same_weights.push(brush_candidate_suggestion_weight(&candidate, &self.weights, &self.catalogs));
                    self.candidate_same_sorted.push(candidate);
                } else {
                    self.candidate_same_remaining = self.candidate_same_sorted.len();
                    self.candidate_same_tree = vec![0.0; self.candidate_same_weights.len() + 1];
                    self.candidate_prepare_cursor = 0;
                    self.candidate_prepare_phase = CandidatePreparePhase::BuildSameWeights;
                }
            }
            CandidatePreparePhase::BuildSameWeights => {
                if let Some(weight) = self.candidate_same_weights.get(self.candidate_prepare_cursor).copied() {
                    fenwick_add(&mut self.candidate_same_tree, self.candidate_prepare_cursor, weight);
                    self.candidate_prepare_cursor += 1;
                } else {
                    self.candidate_prepare_phase = CandidatePreparePhase::OrderSame;
                }
            }
            CandidatePreparePhase::OrderSame => {
                if let Some(index) = weighted_pick(&mut self.candidate_same_weights, &mut self.candidate_same_tree, self.candidate_same_remaining, &mut self.rng_state) {
                    self.candidates.push(self.candidate_same_sorted[index].clone());
                    self.candidate_same_remaining -= 1;
                } else {
                    self.candidate_prepare_phase = CandidatePreparePhase::Finish;
                }
            }
            CandidatePreparePhase::Finish => {
                self.candidate_cursor = 0;
                if self.candidates.is_empty() {
                    self.reject_target("no-compatible-candidate");
                } else {
                    self.stage = FillJobStage::SelectCandidate;
                }
            }
        }
    }

    fn select_candidate(&mut self) {
        let Some(candidate) = self.candidates.get(self.candidate_cursor) else {
            self.reject_target("candidates-exhausted");
            return;
        };
        self.preview.candidate_cursor = self.candidate_cursor;
        self.preview.candidate_object_kind_id = Some(candidate.object_kind_id.clone());
        self.stage = FillJobStage::ConstructPreview;
    }

    fn construct_preview(&mut self) {
        let Some(target) = &self.current_target else {
            self.reject_target("missing-target");
            return;
        };
        let Some(candidate) = self.candidates.get(self.candidate_cursor) else {
            self.reject_target("missing-candidate");
            return;
        };
        let Some(host) = self.fixture.objects.iter().find(|object| object.id == target.object_id) else {
            self.reject_target("missing-host");
            return;
        };
        let Some((position, direction)) = vortex_world_from_object(host, target.vortex_index) else {
            self.reject_candidate("invalid-target-pose");
            return;
        };
        let context = AttractionVortexContext { object_kind: target.object_kind.clone(), vortex_kind: target.vortex_kind.clone() };
        let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
        let Some(preview) = brush_preview_from_candidate(&target.full_id, candidate, &context, world, &self.catalogs, &self.fixture) else {
            self.reject_candidate("preview-unavailable");
            return;
        };
        let Some(body) = self.meshes.get(&preview.mesh_url) else {
            self.reject_candidate("mesh-unavailable");
            return;
        };
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let (min, max) = world_bounds(body, &preview_world);
        if !world_volumes_contain_aabb(&self.target_volumes, min, max) {
            self.reject_candidate("outside-target-volume");
            return;
        }
        self.current_preview = Some(preview);
        self.last_rejection = None;
        self.preview.rejection_reason = None;
        self.preview.candidate_ghost = self.current_preview.clone();
        self.preview.current_pair_object_id = None;
        self.preview.colliding_object_ids.clear();
        self.preview.collision_samples.clear();
        self.stage = FillJobStage::QueryBroadPhase;
    }

    fn query_broad_phase(&mut self) {
        let Some(target) = &self.current_target else {
            self.reject_target("missing-target");
            return;
        };
        let Some(preview) = &self.current_preview else {
            self.reject_candidate("missing-preview");
            return;
        };
        let Some(body) = self.meshes.get(&preview.mesh_url) else {
            self.reject_candidate("mesh-unavailable");
            return;
        };
        if self.broad_phase_bounds.is_none() {
            let world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
            self.broad_phase_bounds = Some(CollisionAabb::from_body(body, &world));
            self.broad_phase_cursor = 0;
            self.broad_phase_ids.clear();
            return;
        }
        if let Some(entry) = self.placed.get(self.broad_phase_cursor) {
            self.broad_phase_cursor += 1;
            if entry.object_id != target.object_id && self.spatial_index.entry_intersects(&entry.object_id, self.broad_phase_bounds.expect("query bounds")) {
                self.broad_phase_ids.push(entry.object_id.clone());
            }
            return;
        }
        self.broad_phase_cursor = 0;
        self.collision = None;
        self.preview.broad_phase_object_ids = self.broad_phase_ids.clone();
        self.stage = FillJobStage::TestCollision;
    }

    fn test_collision(&mut self, context: &mut StepContext<'_>) -> Option<StepOutcome> {
        let Some(pair_id) = self.broad_phase_ids.get(self.broad_phase_cursor).cloned() else {
            self.preview.current_pair_object_id = None;
            self.stage = FillJobStage::AcceptCandidate;
            return None;
        };
        self.preview.current_pair_object_id = Some(pair_id.clone());
        let Some(preview) = &self.current_preview else {
            self.reject_candidate("missing-preview");
            return None;
        };
        let Some(preview_body) = self.meshes.get(&preview.mesh_url) else {
            self.reject_candidate("mesh-unavailable");
            return None;
        };
        let Some(entry) = self.placed_lookup.get(&pair_id).and_then(|index| self.placed.get(*index)) else {
            self.reject_candidate("broad-phase-entry-missing");
            return None;
        };
        let Some(other) = self.meshes.get(&entry.mesh_url) else {
            self.reject_candidate("placed-mesh-unavailable");
            return None;
        };
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let collision = self.collision.get_or_insert_with(|| CollisionOverlapState::new(512, 8, self.overlap_budget));
        let result = collision.step(context, preview_body, &preview_world, other, &entry.world);
        self.preview.sample_cursor = collision.sample_cursor;
        self.preview.inside_both = collision.inside_both;
        self.preview.last_sample = collision.last_sample;
        if let Some(sample) = collision.last_sample {
            if self.preview.collision_samples.last() != Some(&sample) {
                if self.preview.collision_samples.len() == 32 {
                    self.preview.collision_samples.remove(0);
                }
                self.preview.collision_samples.push(sample);
            }
        }
        match result {
            CollisionStepResult::Pending => {}
            CollisionStepResult::Cancelled => return Some(StepOutcome::Cancelled),
            CollisionStepResult::Complete { overlap, .. } if overlap > self.overlap_budget => {
                self.preview.colliding_object_ids.push(pair_id);
                self.reject_candidate("solid-overlap");
            }
            CollisionStepResult::Complete { .. } => {
                self.broad_phase_cursor += 1;
                self.collision = None;
                self.preview.sample_cursor = 0;
                self.preview.inside_both = 0;
                self.preview.last_sample = None;
            }
        }
        None
    }

    fn accept_candidate(&mut self) -> StepOutcome {
        match self.accept_phase {
            AcceptPhase::Validate => {
                let Some(preview) = self.current_preview.clone() else {
                    self.reject_candidate("missing-preview");
                    return StepOutcome::Yield;
                };
                let payload = BrushPlacePayload {
                    target_vortex_full_id: preview.target_vortex_full_id.clone(),
                    object_kind_id: preview.object_kind_id.clone(),
                    source_vortex_index: preview.source_vortex_index,
                    origin: preview.origin,
                    orientation: preview.orientation,
                    scale: preview.scale.clone(),
                };
                let Some(kind) = self.catalogs.objects.iter().find(|kind| kind.id == payload.object_kind_id) else {
                    self.reject_candidate("placement-kind-missing");
                    return StepOutcome::Yield;
                };
                if kind.vortices.get(payload.source_vortex_index).is_none() {
                    self.reject_candidate("placement-vortex-missing");
                    return StepOutcome::Yield;
                }
                let Some(mesh_url) = resolve_object_kind_mesh_url(&payload.object_kind_id, &self.catalogs, &self.fixture) else {
                    self.reject_candidate("placement-mesh-missing");
                    return StepOutcome::Yield;
                };
                let object_id = brush_object_id(&self.fixture, &payload);
                let source_vortex_id = format!("{object_id}:v{}", payload.source_vortex_index);
                let attracted = puzzle3d_vortex_full_id(&object_id, &source_vortex_id);
                self.pending_attraction = Some(AttractionProps {
                    id: format!("attraction-{}-{attracted}", payload.target_vortex_full_id),
                    attracting: payload.target_vortex_full_id.clone(),
                    attracted,
                    gap: 0.0,
                    shift: 0.0,
                    rise: 0.0,
                    rotation: 0.0,
                    turn: 0.0,
                    tilt: 0.0,
                    x: 0.0,
                    y: 0.0,
                });
                self.pending_object = Some(FixtureObject {
                    id: object_id,
                    object_kind: Some(kind.id.clone()),
                    anchor: Default::default(),
                    mesh_url: Some(mesh_url),
                    origin: payload.origin,
                    orientation: Some(payload.orientation),
                    scale: payload.scale.clone().or(kind.scale.clone()),
                    vortices: Vec::new(),
                    reveal_index: None,
                });
                self.pending_payload = Some(payload);
                self.accept_attraction_cursor = 0;
                self.accept_vortex_cursor = 0;
                self.accept_phase = AcceptPhase::CheckAttractions;
                StepOutcome::Yield
            }
            AcceptPhase::CheckAttractions => {
                let Some(pending) = self.pending_attraction.as_ref() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                if let Some(attraction) = self.fixture.attractions.get(self.accept_attraction_cursor) {
                    self.accept_attraction_cursor += 1;
                    if attraction.attracting == pending.attracting || attraction.attracted == pending.attracted {
                        self.reject_candidate("placement-rejected");
                    }
                    return StepOutcome::Yield;
                }
                self.accept_phase = AcceptPhase::BuildVortices;
                StepOutcome::Yield
            }
            AcceptPhase::BuildVortices => {
                let Some(payload) = self.pending_payload.as_ref() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(kind) = self.catalogs.objects.iter().find(|kind| kind.id == payload.object_kind_id) else {
                    self.reject_candidate("placement-kind-missing");
                    return StepOutcome::Yield;
                };
                if let Some(template) = kind.vortices.get(self.accept_vortex_cursor) {
                    let object_id = self.pending_object.as_ref().expect("pending object").id.clone();
                    let index = self.accept_vortex_cursor;
                    self.accept_vortex_cursor += 1;
                    self.pending_object.as_mut().expect("pending object").vortices.push(VortexProps { id: format!("{object_id}:v{index}"), vortex_kind: template.vortex_kind.clone(), position: template.point, direction: template.direction });
                    return StepOutcome::Yield;
                }
                self.accept_phase = AcceptPhase::Commit;
                StepOutcome::Yield
            }
            AcceptPhase::Commit => {
                let Some(payload) = self.pending_payload.take() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(mut placed_object) = self.pending_object.take() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(attraction) = self.pending_attraction.take() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                if let Some(mesh_url) = placed_object.mesh_url.clone() {
                    if let Some(body) = self.meshes.get(&mesh_url) {
                        let world = pose_isometry(placed_object.origin, placed_object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &placed_object.scale);
                        self.spatial_index.upsert(placed_object.id.clone(), CollisionAabb::from_body(body, &world));
                        let index = self.placed.len();
                        self.placed_lookup.insert(placed_object.id.clone(), index);
                        self.placed.push(PlacedCollisionEntry { object_id: placed_object.id.clone(), mesh_url, world });
                    }
                }
                self.fixture.attractions.push(attraction.clone());
                self.fixture.objects.push(placed_object.clone());
                self.sequence.push(payload);
                placed_object.reveal_index = Some(self.appended_objects.len());
                self.appended_objects.push(placed_object);
                self.appended_attractions.push(attraction);
                self.preview.accepted_count = self.sequence.len();
                self.preview.accepted_prefix = self.sequence.clone();
                self.reset_candidate();
                self.stage = if self.sequence.len() >= self.max_count { FillJobStage::Complete } else { FillJobStage::PrepareTargets };
                if self.stage == FillJobStage::Complete {
                    return self.complete();
                }
                StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: self.checkpoint_bytes(), applied_progress: self.applied_count as u64 })
            }
        }
    }

    fn reject_candidate(&mut self, reason: &str) {
        self.last_rejection = Some(reason.to_string());
        self.preview.rejection_reason = self.last_rejection.clone();
        self.rejected_count += 1;
        self.preview.rejected_count = self.rejected_count;
        self.candidate_cursor += 1;
        self.reset_acceptance();
        self.reset_collision(false);
        self.stage = FillJobStage::SelectCandidate;
    }

    fn reject_target(&mut self, reason: &str) {
        self.last_rejection = Some(reason.to_string());
        self.preview.rejection_reason = self.last_rejection.clone();
        self.rejected_count += 1;
        self.preview.rejected_count = self.rejected_count;
        self.target_cursor += 1;
        self.current_target = None;
        self.reset_candidate_preparation();
        self.reset_acceptance();
        self.reset_collision(false);
        self.stage = FillJobStage::SelectTarget;
    }

    fn reset_collision(&mut self, clear_preview: bool) {
        self.current_preview = None;
        self.broad_phase_ids.clear();
        self.broad_phase_cursor = 0;
        self.broad_phase_bounds = None;
        self.collision = None;
        if clear_preview {
            self.preview.candidate_ghost = None;
            self.preview.broad_phase_object_ids.clear();
            self.preview.current_pair_object_id = None;
            self.preview.colliding_object_ids.clear();
            self.preview.sample_cursor = 0;
            self.preview.inside_both = 0;
            self.preview.last_sample = None;
            self.preview.collision_samples.clear();
        }
    }

    fn reset_candidate(&mut self) {
        self.targets.clear();
        self.target_cursor = 0;
        self.target_rotation = 0;
        self.target_prepare_phase = TargetPreparePhase::Blocked;
        self.blocked_vortex_ids.clear();
        self.target_attraction_cursor = 0;
        self.target_object_cursor = 0;
        self.target_vortex_cursor = 0;
        self.seed_targets.clear();
        self.frontier_targets.clear();
        self.seed_target_weights.clear();
        self.frontier_target_weights.clear();
        self.seed_target_tree = vec![0.0];
        self.frontier_target_tree = vec![0.0];
        self.target_prepare_cursor = 0;
        self.seed_target_remaining = 0;
        self.frontier_target_remaining = 0;
        self.current_target = None;
        self.reset_candidate_preparation();
        self.reset_acceptance();
        self.last_rejection = None;
        self.preview.rejection_reason = None;
        self.reset_collision(true);
    }

    fn reset_candidate_preparation(&mut self) {
        self.candidates.clear();
        self.candidate_cursor = 0;
        self.candidate_prepare_phase = CandidatePreparePhase::Enumerate;
        self.candidate_kind_cursor = 0;
        self.candidate_vortex_cursor = 0;
        self.candidate_prepare_cursor = 0;
        self.candidate_seen.clear();
        self.candidate_raw.clear();
        self.candidate_cross.clear();
        self.candidate_same.clear();
        self.candidate_same_sorted.clear();
        self.candidate_same_weights.clear();
        self.candidate_same_tree = vec![0.0];
        self.candidate_same_remaining = 0;
    }

    fn reset_acceptance(&mut self) {
        self.accept_phase = AcceptPhase::Validate;
        self.accept_attraction_cursor = 0;
        self.accept_vortex_cursor = 0;
        self.pending_payload = None;
        self.pending_object = None;
        self.pending_attraction = None;
    }

    fn publish_preview(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        self.preview.sequence = context.next_preview_sequence();
        self.preview.operation = self.operation.operation.0;
        self.preview.base_revision = self.operation.base_revision.0;
        self.preview.generation = self.operation.generation.0;
        self.preview.stage = self.stage_label().to_string();
        self.preview.target_cursor = self.target_cursor;
        self.preview.candidate_cursor = self.candidate_cursor;
        self.preview.accepted_prefix = self.sequence.clone();
        self.preview.search_count = self.transition_count;
        self.preview.rejected_count = self.rejected_count;
        StepOutcome::PreviewReady(serde_json::to_vec(&self.preview).expect("fill preview is serializable"))
    }

    fn complete(&self) -> StepOutcome {
        StepOutcome::Complete(CommitCandidate { state: self.checkpoint_bytes(), output: serde_json::to_vec(&self.progress()).expect("fill progress is serializable") })
    }

    fn stage_label(&self) -> &'static str {
        match self.stage {
            FillJobStage::PrepareTargets => "prepare-targets",
            FillJobStage::SelectTarget => "select-target",
            FillJobStage::PrepareCandidates => "prepare-candidates",
            FillJobStage::SelectCandidate => "select-candidate",
            FillJobStage::ConstructPreview => "construct-preview",
            FillJobStage::QueryBroadPhase => "query-broad-phase",
            FillJobStage::TestCollision => "test-collision",
            FillJobStage::AcceptCandidate => "accept-candidate",
            FillJobStage::Complete => "complete",
        }
    }
}

impl InteractiveJob for FillBuilder {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: b"stale-fill-operation".to_vec() });
        }
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        context.set_stage(self.stage_label());
        let stage = self.stage;
        let outcome = match stage {
            FillJobStage::PrepareTargets => {
                self.prepare_targets();
                None
            }
            FillJobStage::SelectTarget => {
                self.select_target();
                None
            }
            FillJobStage::PrepareCandidates => {
                self.prepare_candidates();
                None
            }
            FillJobStage::SelectCandidate => {
                self.select_candidate();
                None
            }
            FillJobStage::ConstructPreview => {
                self.construct_preview();
                None
            }
            FillJobStage::QueryBroadPhase => {
                self.query_broad_phase();
                None
            }
            FillJobStage::TestCollision => self.test_collision(context),
            FillJobStage::AcceptCandidate => Some(self.accept_candidate()),
            FillJobStage::Complete => return self.complete(),
        };
        self.transition_count += 1;
        if stage != FillJobStage::TestCollision {
            context.consume_fuel(1);
        }
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if stage == self.stage && matches!(stage, FillJobStage::PrepareTargets | FillJobStage::PrepareCandidates | FillJobStage::QueryBroadPhase) {
            return StepOutcome::Yield;
        }
        outcome.unwrap_or_else(|| self.publish_preview(context))
    }
}
//#endregion 🧵️InteractiveFillJob

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle3d::schema::{ObjectKind, ObjectKindRepresentation, ObjectKindVortexTemplate, VortexProps};
    use crate::editor::puzzle3d::precompute::geometry::collision_body_from_buffers;
    use semio_framework_job::{root_cancel_token, Generation, OperationId, RevisionId, StepBudget};
    use std::time::{Duration, Instant};

    fn empty_builder() -> FillBuilder {
        FillBuilder::new(Fixture { objects: vec![], attractions: vec![], target_volumes: vec![] }, 17, &HashMap::new(), &KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] })
    }

    fn test_context<'a>(builder: &FillBuilder, cancel: semio_framework_job::CancelToken, sequence: &'a mut u64) -> StepContext<'a> {
        fn now() -> u64 {
            0
        }
        StepContext::new(builder.operation.operation, builder.operation.generation, StepBudget::new(100, 10), cancel, now, sequence)
    }

    #[test]
    fn checkpoint_restore_is_byte_identical() {
        let mut builder = empty_builder();
        builder.stage = FillJobStage::SelectCandidate;
        builder.target_cursor = 3;
        builder.candidate_cursor = 5;
        builder.rng_state = 0x1234_5678;
        builder.preview.sequence = 9;
        let checkpoint = builder.checkpoint_bytes();
        builder.stage = FillJobStage::Complete;
        builder.target_cursor = 0;
        builder.rng_state = 0;
        builder.restore_checkpoint(&checkpoint).expect("checkpoint");
        assert_eq!(builder.checkpoint_bytes(), checkpoint);
    }

    #[test]
    fn cancellation_is_observed_before_the_next_transition() {
        let mut builder = empty_builder();
        let before = builder.checkpoint_bytes();
        let cancel = root_cancel_token();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut context = test_context(&builder, cancel, &mut sequence);
        assert_eq!(builder.step(&mut context), StepOutcome::Cancelled);
        assert_eq!(builder.checkpoint_bytes(), before);
    }

    #[test]
    fn preview_payload_is_typed_revisioned_and_checkpointed() {
        let mut builder = empty_builder();
        builder.configure(Operation::new(OperationId(41), RevisionId(7), Generation(3), 17), BrushKindWeights::default(), Vec::new(), BrushHostRules::default(), Vec::new(), 0.0);
        builder.current_preview =
            Some(BrushPreviewState { target_vortex_full_id: "host:v0".into(), object_kind_id: "candidate".into(), source_vortex_index: 2, mesh_url: "/candidate.glb".into(), origin: [1.0, 2.0, 3.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None });
        builder.preview.candidate_ghost = builder.current_preview.clone();
        builder.preview.broad_phase_object_ids = vec!["a".into(), "b".into()];
        builder.preview.current_pair_object_id = Some("a".into());
        builder.preview.colliding_object_ids = vec!["a".into()];
        builder.preview.collision_samples = vec![[0.25, 0.5, 0.75]];
        builder.preview.rejection_reason = Some("solid-overlap".into());
        builder.transition_count = 23;
        builder.rejected_count = 4;
        let mut sequence = 0;
        let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
        let StepOutcome::PreviewReady(bytes) = builder.publish_preview(&mut context) else { panic!("preview") };
        let decoded: FillBuildPreview = serde_json::from_slice(&bytes).expect("typed preview");
        assert_eq!(decoded, builder.preview);
        assert_eq!((decoded.operation, decoded.base_revision, decoded.generation), (41, 7, 3));
        assert_eq!(decoded.candidate_ghost.as_ref().map(|ghost| ghost.mesh_url.as_str()), Some("/candidate.glb"));
        assert_eq!(decoded.colliding_object_ids, ["a"]);
        assert_eq!((decoded.search_count, decoded.rejected_count), (23, 4));
        let checkpoint = builder.checkpoint_bytes();
        builder.preview = empty_builder().preview;
        builder.restore_checkpoint(&checkpoint).expect("checkpoint");
        assert_eq!(builder.preview, decoded);
    }

    #[test]
    fn stale_generation_faults_without_progress() {
        fn now() -> u64 {
            0
        }
        let mut builder = empty_builder();
        let before = builder.checkpoint_bytes();
        let mut sequence = 0;
        let mut context = StepContext::new(OperationId(builder.operation.operation.0), Generation(builder.operation.generation.0 + 1), StepBudget::new(100, 10), root_cancel_token(), now, &mut sequence);
        assert!(matches!(builder.step(&mut context), StepOutcome::Fault(_)));
        assert_eq!(builder.checkpoint_bytes(), before);
        assert_eq!(builder.operation.base_revision, RevisionId(0));
    }

    #[test]
    fn empty_fill_transition_stays_below_watchdog_ceiling() {
        let mut builder = empty_builder();
        let mut sequence = 0;
        for _ in 0..16 {
            let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
            let started = Instant::now();
            let _ = builder.step(&mut context);
            assert!(started.elapsed() < Duration::from_millis(8));
            if builder.stage == FillJobStage::Complete {
                break;
            }
        }
        assert_eq!(builder.stage, FillJobStage::Complete);
    }

    #[test]
    fn adversarial_broad_phase_fill_is_end_to_end_resumable_below_eight_ms() {
        let representation = |id: &str| ObjectKindRepresentation { id: id.into(), name: String::new(), url: "/stress/box.glb".into(), mime: String::new(), tags: Vec::new(), lod: None, description: String::new() };
        let candidate_vortex = ObjectKindVortexTemplate { vortex_kind: Some("port-a".into()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() };
        let catalogs = KindCatalogBundle {
            objects: vec![
                ObjectKind { id: "Host".into(), representations: vec![representation("host")], scale: None, vortices: Vec::new() },
                ObjectKind { id: "Obstacle".into(), representations: vec![representation("obstacle")], scale: None, vortices: Vec::new() },
                ObjectKind { id: "Placed".into(), representations: vec![representation("placed")], scale: None, vortices: vec![candidate_vortex] },
            ],
            vortices: Vec::new(),
            cables: Vec::new(),
        };
        let host = FixtureObject {
            id: "host".into(),
            object_kind: Some("Host".into()),
            anchor: Default::default(),
            mesh_url: Some("/stress/box.glb".into()),
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            vortices: vec![VortexProps { id: "v0".into(), vortex_kind: Some("port-a".into()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
            reveal_index: None,
        };
        let mut objects = vec![host];
        objects.extend((0..1_024).map(|index| FixtureObject {
            id: format!("obstacle-{index:04}"),
            object_kind: Some("Obstacle".into()),
            anchor: Default::default(),
            mesh_url: Some("/stress/box.glb".into()),
            origin: [10_000.0 + index as f64 * 16.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            vortices: Vec::new(),
            reveal_index: None,
        }));
        let positions = [-4.0, -4.0, 0.0, 4.0, -4.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 8.0];
        let indices = [0, 1, 2, 0, 1, 3, 1, 2, 3, 2, 0, 3];
        let body = collision_body_from_buffers(&positions, &indices).expect("stress body");
        let meshes = HashMap::from([("/stress/box.glb".to_string(), body)]);
        let mut builder = FillBuilder::new(Fixture { objects, attractions: Vec::new(), target_volumes: Vec::new() }, 29, &meshes, &catalogs);
        let mut sequence = 0;
        let started = Instant::now();
        let mut first_candidate = None;
        let mut max_step = Duration::ZERO;
        for _ in 0..50_000 {
            let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
            let step_started = Instant::now();
            let outcome = builder.step(&mut context);
            let step_elapsed = step_started.elapsed();
            max_step = max_step.max(step_elapsed);
            assert!(step_elapsed < Duration::from_millis(8), "stage {:?} reached the 8ms ceiling", builder.stage);
            if first_candidate.is_none() && builder.preview.candidate_ghost.is_some() {
                first_candidate = Some(started.elapsed());
            }
            if outcome.is_terminal() {
                break;
            }
        }
        assert!(first_candidate.is_some_and(|elapsed| elapsed < Duration::from_millis(50)), "adversarial fill did not publish its first candidate within 50ms: {first_candidate:?}");
        assert_eq!(builder.stage, FillJobStage::Complete);
        assert_eq!(builder.sequence.len(), 1);
        eprintln!("[DEBUG] puzzle3d-fill-adversarial first-preview-us={} max-step-us={} transitions={}", first_candidate.expect("first candidate").as_micros(), max_step.as_micros(), builder.transition_count);
    }
}
//#endregion 🧪️Tests
