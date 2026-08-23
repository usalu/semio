//! 🪣️ Puzzle 3d play app — the precompute fill planner's own state: the running `FillBuilder` (base
//! scene, the growing plan sequence and its appended objects/attractions, the placed collision
//! entries the next step tests against, the per-session RNG stream) plus its progress readout. The
//! stepping itself lives in the sibling `⏳️precompute/🦀️component.rs`, which owns the two precompute
//! lanes. Rehomed from the former `⚙️engine/🪣️fill` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this is interactive fill-tool session state,
//! so it lives with the app, not the artifact.

use crate::artifacts::puzzle3d::schema::{
    puzzle3d_vortex_full_id, AttractionProps, BrushCompatibleCandidate, BrushHostRules, BrushKindWeights, BrushPlacePayload, BrushPreviewState, FillBuildPreview, FillBuildProgress, Fixture, FixtureObject, KindCatalogBundle, KindCompatEntry, SceneConfig,
    VortexProps,
};
use crate::editor::puzzle3d::precompute::brush::{
    brush_fill_candidate_at, brush_object_id, brush_preview_from_candidate, brush_stack_mate_pair, fill_candidate_diversity_score, fill_rng, resolve_object_kind_mesh_url, vortex_world_from_object, AttractionVortexContext, BrushFillVortexTarget,
    TargetVortexWorld,
};
use crate::editor::puzzle3d::precompute::geometry::{
    pose_isometry, world_bounds, world_volumes_contain_aabb, CollisionAabb, CollisionBody, CollisionIndexMutation, CollisionIndexOwner, CollisionIndexOwnerCensusCursor, CollisionIndexOwnerCensusStep,
    CollisionIndexRejectedOwner, CollisionMutationStep, CollisionOverlapState, CollisionQueryCursor, CollisionQueryStep, CollisionSpatialIndex, CollisionStepResult, FixedOwnerMap, FixedOwnerMapInsert, FixedOwnerSet,
    FixedOwnerSetInsert, Pose3d, FIXED_OWNER_PAGE_BYTES, FIXED_OWNER_SLOTS,
};
use crate::editor::puzzle3d::precompute::FILL_COUNT_MAX;
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, Operation, StepContext, StepOutcome};
use std::collections::HashMap;
use std::sync::Arc;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FillJobStage {
    PrepareFixture,
    PrepareCatalogs,
    PrepareMeshes,
    PrepareEntries,
    PrepareSpatial,
    PrepareLookup,
    PrepareConfiguration,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TargetPreparePhase {
    Blocked,
    Enumerate,
    BuildSeedWeights,
    BuildFrontierWeights,
    OrderSeed,
    OrderFrontier,
    Finish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CandidatePreparePhase {
    Enumerate,
    Classify,
    DrainCross,
    DrainSame,
    BuildSameWeights,
    OrderSame,
    Finish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AcceptPhase {
    Validate,
    CheckAttractions,
    BuildVortices,
    BeginSpatial,
    StepSpatial,
    InstallLookup,
    Commit,
}

pub(crate) struct FillPreparationRoots {
    scene: Arc<SceneConfig>,
    meshes: Arc<HashMap<String, CollisionBody>>,
}

impl FillPreparationRoots {
    pub(crate) fn new(scene: Arc<SceneConfig>, meshes: Arc<HashMap<String, CollisionBody>>) -> Self {
        Self { scene, meshes }
    }
}

pub(crate) struct FillBuilder {
    pub(crate) base: Fixture,
    preparation_roots: Option<FillPreparationRoots>,
    preparation_cursor: usize,
    preparation_inner_cursor: usize,
    preparation_spatial: Option<CollisionIndexMutation>,
    pub(crate) fixture: Fixture,
    pub(crate) applied_count: usize,
    pub(crate) sequence: Vec<BrushPlacePayload>,
    pub(crate) appended_objects: Vec<FixtureObject>,
    pub(crate) appended_attractions: Vec<AttractionProps>,
    pub(crate) placed: Vec<PlacedCollisionEntry>,
    placed_lookup: FixedOwnerMap<String, usize>,
    pub(crate) candidate_cache: FixedOwnerMap<String, Vec<BrushCompatibleCandidate>>,
    pub(crate) seed_object_ids: FixedOwnerSet<String>,
    pub(crate) rng_state: u32,
    pub(crate) stalled: bool,
    pub(crate) max_count: usize,
    pub(crate) operation: Operation,
    pub(crate) stage: FillJobStage,
    pub(crate) preview: FillBuildPreview,
    catalogs: KindCatalogBundle,
    weights: RetainedBrushKindWeights,
    kind_compatibility: Vec<KindCompatEntry>,
    host_rules: BrushHostRules,
    target_volumes: Vec<crate::artifacts::puzzle3d::schema::WorldVolumeProps>,
    overlap_budget: f64,
    meshes: FixedOwnerMap<String, CollisionBody>,
    spatial_index: CollisionSpatialIndex,
    targets: Vec<BrushFillVortexTarget>,
    target_cursor: usize,
    target_rotation: usize,
    target_prepare_phase: TargetPreparePhase,
    blocked_vortex_ids: FixedOwnerSet<String>,
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
    candidate_seen: FixedOwnerSet<String>,
    candidate_raw: Vec<BrushCompatibleCandidate>,
    candidate_cross: FixedOwnerMap<String, BrushCompatibleCandidate>,
    candidate_same: FixedOwnerMap<String, BrushCompatibleCandidate>,
    candidate_same_sorted: Vec<BrushCompatibleCandidate>,
    candidate_same_weights: Vec<f64>,
    candidate_same_tree: Vec<f64>,
    candidate_same_remaining: usize,
    current_preview: Option<BrushPreviewState>,
    broad_phase_query: Option<CollisionQueryCursor>,
    broad_phase_cursor: usize,
    broad_phase_bounds: Option<CollisionAabb>,
    collision: Option<CollisionOverlapState>,
    accept_phase: AcceptPhase,
    accept_attraction_cursor: usize,
    accept_vortex_cursor: usize,
    pending_payload: Option<BrushPlacePayload>,
    pending_object: Option<FixtureObject>,
    pending_attraction: Option<AttractionProps>,
    pending_spatial: Option<CollisionIndexMutation>,
    last_rejection: Option<String>,
    fixed_rejection: Option<FillRetiredOwner>,
    collection_over_capacity: bool,
    transition_count: u64,
    rejected_count: u64,
}

pub(crate) const FILL_BUILDER_OWNER_PAGE_BYTES: usize = 16 * 1024;
const FILL_BUILDER_NESTED_ITEMS: usize = 32;
const FILL_BUILDER_STD_COLLECTIONS: usize = 10;

struct RetainedBrushKindWeights {
    object_weights: FixedOwnerMap<String, f64>,
    vortex_weights: FixedOwnerMap<String, f64>,
}

impl RetainedBrushKindWeights {
    fn new() -> Self {
        Self { object_weights: FixedOwnerMap::new(), vortex_weights: FixedOwnerMap::new() }
    }

    fn object_value(&self, id: &str) -> f64 {
        self.object_weights.get(id).copied().unwrap_or(1.0)
    }

    fn vortex_value(&self, id: &str) -> f64 {
        self.vortex_weights.get(id).copied().unwrap_or(1.0)
    }
}

fn retained_fill_vortex_target_weight(target: &BrushFillVortexTarget, weights: &RetainedBrushKindWeights) -> f64 {
    weights.vortex_value(target.vortex_kind.as_deref().unwrap_or(""))
}

fn retained_candidate_suggestion_weight(candidate: &BrushCompatibleCandidate, weights: &RetainedBrushKindWeights, catalogs: &KindCatalogBundle) -> f64 {
    let vortex_kind = catalogs.objects.iter().find(|kind| kind.id == candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|template| template.vortex_kind.as_deref()).unwrap_or("");
    weights.object_value(&candidate.object_kind_id) * weights.vortex_value(vortex_kind)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct FillBuilderOwnerCredit {
    pub(crate) items: usize,
    pub(crate) bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FillBuilderOwnerCensusStep {
    Pending,
    Complete(FillBuilderOwnerCredit),
    Rejected,
}

pub(crate) struct FillBuilderOwnerCensusCursor {
    field: u8,
    section: u8,
    phase: u8,
    index: usize,
    inner: usize,
    leaf: usize,
    dsl: Option<FillDslOwnerCensusCursor>,
    spatial: CollisionIndexOwnerCensusCursor,
    credit: FillBuilderOwnerCredit,
}

impl Default for FillBuilderOwnerCensusCursor {
    fn default() -> Self {
        Self { field: 0, section: 0, phase: 0, index: 0, inner: 0, leaf: 0, dsl: None, spatial: CollisionIndexOwnerCensusCursor::default(), credit: FillBuilderOwnerCredit::default() }
    }
}

enum FillOwnerCensusUnit {
    Credit(FillBuilderOwnerCredit),
    Advance,
    Rejected,
}

#[derive(Clone, Copy)]
enum FillDslOwnerRoot {
    FixtureObject { fixture: u8, index: usize },
    FixtureVolume { fixture: u8, index: usize },
    SequencePayload(usize),
    AppendedObject(usize),
    CatalogObject(usize),
    TargetVolume(usize),
    CurrentPreview,
    PendingPayload,
    PendingObject,
    PreviewGhost,
    PreviewAccepted(usize),
}

struct FillDslOwnerCensusCursor {
    root: FillDslOwnerRoot,
    depth: usize,
    path: [usize; 16],
    phase: [u8; 17],
    child: [usize; 17],
}

impl FillDslOwnerCensusCursor {
    fn new(root: FillDslOwnerRoot) -> Self {
        Self { root, depth: 0, path: [0; 16], phase: [0; 17], child: [0; 17] }
    }

    fn root<'a>(&self, fill: &'a FillBuilder) -> Option<&'a dsl::DslValue> {
        match self.root {
            FillDslOwnerRoot::FixtureObject { fixture, index } => {
                let value = if fixture == 0 { fill.base.objects.get(index) } else { fill.fixture.objects.get(index) }?;
                value.scale.as_ref()
            }
            FillDslOwnerRoot::FixtureVolume { fixture, index } => {
                let value = if fixture == 0 { fill.base.target_volumes.get(index) } else { fill.fixture.target_volumes.get(index) }?;
                value.scale.as_ref()
            }
            FillDslOwnerRoot::SequencePayload(index) => fill.sequence.get(index)?.scale.as_ref(),
            FillDslOwnerRoot::AppendedObject(index) => fill.appended_objects.get(index)?.scale.as_ref(),
            FillDslOwnerRoot::CatalogObject(index) => fill.catalogs.objects.get(index)?.scale.as_ref(),
            FillDslOwnerRoot::TargetVolume(index) => fill.target_volumes.get(index)?.scale.as_ref(),
            FillDslOwnerRoot::CurrentPreview => fill.current_preview.as_ref()?.scale.as_ref(),
            FillDslOwnerRoot::PendingPayload => fill.pending_payload.as_ref()?.scale.as_ref(),
            FillDslOwnerRoot::PendingObject => fill.pending_object.as_ref()?.scale.as_ref(),
            FillDslOwnerRoot::PreviewGhost => fill.preview.candidate_ghost.as_ref()?.scale.as_ref(),
            FillDslOwnerRoot::PreviewAccepted(index) => fill.preview.accepted_prefix.get(index)?.scale.as_ref(),
        }
    }

    fn value<'a>(&self, fill: &'a FillBuilder) -> Option<&'a dsl::DslValue> {
        let mut value = self.root(fill)?;
        for depth in 0..self.depth {
            value = match value {
                dsl::DslValue::Array(values) => values.get(self.path[depth])?,
                dsl::DslValue::Object(values) => &values.get(self.path[depth])?.1,
                _ => return None,
            };
        }
        Some(value)
    }

    fn step(&mut self, fill: &FillBuilder) -> Result<Option<FillBuilderOwnerCredit>, ()> {
        let Some(value) = self.value(fill) else { return Err(()) };
        if self.phase[self.depth] == 0 {
            self.phase[self.depth] = 1;
            let bytes = match value {
                dsl::DslValue::String(value) => value.capacity(),
                dsl::DslValue::Array(values) => values.capacity().checked_mul(std::mem::size_of::<dsl::DslValue>()).ok_or(())?,
                dsl::DslValue::Object(values) => values.capacity().checked_mul(std::mem::size_of::<(String, dsl::DslValue)>()).ok_or(())?,
                _ => 0,
            };
            if bytes > FILL_BUILDER_OWNER_PAGE_BYTES {
                return Err(());
            }
            return Ok(Some(FillBuilderOwnerCredit { items: usize::from(bytes != 0), bytes }));
        }
        match value {
            dsl::DslValue::Array(values) if self.child[self.depth] < values.len() => {
                if self.depth == 16 {
                    return Err(());
                }
                self.path[self.depth] = self.child[self.depth];
                self.depth += 1;
                return Ok(None);
            }
            dsl::DslValue::Object(values) if self.child[self.depth] < values.len() => {
                if self.phase[self.depth] == 1 {
                    self.phase[self.depth] = 2;
                    let bytes = values[self.child[self.depth]].0.capacity();
                    if bytes > FILL_BUILDER_OWNER_PAGE_BYTES {
                        return Err(());
                    }
                    return Ok(Some(FillBuilderOwnerCredit { items: usize::from(bytes != 0), bytes }));
                }
                if self.depth == 16 {
                    return Err(());
                }
                self.path[self.depth] = self.child[self.depth];
                self.depth += 1;
                return Ok(None);
            }
            _ => {}
        }
        if self.depth == 0 {
            return Ok(None);
        }
        self.phase[self.depth] = 0;
        self.child[self.depth] = 0;
        self.depth -= 1;
        self.child[self.depth] += 1;
        if matches!(self.value(fill), Some(dsl::DslValue::Object(_))) {
            self.phase[self.depth] = 1;
        }
        Ok(None)
    }

    fn complete(&self, fill: &FillBuilder) -> bool {
        let Some(value) = self.value(fill) else { return true };
        self.depth == 0
            && self.phase[0] != 0
            && match value {
                dsl::DslValue::Array(values) => self.child[0] >= values.len(),
                dsl::DslValue::Object(values) => self.child[0] >= values.len(),
                _ => true,
            }
    }
}

fn fill_owner_strings<const N: usize>(values: [Option<&String>; N]) -> Option<FillBuilderOwnerCredit> {
    let mut credit = FillBuilderOwnerCredit::default();
    for value in values.into_iter().flatten() {
        if value.capacity() > FILL_BUILDER_OWNER_PAGE_BYTES {
            return None;
        }
        credit.items = credit.items.checked_add(usize::from(value.capacity() != 0))?;
        credit.bytes = credit.bytes.checked_add(value.capacity())?;
        if credit.bytes > FILL_BUILDER_OWNER_PAGE_BYTES {
            return None;
        }
    }
    Some(credit)
}

fn fill_owner_vec<T>(capacity: usize) -> Option<FillBuilderOwnerCredit> {
    let bytes = capacity.checked_mul(std::mem::size_of::<T>())?;
    (capacity <= FILL_BUILDER_NESTED_ITEMS && bytes <= FILL_BUILDER_OWNER_PAGE_BYTES).then_some(FillBuilderOwnerCredit { items: usize::from(bytes != 0), bytes })
}

fn fill_owner_collection(occupied: usize) -> Option<FillBuilderOwnerCredit> {
    (occupied <= FILL_BUILDER_NESTED_ITEMS).then_some(FillBuilderOwnerCredit::default())
}

fn fill_collection_backing_credit(fill: &FillBuilder, index: usize) -> Option<FillBuilderOwnerCredit> {
    let credit = match index {
        0 => fill.placed_lookup.backing_credit(),
        1 => fill.candidate_cache.backing_credit(),
        2 => fill.seed_object_ids.backing_credit(),
        3 => fill.weights.object_weights.backing_credit(),
        4 => fill.weights.vortex_weights.backing_credit(),
        5 => fill.meshes.backing_credit(),
        6 => fill.blocked_vortex_ids.backing_credit(),
        7 => fill.candidate_seen.backing_credit(),
        8 => fill.candidate_cross.backing_credit(),
        9 => fill.candidate_same.backing_credit(),
        _ => return None,
    }?;
    (credit.1 <= FIXED_OWNER_PAGE_BYTES).then_some(FillBuilderOwnerCredit { items: credit.0, bytes: credit.1 })
}

impl FillBuilderOwnerCensusCursor {
    fn finish_field(&mut self) -> FillOwnerCensusUnit {
        self.field += 1;
        self.section = 0;
        self.phase = 0;
        self.index = 0;
        self.inner = 0;
        self.leaf = 0;
        FillOwnerCensusUnit::Advance
    }

    pub(crate) fn step(&mut self, fill: &FillBuilder, max_items: usize, max_bytes: usize) -> FillBuilderOwnerCensusStep {
        if fill.collection_over_capacity || fill.fixed_rejection.is_some() {
            return FillBuilderOwnerCensusStep::Rejected;
        }
        if self.field > 13 {
            return FillBuilderOwnerCensusStep::Complete(self.credit);
        }
        if let Some(dsl) = self.dsl.as_mut() {
            if dsl.complete(fill) {
                self.dsl = None;
                return FillBuilderOwnerCensusStep::Pending;
            }
            let unit = match dsl.step(fill) {
                Ok(Some(credit)) => FillOwnerCensusUnit::Credit(credit),
                Ok(None) => FillOwnerCensusUnit::Advance,
                Err(()) => FillOwnerCensusUnit::Rejected,
            };
            return self.apply_unit(unit, max_items, max_bytes);
        }
        let unit = self.next_unit(fill);
        self.apply_unit(unit, max_items, max_bytes)
    }

    fn apply_unit(&mut self, unit: FillOwnerCensusUnit, max_items: usize, max_bytes: usize) -> FillBuilderOwnerCensusStep {
        let FillOwnerCensusUnit::Credit(credit) = unit else {
            return if matches!(unit, FillOwnerCensusUnit::Rejected) { FillBuilderOwnerCensusStep::Rejected } else { FillBuilderOwnerCensusStep::Pending };
        };
        let Some(items) = self.credit.items.checked_add(credit.items) else { return FillBuilderOwnerCensusStep::Rejected };
        let Some(bytes) = self.credit.bytes.checked_add(credit.bytes) else { return FillBuilderOwnerCensusStep::Rejected };
        if items > max_items || bytes > max_bytes {
            return FillBuilderOwnerCensusStep::Rejected;
        }
        self.credit = FillBuilderOwnerCredit { items, bytes };
        FillBuilderOwnerCensusStep::Pending
    }

    fn next_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.field {
            0 => {
                if self.section == 0 {
                    self.section = 1;
                    return FillOwnerCensusUnit::Credit(FillBuilderOwnerCredit { items: 1, bytes: std::mem::size_of::<FillBuilder>() });
                }
                if self.index < FILL_BUILDER_STD_COLLECTIONS {
                    let Some(credit) = fill_collection_backing_credit(fill, self.index) else { return FillOwnerCensusUnit::Rejected };
                    self.index += 1;
                    return FillOwnerCensusUnit::Credit(credit);
                }
                self.finish_field()
            }
            1 | 2 => self.fixture_unit(fill, self.field - 1),
            3 => self.sequence_unit(fill),
            4 => self.lookup_unit(fill),
            5 => self.catalog_unit(fill),
            6 => self.weight_mesh_unit(fill),
            7 => self.target_unit(fill),
            8 => self.target_weight_unit(fill),
            9 => self.candidate_unit(fill),
            10 => self.candidate_order_unit(fill),
            11 => self.pending_unit(fill),
            12 => self.preview_unit(fill),
            13 => self.final_unit(fill),
            _ => FillOwnerCensusUnit::Advance,
        }
    }

    fn credit(value: Option<FillBuilderOwnerCredit>) -> FillOwnerCensusUnit {
        value.map_or(FillOwnerCensusUnit::Rejected, FillOwnerCensusUnit::Credit)
    }

    fn start_dsl(&mut self, root: FillDslOwnerRoot) -> FillOwnerCensusUnit {
        self.dsl = Some(FillDslOwnerCensusCursor::new(root));
        self.phase += 1;
        FillOwnerCensusUnit::Advance
    }

    fn fixture_object_unit(&mut self, value: &FixtureObject, root: FillDslOwnerRoot) -> Option<FillOwnerCensusUnit> {
        let unit = match self.phase {
            0 => {
                self.phase = 1;
                Self::credit(fill_owner_strings([Some(&value.id), value.object_kind.as_ref(), value.mesh_url.as_ref()]))
            }
            1 if value.scale.is_some() => self.start_dsl(root),
            1 => {
                self.phase = 2;
                FillOwnerCensusUnit::Advance
            }
            2 => {
                self.phase = 3;
                Self::credit(fill_owner_vec::<VortexProps>(value.vortices.capacity()))
            }
            _ => match value.vortices.get(self.inner) {
                Some(vortex) => {
                    self.inner += 1;
                    Self::credit(fill_owner_strings([Some(&vortex.id), vortex.vortex_kind.as_ref()]))
                }
                None => {
                    self.phase = 0;
                    self.inner = 0;
                    return None;
                }
            },
        };
        Some(unit)
    }

    fn world_volume_unit(&mut self, value: &crate::artifacts::puzzle3d::schema::WorldVolumeProps, root: FillDslOwnerRoot) -> Option<FillOwnerCensusUnit> {
        match self.phase {
            0 => {
                self.phase = 1;
                Some(Self::credit(fill_owner_strings([Some(&value.id)])))
            }
            1 if value.scale.is_some() => Some(self.start_dsl(root)),
            _ => {
                self.phase = 0;
                None
            }
        }
    }

    fn payload_unit(&mut self, value: &BrushPlacePayload, root: FillDslOwnerRoot) -> Option<FillOwnerCensusUnit> {
        match self.phase {
            0 => {
                self.phase = 1;
                Some(Self::credit(fill_owner_strings([Some(&value.target_vortex_full_id), Some(&value.object_kind_id)])))
            }
            1 if value.scale.is_some() => Some(self.start_dsl(root)),
            _ => {
                self.phase = 0;
                None
            }
        }
    }

    fn fixture_unit(&mut self, fill: &FillBuilder, fixture_id: u8) -> FillOwnerCensusUnit {
        let fixture = if fixture_id == 0 { &fill.base } else { &fill.fixture };
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_vec::<FixtureObject>(fixture.objects.capacity()))
            }
            1 => match fixture.objects.get(self.index) {
                Some(value) => match self.fixture_object_unit(value, FillDslOwnerRoot::FixtureObject { fixture: fixture_id, index: self.index }) {
                    Some(unit) => unit,
                    None => {
                        self.index += 1;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_vec::<AttractionProps>(fixture.attractions.capacity()))
            }
            3 => match fixture.attractions.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.id), Some(&value.attracting), Some(&value.attracted)]))
                }
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<crate::artifacts::puzzle3d::schema::WorldVolumeProps>(fixture.target_volumes.capacity()))
            }
            5 => match fixture.target_volumes.get(self.index) {
                Some(value) => match self.world_volume_unit(value, FillDslOwnerRoot::FixtureVolume { fixture: fixture_id, index: self.index }) {
                    Some(unit) => unit,
                    None => {
                        self.index += 1;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn sequence_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_vec::<BrushPlacePayload>(fill.sequence.capacity()))
            }
            1 => match fill.sequence.get(self.index) {
                Some(value) => match self.payload_unit(value, FillDslOwnerRoot::SequencePayload(self.index)) {
                    Some(unit) => unit,
                    None => {
                        self.index += 1;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_vec::<FixtureObject>(fill.appended_objects.capacity()))
            }
            3 => match fill.appended_objects.get(self.index) {
                Some(value) => match self.fixture_object_unit(value, FillDslOwnerRoot::AppendedObject(self.index)) {
                    Some(unit) => unit,
                    None => {
                        self.index += 1;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<AttractionProps>(fill.appended_attractions.capacity()))
            }
            5 => match fill.appended_attractions.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.id), Some(&value.attracting), Some(&value.attracted)]))
                }
                None => {
                    self.section = 6;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            6 => {
                self.section = 7;
                Self::credit(fill_owner_vec::<PlacedCollisionEntry>(fill.placed.capacity()))
            }
            7 => match fill.placed.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.object_id), Some(&value.mesh_url)]))
                }
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn lookup_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_collection(fill.placed_lookup.len()))
            }
            1 => match fill.placed_lookup.keys().nth(self.index) {
                Some(key) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(key)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    Self::credit(Some(credit))
                }
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_collection(fill.candidate_cache.len()))
            }
            3 => match fill.candidate_cache.iter().nth(self.index) {
                Some((key, values)) if self.phase == 0 => {
                    self.phase = 1;
                    let Some(mut credit) = fill_owner_strings([Some(key)]) else { return FillOwnerCensusUnit::Rejected };
                    let Some(backing) = fill_owner_vec::<BrushCompatibleCandidate>(values.capacity()) else { return FillOwnerCensusUnit::Rejected };
                    credit.items = credit.items.saturating_add(backing.items).saturating_add(1);
                    credit.bytes = credit.bytes.saturating_add(backing.bytes);
                    FillOwnerCensusUnit::Credit(credit)
                }
                Some((_, values)) => match values.get(self.inner) {
                    Some(value) => {
                        self.inner += 1;
                        Self::credit(fill_owner_strings([Some(&value.object_kind_id)]))
                    }
                    None => {
                        self.index += 1;
                        self.inner = 0;
                        self.phase = 0;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 4;
                    self.index = 0;
                    self.phase = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_collection(fill.seed_object_ids.len()))
            }
            5 => match fill.seed_object_ids.iter().nth(self.index) {
                Some(value) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(value)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    Self::credit(Some(credit))
                }
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn catalog_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_vec::<crate::artifacts::puzzle3d::schema::ObjectKind>(fill.catalogs.objects.capacity()))
            }
            1 => match fill.catalogs.objects.get(self.index) {
                Some(value) => match self.phase {
                    0 => {
                        self.phase = 1;
                        Self::credit(fill_owner_strings([Some(&value.id)]))
                    }
                    1 if value.scale.is_some() => self.start_dsl(FillDslOwnerRoot::CatalogObject(self.index)),
                    1 => {
                        self.phase = 2;
                        FillOwnerCensusUnit::Advance
                    }
                    2 => {
                        self.phase = 3;
                        Self::credit(fill_owner_vec::<crate::artifacts::puzzle3d::schema::ObjectKindRepresentation>(value.representations.capacity()))
                    }
                    3 => match value.representations.get(self.inner) {
                        Some(representation) if self.leaf == 0 => {
                            self.leaf = 1;
                            Self::credit(fill_owner_strings([Some(&representation.id), Some(&representation.name), Some(&representation.url), Some(&representation.mime), representation.lod.as_ref(), Some(&representation.description)]))
                        }
                        Some(representation) if self.leaf == 1 => {
                            self.leaf = 2;
                            Self::credit(fill_owner_vec::<String>(representation.tags.capacity()))
                        }
                        Some(representation) => match representation.tags.get(self.leaf - 2) {
                            Some(tag) => {
                                self.leaf += 1;
                                Self::credit(fill_owner_strings([Some(tag)]))
                            }
                            None => {
                                self.inner += 1;
                                self.leaf = 0;
                                FillOwnerCensusUnit::Advance
                            }
                        },
                        None => {
                            self.phase = 4;
                            self.inner = 0;
                            self.leaf = 0;
                            FillOwnerCensusUnit::Advance
                        }
                    },
                    4 => {
                        self.phase = 5;
                        Self::credit(fill_owner_vec::<crate::artifacts::puzzle3d::schema::ObjectKindVortexTemplate>(value.vortices.capacity()))
                    }
                    _ => match value.vortices.get(self.inner) {
                        Some(vortex) => {
                            self.inner += 1;
                            Self::credit(fill_owner_strings([Some(&vortex.id), Some(&vortex.name), Some(&vortex.label), Some(&vortex.description), Some(&vortex.icon), vortex.vortex_kind.as_ref()]))
                        }
                        None => {
                            self.index += 1;
                            self.inner = 0;
                            self.phase = 0;
                            FillOwnerCensusUnit::Advance
                        }
                    },
                },
                None => {
                    self.section = 2;
                    self.index = 0;
                    self.phase = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_vec::<crate::artifacts::puzzle3d::schema::VortexKindCatalog>(fill.catalogs.vortices.capacity()))
            }
            3 => match fill.catalogs.vortices.get(self.index) {
                Some(value) if self.phase == 0 => {
                    self.phase = 1;
                    Self::credit(fill_owner_strings([Some(&value.id), value.code.as_ref(), value.label.as_ref(), Some(&value.description), Some(&value.icon), Some(&value.color), value.default_cable_kind.as_ref()]))
                }
                Some(value) if self.phase == 1 => {
                    self.phase = 2;
                    Self::credit(fill_owner_vec::<String>(value.compatible_with.capacity()))
                }
                Some(value) => match value.compatible_with.get(self.inner) {
                    Some(entry) => {
                        self.inner += 1;
                        Self::credit(fill_owner_strings([Some(entry)]))
                    }
                    None => {
                        self.index += 1;
                        self.inner = 0;
                        self.phase = 0;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 4;
                    self.index = 0;
                    self.phase = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<crate::artifacts::puzzle3d::schema::CableKindCatalog>(fill.catalogs.cables.capacity()))
            }
            5 => match fill.catalogs.cables.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.id), value.default_attraction_kind.as_ref()]))
                }
                None => {
                    self.section = 6;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            6 => {
                self.section = 7;
                Self::credit(fill_owner_vec::<KindCompatEntry>(fill.kind_compatibility.capacity()))
            }
            7 => match fill.kind_compatibility.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.source), Some(&value.target), value.specificity.as_ref()]))
                }
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn weight_mesh_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_collection(fill.weights.object_weights.len()))
            }
            1 => match fill.weights.object_weights.keys().nth(self.index) {
                Some(key) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(key)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_collection(fill.weights.vortex_weights.len()))
            }
            3 => match fill.weights.vortex_weights.keys().nth(self.index) {
                Some(key) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(key)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<crate::artifacts::puzzle3d::schema::WorldVolumeProps>(fill.target_volumes.capacity()))
            }
            5 => match fill.target_volumes.get(self.index) {
                Some(value) => match self.world_volume_unit(value, FillDslOwnerRoot::TargetVolume(self.index)) {
                    Some(unit) => unit,
                    None => {
                        self.index += 1;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 6;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            6 => {
                self.section = 7;
                Self::credit(fill_owner_collection(fill.meshes.len()))
            }
            7 => match fill.meshes.iter().nth(self.index) {
                Some((key, body)) if self.phase == 0 => {
                    self.phase = 1;
                    let Some(mut credit) = fill_owner_strings([Some(key)]) else { return FillOwnerCensusUnit::Rejected };
                    let Some((items, bytes)) = body.retained_parts_backing_credit() else { return FillOwnerCensusUnit::Rejected };
                    credit.items = credit.items.saturating_add(items).saturating_add(1);
                    credit.bytes = credit.bytes.saturating_add(bytes);
                    FillOwnerCensusUnit::Credit(credit)
                }
                Some((_, body)) => match body.retained_part_credit(self.inner) {
                    Some((items, bytes)) => {
                        self.inner += 1;
                        FillOwnerCensusUnit::Credit(FillBuilderOwnerCredit { items, bytes })
                    }
                    None if self.inner < body.parts.len() => FillOwnerCensusUnit::Rejected,
                    None => {
                        self.index += 1;
                        self.inner = 0;
                        self.phase = 0;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn target_credit(value: &BrushFillVortexTarget) -> Option<FillBuilderOwnerCredit> {
        fill_owner_strings([Some(&value.full_id), Some(&value.object_id), value.object_kind.as_ref(), value.vortex_kind.as_ref()])
    }

    fn target_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_vec::<BrushFillVortexTarget>(fill.targets.capacity()))
            }
            1 => match fill.targets.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(Self::target_credit(value))
                }
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_collection(fill.blocked_vortex_ids.len()))
            }
            3 => match fill.blocked_vortex_ids.iter().nth(self.index) {
                Some(value) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(value)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<BrushFillVortexTarget>(fill.seed_targets.capacity()))
            }
            5 => match fill.seed_targets.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(Self::target_credit(value))
                }
                None => {
                    self.section = 6;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            6 => {
                self.section = 7;
                Self::credit(fill_owner_vec::<BrushFillVortexTarget>(fill.frontier_targets.capacity()))
            }
            7 => match fill.frontier_targets.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(Self::target_credit(value))
                }
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn target_weight_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        let credit = match self.section {
            0 => fill_owner_vec::<f64>(fill.seed_target_weights.capacity()),
            1 => fill_owner_vec::<f64>(fill.frontier_target_weights.capacity()),
            2 => fill_owner_vec::<f64>(fill.seed_target_tree.capacity()),
            3 => fill_owner_vec::<f64>(fill.frontier_target_tree.capacity()),
            _ => return self.finish_field(),
        };
        self.section += 1;
        Self::credit(credit)
    }

    fn candidate_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_vec::<BrushCompatibleCandidate>(fill.candidates.capacity()))
            }
            1 => match fill.candidates.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.object_kind_id)]))
                }
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_collection(fill.candidate_seen.len()))
            }
            3 => match fill.candidate_seen.iter().nth(self.index) {
                Some(value) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(value)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<BrushCompatibleCandidate>(fill.candidate_raw.capacity()))
            }
            5 => match fill.candidate_raw.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.object_kind_id)]))
                }
                None => {
                    self.section = 6;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            6 => {
                self.section = 7;
                Self::credit(fill_owner_collection(fill.candidate_cross.len()))
            }
            7 => match fill.candidate_cross.iter().nth(self.index) {
                Some((key, value)) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(key), Some(&value.object_kind_id)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn candidate_order_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_collection(fill.candidate_same.len()))
            }
            1 => match fill.candidate_same.iter().nth(self.index) {
                Some((key, value)) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(key), Some(&value.object_kind_id)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_vec::<BrushCompatibleCandidate>(fill.candidate_same_sorted.capacity()))
            }
            3 => match fill.candidate_same_sorted.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.object_kind_id)]))
                }
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<f64>(fill.candidate_same_weights.capacity()))
            }
            5 => {
                self.section = 6;
                Self::credit(fill_owner_vec::<f64>(fill.candidate_same_tree.capacity()))
            }
            _ => self.finish_field(),
        }
    }

    fn preview_state_unit(&mut self, value: &BrushPreviewState, root: FillDslOwnerRoot) -> Option<FillOwnerCensusUnit> {
        match self.phase {
            0 => {
                self.phase = 1;
                Some(Self::credit(fill_owner_strings([Some(&value.target_vortex_full_id), Some(&value.object_kind_id), Some(&value.mesh_url)])))
            }
            1 if value.scale.is_some() => Some(self.start_dsl(root)),
            _ => {
                self.phase = 0;
                None
            }
        }
    }

    fn pending_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_vec::<String>(fill.broad_phase_ids.capacity()))
            }
            1 => match fill.broad_phase_ids.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(value)]))
                }
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                fill.current_target.as_ref().map_or(FillOwnerCensusUnit::Advance, |value| Self::credit(Self::target_credit(value)))
            }
            3 => match fill.current_preview.as_ref() {
                Some(value) => match self.preview_state_unit(value, FillDslOwnerRoot::CurrentPreview) {
                    Some(unit) => unit,
                    None => {
                        self.section = 4;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 4;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => match fill.pending_payload.as_ref() {
                Some(value) => match self.payload_unit(value, FillDslOwnerRoot::PendingPayload) {
                    Some(unit) => unit,
                    None => {
                        self.section = 5;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 5;
                    FillOwnerCensusUnit::Advance
                }
            },
            5 => match fill.pending_object.as_ref() {
                Some(value) => match self.fixture_object_unit(value, FillDslOwnerRoot::PendingObject) {
                    Some(unit) => unit,
                    None => {
                        self.section = 6;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 6;
                    FillOwnerCensusUnit::Advance
                }
            },
            6 => {
                self.section = 7;
                fill.pending_attraction.as_ref().map_or(FillOwnerCensusUnit::Advance, |value| Self::credit(fill_owner_strings([Some(&value.id), Some(&value.attracting), Some(&value.attracted)])))
            }
            _ => self.finish_field(),
        }
    }

    fn preview_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        let preview = &fill.preview;
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_strings([Some(&preview.stage), preview.target_vortex_full_id.as_ref(), preview.candidate_object_kind_id.as_ref(), preview.current_pair_object_id.as_ref(), preview.rejection_reason.as_ref()]))
            }
            1 => match preview.candidate_ghost.as_ref() {
                Some(value) => match self.preview_state_unit(value, FillDslOwnerRoot::PreviewGhost) {
                    Some(unit) => unit,
                    None => {
                        self.section = 2;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 2;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_vec::<String>(preview.broad_phase_object_ids.capacity()))
            }
            3 => match preview.broad_phase_object_ids.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(value)]))
                }
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<String>(preview.colliding_object_ids.capacity()))
            }
            5 => match preview.colliding_object_ids.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(value)]))
                }
                None => {
                    self.section = 6;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            6 => {
                self.section = 7;
                Self::credit(fill_owner_vec::<[f32; 3]>(preview.collision_samples.capacity()))
            }
            7 => {
                self.section = 8;
                Self::credit(fill_owner_vec::<BrushPlacePayload>(preview.accepted_prefix.capacity()))
            }
            8 => match preview.accepted_prefix.get(self.index) {
                Some(value) => match self.payload_unit(value, FillDslOwnerRoot::PreviewAccepted(self.index)) {
                    Some(unit) => unit,
                    None => {
                        self.index += 1;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn final_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        if self.section == 0 {
            self.section = 1;
            return Self::credit(fill_owner_strings([fill.last_rejection.as_ref()]));
        }
        match fill.spatial_index.census_one_owner(&mut self.spatial) {
            CollisionIndexOwnerCensusStep::Pending { items, bytes } => FillOwnerCensusUnit::Credit(FillBuilderOwnerCredit { items, bytes }),
            CollisionIndexOwnerCensusStep::Complete => self.finish_field(),
            CollisionIndexOwnerCensusStep::Rejected => FillOwnerCensusUnit::Rejected,
        }
    }
}

pub(crate) struct FillBuilderRetirementCursor {
    fill: Option<FillBuilder>,
    field: u8,
    current: Option<FillRetiredOwner>,
}

enum FillRetiredOwner {
    String(String),
    FixtureObject(FixtureObject),
    Attraction(AttractionProps),
    WorldVolume(crate::artifacts::puzzle3d::schema::WorldVolumeProps),
    Payload(BrushPlacePayload),
    Placed(PlacedCollisionEntry),
    Candidate(BrushCompatibleCandidate),
    Target(BrushFillVortexTarget),
    PreviewState(BrushPreviewState),
    ObjectKind(crate::artifacts::puzzle3d::schema::ObjectKind),
    VortexKind(crate::artifacts::puzzle3d::schema::VortexKindCatalog),
    CableKind(crate::artifacts::puzzle3d::schema::CableKindCatalog),
    Compat(KindCompatEntry),
    CandidateCache(String, Vec<BrushCompatibleCandidate>),
    CandidateMap(String, BrushCompatibleCandidate),
    Mesh(String, CollisionBody),
    Spatial(CollisionIndexRejectedOwner),
}

fn retire_string(value: &mut String) -> bool {
    if value.capacity() == 0 {
        return true;
    }
    drop(std::mem::take(value));
    false
}

fn retire_option_string(value: &mut Option<String>) -> bool {
    let Some(string) = value.as_mut() else { return true };
    if !retire_string(string) {
        return false;
    }
    value.take();
    false
}

fn retire_dsl_one(value: &mut dsl::DslValue, depth: usize) -> bool {
    if depth > 16 {
        return false;
    }
    match value {
        dsl::DslValue::String(string) => {
            if !retire_string(string) {
                return false;
            }
            *value = dsl::DslValue::Null;
            false
        }
        dsl::DslValue::Array(values) => {
            if let Some(child) = values.last_mut() {
                if !retire_dsl_one(child, depth + 1) {
                    return false;
                }
                values.pop();
                return false;
            }
            if values.capacity() != 0 {
                drop(std::mem::take(values));
                return false;
            }
            *value = dsl::DslValue::Null;
            false
        }
        dsl::DslValue::Object(values) => {
            if let Some((key, child)) = values.last_mut() {
                if !retire_string(key) || !retire_dsl_one(child, depth + 1) {
                    return false;
                }
                values.pop();
                return false;
            }
            if values.capacity() != 0 {
                drop(std::mem::take(values));
                return false;
            }
            *value = dsl::DslValue::Null;
            false
        }
        dsl::DslValue::Null | dsl::DslValue::Bool(_) | dsl::DslValue::Number(_) => true,
    }
}

fn retire_option_dsl(value: &mut Option<dsl::DslValue>) -> bool {
    let Some(dsl) = value.as_mut() else { return true };
    if !retire_dsl_one(dsl, 0) {
        return false;
    }
    value.take();
    false
}

fn retire_fixture_object(value: &mut FixtureObject) -> bool {
    if !retire_string(&mut value.id) || !retire_option_string(&mut value.object_kind) || !retire_option_string(&mut value.mesh_url) || !retire_option_dsl(&mut value.scale) {
        return false;
    }
    if let Some(vortex) = value.vortices.last_mut() {
        if !retire_string(&mut vortex.id) || !retire_option_string(&mut vortex.vortex_kind) {
            return false;
        }
        value.vortices.pop();
        return false;
    }
    if value.vortices.capacity() != 0 {
        drop(std::mem::take(&mut value.vortices));
        return false;
    }
    true
}

fn retire_attraction(value: &mut AttractionProps) -> bool {
    retire_string(&mut value.id) && retire_string(&mut value.attracting) && retire_string(&mut value.attracted)
}

fn retire_world_volume(value: &mut crate::artifacts::puzzle3d::schema::WorldVolumeProps) -> bool {
    retire_string(&mut value.id) && retire_option_dsl(&mut value.scale)
}

fn retire_payload(value: &mut BrushPlacePayload) -> bool {
    retire_string(&mut value.target_vortex_full_id) && retire_string(&mut value.object_kind_id) && retire_option_dsl(&mut value.scale)
}

fn retire_candidate(value: &mut BrushCompatibleCandidate) -> bool {
    retire_string(&mut value.object_kind_id)
}

fn retire_target(value: &mut BrushFillVortexTarget) -> bool {
    retire_string(&mut value.full_id) && retire_string(&mut value.object_id) && retire_option_string(&mut value.object_kind) && retire_option_string(&mut value.vortex_kind)
}

fn retire_preview_state(value: &mut BrushPreviewState) -> bool {
    retire_string(&mut value.target_vortex_full_id) && retire_string(&mut value.object_kind_id) && retire_string(&mut value.mesh_url) && retire_option_dsl(&mut value.scale)
}

fn retire_fill_preview(value: &mut FillBuildPreview) -> bool {
    if !retire_string(&mut value.stage) || !retire_option_string(&mut value.target_vortex_full_id) || !retire_option_string(&mut value.candidate_object_kind_id) || value.candidate_ghost.as_mut().is_some_and(|preview| !retire_preview_state(preview)) {
        return false;
    }
    if value.candidate_ghost.is_some() {
        value.candidate_ghost.take();
        return false;
    }
    for values in [&mut value.broad_phase_object_ids, &mut value.colliding_object_ids] {
        if let Some(string) = values.last_mut() {
            if !retire_string(string) {
                return false;
            }
            values.pop();
            return false;
        }
        if values.capacity() != 0 {
            drop(std::mem::take(values));
            return false;
        }
    }
    if !retire_option_string(&mut value.current_pair_object_id) || !retire_option_string(&mut value.rejection_reason) {
        return false;
    }
    if let Some(payload) = value.accepted_prefix.last_mut() {
        if !retire_payload(payload) {
            return false;
        }
        value.accepted_prefix.pop();
        return false;
    }
    if value.accepted_prefix.capacity() != 0 {
        drop(std::mem::take(&mut value.accepted_prefix));
        return false;
    }
    if value.collision_samples.capacity() != 0 {
        drop(std::mem::take(&mut value.collision_samples));
        return false;
    }
    true
}

fn retire_object_kind(value: &mut crate::artifacts::puzzle3d::schema::ObjectKind) -> bool {
    if !retire_string(&mut value.id) || !retire_option_dsl(&mut value.scale) {
        return false;
    }
    if let Some(representation) = value.representations.last_mut() {
        if !retire_string(&mut representation.id)
            || !retire_string(&mut representation.name)
            || !retire_string(&mut representation.url)
            || !retire_string(&mut representation.mime)
            || !retire_option_string(&mut representation.lod)
            || !retire_string(&mut representation.description)
        {
            return false;
        }
        if let Some(tag) = representation.tags.last_mut() {
            if !retire_string(tag) {
                return false;
            }
            representation.tags.pop();
            return false;
        }
        if representation.tags.capacity() != 0 {
            drop(std::mem::take(&mut representation.tags));
            return false;
        }
        value.representations.pop();
        return false;
    }
    if value.representations.capacity() != 0 {
        drop(std::mem::take(&mut value.representations));
        return false;
    }
    if let Some(vortex) = value.vortices.last_mut() {
        if !retire_string(&mut vortex.id) || !retire_string(&mut vortex.name) || !retire_string(&mut vortex.label) || !retire_string(&mut vortex.description) || !retire_string(&mut vortex.icon) || !retire_option_string(&mut vortex.vortex_kind) {
            return false;
        }
        value.vortices.pop();
        return false;
    }
    if value.vortices.capacity() != 0 {
        drop(std::mem::take(&mut value.vortices));
        return false;
    }
    true
}

fn retire_retained_owner(owner: &mut FillRetiredOwner) -> bool {
    match owner {
        FillRetiredOwner::String(value) => retire_string(value),
        FillRetiredOwner::FixtureObject(value) => retire_fixture_object(value),
        FillRetiredOwner::Attraction(value) => retire_attraction(value),
        FillRetiredOwner::WorldVolume(value) => retire_world_volume(value),
        FillRetiredOwner::Payload(value) => retire_payload(value),
        FillRetiredOwner::Placed(value) => retire_string(&mut value.object_id) && retire_string(&mut value.mesh_url),
        FillRetiredOwner::Candidate(value) => retire_candidate(value),
        FillRetiredOwner::Target(value) => retire_target(value),
        FillRetiredOwner::PreviewState(value) => retire_preview_state(value),
        FillRetiredOwner::ObjectKind(value) => retire_object_kind(value),
        FillRetiredOwner::VortexKind(value) => {
            if !retire_string(&mut value.id)
                || !retire_option_string(&mut value.code)
                || !retire_option_string(&mut value.label)
                || !retire_string(&mut value.description)
                || !retire_string(&mut value.icon)
                || !retire_string(&mut value.color)
                || !retire_option_string(&mut value.default_cable_kind)
            {
                return false;
            }
            if let Some(entry) = value.compatible_with.last_mut() {
                if !retire_string(entry) {
                    return false;
                }
                value.compatible_with.pop();
                return false;
            }
            if value.compatible_with.capacity() != 0 {
                drop(std::mem::take(&mut value.compatible_with));
                return false;
            }
            true
        }
        FillRetiredOwner::CableKind(value) => retire_string(&mut value.id) && retire_option_string(&mut value.default_attraction_kind),
        FillRetiredOwner::Compat(value) => retire_string(&mut value.source) && retire_string(&mut value.target) && retire_option_string(&mut value.specificity),
        FillRetiredOwner::CandidateCache(key, values) => {
            if !retire_string(key) {
                return false;
            }
            if let Some(value) = values.last_mut() {
                if !retire_candidate(value) {
                    return false;
                }
                values.pop();
                return false;
            }
            if values.capacity() != 0 {
                drop(std::mem::take(values));
                return false;
            }
            true
        }
        FillRetiredOwner::CandidateMap(key, value) => retire_string(key) && retire_candidate(value),
        FillRetiredOwner::Mesh(key, body) => {
            if !retire_string(key) {
                return false;
            }
            if body.parts.pop().is_some() {
                return false;
            }
            if body.parts.capacity() != 0 {
                drop(std::mem::take(&mut body.parts));
                return false;
            }
            true
        }
        FillRetiredOwner::Spatial(owner) => owner.retire_one(),
    }
}

fn release_vec_backing<T>(values: &mut Vec<T>) -> bool {
    if values.capacity() == 0 {
        return false;
    }
    debug_assert!(values.is_empty());
    drop(std::mem::take(values));
    true
}

fn take_string_vec_owner(values: &mut Vec<String>, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = values.pop() {
        *current = Some(FillRetiredOwner::String(value));
        return true;
    }
    release_vec_backing(values)
}

fn take_fixture_owner(value: &mut Fixture, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = value.objects.pop() {
        *current = Some(FillRetiredOwner::FixtureObject(value));
        return true;
    }
    if release_vec_backing(&mut value.objects) {
        return true;
    }
    if let Some(value) = value.attractions.pop() {
        *current = Some(FillRetiredOwner::Attraction(value));
        return true;
    }
    if release_vec_backing(&mut value.attractions) {
        return true;
    }
    if let Some(value) = value.target_volumes.pop() {
        *current = Some(FillRetiredOwner::WorldVolume(value));
        return true;
    }
    release_vec_backing(&mut value.target_volumes)
}

fn take_sequence_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.sequence.pop() {
        *current = Some(FillRetiredOwner::Payload(value));
        return true;
    }
    if release_vec_backing(&mut fill.sequence) {
        return true;
    }
    if let Some(value) = fill.appended_objects.pop() {
        *current = Some(FillRetiredOwner::FixtureObject(value));
        return true;
    }
    if release_vec_backing(&mut fill.appended_objects) {
        return true;
    }
    if let Some(value) = fill.appended_attractions.pop() {
        *current = Some(FillRetiredOwner::Attraction(value));
        return true;
    }
    if release_vec_backing(&mut fill.appended_attractions) {
        return true;
    }
    if let Some(value) = fill.placed.pop() {
        *current = Some(FillRetiredOwner::Placed(value));
        return true;
    }
    release_vec_backing(&mut fill.placed)
}

fn take_lookup_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some((key, _)) = fill.placed_lookup.pop_first() {
        *current = Some(FillRetiredOwner::String(key));
        return true;
    }
    if let Some((key, values)) = fill.candidate_cache.pop_first() {
        *current = Some(FillRetiredOwner::CandidateCache(key, values));
        return true;
    }
    if let Some(value) = fill.seed_object_ids.pop_first() {
        *current = Some(FillRetiredOwner::String(value));
        return true;
    }
    false
}

