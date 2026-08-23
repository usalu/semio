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
use crate::editor::puzzle3d::precompute::geometry::{
    pose_isometry, world_bounds, world_volumes_contain_aabb, CollisionAabb, CollisionBody, CollisionIndexOwnerCensusCursor, CollisionIndexOwnerCensusStep, CollisionOverlapState, CollisionSpatialIndex, CollisionStepResult, Pose3d,
};
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
    collection_backings: FillBuilderCollectionBackings,
    transition_count: u64,
    rejected_count: u64,
}

pub(crate) const FILL_BUILDER_OWNER_PAGE_BYTES: usize = 16 * 1024;
const FILL_BUILDER_NESTED_ITEMS: usize = 32;
const FILL_BUILDER_STD_COLLECTION_BACKING_BYTES: usize = FILL_BUILDER_OWNER_PAGE_BYTES;
const FILL_BUILDER_STD_COLLECTIONS: usize = 10;

struct FillBuilderCollectionBackings {
    pages: [Option<Box<[u8; FILL_BUILDER_OWNER_PAGE_BYTES]>>; FILL_BUILDER_STD_COLLECTIONS],
}

impl FillBuilderCollectionBackings {
    fn new() -> Self {
        Self { pages: std::array::from_fn(|_| Some(Box::new([0; FILL_BUILDER_OWNER_PAGE_BYTES]))) }
    }

    fn retire_one(&mut self) -> bool {
        let Some(page) = self.pages.iter_mut().find(|page| page.is_some()) else { return false };
        page.take();
        true
    }

    fn terminal_owners_empty(&self) -> bool {
        self.pages.iter().all(Option::is_none)
    }
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
                if fill.collection_backings.pages.get(self.index).is_some_and(Option::is_some) {
                    self.index += 1;
                    return FillOwnerCensusUnit::Credit(FillBuilderOwnerCredit { items: 1, bytes: FILL_BUILDER_OWNER_PAGE_BYTES });
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
    if let Some(value) = fill.seed_object_ids.extract_if(|_| true).next() {
        *current = Some(FillRetiredOwner::String(value));
        return true;
    }
    if fill.seed_object_ids.capacity() != 0 {
        drop(std::mem::take(&mut fill.seed_object_ids));
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

fn take_weight_map_owner(values: &mut BTreeMap<String, f64>, current: &mut Option<FillRetiredOwner>) -> bool {
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
    if let Some((key, body)) = fill.meshes.extract_if(|_, _| true).next() {
        *current = Some(FillRetiredOwner::Mesh(key, body));
        return true;
    }
    if fill.meshes.capacity() != 0 {
        drop(std::mem::take(&mut fill.meshes));
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
            19 => fill.collection_backings.retire_one(),
            20 => {
                if !fill.spatial_index.retire_one_owner() {
                    true
                } else {
                    false
                }
            }
            21 => false,
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
            && self.seed_object_ids.capacity() == 0
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
            && self.meshes.capacity() == 0
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
            && self.collection_backings.terminal_owners_empty()
            && preview_terminal_owners_empty(&self.preview)
    }

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
            collection_backings: FillBuilderCollectionBackings::new(),
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
        StepOutcome::PreviewReady(Vec::new())
    }

    fn complete(&self) -> StepOutcome {
        StepOutcome::Complete(CommitCandidate { state: Vec::new(), output: Vec::new() })
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
    fn retained_owner_census_uses_fixed_backing_pages_not_pair_size_heuristics() {
        let builder = empty_builder();
        let mut cursor = FillBuilderOwnerCensusCursor::default();
        assert_eq!(cursor.step(&builder, usize::MAX, usize::MAX), FillBuilderOwnerCensusStep::Pending);
        for page in 0..FILL_BUILDER_STD_COLLECTIONS {
            let before = cursor.credit;
            assert_eq!(cursor.step(&builder, usize::MAX, usize::MAX), FillBuilderOwnerCensusStep::Pending);
            assert_eq!(cursor.credit.items - before.items, 1, "fixed backing page {page} has one exact owner");
            assert_eq!(cursor.credit.bytes - before.bytes, FILL_BUILDER_OWNER_PAGE_BYTES, "fixed backing page {page} owns one exact admitted page");
        }
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
