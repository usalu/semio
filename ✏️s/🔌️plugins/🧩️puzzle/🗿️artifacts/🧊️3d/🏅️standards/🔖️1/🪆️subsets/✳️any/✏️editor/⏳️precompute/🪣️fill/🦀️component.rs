//! 🪣️ Puzzle 3d play app — the precompute fill planner's own state: the running `FillBuilder` (base
//! scene, the growing plan sequence and its appended objects/attractions, the placed collision
//! entries the next step tests against, the per-session RNG stream) plus its progress readout. The
//! stepping itself lives in the sibling `⏳️precompute/🦀️component.rs`, which owns the two precompute
//! lanes. Rehomed from the former `⚙️engine/🪣️fill` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this is interactive fill-tool session state,
//! so it lives with the app, not the artifact.

use crate::artifacts::puzzle3d::schema::{
    AttractionProps, BrushCompatibleCandidate, BrushHostRules, BrushKindWeights, BrushPlacePayload, BrushPreviewState, FillBuildPreview, FillBuildProgress, Fixture, FixtureObject, KindCatalogBundle, KindCompatEntry,
};
use crate::editor::puzzle3d::precompute::brush::{
    apply_brush_placement_to_fixture, brush_compatible_candidates, brush_preview_from_candidate, enumerate_brush_fill_vortex_targets, order_brush_fill_compatible_candidates, resolve_object_kind_mesh_url, vortex_world_from_object,
    weighted_order_fill_vortex_targets, AttractionVortexContext, BrushFillVortexTarget, TargetVortexWorld,
};
use crate::editor::puzzle3d::precompute::geometry::{pose_isometry, world_bounds, world_volumes_contain_aabb, CollisionAabb, CollisionBody, CollisionOverlapState, CollisionSpatialIndex, CollisionStepResult, Pose3d};
use crate::editor::puzzle3d::precompute::FILL_COUNT_MAX;
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, Operation, StepContext, StepOutcome};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// 🧱️ One already-placed object's collision footprint, kept alongside the plan so each new fill step
/// only has to test the candidate against bodies it can actually hit.
#[derive(Clone)]
pub(crate) struct PlacedCollisionEntry {
    pub(crate) object_id: String,
    pub(crate) mesh_url: String,
    pub(crate) world: Pose3d,
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

#[derive(Clone, Serialize, Deserialize)]
struct FillJobCheckpoint {
    fixture: Fixture,
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
    current_target: Option<BrushFillVortexTarget>,
    candidates: Vec<BrushCompatibleCandidate>,
    candidate_cursor: usize,
    current_preview: Option<BrushPreviewState>,
    broad_phase_ids: Vec<String>,
    broad_phase_cursor: usize,
    collision: Option<CollisionOverlapState>,
    last_rejection: Option<String>,
    preview: FillBuildPreview,
    transition_count: u64,
}

pub(crate) struct FillBuilder {
    pub(crate) base: Fixture,
    pub(crate) fixture: Fixture,
    pub(crate) applied_count: usize,
    pub(crate) sequence: Vec<BrushPlacePayload>,
    pub(crate) appended_objects: Vec<FixtureObject>,
    pub(crate) appended_attractions: Vec<AttractionProps>,
    pub(crate) placed: Vec<PlacedCollisionEntry>,
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
    current_target: Option<BrushFillVortexTarget>,
    candidates: Vec<BrushCompatibleCandidate>,
    candidate_cursor: usize,
    current_preview: Option<BrushPreviewState>,
    broad_phase_ids: Vec<String>,
    broad_phase_cursor: usize,
    collision: Option<CollisionOverlapState>,
    last_rejection: Option<String>,
    transition_count: u64,
}

impl FillBuilder {
    pub(crate) fn new(base: Fixture, seed: u32, meshes: &HashMap<String, CollisionBody>, catalogs: &KindCatalogBundle) -> Self {
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
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), seed as u64);
        Self {
            base: base.clone(),
            fixture: base,
            applied_count: 0,
            sequence: Vec::new(),
            appended_objects: Vec::new(),
            appended_attractions: Vec::new(),
            placed,
            candidate_cache: BTreeMap::new(),
            seed_object_ids,
            rng_state: seed,
            stalled: false,
            max_count: FILL_COUNT_MAX,
            operation,
            stage: FillJobStage::PrepareTargets,
            preview: FillBuildPreview {
                sequence: 0,
                generation: 0,
                stage: "prepare-targets".into(),
                target_vortex_full_id: None,
                candidate_object_kind_id: None,
                broad_phase_object_ids: Vec::new(),
                current_pair_object_id: None,
                sample_cursor: 0,
                inside_both: 0,
                last_sample: None,
                rejection_reason: None,
                target_cursor: 0,
                candidate_cursor: 0,
                accepted_count: 0,
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
            current_target: None,
            candidates: Vec::new(),
            candidate_cursor: 0,
            current_preview: None,
            broad_phase_ids: Vec::new(),
            broad_phase_cursor: 0,
            collision: None,
            last_rejection: None,
            transition_count: 0,
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
            fixture: self.fixture.clone(),
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
            current_target: self.current_target.clone(),
            candidates: self.candidates.clone(),
            candidate_cursor: self.candidate_cursor,
            current_preview: self.current_preview.clone(),
            broad_phase_ids: self.broad_phase_ids.clone(),
            broad_phase_cursor: self.broad_phase_cursor,
            collision: self.collision.clone(),
            last_rejection: self.last_rejection.clone(),
            preview: self.preview.clone(),
            transition_count: self.transition_count,
        })
        .expect("fill checkpoint state is serializable")
    }

    pub(crate) fn restore_checkpoint(&mut self, bytes: &[u8]) -> Result<(), serde_json::Error> {
        let checkpoint: FillJobCheckpoint = serde_json::from_slice(bytes)?;
        self.fixture = checkpoint.fixture;
        self.applied_count = checkpoint.applied_count;
        self.sequence = checkpoint.sequence;
        self.appended_objects = checkpoint.appended_objects;
        self.appended_attractions = checkpoint.appended_attractions;
        self.candidate_cache = checkpoint.candidate_cache;
        self.rng_state = checkpoint.rng_state;
        self.stalled = checkpoint.stalled;
        self.stage = checkpoint.stage;
        self.targets = checkpoint.targets;
        self.target_cursor = checkpoint.target_cursor;
        self.current_target = checkpoint.current_target;
        self.candidates = checkpoint.candidates;
        self.candidate_cursor = checkpoint.candidate_cursor;
        self.current_preview = checkpoint.current_preview;
        self.broad_phase_ids = checkpoint.broad_phase_ids;
        self.broad_phase_cursor = checkpoint.broad_phase_cursor;
        self.collision = checkpoint.collision;
        self.last_rejection = checkpoint.last_rejection;
        self.preview = checkpoint.preview;
        self.transition_count = checkpoint.transition_count;
        self.rebuild_collision_index();
        Ok(())
    }

    pub(crate) fn rebuild_collision_index(&mut self) {
        self.placed.clear();
        self.spatial_index = CollisionSpatialIndex::new(8.0);
        for object in &self.fixture.objects {
            let Some(mesh_url) = resolve_object_kind_mesh_url(object.object_kind.as_deref().unwrap_or(""), &self.catalogs, &self.fixture) else {
                continue;
            };
            let Some(body) = self.meshes.get(&mesh_url) else { continue };
            let world = pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale);
            self.spatial_index.upsert(object.id.clone(), CollisionAabb::from_body(body, &world));
            self.placed.push(PlacedCollisionEntry { object_id: object.id.clone(), mesh_url, world });
        }
    }

    fn prepare_targets(&mut self) {
        let free = enumerate_brush_fill_vortex_targets(&self.fixture);
        let (seed_targets, frontier_targets): (Vec<_>, Vec<_>) = free.into_iter().partition(|target| self.seed_object_ids.contains(&target.object_id));
        self.targets = weighted_order_fill_vortex_targets(&seed_targets, &self.weights, &mut self.rng_state).into_iter().chain(weighted_order_fill_vortex_targets(&frontier_targets, &self.weights, &mut self.rng_state)).collect();
        if self.targets.is_empty() {
            self.stalled = true;
            self.stage = FillJobStage::Complete;
            return;
        }
        let start = self.sequence.len() % self.targets.len();
        self.targets.rotate_left(start);
        self.target_cursor = 0;
        self.stage = FillJobStage::SelectTarget;
    }

    fn select_target(&mut self) {
        let Some(target) = self.targets.get(self.target_cursor).cloned() else {
            self.stalled = true;
            self.stage = FillJobStage::Complete;
            return;
        };
        self.preview.target_vortex_full_id = Some(target.full_id.clone());
        self.preview.target_cursor = self.target_cursor;
        self.current_target = Some(target);
        self.stage = FillJobStage::PrepareCandidates;
    }

    fn prepare_candidates(&mut self) {
        let Some(target) = &self.current_target else {
            self.reject_target("missing-target");
            return;
        };
        let context = AttractionVortexContext { object_kind: target.object_kind.clone(), vortex_kind: target.vortex_kind.clone() };
        let key = format!("{}{}", target.object_kind.as_deref().unwrap_or(""), target.vortex_kind.as_deref().unwrap_or(""));
        let compatible = self.candidate_cache.entry(key).or_insert_with(|| brush_compatible_candidates(&context, &self.catalogs, &self.kind_compatibility, &self.host_rules)).clone();
        self.candidates = order_brush_fill_compatible_candidates(&compatible, target.vortex_kind.as_deref(), target.vortex_index, target.object_kind.as_deref(), &self.catalogs, &self.weights, &mut self.rng_state);
        self.candidate_cursor = 0;
        if self.candidates.is_empty() {
            self.reject_target("no-compatible-candidate");
        } else {
            self.stage = FillJobStage::SelectCandidate;
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
        let world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        self.broad_phase_ids = self.spatial_index.query(CollisionAabb::from_body(body, &world)).into_iter().filter(|id| id != &target.object_id).collect();
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
        let Some(entry) = self.placed.iter().find(|entry| entry.object_id == pair_id) else {
            self.reject_candidate("broad-phase-entry-missing");
            return None;
        };
        let Some(other) = self.meshes.get(&entry.mesh_url) else {
            self.reject_candidate("placed-mesh-unavailable");
            return None;
        };
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let collision = self.collision.get_or_insert_with(|| CollisionOverlapState::new(512, 8, self.overlap_budget));
        match collision.step(context, preview_body, &preview_world, other, &entry.world) {
            CollisionStepResult::Pending => {
                self.preview.sample_cursor = collision.sample_cursor;
                self.preview.inside_both = collision.inside_both;
                self.preview.last_sample = collision.last_sample;
            }
            CollisionStepResult::Cancelled => return Some(StepOutcome::Cancelled),
            CollisionStepResult::Complete { overlap, .. } if overlap > self.overlap_budget => self.reject_candidate("solid-overlap"),
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
        let next_fixture = apply_brush_placement_to_fixture(&self.fixture, &payload, &self.catalogs);
        if next_fixture.objects.len() == self.fixture.objects.len() {
            self.reject_candidate("placement-rejected");
            return StepOutcome::Yield;
        }
        let mut placed_object = next_fixture.objects.last().cloned().expect("accepted placement appends an object");
        if let Some(mesh_url) = resolve_object_kind_mesh_url(placed_object.object_kind.as_deref().unwrap_or(""), &self.catalogs, &next_fixture) {
            if let Some(body) = self.meshes.get(&mesh_url) {
                let world = pose_isometry(placed_object.origin, placed_object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &placed_object.scale);
                self.spatial_index.upsert(placed_object.id.clone(), CollisionAabb::from_body(body, &world));
                self.placed.push(PlacedCollisionEntry { object_id: placed_object.id.clone(), mesh_url, world });
            }
        }
        let attraction = next_fixture.attractions.last().cloned().expect("accepted placement appends an attraction");
        self.fixture = next_fixture;
        self.sequence.push(payload);
        placed_object.reveal_index = Some(self.appended_objects.len());
        self.appended_objects.push(placed_object);
        self.appended_attractions.push(attraction);
        self.preview.accepted_count = self.sequence.len();
        self.reset_candidate();
        self.stage = if self.sequence.len() >= self.max_count { FillJobStage::Complete } else { FillJobStage::PrepareTargets };
        if self.stage == FillJobStage::Complete {
            return self.complete();
        }
        StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: self.checkpoint_bytes(), applied_progress: self.applied_count as u64 })
    }

    fn reject_candidate(&mut self, reason: &str) {
        self.last_rejection = Some(reason.to_string());
        self.preview.rejection_reason = self.last_rejection.clone();
        self.candidate_cursor += 1;
        self.reset_collision();
        self.stage = FillJobStage::SelectCandidate;
    }

    fn reject_target(&mut self, reason: &str) {
        self.last_rejection = Some(reason.to_string());
        self.preview.rejection_reason = self.last_rejection.clone();
        self.target_cursor += 1;
        self.current_target = None;
        self.candidates.clear();
        self.candidate_cursor = 0;
        self.reset_collision();
        self.stage = FillJobStage::SelectTarget;
    }

    fn reset_collision(&mut self) {
        self.current_preview = None;
        self.broad_phase_ids.clear();
        self.broad_phase_cursor = 0;
        self.collision = None;
        self.preview.broad_phase_object_ids.clear();
        self.preview.current_pair_object_id = None;
        self.preview.sample_cursor = 0;
        self.preview.inside_both = 0;
        self.preview.last_sample = None;
    }

    fn reset_candidate(&mut self) {
        self.targets.clear();
        self.target_cursor = 0;
        self.current_target = None;
        self.candidates.clear();
        self.candidate_cursor = 0;
        self.last_rejection = None;
        self.preview.rejection_reason = None;
        self.reset_collision();
    }

    fn publish_preview(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        self.preview.sequence = context.next_preview_sequence();
        self.preview.generation = self.operation.generation.0;
        self.preview.stage = self.stage_label().to_string();
        self.preview.target_cursor = self.target_cursor;
        self.preview.candidate_cursor = self.candidate_cursor;
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
            FillJobStage::AcceptCandidate => return self.accept_candidate(),
            FillJobStage::Complete => return self.complete(),
        };
        self.transition_count += 1;
        if stage != FillJobStage::TestCollision {
            context.consume_fuel(1);
        }
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        outcome.unwrap_or_else(|| self.publish_preview(context))
    }
}
//#endregion 🧵️InteractiveFillJob

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
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
        let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
        let started = Instant::now();
        assert!(matches!(builder.step(&mut context), StepOutcome::PreviewReady(_)));
        assert!(started.elapsed() < Duration::from_millis(8));
        assert_eq!(builder.stage, FillJobStage::Complete);
    }
}
//#endregion 🧪️Tests