fn take_catalog_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.catalogs.objects.pop() {
        *current = Some(FillRetiredOwner::ObjectKind(value));
        return true;
    }
    if release_vec_backing(&mut fill.catalogs.objects) {
        return true;
    }
    if let Some(value) = fill.catalogs.vortices.pop() {
        *current = Some(FillRetiredOwner::VortexKind(value));
        return true;
    }
    if release_vec_backing(&mut fill.catalogs.vortices) {
        return true;
    }
    if let Some(value) = fill.catalogs.cables.pop() {
        *current = Some(FillRetiredOwner::CableKind(value));
        return true;
    }
    if release_vec_backing(&mut fill.catalogs.cables) {
        return true;
    }
    if let Some(value) = fill.kind_compatibility.pop() {
        *current = Some(FillRetiredOwner::Compat(value));
        return true;
    }
    release_vec_backing(&mut fill.kind_compatibility)
}

fn take_weight_map_owner(values: &mut FixedOwnerMap<String, f64>, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some((key, _)) = values.pop_first() {
        *current = Some(FillRetiredOwner::String(key));
        return true;
    }
    false
}

fn take_weight_mesh_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if take_weight_map_owner(&mut fill.weights.object_weights, current) || take_weight_map_owner(&mut fill.weights.vortex_weights, current) {
        return true;
    }
    if let Some(value) = fill.target_volumes.pop() {
        *current = Some(FillRetiredOwner::WorldVolume(value));
        return true;
    }
    if release_vec_backing(&mut fill.target_volumes) {
        return true;
    }
    if let Some((key, body)) = fill.meshes.pop_first() {
        *current = Some(FillRetiredOwner::Mesh(key, body));
        return true;
    }
    false
}

fn take_target_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.targets.pop() {
        *current = Some(FillRetiredOwner::Target(value));
        return true;
    }
    if release_vec_backing(&mut fill.targets) {
        return true;
    }
    if let Some(value) = fill.blocked_vortex_ids.pop_first() {
        *current = Some(FillRetiredOwner::String(value));
        return true;
    }
    if let Some(value) = fill.seed_targets.pop() {
        *current = Some(FillRetiredOwner::Target(value));
        return true;
    }
    if release_vec_backing(&mut fill.seed_targets) {
        return true;
    }
    if let Some(value) = fill.frontier_targets.pop() {
        *current = Some(FillRetiredOwner::Target(value));
        return true;
    }
    release_vec_backing(&mut fill.frontier_targets)
}

fn take_candidate_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.candidates.pop() {
        *current = Some(FillRetiredOwner::Candidate(value));
        return true;
    }
    if release_vec_backing(&mut fill.candidates) {
        return true;
    }
    if let Some(value) = fill.candidate_seen.pop_first() {
        *current = Some(FillRetiredOwner::String(value));
        return true;
    }
    if let Some(value) = fill.candidate_raw.pop() {
        *current = Some(FillRetiredOwner::Candidate(value));
        return true;
    }
    if release_vec_backing(&mut fill.candidate_raw) {
        return true;
    }
    if let Some((key, value)) = fill.candidate_cross.pop_first() {
        *current = Some(FillRetiredOwner::CandidateMap(key, value));
        return true;
    }
    false
}

fn take_candidate_order_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some((key, value)) = fill.candidate_same.pop_first() {
        *current = Some(FillRetiredOwner::CandidateMap(key, value));
        return true;
    }
    if let Some(value) = fill.candidate_same_sorted.pop() {
        *current = Some(FillRetiredOwner::Candidate(value));
        return true;
    }
    if release_vec_backing(&mut fill.candidate_same_sorted) {
        return true;
    }
    for values in [&mut fill.candidate_same_weights, &mut fill.candidate_same_tree] {
        if values.pop().is_some() || release_vec_backing(values) {
            return true;
        }
    }
    false
}

fn retire_fixed_collection_backing(fill: &mut FillBuilder) -> bool {
    fill.placed_lookup.retire_backing()
        || fill.candidate_cache.retire_backing()
        || fill.seed_object_ids.retire_backing()
        || fill.weights.object_weights.retire_backing()
        || fill.weights.vortex_weights.retire_backing()
        || fill.meshes.retire_backing()
        || fill.blocked_vortex_ids.retire_backing()
        || fill.candidate_seen.retire_backing()
        || fill.candidate_cross.retire_backing()
        || fill.candidate_same.retire_backing()
}

fn take_target_weight_owner(fill: &mut FillBuilder) -> bool {
    for values in [&mut fill.seed_target_weights, &mut fill.frontier_target_weights, &mut fill.seed_target_tree, &mut fill.frontier_target_tree] {
        if values.pop().is_some() || release_vec_backing(values) {
            return true;
        }
    }
    false
}

fn fixture_terminal_owners_empty(value: &Fixture) -> bool {
    value.objects.is_empty() && value.objects.capacity() == 0 && value.attractions.is_empty() && value.attractions.capacity() == 0 && value.target_volumes.is_empty() && value.target_volumes.capacity() == 0
}

fn preview_terminal_owners_empty(value: &FillBuildPreview) -> bool {
    value.stage.capacity() == 0
        && value.target_vortex_full_id.is_none()
        && value.candidate_object_kind_id.is_none()
        && value.candidate_ghost.is_none()
        && value.broad_phase_object_ids.is_empty()
        && value.broad_phase_object_ids.capacity() == 0
        && value.current_pair_object_id.is_none()
        && value.colliding_object_ids.is_empty()
        && value.colliding_object_ids.capacity() == 0
        && value.collision_samples.is_empty()
        && value.collision_samples.capacity() == 0
        && value.rejection_reason.is_none()
        && value.accepted_prefix.is_empty()
        && value.accepted_prefix.capacity() == 0
}

impl FillBuilderRetirementCursor {
    pub(crate) fn new(fill: FillBuilder) -> Self {
        Self { fill: Some(fill), field: 0, current: None }
    }

    pub(crate) fn retire_one(&mut self) -> bool {
        if let Some(current) = self.current.as_mut() {
            if retire_retained_owner(current) {
                self.current = None;
            }
            return false;
        }
        let Some(fill) = self.fill.as_mut() else {
            return true;
        };
        let retired = match self.field {
            0 => take_fixture_owner(&mut fill.base, &mut self.current),
            1 => take_fixture_owner(&mut fill.fixture, &mut self.current),
            2 => take_sequence_owner(fill, &mut self.current),
            3 => take_lookup_owner(fill, &mut self.current),
            4 => take_catalog_owner(fill, &mut self.current),
            5 => take_weight_mesh_owner(fill, &mut self.current),
            6 => take_target_owner(fill, &mut self.current),
            7 => take_target_weight_owner(fill),
            8 => take_candidate_owner(fill, &mut self.current),
            9 => take_candidate_order_owner(fill, &mut self.current),
            10 => take_string_vec_owner(&mut fill.broad_phase_ids, &mut self.current),
            11 => fill.pending_payload.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::Payload(value));
                true
            }),
            12 => fill.pending_object.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::FixtureObject(value));
                true
            }),
            13 => fill.pending_attraction.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::Attraction(value));
                true
            }),
            14 => fill.current_target.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::Target(value));
                true
            }),
            15 => fill.current_preview.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::PreviewState(value));
                true
            }),
            16 => fill.last_rejection.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::String(value));
                true
            }),
            17 => fill.collision.take().is_some(),
            18 => !retire_fill_preview(&mut fill.preview),
            19 => match fill.fixed_rejection.as_mut() {
                Some(rejected) => {
                    if rejected.retire_one() {
                        fill.fixed_rejection.take();
                    }
                    true
                }
                None => false,
            },
            20 => retire_fixed_collection_backing(fill),
            21 => {
                if !fill.spatial_index.retire_one_owner() {
                    true
                } else {
                    false
                }
            }
            22 if fill.collection_over_capacity => {
                fill.collection_over_capacity = false;
                true
            }
            22 => false,
            _ => {
                if !fill.terminal_owners_empty() {
                    return false;
                }
                let shell = self.fill.take().expect("terminal-empty builder shell");
                drop(shell);
                return self.fill.is_none() && self.current.is_none();
            }
        };
        if !retired {
            self.field += 1;
        }
        false
    }
}

impl FillBuilder {
    #[cfg(test)]
    pub(crate) fn inject_nested_owner_page_plus_one_for_test(&mut self) {
        let mut owner = String::with_capacity(FILL_BUILDER_OWNER_PAGE_BYTES + 1);
        owner.push_str("nested-owner");
        self.catalogs.objects[0].representations[0].tags.push(owner);
    }

    #[cfg(test)]
    pub(crate) fn fixed_backing_witness_for_test(&self) -> [(usize, usize, usize); 13] {
        let mut witness = [
            (self.placed_lookup.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, usize>::page_bytes(), self.placed_lookup.len()),
            (self.candidate_cache.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, Vec<BrushCompatibleCandidate>>::page_bytes(), self.candidate_cache.len()),
            (self.seed_object_ids.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, ()>::page_bytes(), self.seed_object_ids.len()),
            (self.weights.object_weights.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, f64>::page_bytes(), self.weights.object_weights.len()),
            (self.weights.vortex_weights.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, f64>::page_bytes(), self.weights.vortex_weights.len()),
            (self.meshes.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, CollisionBody>::page_bytes(), self.meshes.len()),
            (self.blocked_vortex_ids.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, ()>::page_bytes(), self.blocked_vortex_ids.len()),
            (self.candidate_seen.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, ()>::page_bytes(), self.candidate_seen.len()),
            (self.candidate_cross.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, BrushCompatibleCandidate>::page_bytes(), self.candidate_cross.len()),
            (self.candidate_same.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, BrushCompatibleCandidate>::page_bytes(), self.candidate_same.len()),
            (0, 0, 0),
            (0, 0, 0),
            (0, 0, 0),
        ];
        witness[10..].copy_from_slice(&self.spatial_index.fixed_backing_witness_for_test());
        witness
    }

    fn terminal_owners_empty(&self) -> bool {
        fixture_terminal_owners_empty(&self.base)
            && fixture_terminal_owners_empty(&self.fixture)
            && self.sequence.is_empty()
            && self.sequence.capacity() == 0
            && self.appended_objects.is_empty()
            && self.appended_objects.capacity() == 0
            && self.appended_attractions.is_empty()
            && self.appended_attractions.capacity() == 0
            && self.placed.is_empty()
            && self.placed.capacity() == 0
            && self.placed_lookup.is_empty()
            && self.candidate_cache.is_empty()
            && self.seed_object_ids.is_empty()
            && self.catalogs.objects.is_empty()
            && self.catalogs.objects.capacity() == 0
            && self.catalogs.vortices.is_empty()
            && self.catalogs.vortices.capacity() == 0
            && self.catalogs.cables.is_empty()
            && self.catalogs.cables.capacity() == 0
            && self.weights.object_weights.is_empty()
            && self.weights.vortex_weights.is_empty()
            && self.kind_compatibility.is_empty()
            && self.kind_compatibility.capacity() == 0
            && self.target_volumes.is_empty()
            && self.target_volumes.capacity() == 0
            && self.meshes.is_empty()
            && self.spatial_index.terminal_owners_empty()
            && self.targets.is_empty()
            && self.targets.capacity() == 0
            && self.blocked_vortex_ids.is_empty()
            && self.seed_targets.is_empty()
            && self.seed_targets.capacity() == 0
            && self.frontier_targets.is_empty()
            && self.frontier_targets.capacity() == 0
            && self.seed_target_weights.is_empty()
            && self.seed_target_weights.capacity() == 0
            && self.frontier_target_weights.is_empty()
            && self.frontier_target_weights.capacity() == 0
            && self.seed_target_tree.is_empty()
            && self.seed_target_tree.capacity() == 0
            && self.frontier_target_tree.is_empty()
            && self.frontier_target_tree.capacity() == 0
            && self.current_target.is_none()
            && self.candidates.is_empty()
            && self.candidates.capacity() == 0
            && self.candidate_seen.is_empty()
            && self.candidate_raw.is_empty()
            && self.candidate_raw.capacity() == 0
            && self.candidate_cross.is_empty()
            && self.candidate_same.is_empty()
            && self.candidate_same_sorted.is_empty()
            && self.candidate_same_sorted.capacity() == 0
            && self.candidate_same_weights.is_empty()
            && self.candidate_same_weights.capacity() == 0
            && self.candidate_same_tree.is_empty()
            && self.candidate_same_tree.capacity() == 0
            && self.current_preview.is_none()
            && self.broad_phase_ids.is_empty()
            && self.broad_phase_ids.capacity() == 0
            && self.collision.is_none()
            && self.pending_payload.is_none()
            && self.pending_object.is_none()
            && self.pending_attraction.is_none()
            && self.last_rejection.is_none()
            && self.fixed_rejection.is_none()
            && !self.collection_over_capacity
            && self.placed_lookup.terminal_owners_empty()
            && self.candidate_cache.terminal_owners_empty()
            && self.seed_object_ids.terminal_owners_empty()
            && self.weights.object_weights.terminal_owners_empty()
            && self.weights.vortex_weights.terminal_owners_empty()
            && self.meshes.terminal_owners_empty()
            && self.blocked_vortex_ids.terminal_owners_empty()
            && self.candidate_seen.terminal_owners_empty()
            && self.candidate_cross.terminal_owners_empty()
            && self.candidate_same.terminal_owners_empty()
            && preview_terminal_owners_empty(&self.preview)
    }

    pub(crate) fn begin_preparation(roots: FillPreparationRoots, operation: Operation) -> Self {
        let seed = roots.scene.seed;
        let collection_over_capacity = roots.scene.fixture.objects.len() > FIXED_OWNER_SLOTS
            || roots.meshes.len() > FIXED_OWNER_SLOTS
            || roots.scene.weights.object_weights.len() > FIXED_OWNER_SLOTS
            || roots.scene.weights.vortex_weights.len() > FIXED_OWNER_SLOTS;
        Self {
            base: Fixture::default(),
            preparation_roots: Some(roots),
            preparation_cursor: 0,
            preparation_inner_cursor: 0,
            preparation_spatial: None,
            fixture: Fixture::default(),
            applied_count: 0,
            sequence: Vec::new(),
            appended_objects: Vec::new(),
            appended_attractions: Vec::new(),
            placed: Vec::new(),
            placed_lookup: FixedOwnerMap::new(),
            candidate_cache: FixedOwnerMap::new(),
            seed_object_ids: FixedOwnerSet::new(),
            rng_state: seed,
            stalled: false,
            max_count: FILL_COUNT_MAX,
            operation,
            stage: FillJobStage::PrepareFixture,
            preview: FillBuildPreview {
                operation: operation.operation.0,
                base_revision: operation.base_revision.0,
                sequence: 0,
                generation: 0,
                stage: "prepare-fixture".into(),
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
            catalogs: KindCatalogBundle::default(),
            weights: RetainedBrushKindWeights::new(),
            kind_compatibility: Vec::new(),
            host_rules: BrushHostRules::default(),
            target_volumes: Vec::new(),
            overlap_budget: 0.0,
            meshes: FixedOwnerMap::new(),
            spatial_index: CollisionSpatialIndex::new(8.0),
            targets: Vec::new(),
            target_cursor: 0,
            target_rotation: 0,
            target_prepare_phase: TargetPreparePhase::Blocked,
            blocked_vortex_ids: FixedOwnerSet::new(),
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
            candidate_seen: FixedOwnerSet::new(),
            candidate_raw: Vec::new(),
            candidate_cross: FixedOwnerMap::new(),
            candidate_same: FixedOwnerMap::new(),
            candidate_same_sorted: Vec::new(),
            candidate_same_weights: Vec::new(),
            candidate_same_tree: vec![0.0],
            candidate_same_remaining: 0,
            current_preview: None,
            broad_phase_query: None,
            broad_phase_cursor: 0,
            broad_phase_bounds: None,
            collision: None,
            accept_phase: AcceptPhase::Validate,
            accept_attraction_cursor: 0,
            accept_vortex_cursor: 0,
            pending_payload: None,
            pending_object: None,
            pending_attraction: None,
            pending_spatial: None,
            last_rejection: None,
            fixed_rejection: None,
            collection_over_capacity,
            transition_count: 0,
            rejected_count: 0,
        }
    }

    pub(crate) fn progress(&self) -> FillBuildProgress {
        FillBuildProgress {
            count: self.sequence.len(),
            applied_count: self.applied_count,
            max_count: self.max_count,
            done: self.stalled || self.sequence.len() >= self.max_count,
            appended_objects: Vec::new(),
            appended_attractions: Vec::new(),
            sequence: Vec::new(),
            preview: Some(self.preview.clone()),
        }
    }

    fn collision_owner(&self) -> CollisionIndexOwner {
        CollisionIndexOwner { operation: self.operation.operation.0, generation: self.operation.generation.0 }
    }

    fn prepare_one(&mut self) {
        if self.collection_over_capacity {
            self.last_rejection = Some("preparation-capacity".into());
            self.preview.rejection_reason = self.last_rejection.clone();
            self.stalled = true;
            self.stage = FillJobStage::Complete;
            return;
        }
        match self.stage {
            FillJobStage::PrepareFixture => self.prepare_fixture_one(),
            FillJobStage::PrepareCatalogs => self.prepare_catalog_one(),
            FillJobStage::PrepareMeshes => self.prepare_mesh_one(),
            FillJobStage::PrepareEntries => self.prepare_entry_one(),
            FillJobStage::PrepareSpatial => self.prepare_spatial_one(),
            FillJobStage::PrepareLookup => self.prepare_lookup_one(),
            FillJobStage::PrepareConfiguration => self.prepare_configuration_one(),
            _ => {}
        }
    }

    fn prepare_fixture_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let fixture = &roots.scene.fixture;
        let value = match self.preparation_inner_cursor {
            0 => fixture.attractions.get(self.preparation_cursor).map(|value| {
                self.base.attractions.push(value.clone());
                self.fixture.attractions.push(value.clone());
            }),
            1 => fixture.objects.get(self.preparation_cursor).map(|value| {
                self.base.objects.push(value.clone());
                self.fixture.objects.push(value.clone());
            }),
            _ => fixture.target_volumes.get(self.preparation_cursor).map(|value| {
                self.base.target_volumes.push(value.clone());
                self.fixture.target_volumes.push(value.clone());
                self.target_volumes.push(value.clone());
            }),
        };
        if value.is_some() {
            self.preparation_cursor += 1;
            return;
        }
        self.preparation_cursor = 0;
        self.preparation_inner_cursor += 1;
        if self.preparation_inner_cursor == 3 {
            self.preparation_inner_cursor = 0;
            self.stage = FillJobStage::PrepareCatalogs;
        }
    }

    fn prepare_catalog_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let catalogs = roots.scene.kind_catalogs.as_ref();
        let value = match self.preparation_inner_cursor {
            0 => catalogs.and_then(|value| value.objects.get(self.preparation_cursor)).map(|value| self.catalogs.objects.push(value.clone())),
            1 => catalogs.and_then(|value| value.vortices.get(self.preparation_cursor)).map(|value| self.catalogs.vortices.push(value.clone())),
            _ => catalogs.and_then(|value| value.cables.get(self.preparation_cursor)).map(|value| self.catalogs.cables.push(value.clone())),
        };
        if value.is_some() {
            self.preparation_cursor += 1;
            return;
        }
        self.preparation_cursor = 0;
        self.preparation_inner_cursor += 1;
        if self.preparation_inner_cursor == 3 {
            self.preparation_inner_cursor = 0;
            self.stage = FillJobStage::PrepareMeshes;
        }
    }

    fn prepare_mesh_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let Some((url, body)) = roots.meshes.iter().nth(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareEntries;
            return;
        };
        self.preparation_cursor += 1;
        match self.meshes.try_insert(url.clone(), body.clone()) {
            Ok(FixedOwnerMapInsert::Inserted) => {}
            Ok(FixedOwnerMapInsert::Occupied { input_key: url, input_value: body }) | Err((url, body)) => {
                self.fixed_rejection = Some(FillRetiredOwner::Mesh(url, body));
                self.collection_over_capacity = true;
            }
        }
    }

    fn prepare_entry_one(&mut self) {
        let Some(object) = self.base.objects.get(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareSpatial;
            return;
        };
        self.preparation_cursor += 1;
        match self.seed_object_ids.try_insert(object.id.clone()) {
            Ok(FixedOwnerSetInsert::Inserted) => {}
            Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
            Err(input) => {
                self.fixed_rejection = Some(FillRetiredOwner::String(input));
                self.collection_over_capacity = true;
                return;
            }
        }
        let Some(mesh_url) = resolve_object_kind_mesh_url(object.object_kind.as_deref().unwrap_or(""), &self.catalogs, &self.base) else {
            return;
        };
        if self.meshes.get(&mesh_url).is_none() {
            return;
        }
        self.placed.push(PlacedCollisionEntry {
            object_id: object.id.clone(),
            mesh_url,
            world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale),
        });
    }

    fn prepare_spatial_one(&mut self) {
        let owner = self.collision_owner();
        if let Some(mutation) = self.preparation_spatial.as_mut() {
            match self.spatial_index.step_replacement(mutation, owner) {
                CollisionMutationStep::Pending => return,
                CollisionMutationStep::Complete => {
                    self.preparation_spatial = None;
                    self.preparation_cursor += 1;
                    return;
                }
                CollisionMutationStep::Rejected(rejected) => {
                    self.fixed_rejection = Some(FillRetiredOwner::Spatial(rejected));
                    self.collection_over_capacity = true;
                    return;
                }
                CollisionMutationStep::Stale => {
                    self.stalled = true;
                    return;
                }
            }
        }
        let Some(entry) = self.placed.get(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareLookup;
            return;
        };
        let Some(body) = self.meshes.get(&entry.mesh_url) else {
            self.preparation_cursor += 1;
            return;
        };
        self.preparation_spatial = Some(self.spatial_index.begin_replacement(owner, entry.object_id.clone(), CollisionAabb::from_body(body, &entry.world)));
    }

    fn prepare_lookup_one(&mut self) {
        let Some(entry) = self.placed.get(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareConfiguration;
            return;
        };
        let index = self.preparation_cursor;
        self.preparation_cursor += 1;
        match self.placed_lookup.try_insert(entry.object_id.clone(), index) {
            Ok(FixedOwnerMapInsert::Inserted) => {}
            Ok(FixedOwnerMapInsert::Occupied { input_key, input_value: _ }) | Err((input_key, _)) => {
                self.fixed_rejection = Some(FillRetiredOwner::String(input_key));
                self.collection_over_capacity = true;
            }
        }
    }

    fn prepare_configuration_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let value = match self.preparation_inner_cursor {
            0 => roots.scene.weights.object_weights.iter().nth(self.preparation_cursor).map(|(id, weight)| {
                let _ = self.weights.object_weights.try_insert(id.clone(), *weight);
            }),
            1 => roots.scene.weights.vortex_weights.iter().nth(self.preparation_cursor).map(|(id, weight)| {
                let _ = self.weights.vortex_weights.try_insert(id.clone(), *weight);
            }),
            _ => roots.scene.kind_compatibility.get(self.preparation_cursor).map(|value| self.kind_compatibility.push(value.clone())),
        };
        if value.is_some() {
            self.preparation_cursor += 1;
            return;
        }
        self.preparation_cursor = 0;
        self.preparation_inner_cursor += 1;
        if self.preparation_inner_cursor == 3 {
            self.host_rules = roots.scene.host_rules.clone();
            self.overlap_budget = roots.scene.overlap_budget;
            self.preparation_roots = None;
            self.preparation_inner_cursor = 0;
            self.stage = FillJobStage::PrepareTargets;
        }
    }
}

//#region 🧵️InteractiveFillJob
impl FillBuilder {

    fn prepare_targets(&mut self) {
        match self.target_prepare_phase {
            TargetPreparePhase::Blocked => {
                if let Some(attraction) = self.fixture.attractions.get(self.target_attraction_cursor) {
                    match self.blocked_vortex_ids.try_insert(attraction.attracting.clone()) {
                        Ok(FixedOwnerSetInsert::Inserted) => {}
                        Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                        Err(value) => {
                            self.fixed_rejection = Some(FillRetiredOwner::String(value));
                            return;
                        }
                    }
                    match self.blocked_vortex_ids.try_insert(attraction.attracted.clone()) {
                        Ok(FixedOwnerSetInsert::Inserted) => {}
                        Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                        Err(value) => {
                            self.fixed_rejection = Some(FillRetiredOwner::String(value));
                            return;
                        }
                    }
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
                let weight = retained_fill_vortex_target_weight(&target, &self.weights);
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
                match self.candidate_seen.try_insert(key) {
                    Ok(FixedOwnerSetInsert::Inserted) => self.candidate_raw.push(candidate),
                    Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                    Err(key) => self.fixed_rejection = Some(FillRetiredOwner::String(key)),
                }
            }
            CandidatePreparePhase::Classify => {
                let Some(candidate) = self.candidate_raw.get(self.candidate_prepare_cursor).cloned() else {
                    self.candidate_prepare_phase = CandidatePreparePhase::DrainCross;
                    return;
                };
                self.candidate_prepare_cursor += 1;
                if retained_candidate_suggestion_weight(&candidate, &self.weights, &self.catalogs) <= 0.0 {
                    return;
                }
                let source_vortex = self.catalogs.objects.iter().find(|kind| kind.id == candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|vortex| vortex.vortex_kind.as_deref()).unwrap_or("");
                let target_vortex = target.vortex_kind.as_deref().unwrap_or("");
                if source_vortex != target_vortex || brush_stack_mate_pair(source_vortex, target_vortex) {
                    let score = fill_candidate_diversity_score(&candidate, target.vortex_index, target.object_kind.as_deref()).max(0) as u64;
                    let key = format!("{:016x}\u{1}{}\u{1}{:016x}", u64::MAX - score, candidate.object_kind_id, candidate.source_vortex_index);
                    match self.candidate_cross.try_insert(key, candidate) {
                        Ok(FixedOwnerMapInsert::Inserted) => {}
                        Ok(FixedOwnerMapInsert::Occupied { input_key: key, input_value: candidate }) | Err((key, candidate)) => {
                            self.fixed_rejection = Some(FillRetiredOwner::CandidateMap(key, candidate));
                        }
                    }
                } else {
                    let key = format!("{}\u{1}{:016x}", candidate.object_kind_id, candidate.source_vortex_index);
                    match self.candidate_same.try_insert(key, candidate) {
                        Ok(FixedOwnerMapInsert::Inserted) => {}
                        Ok(FixedOwnerMapInsert::Occupied { input_key: key, input_value: candidate }) | Err((key, candidate)) => {
                            self.fixed_rejection = Some(FillRetiredOwner::CandidateMap(key, candidate));
                        }
                    }
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
                    self.candidate_same_weights.push(retained_candidate_suggestion_weight(&candidate, &self.weights, &self.catalogs));
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
                        if let Err(rejected) = self.spatial_index.upsert(placed_object.id.clone(), CollisionAabb::from_body(body, &world)) {
                            self.fixed_rejection = Some(FillRetiredOwner::Spatial(rejected));
                            return StepOutcome::Yield;
                        }
                        let index = self.placed.len();
                        match self.placed_lookup.try_insert(placed_object.id.clone(), index) {
                            Ok(FixedOwnerMapInsert::Inserted) => {}
                            Ok(FixedOwnerMapInsert::Occupied { input_key: key, input_value: _ }) | Err((key, _)) => {
                                self.fixed_rejection = Some(FillRetiredOwner::String(key));
                                return StepOutcome::Yield;
                            }
                        }
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
                StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: Vec::new(), applied_progress: self.applied_count as u64 })
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
        self.blocked_vortex_ids.clear_for_rebuild_residual();
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
        self.candidate_seen.clear_for_rebuild_residual();
        self.candidate_raw.clear();
        self.candidate_cross.clear_for_rebuild_residual();
        self.candidate_same.clear_for_rebuild_residual();
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
        StepOutcome::PreviewReady(Vec::new())
    }

    fn complete(&self) -> StepOutcome {
        StepOutcome::Complete(CommitCandidate { state: Vec::new(), output: Vec::new() })
    }

    fn stage_label(&self) -> &'static str {
        match self.stage {
            FillJobStage::PrepareFixture => "prepare-fixture",
            FillJobStage::PrepareCatalogs => "prepare-catalogs",
            FillJobStage::PrepareMeshes => "prepare-meshes",
            FillJobStage::PrepareEntries => "prepare-entries",
            FillJobStage::PrepareSpatial => "prepare-spatial",
            FillJobStage::PrepareLookup => "prepare-lookup",
            FillJobStage::PrepareConfiguration => "prepare-configuration",
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
        if self.collection_over_capacity || self.fixed_rejection.is_some() {
            return StepOutcome::Fault(JobFault { detail: b"fill-fixed-collection-capacity".to_vec() });
        }
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        context.set_stage(self.stage_label());
        let stage = self.stage;
        let outcome = match stage {
            FillJobStage::PrepareFixture
            | FillJobStage::PrepareCatalogs
            | FillJobStage::PrepareMeshes
            | FillJobStage::PrepareEntries
            | FillJobStage::PrepareSpatial
            | FillJobStage::PrepareLookup
            | FillJobStage::PrepareConfiguration => {
                self.prepare_one();
                None
            }
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
        if stage == self.stage
            && matches!(
                stage,
                FillJobStage::PrepareFixture
                    | FillJobStage::PrepareCatalogs
                    | FillJobStage::PrepareMeshes
                    | FillJobStage::PrepareEntries
                    | FillJobStage::PrepareSpatial
                    | FillJobStage::PrepareLookup
                    | FillJobStage::PrepareConfiguration
                    | FillJobStage::PrepareTargets
                    | FillJobStage::PrepareCandidates
                    | FillJobStage::QueryBroadPhase
            )
        {
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
    fn retained_owner_census_advances_one_fixed_unit_and_rejects_collection_cap_plus_one() {
        let mut builder = empty_builder();
        let mut tags = Vec::with_capacity(FILL_BUILDER_NESTED_ITEMS);
        tags.extend((0..FILL_BUILDER_NESTED_ITEMS).map(|index| format!("tag-{index}")));
        builder.catalogs.objects.push(ObjectKind {
            id: "bounded-kind".into(),
            representations: vec![ObjectKindRepresentation { id: "r".into(), name: "n".into(), url: "u".into(), mime: "m".into(), tags, lod: Some("l".into()), description: "d".into() }],
            scale: Some(dsl::DslValue::Array(vec![dsl::DslValue::String("nested".into())])),
            vortices: Vec::new(),
        });
        let mut cursor = FillBuilderOwnerCensusCursor::default();
        let mut grants = 0;
        loop {
            let before = cursor.credit;
            match cursor.step(&builder, usize::MAX, usize::MAX) {
                FillBuilderOwnerCensusStep::Pending => {
                    assert!(cursor.credit.items.saturating_sub(before.items) <= 7, "one grant visits one entry or fixed schema unit");
                    assert!(cursor.credit.bytes.saturating_sub(before.bytes) <= FILL_BUILDER_OWNER_PAGE_BYTES, "one grant accounts at most one exact page");
                    grants += 1;
                }
                FillBuilderOwnerCensusStep::Complete(_) => break,
                FillBuilderOwnerCensusStep::Rejected => panic!("fixed boundary must admit"),
            }
        }
        assert!(grants > FILL_BUILDER_NESTED_ITEMS, "max-cardinality tags and nested DSL cannot be scanned in one admission grant");

        let mut rejected = empty_builder();
        let mut tags = Vec::with_capacity(FILL_BUILDER_NESTED_ITEMS + 1);
        tags.extend((0..=FILL_BUILDER_NESTED_ITEMS).map(|index| format!("tag-{index}")));
        rejected.catalogs.objects.push(ObjectKind {
            id: "rejected-kind".into(),
            representations: vec![ObjectKindRepresentation { id: String::new(), name: String::new(), url: String::new(), mime: String::new(), tags, lod: None, description: String::new() }],
            scale: None,
            vortices: Vec::new(),
        });
        let mut cursor = FillBuilderOwnerCensusCursor::default();
        assert!((0..256).any(|_| matches!(cursor.step(&rejected, usize::MAX, usize::MAX), FillBuilderOwnerCensusStep::Rejected)), "collection cap + 1 rejects before admission credit publication");
    }

    #[test]
    fn retained_owner_census_credits_each_actual_fixed_slot_page_not_a_layout_heuristic() {
        let builder = empty_builder();
        let expected = [
            builder.placed_lookup.backing_credit().expect("placed page").1,
            builder.candidate_cache.backing_credit().expect("cache page").1,
            builder.seed_object_ids.backing_credit().expect("seed page").1,
            builder.weights.object_weights.backing_credit().expect("object-weight page").1,
            builder.weights.vortex_weights.backing_credit().expect("vortex-weight page").1,
            builder.meshes.backing_credit().expect("mesh page").1,
            builder.blocked_vortex_ids.backing_credit().expect("blocked page").1,
            builder.candidate_seen.backing_credit().expect("seen page").1,
            builder.candidate_cross.backing_credit().expect("cross page").1,
            builder.candidate_same.backing_credit().expect("same page").1,
        ];
        let mut cursor = FillBuilderOwnerCensusCursor::default();
        assert_eq!(cursor.step(&builder, usize::MAX, usize::MAX), FillBuilderOwnerCensusStep::Pending);
        for (page, expected_bytes) in expected.into_iter().enumerate() {
            let before = cursor.credit;
            assert_eq!(cursor.step(&builder, usize::MAX, usize::MAX), FillBuilderOwnerCensusStep::Pending);
            assert_eq!(cursor.credit.items - before.items, 1, "fixed backing page {page} has one exact owner");
            assert_eq!(cursor.credit.bytes - before.bytes, expected_bytes, "fixed backing credit equals the actual slot array allocation");
            assert!(expected_bytes <= FILL_BUILDER_OWNER_PAGE_BYTES);
        }
    }

    #[test]
    fn all_fill_fixed_collections_store_max_entries_in_the_credited_page_and_return_plus_one() {
        fn map_boundary<V>(mut value: impl FnMut(usize) -> V) {
            let mut map = FixedOwnerMap::<String, V>::new();
            let page = map.backing_ptr().expect("actual fixed page");
            let credit = map.backing_credit().expect("credited fixed page");
            assert_eq!(credit, (1, FixedOwnerMap::<String, V>::page_bytes()));
            assert!(credit.1 <= FIXED_OWNER_PAGE_BYTES);
            for index in 0..FIXED_OWNER_SLOTS {
                assert!(matches!(map.try_insert(format!("key-{index:02}"), value(index)), Ok(FixedOwnerMapInsert::Inserted)));
            }
            let rejected = String::from("key-plus-one");
            let rejected_ptr = rejected.as_ptr();
            let Err((rejected, _)) = map.try_insert(rejected, value(FIXED_OWNER_SLOTS)) else { panic!("cap + 1 must reject") };
            assert_eq!(rejected.as_ptr(), rejected_ptr, "cap + 1 returns the identical key owner");
            assert_eq!(map.backing_ptr(), Some(page), "no second backing can be allocated");
            for _ in 0..FIXED_OWNER_SLOTS {
                drop(map.pop_first().expect("one semantic owner per close grant"));
                assert_eq!(map.backing_ptr(), Some(page));
            }
            assert!(map.retire_backing(), "the same actual slot page returns after semantic owners");
            assert!(map.terminal_owners_empty());
        }

        fn set_boundary() {
            let mut set = FixedOwnerSet::<String>::new();
            let page = set.backing_ptr().expect("actual fixed page");
            for index in 0..FIXED_OWNER_SLOTS {
                assert!(matches!(set.try_insert(format!("set-{index:02}")), Ok(FixedOwnerSetInsert::Inserted)));
            }
            let rejected = String::from("set-plus-one");
            let rejected_ptr = rejected.as_ptr();
            let Err(rejected) = set.try_insert(rejected) else { panic!("cap + 1 must reject") };
            assert_eq!(rejected.as_ptr(), rejected_ptr, "cap + 1 returns the identical set owner");
            assert_eq!(set.backing_ptr(), Some(page));
            for _ in 0..FIXED_OWNER_SLOTS {
                drop(set.pop_first().expect("one semantic owner per close grant"));
            }
            assert!(set.retire_backing());
            assert!(set.terminal_owners_empty());
        }

        map_boundary(|index| index);
        map_boundary(|index| vec![BrushCompatibleCandidate { object_kind_id: format!("cache-{index}"), source_vortex_index: index }]);
        set_boundary();
        map_boundary(|index| index as f64);
        map_boundary(|index| index as f64);
        let body = collision_body_from_buffers(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], &[0, 1, 2]).expect("body");
        map_boundary(|_| body.clone());
        set_boundary();
        set_boundary();
        map_boundary(|index| BrushCompatibleCandidate { object_kind_id: format!("cross-{index}"), source_vortex_index: index });
        map_boundary(|index| BrushCompatibleCandidate { object_kind_id: format!("same-{index}"), source_vortex_index: index });

        let mut cache = FixedOwnerMap::<String, Vec<BrushCompatibleCandidate>>::new();
        for index in 0..FIXED_OWNER_SLOTS {
            assert!(matches!(cache.try_insert(format!("cache-{index:02}"), Vec::new()), Ok(FixedOwnerMapInsert::Inserted)));
        }
        let rejected_key = String::from("cache-plus-one");
        let rejected_key_ptr = rejected_key.as_ptr();
        let rejected_value = vec![BrushCompatibleCandidate { object_kind_id: "identical-value".into(), source_vortex_index: 0 }];
        let rejected_value_ptr = rejected_value.as_ptr();
        let rejected_nested_ptr = rejected_value[0].object_kind_id.as_ptr();
        let Err((rejected_key, rejected_value)) = cache.try_insert(rejected_key, rejected_value) else { panic!("cache cap + 1") };
        assert_eq!(rejected_key.as_ptr(), rejected_key_ptr);
        assert_eq!(rejected_value.as_ptr(), rejected_value_ptr, "cap + 1 returns the identical nested value owner");
        assert_eq!(rejected_value[0].object_kind_id.as_ptr(), rejected_nested_ptr);
        drop(rejected_key);
        drop(rejected_value);
        for _ in 0..FIXED_OWNER_SLOTS {
            drop(cache.pop_first().expect("one retained cache entry per close grant"));
        }
        assert!(cache.retire_backing());
        assert!(cache.terminal_owners_empty());
    }

    #[test]
    fn occupied_fixed_slot_returns_the_distinct_input_owners_without_replacing_stored_owners() {
        let mut map = FixedOwnerMap::<String, Vec<String>>::new();
        let mut stored_key = String::with_capacity(64);
        stored_key.push_str("equal-key");
        let stored_key_ptr = stored_key.as_ptr();
        let stored_value = vec![String::from("stored-value")];
        let stored_value_ptr = stored_value.as_ptr();
        assert!(matches!(map.try_insert(stored_key, stored_value), Ok(FixedOwnerMapInsert::Inserted)));

        let mut input_key = String::with_capacity(256);
        input_key.push_str("equal-key");
        let input_key_ptr = input_key.as_ptr();
        let input_value = vec![String::from("input-value")];
        let input_value_ptr = input_value.as_ptr();
        let Ok(FixedOwnerMapInsert::Occupied { input_key, input_value }) = map.try_insert(input_key, input_value) else { panic!("equal key must return a typed occupied outcome") };
        assert_eq!(input_key.as_ptr(), input_key_ptr);
        assert_eq!(input_value.as_ptr(), input_value_ptr);
        let (retained_key, retained_value) = map.iter().next().expect("stored owner remains retained");
        assert_eq!(retained_key.as_ptr(), stored_key_ptr);
        assert_eq!(retained_value.as_ptr(), stored_value_ptr);

        drop(input_key);
        drop(input_value);
        drop(map.pop_first().expect("stored pair retires as one semantic owner"));
        assert!(map.retire_backing(), "actual page retires only after its stored pair");
        assert!(map.terminal_owners_empty());
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
        assert!(builder.configure(Operation::new(OperationId(41), RevisionId(7), Generation(3), 17), BrushKindWeights::default(), Vec::new(), BrushHostRules::default(), Vec::new(), 0.0).is_ok());
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
        assert!(bytes.is_empty(), "the retained envelope observes the shared preview without serializing it");
        assert_eq!((builder.preview.operation, builder.preview.base_revision, builder.preview.generation), (41, 7, 3));
        assert_eq!(builder.preview.candidate_ghost.as_ref().map(|ghost| ghost.mesh_url.as_str()), Some("/candidate.glb"));
        assert_eq!(builder.preview.colliding_object_ids, ["a"]);
        assert_eq!((builder.preview.search_count, builder.preview.rejected_count), (23, 4));
        let checkpoint = builder.checkpoint_bytes();
        let decoded = builder.preview.clone();
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
    }
}
//#endregion 🧪️Tests
