//! 🪣️ Puzzle 3d play app — the fill planner (`FillBuilder`: base scene, the growing plan sequence and its
//! appended objects/attractions, the placed collision entries the next step tests against, the per-run RNG
//! stream) and the fill tool run's two jobs, [`FillRunJob`] and [`FillRevalidateJob`], which the framework
//! tool run ledger owns and steps (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS `📋️tool-run-contract.md`
//! §2.7, §3.7).

use crate::editor::puzzle3d::modes::edit::windows::main::VORTEX_MARKER_MESH_KIND;
use crate::editor::puzzle3d::precompute::brush::{
    brush_fill_candidate_at, brush_object_id, brush_preview_from_candidate, fill_rng, resolve_object_kind_mesh_url, resolve_placed_object_mesh_url, vortex_world_from_object, AttractionVortexContext, BrushCatalogView,
    BrushFillVortexTarget, BrushFixtureView, TargetVortexWorld,
};
use crate::editor::puzzle3d::precompute::geometry::{
    pose_isometry, world_bounds, world_volumes_contain_aabb, CollisionAabb, CollisionBody, CollisionIndexMutation, CollisionStepContext, CollisionIndexOwner, CollisionIndexRejectedOwner,
    CollisionIndexRemoval, CollisionMutationStep, CollisionPenetrationState, CollisionQueryCursor, CollisionQueryStep, CollisionSpatialIndex, CollisionStepResult, FixedOwnerMap, FixedOwnerMapInsert, FixedOwnerSet, FixedOwnerSetInsert, FixedOwnerVec,
    Pose3d, DOCUMENT_ATTRACTION_SLOTS, DOCUMENT_CANDIDATE_SLOTS, DOCUMENT_KIND_SLOTS, DOCUMENT_OBJECT_SLOTS, DOCUMENT_VOLUME_SLOTS, DOCUMENT_VORTEX_SLOTS,
};
use crate::editor::puzzle3d::{empty_fixture, puzzle3d_next_object_label, Puzzle3dFixture, Puzzle3dObject};
use crate::standards::v1::subsets::any::schema::{
    puzzle3d_vortex_full_id, AttractionProps, BrushCompatibleCandidate, BrushHostRules, BrushPlacePayload, BrushPreviewState, CableKindCatalog, FixtureObject, FillRunCheckpoint, FillRunCounter, FillRunReason, FillRunStage, KindCompatEntry,
    KindCatalogBundle, ObjectKind, SceneConfig, VortexKindCatalog, VortexProps, WorldVolumeProps,
};
use semio_framework_job::{CommitCandidate, Generation, InteractiveJob, JobFault, JobPayloadStream, Operation, OperationId, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_tool_run::{
    ToolRunCounter, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStep, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTick, ToolRunTickWriter, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_PROVISIONAL_OPS_MAX,
    TOOL_RUN_REASON_CONFLICT, TOOL_RUN_REASON_PROVISIONAL_CAP,
};
use std::collections::HashMap;
use std::sync::Arc;

/// 🛑️ Why a fill plan stopped below its requested count — the only reasons a plan may stall with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FillStall {
    /// 🕳️ The round enumerated no open vortex at all — every vortex is blocked or zero-weighted.
    NoOpenVortex,
    /// 🧩️ Open vortices existed, but no catalog kind is compatible with any of them.
    NoCompatibleKind,
    /// 🚧️ Candidates were built and every one of them was refused — the artifact has no room left.
    NoFreePlacement,
    /// 📄️ A fixed artifact page refused an owner: a visible stall, never a fault.
    ArtifactCapacity,
}

impl FillStall {
    pub(crate) const ALL: [Self; 4] = [Self::NoOpenVortex, Self::NoCompatibleKind, Self::NoFreePlacement, Self::ArtifactCapacity];

    /// 🏷️ The run reason the stall's `warning` step names.
    pub(crate) const fn reason(self) -> FillRunReason {
        match self {
            Self::NoOpenVortex => FillRunReason::NoOpenVortex,
            Self::NoCompatibleKind => FillRunReason::NoCompatibleKind,
            Self::NoFreePlacement => FillRunReason::NoFreePlacement,
            Self::ArtifactCapacity => FillRunReason::ArtifactCapacity,
        }
    }
}

/// 🏁️ How a fill plan ended: it reached its requested count, or it stalled with a declared reason. A plan cannot complete
/// without one, because [`FillJobStage::Complete`] carries it — there is no silent end to construct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FillPlanEnd {
    Reached,
    Stalled(FillStall),
}

/// 🧊️ The planner's own spatial index answered for another owner: an invariant break the planner faults on.
struct StaleSpatialIndex;

/// 🧭️ What one planner transition needs from its caller: the collision budget seam plus identity,
/// stage and fault payloads. The job context forwards everything; the fill run job
/// hands the planner a transition context whose fuel is the run's own candidate budget instead.
pub(crate) trait FillStepContext: CollisionStepContext {
    fn operation(&self) -> OperationId;
    fn generation(&self) -> Generation;
    fn set_stage(&mut self, label: &'static str);
    fn fault_payload(&mut self, bytes: &[u8]) -> RetainedJobPayload;
}

impl FillStepContext for StepContext<'_> {
    fn operation(&self) -> OperationId {
        StepContext::operation(self)
    }

    fn generation(&self) -> Generation {
        StepContext::generation(self)
    }

    fn set_stage(&mut self, label: &'static str) {
        StepContext::set_stage(self, label);
    }

    fn fault_payload(&mut self, bytes: &[u8]) -> RetainedJobPayload {
        self.payload_from_bytes(JobPayloadStream::Fault, bytes).unwrap_or_else(|rejected| {
            drop(rejected.into_source());
            RetainedJobPayload::empty(JobPayloadStream::Fault)
        })
    }
}

/// 🛰️ One observation the planner reports to a fill run job, in the order it happened.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FillRunEvent {
    Constructed { mesh_url: String, origin: [f64; 3], orientation: [f64; 4], scale: f32 },
    Refused(FillRunReason),
    /// 🎯️ A vortex whose every compatible candidate was refused: it is marked and never picked again.
    VortexMarked { position: [f64; 3] },
    Accepted,
    Abandoned,
    Discarded,
}

/// 📏️ Uniform trace scale of an object scale value: a number, the first component of a vector, else 1.
fn fill_run_scale(scale: &Option<dsl::DslValue>) -> f32 {
    scale.as_ref().and_then(|value| value.as_f64().or_else(|| value.as_array().and_then(|values| values.first()).and_then(dsl::DslValue::as_f64))).map_or(1.0, |value| value as f32)
}

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

fn fenwick_prefix(tree: &[f64], count: usize) -> f64 {
    let mut cursor = count.min(tree.len().saturating_sub(1));
    let mut total = 0.0;
    while cursor > 0 {
        total += tree[cursor];
        cursor &= cursor - 1;
    }
    total
}

/// ➕️ Appends one weight to a Fenwick tree in O(log n): the new node covers `(i - lowbit(i), i]`.
fn fenwick_push(tree: &mut Vec<f64>, weight: f64) {
    let index = tree.len();
    let covered = weight + fenwick_prefix(tree, index - 1) - fenwick_prefix(tree, index - (index & index.wrapping_neg()));
    tree.push(covered);
}

/// 🎲️ One index drawn with probability proportional to its weight, the weights left as they are.
fn weighted_draw(weights: &[f64], tree: &[f64], rng_state: &mut u32) -> Option<usize> {
    let total = fenwick_total(tree);
    if total <= 0.0 {
        return None;
    }
    let index = fenwick_pick(tree, (fill_rng(rng_state) * total).max(f64::MIN_POSITIVE));
    if weights.get(index).is_some_and(|weight| *weight > 0.0) {
        return Some(index);
    }
    weights.iter().position(|weight| *weight > 0.0)
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
    RetractTail,
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
    Complete(FillPlanEnd),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TargetPreparePhase {
    Reset,
    Blocked,
    Enumerate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CandidatePreparePhase {
    Reset,
    Enumerate,
    Order,
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
    AppendTargets,
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

#[derive(Debug)]
pub(crate) struct FixedFixtureOwner {
    pub(crate) objects: FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS>,
    pub(crate) attractions: FixedOwnerVec<AttractionProps, DOCUMENT_ATTRACTION_SLOTS>,
    pub(crate) target_volumes: FixedOwnerVec<WorldVolumeProps, DOCUMENT_VOLUME_SLOTS>,
}

impl FixedFixtureOwner {
    fn new() -> Self {
        Self { objects: FixedOwnerVec::new(), attractions: FixedOwnerVec::new(), target_volumes: FixedOwnerVec::new() }
    }

}

#[derive(Debug)]
struct FixedCatalogOwner {
    objects: FixedOwnerVec<ObjectKind, DOCUMENT_KIND_SLOTS>,
    vortices: FixedOwnerVec<VortexKindCatalog, DOCUMENT_KIND_SLOTS>,
    cables: FixedOwnerVec<CableKindCatalog, DOCUMENT_KIND_SLOTS>,
}

impl FixedCatalogOwner {
    fn new() -> Self {
        Self { objects: FixedOwnerVec::new(), vortices: FixedOwnerVec::new(), cables: FixedOwnerVec::new() }
    }
}

impl BrushCatalogView for FixedCatalogOwner {
    fn objects(&self) -> &[ObjectKind] {
        self.objects.as_slice()
    }

    fn vortices(&self) -> &[VortexKindCatalog] {
        self.vortices.as_slice()
    }

    fn cables(&self) -> &[CableKindCatalog] {
        self.cables.as_slice()
    }
}

struct FillFixtureView<'a> {
    base: &'a FixedFixtureOwner,
    appended: &'a [FixtureObject],
}

impl BrushFixtureView for FillFixtureView<'_> {
    fn object_count(&self) -> usize {
        self.base.objects.len() + self.appended.len()
    }

    fn find_object_kind(&self, kind_id: &str) -> Option<&FixtureObject> {
        self.base.objects.iter().chain(self.appended).find(|object| object.object_kind.as_deref() == Some(kind_id))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PreparationCapacityBranch {
    FixtureObjects,
    FixtureAttractions,
    FixtureTargetVolumes,
    Meshes,
    CatalogObjects,
    CatalogVortices,
    CatalogCables,
    KindCompatibility,
    ObjectWeights,
    VortexWeights,
}

impl PreparationCapacityBranch {
    fn label(self) -> &'static str {
        match self {
            Self::FixtureObjects => "fixture-objects",
            Self::FixtureAttractions => "fixture-attractions",
            Self::FixtureTargetVolumes => "fixture-target-volumes",
            Self::Meshes => "meshes",
            Self::CatalogObjects => "catalog-objects",
            Self::CatalogVortices => "catalog-vortices",
            Self::CatalogCables => "catalog-cables",
            Self::KindCompatibility => "kind-compatibility",
            Self::ObjectWeights => "object-weights",
            Self::VortexWeights => "vortex-weights",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PreparationCapacityRefusal {
    branch: PreparationCapacityBranch,
    omitted_index: usize,
    published: bool,
}

impl PreparationCapacityRefusal {
    /// 🏷️ `preparation-capacity:<branch>:<limit>` — the refused document-scale branch and the exact
    /// capacity its fixed page declares, so the published diagnostic never hides which limit bound.
    fn diagnostic(self) -> String {
        format!("preparation-capacity:{}:{}", self.branch.label(), self.omitted_index)
    }
}

/// 🚧️ Preflights every document-scale owner the preparation stages install into, each against the
/// capacity its own fixed page declares — objects and attractions at `DOCUMENT_OBJECT_SLOTS` /
/// `DOCUMENT_ATTRACTION_SLOTS` (the scene plus the room a plan can still claim), catalogs, compatibility
/// rows, mesh urls and kind weights at `DOCUMENT_KIND_SLOTS`. The refusal carries that capacity so
/// the published diagnostic names the branch and the limit it exceeded.
fn preparation_capacity_refusal(roots: &FillPreparationRoots) -> Option<PreparationCapacityRefusal> {
    let catalogs = roots.scene.kind_catalogs.as_ref();
    let (branch, capacity) = [
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
    .find_map(|(branch, len, capacity)| (len > capacity).then_some((branch, capacity)))?;
    Some(PreparationCapacityRefusal { branch, omitted_index: capacity, published: false })
}

pub(crate) struct FillBuilder {
    pub(crate) base: FixedFixtureOwner,
    preparation_roots: Option<FillPreparationRoots>,
    preparation_cursor: usize,
    preparation_inner_cursor: usize,
    preparation_spatial: Option<CollisionIndexMutation>,
    /// 🎚️ Live withdrawal of ONE unapplied placement during a weight-only replan — the same
    /// resumable index mutation shape as `preparation_spatial`, so the discard never scans.
    tail_removal: Option<CollisionIndexRemoval>,
    preparation_capacity_refusal: Option<PreparationCapacityRefusal>,
    pub(crate) applied_count: usize,
    pub(crate) sequence: Vec<BrushPlacePayload>,
    pub(crate) appended_objects: Vec<FixtureObject>,
    pub(crate) appended_attractions: Vec<AttractionProps>,
    pub(crate) placed: Vec<PlacedCollisionEntry>,
    placed_lookup: FixedOwnerMap<String, usize, DOCUMENT_OBJECT_SLOTS>,
    /// 🗄️ Keeps the bookkeeping page width: no stage installs a per-target-vortex candidate list
    /// yet, so this owner exists for the census/retirement walk only and never reaches document
    /// scale. Wiring a real cache means widening it to `DOCUMENT_VORTEX_SLOTS` in the same move.
    pub(crate) candidate_cache: FixedOwnerMap<String, Vec<BrushCompatibleCandidate>>,
    pub(crate) rng_state: u32,
    pub(crate) max_count: usize,
    /// 🪜️ Where a running tail discard stops: the applied prefix for a weight replan, the newly
    /// requested count for a lowered ask.
    tail_floor: usize,
    /// 🔎️ Whether the plan ever built a pose, which tells a bare document apart from one whose every
    /// candidate was refused.
    ever_constructed: bool,
    pub(crate) operation: Operation,
    pub(crate) stage: FillJobStage,
    catalogs: FixedCatalogOwner,
    weights: RetainedBrushKindWeights,
    kind_compatibility: FixedOwnerVec<KindCompatEntry, DOCUMENT_KIND_SLOTS>,
    host_rules: BrushHostRules,
    contact_tolerance: f64,
    meshes: FixedOwnerMap<String, CollisionBody, DOCUMENT_KIND_SLOTS>,
    spatial_index: CollisionSpatialIndex,
    /// 🌀️ The pool of free vortices, drawn from by `target_weights` (a vortex kind's distribution weight, zero once the
    /// vortex is consumed by a placement or marked) through the Fenwick tree `target_tree`.
    targets: Vec<BrushFillVortexTarget>,
    target_weights: Vec<f64>,
    target_tree: Vec<f64>,
    targets_ready: bool,
    current_target_index: Option<usize>,
    target_prepare_phase: TargetPreparePhase,
    blocked_vortex_ids: FixedOwnerSet<String, DOCUMENT_VORTEX_SLOTS>,
    target_attraction_cursor: usize,
    target_object_cursor: usize,
    target_vortex_cursor: usize,
    current_target: Option<BrushFillVortexTarget>,
    candidates: Vec<BrushCompatibleCandidate>,
    candidate_cursor: usize,
    candidate_prepare_phase: CandidatePreparePhase,
    candidate_kind_cursor: usize,
    candidate_vortex_cursor: usize,
    candidate_prepare_cursor: usize,
    candidate_seen: FixedOwnerSet<String, DOCUMENT_CANDIDATE_SLOTS>,
    candidate_raw: Vec<BrushCompatibleCandidate>,
    candidate_weights: Vec<f64>,
    candidate_tree: Vec<f64>,
    candidate_remaining: usize,
    current_preview: Option<BrushPreviewState>,
    broad_phase_query: Option<CollisionQueryCursor>,
    broad_phase_cursor: usize,
    broad_phase_bounds: Option<CollisionAabb>,
    collision: Option<CollisionPenetrationState>,
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
    pub(crate) transition_count: u64,
    rejected_count: u64,
    close_field: u8,
    close_current: Option<FillRetiredOwner>,
    closing: bool,
    /// 🛰️ Observations for a fill run job; `None` for the retained session lane, which never reads them.
    run_events: Option<Vec<FillRunEvent>>,
    /// 👻️ A constructed candidate still owes the run its verdict.
    run_candidate_live: bool,
}

pub(crate) const FILL_BUILDER_OWNER_PAGE_BYTES: usize = 16 * 1024;

struct RetainedBrushKindWeights {
    object_weights: FixedOwnerMap<String, f64, DOCUMENT_KIND_SLOTS>,
    vortex_weights: FixedOwnerMap<String, f64, DOCUMENT_KIND_SLOTS>,
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
    let object_kind = target.object_kind.as_deref().unwrap_or("");
    let vortex_kind = target.vortex_kind.as_deref().unwrap_or("");
    retained_joint_weight(weights, object_kind, vortex_kind)
}

fn retained_candidate_suggestion_weight(candidate: &BrushCompatibleCandidate, weights: &RetainedBrushKindWeights, catalogs: &impl BrushCatalogView) -> f64 {
    let vortex_kind = catalogs.objects().iter().find(|kind| kind.id == candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|template| template.vortex_kind.as_deref()).unwrap_or("");
    retained_joint_weight(weights, &candidate.object_kind_id, vortex_kind)
}

fn retained_joint_weight(weights: &RetainedBrushKindWeights, object_kind_id: &str, vortex_kind_id: &str) -> f64 {
    let sep = crate::editor::puzzle3d::PUZZLE3D_JOINT_WEIGHT_SEP;
    let composite = format!("{object_kind_id}{sep}{vortex_kind_id}");
    if let Some(weight) = weights.vortex_weights.get(&composite) {
        return *weight;
    }
    weights.object_value(object_kind_id) * weights.vortex_value(vortex_kind_id)
}

enum FillRetiredOwner {
    String(String),
    FixtureObject(FixtureObject),
    Attraction(AttractionProps),
    WorldVolume(WorldVolumeProps),
    Payload(BrushPlacePayload),
    Placed(PlacedCollisionEntry),
    Candidate(BrushCompatibleCandidate),
    Target(BrushFillVortexTarget),
    PreviewState(BrushPreviewState),
    ObjectKind(ObjectKind),
    VortexKind(VortexKindCatalog),
    CableKind(CableKindCatalog),
    Compat(KindCompatEntry),
    CandidateCache(String, Vec<BrushCompatibleCandidate>),
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

fn retire_world_volume(value: &mut WorldVolumeProps) -> bool {
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

fn retire_object_kind(value: &mut ObjectKind) -> bool {
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

fn take_fixture_owner(value: &mut FixedFixtureOwner, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = value.objects.pop() {
        *current = Some(FillRetiredOwner::FixtureObject(value));
        return true;
    }
    if value.objects.retire_backing() {
        return true;
    }
    if let Some(value) = value.attractions.pop() {
        *current = Some(FillRetiredOwner::Attraction(value));
        return true;
    }
    if value.attractions.retire_backing() {
        return true;
    }
    if let Some(value) = value.target_volumes.pop() {
        *current = Some(FillRetiredOwner::WorldVolume(value));
        return true;
    }
    value.target_volumes.retire_backing()
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
    false
}

fn take_catalog_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.catalogs.objects.pop() {
        *current = Some(FillRetiredOwner::ObjectKind(value));
        return true;
    }
    if fill.catalogs.objects.retire_backing() {
        return true;
    }
    if let Some(value) = fill.catalogs.vortices.pop() {
        *current = Some(FillRetiredOwner::VortexKind(value));
        return true;
    }
    if fill.catalogs.vortices.retire_backing() {
        return true;
    }
    if let Some(value) = fill.catalogs.cables.pop() {
        *current = Some(FillRetiredOwner::CableKind(value));
        return true;
    }
    if fill.catalogs.cables.retire_backing() {
        return true;
    }
    if let Some(value) = fill.kind_compatibility.pop() {
        *current = Some(FillRetiredOwner::Compat(value));
        return true;
    }
    fill.kind_compatibility.retire_backing()
}

fn take_weight_map_owner<const N: usize>(values: &mut FixedOwnerMap<String, f64, N>, current: &mut Option<FillRetiredOwner>) -> bool {
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
    false
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
    release_vec_backing(&mut fill.candidate_raw)
}

fn take_candidate_order_owner(fill: &mut FillBuilder) -> bool {
    for values in [&mut fill.candidate_weights, &mut fill.candidate_tree] {
        if values.pop().is_some() || release_vec_backing(values) {
            return true;
        }
    }
    false
}

fn retire_fixed_collection_backing(fill: &mut FillBuilder) -> bool {
    fill.placed_lookup.retire_backing()
        || fill.candidate_cache.retire_backing()
        || fill.weights.object_weights.retire_backing()
        || fill.weights.vortex_weights.retire_backing()
        || fill.meshes.retire_backing()
        || fill.blocked_vortex_ids.retire_backing()
        || fill.candidate_seen.retire_backing()
}

fn take_target_weight_owner(fill: &mut FillBuilder) -> bool {
    for values in [&mut fill.target_weights, &mut fill.target_tree] {
        if values.pop().is_some() || release_vec_backing(values) {
            return true;
        }
    }
    false
}

fn fixture_terminal_owners_empty(value: &FixedFixtureOwner) -> bool {
    value.objects.terminal_owners_empty() && value.attractions.terminal_owners_empty() && value.target_volumes.terminal_owners_empty()
}

impl FillBuilder {
    #[cfg(test)]
    fn preparation_refusal_owner_for_test(&self) -> Option<(&'static str, usize, String, Option<f64>)> {
        let refusal = self.preparation_capacity_refusal?;
        let roots = self.preparation_roots.as_ref()?;
        let (owner, weight) = match refusal.branch {
            PreparationCapacityBranch::FixtureObjects => (roots.scene.fixture.objects.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::FixtureAttractions => (roots.scene.fixture.attractions.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::FixtureTargetVolumes => (roots.scene.fixture.target_volumes.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::Meshes => (roots.meshes.keys().nth(refusal.omitted_index)?.clone(), None),
            PreparationCapacityBranch::CatalogObjects => (roots.scene.kind_catalogs.as_ref()?.objects.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::CatalogVortices => (roots.scene.kind_catalogs.as_ref()?.vortices.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::CatalogCables => (roots.scene.kind_catalogs.as_ref()?.cables.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::KindCompatibility => (roots.scene.kind_compatibility.get(refusal.omitted_index)?.source.clone(), None),
            PreparationCapacityBranch::ObjectWeights => {
                let (key, value) = roots.scene.weights.object_weights.iter().nth(refusal.omitted_index)?;
                (key.clone(), Some(*value))
            }
            PreparationCapacityBranch::VortexWeights => {
                let (key, value) = roots.scene.weights.vortex_weights.iter().nth(refusal.omitted_index)?;
                (key.clone(), Some(*value))
            }
        };
        Some((refusal.branch.label(), refusal.omitted_index, owner, weight))
    }

    /// 🔎️ The FIRST retained-owner clause this builder still fails, BY NAME. A close ladder that wedges
    /// has to be able to say which owner nothing drains, instead of only that the builder is "not empty":
    /// the close walk's terminal arm returns `false` forever in that case, so the whole teardown spins
    /// without progress (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B42).
    pub(crate) fn terminal_owner_debt(&self) -> Option<&'static str> {
        [
            ("fixture_terminal_owners_empty base", fixture_terminal_owners_empty(&self.base)),
            ("sequence.is_empty", self.sequence.is_empty()),
            ("sequence.capacity 0", self.sequence.capacity() == 0),
            ("appended_objects.is_empty", self.appended_objects.is_empty()),
            ("appended_objects.capacity 0", self.appended_objects.capacity() == 0),
            ("appended_attractions.is_empty", self.appended_attractions.is_empty()),
            ("appended_attractions.capacity 0", self.appended_attractions.capacity() == 0),
            ("placed.is_empty", self.placed.is_empty()),
            ("placed.capacity 0", self.placed.capacity() == 0),
            ("placed_lookup.is_empty", self.placed_lookup.is_empty()),
            ("candidate_cache.is_empty", self.candidate_cache.is_empty()),
            ("catalogs.objects.terminal_owners_empty", self.catalogs.objects.terminal_owners_empty()),
            ("catalogs.vortices.terminal_owners_empty", self.catalogs.vortices.terminal_owners_empty()),
            ("catalogs.cables.terminal_owners_empty", self.catalogs.cables.terminal_owners_empty()),
            ("weights.object_weights.is_empty", self.weights.object_weights.is_empty()),
            ("weights.vortex_weights.is_empty", self.weights.vortex_weights.is_empty()),
            ("kind_compatibility.terminal_owners_empty", self.kind_compatibility.terminal_owners_empty()),
            ("meshes.is_empty", self.meshes.is_empty()),
            ("spatial_index.terminal_owners_empty", self.spatial_index.terminal_owners_empty()),
            ("targets.is_empty", self.targets.is_empty()),
            ("targets.capacity 0", self.targets.capacity() == 0),
            ("blocked_vortex_ids.is_empty", self.blocked_vortex_ids.is_empty()),
            ("target_weights.is_empty", self.target_weights.is_empty()),
            ("target_weights.capacity 0", self.target_weights.capacity() == 0),
            ("target_tree.is_empty", self.target_tree.is_empty()),
            ("target_tree.capacity 0", self.target_tree.capacity() == 0),
            ("current_target.is_none", self.current_target.is_none()),
            ("candidates.is_empty", self.candidates.is_empty()),
            ("candidates.capacity 0", self.candidates.capacity() == 0),
            ("candidate_seen.is_empty", self.candidate_seen.is_empty()),
            ("candidate_raw.is_empty", self.candidate_raw.is_empty()),
            ("candidate_raw.capacity 0", self.candidate_raw.capacity() == 0),
            ("candidate_weights.is_empty", self.candidate_weights.is_empty()),
            ("candidate_weights.capacity 0", self.candidate_weights.capacity() == 0),
            ("candidate_tree.is_empty", self.candidate_tree.is_empty()),
            ("candidate_tree.capacity 0", self.candidate_tree.capacity() == 0),
            ("current_preview.is_none", self.current_preview.is_none()),
            ("broad_phase_query.as_ref .is_none_or CollisionQueryCursor terminal_owners_empty", self.broad_phase_query.as_ref().is_none_or(CollisionQueryCursor::terminal_owners_empty)),
            ("collision.is_none", self.collision.is_none()),
            ("pending_payload.is_none", self.pending_payload.is_none()),
            ("pending_object.is_none", self.pending_object.is_none()),
            ("pending_attraction.is_none", self.pending_attraction.is_none()),
            ("pending_spatial.is_none", self.pending_spatial.is_none()),
            ("preparation_spatial.is_none", self.preparation_spatial.is_none()),
            ("tail_removal.is_none", self.tail_removal.is_none()),
            ("preparation_roots.is_none", self.preparation_roots.is_none()),
            ("preparation_capacity_refusal.is_none", self.preparation_capacity_refusal.is_none()),
            ("last_rejection.is_none", self.last_rejection.is_none()),
            ("fixed_rejection.is_none", self.fixed_rejection.is_none()),
            ("collection_over_capacity", !self.collection_over_capacity),
            ("placed_lookup.terminal_owners_empty", self.placed_lookup.terminal_owners_empty()),
            ("candidate_cache.terminal_owners_empty", self.candidate_cache.terminal_owners_empty()),
            ("weights.object_weights.terminal_owners_empty", self.weights.object_weights.terminal_owners_empty()),
            ("weights.vortex_weights.terminal_owners_empty", self.weights.vortex_weights.terminal_owners_empty()),
            ("meshes.terminal_owners_empty", self.meshes.terminal_owners_empty()),
            ("blocked_vortex_ids.terminal_owners_empty", self.blocked_vortex_ids.terminal_owners_empty()),
            ("candidate_seen.terminal_owners_empty", self.candidate_seen.terminal_owners_empty()),
            ("run_events.is_none", self.run_events.is_none()),
        ]
        .into_iter()
        .find_map(|(owner, empty)| (!empty).then_some(owner))
    }

    fn terminal_owners_empty(&self) -> bool {
        self.terminal_owner_debt().is_none()
    }

    /// 🎯️ Seeds a planner that targets exactly what was requested — there is no hidden ceiling it
    /// races toward in the background, so every placement the user sees was asked for.
    pub(crate) fn begin_preparation(roots: FillPreparationRoots, operation: Operation, requested_count: usize) -> Self {
        let seed = roots.scene.seed;
        let preparation_capacity_refusal = preparation_capacity_refusal(&roots);
        Self {
            base: FixedFixtureOwner::new(),
            preparation_roots: Some(roots),
            preparation_cursor: 0,
            preparation_inner_cursor: 0,
            preparation_spatial: None,
            tail_removal: None,
            preparation_capacity_refusal,
            applied_count: 0,
            sequence: Vec::new(),
            appended_objects: Vec::new(),
            appended_attractions: Vec::new(),
            placed: Vec::new(),
            placed_lookup: FixedOwnerMap::new(),
            candidate_cache: FixedOwnerMap::new(),
            rng_state: seed,
            max_count: requested_count,
            tail_floor: 0,
            ever_constructed: false,
            operation,
            stage: FillJobStage::PrepareFixture,
            catalogs: FixedCatalogOwner::new(),
            weights: RetainedBrushKindWeights::new(),
            kind_compatibility: FixedOwnerVec::new(),
            host_rules: BrushHostRules::default(),
            contact_tolerance: 0.0,
            meshes: FixedOwnerMap::new(),
            spatial_index: CollisionSpatialIndex::new(8.0),
            targets: Vec::new(),
            target_weights: Vec::new(),
            target_tree: vec![0.0],
            targets_ready: false,
            current_target_index: None,
            target_prepare_phase: TargetPreparePhase::Reset,
            blocked_vortex_ids: FixedOwnerSet::new(),
            target_attraction_cursor: 0,
            target_object_cursor: 0,
            target_vortex_cursor: 0,
            current_target: None,
            candidates: Vec::new(),
            candidate_cursor: 0,
            candidate_prepare_phase: CandidatePreparePhase::Reset,
            candidate_kind_cursor: 0,
            candidate_vortex_cursor: 0,
            candidate_prepare_cursor: 0,
            candidate_seen: FixedOwnerSet::new(),
            candidate_raw: Vec::new(),
            candidate_weights: Vec::new(),
            candidate_tree: vec![0.0],
            candidate_remaining: 0,
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
            collection_over_capacity: false,
            transition_count: 0,
            rejected_count: 0,
            close_field: 0,
            close_current: None,
            closing: false,
            run_events: None,
            run_candidate_live: false,
        }
    }

    fn retire_one_close_owner(&mut self) -> bool {
        if let Some(current) = self.close_current.as_mut() {
            if retire_retained_owner(current) {
                self.close_current = None;
            }
            return false;
        }
        let mut current = None;
        let retired = match self.close_field {
            0 => take_fixture_owner(&mut self.base, &mut current),
            1 => false,
            2 => take_sequence_owner(self, &mut current),
            3 => take_lookup_owner(self, &mut current),
            4 => take_catalog_owner(self, &mut current),
            5 => take_weight_mesh_owner(self, &mut current),
            6 => take_target_owner(self, &mut current),
            7 => take_target_weight_owner(self),
            8 => take_candidate_owner(self, &mut current),
            9 => take_candidate_order_owner(self),
            10 => match self.broad_phase_query.as_mut() {
                Some(query) => {
                    if query.retire_one_owner() {
                        self.broad_phase_query.take();
                    }
                    true
                }
                None => false,
            },
            11 => self.pending_payload.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::Payload(value));
                true
            }),
            12 => self.pending_object.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::FixtureObject(value));
                true
            }),
            13 => self.pending_attraction.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::Attraction(value));
                true
            }),
            14 => self.current_target.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::Target(value));
                true
            }),
            15 => self.current_preview.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::PreviewState(value));
                true
            }),
            16 => self.last_rejection.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::String(value));
                true
            }),
            17 => self.collision.take().is_some(),
            18 => match self.fixed_rejection.as_mut() {
                Some(rejected) => {
                    if retire_retained_owner(rejected) {
                        self.fixed_rejection.take();
                    }
                    true
                }
                None => false,
            },
            19 => retire_fixed_collection_backing(self),
            20 => !self.spatial_index.retire_one_owner(),
            21 if self.collection_over_capacity => {
                self.collection_over_capacity = false;
                true
            }
            21 => false,
            22 => match self.preparation_spatial.as_mut() {
                Some(mutation) => {
                    if mutation.retire_one_owner() {
                        self.preparation_spatial.take();
                    }
                    true
                }
                None => false,
            },
            23 => match self.pending_spatial.as_mut() {
                Some(mutation) => {
                    if mutation.retire_one_owner() {
                        self.pending_spatial.take();
                    }
                    true
                }
                None => false,
            },
            24 => match self.tail_removal.as_mut() {
                Some(removal) => {
                    if removal.retire_one_owner() {
                        self.tail_removal.take();
                    }
                    true
                }
                None => false,
            },
            25 if self.preparation_roots.take().is_some() => true,
            25 => false,
            26 if self.preparation_capacity_refusal.take().is_some() => true,
            26 => false,
            27 if self.run_events.take().is_some() => true,
            27 => false,
            _ => return self.terminal_owners_empty() && self.close_current.is_none(),
        };
        self.close_current = current;
        if !retired {
            self.close_field = self.close_field.saturating_add(1);
        }
        false
    }

    fn collision_owner(&self) -> CollisionIndexOwner {
        CollisionIndexOwner { operation: self.operation.operation.0, generation: self.operation.generation.0 }
    }

    fn fixture_object(&self, index: usize) -> Option<&FixtureObject> {
        self.base.objects.get(index).or_else(|| self.appended_objects.get(index.saturating_sub(self.base.objects.len())))
    }

    fn fixture_attraction(&self, index: usize) -> Option<&AttractionProps> {
        self.base.attractions.get(index).or_else(|| self.appended_attractions.get(index.saturating_sub(self.base.attractions.len())))
    }

    fn fixture_view(&self) -> FillFixtureView<'_> {
        FillFixtureView { base: &self.base, appended: &self.appended_objects }
    }

    fn label_catalog_fixture(&self) -> Puzzle3dFixture {
        let mut fixture = empty_fixture();
        if let Some(roots) = self.preparation_roots.as_ref() {
            if let Some(catalogs) = roots.scene.kind_catalogs.as_ref() {
                fixture.meta.kind_catalogs = Some(dsl::ToValue::to_value(catalogs));
            }
        }
        fixture
    }

    fn label_peers(&self) -> Vec<Puzzle3dObject> {
        let mut peers = Vec::new();
        for object in self.base.objects.iter() {
            if let Ok(peer) = dsl::FromValue::from_value(dsl::ToValue::to_value(object)) {
                peers.push(peer);
            }
        }
        for object in self.appended_objects.iter().take(self.appended_objects.len().saturating_sub(1)) {
            if let Ok(peer) = dsl::FromValue::from_value(dsl::ToValue::to_value(object)) {
                peers.push(peer);
            }
        }
        peers
    }

    /// 🏁️ How the plan ended, `None` while it still runs.
    pub(crate) fn end(&self) -> Option<FillPlanEnd> {
        match self.stage {
            FillJobStage::Complete(end) => Some(end),
            _ => None,
        }
    }

    /// 🔁️ Where a round goes after it placed or retracted: complete as reached once the plan holds what was asked for,
    /// else the next vortex draw — from the resident pool, or after rebuilding it when a retraction dropped it.
    fn next_round_stage(&self) -> FillJobStage {
        if self.sequence.len() >= self.max_count {
            FillJobStage::Complete(FillPlanEnd::Reached)
        } else if self.targets_ready {
            FillJobStage::SelectTarget
        } else {
            FillJobStage::PrepareTargets
        }
    }

    /// 🎯️ What the user is asking for right now.
    pub(crate) fn requested_count(&self) -> usize {
        self.max_count
    }

    /// 🎚️ Retargets the live plan without ever rewinding its RNG stream. Raising simply lifts the
    /// ceiling and wakes a planner that had reached it, so the longer plan keeps the shorter one as
    /// its exact prefix. Lowering hands the surplus tail back through the same one-placement-per-turn
    /// [`FillJobStage::RetractTail`] walk that a weight replan uses, stopping at whatever is already
    /// applied to the document.
    pub(crate) fn set_requested_count(&mut self, requested: usize) {
        if requested == self.max_count {
            return;
        }
        let raising = requested > self.max_count;
        self.max_count = requested;
        if raising {
            if matches!(self.stage, FillJobStage::Complete(_)) && self.sequence.len() < self.max_count {
                self.stage = self.next_round_stage();
            }
            return;
        }
        if self.sequence.len() <= self.max_count {
            return;
        }
        self.abandon_run_candidate();
        self.applied_count = self.applied_count.min(self.sequence.len());
        self.tail_floor = self.max_count.max(self.applied_count);
        self.stage = FillJobStage::RetractTail;
    }

    /// ♻️ One unapplied placement per turn: withdraw its spatial owner, drop its lookup and placement
    /// rows, then pop the plan row itself. Reaching the applied prefix rewinds the planner.
    fn discard_tail_one(&mut self) -> Result<(), StaleSpatialIndex> {
        let owner = self.collision_owner();
        if let Some(removal) = self.tail_removal.as_mut() {
            match self.spatial_index.step_removal(removal, owner) {
                CollisionMutationStep::Pending => {}
                CollisionMutationStep::Complete => self.tail_removal = None,
                CollisionMutationStep::Rejected(rejected) => {
                    self.fixed_rejection = Some(FillRetiredOwner::Spatial(rejected));
                    self.collection_over_capacity = true;
                }
                CollisionMutationStep::Stale => return Err(StaleSpatialIndex),
            }
            return Ok(());
        }
        if self.appended_objects.len() <= self.tail_floor {
            self.reset_targets();
            self.reset_candidate();
            self.stage = self.next_round_stage();
            return Ok(());
        }
        let Some(object) = self.appended_objects.pop() else {
            self.reset_targets();
            self.stage = FillJobStage::PrepareTargets;
            return Ok(());
        };
        self.sequence.pop();
        self.appended_attractions.pop();
        self.run_event(FillRunEvent::Discarded);
        if let Some((id, index)) = self.placed_lookup.remove_entry(object.id.as_str()) {
            if index + 1 == self.placed.len() {
                self.placed.pop();
            }
            drop(id);
        }
        self.tail_removal = self.spatial_index.begin_removal(owner, object.id.clone());
        drop(object);
        Ok(())
    }

    pub(crate) fn prepare_one(&mut self) -> Result<(), StaleSpatialIndex> {
        if self.collection_over_capacity {
            self.last_rejection = Some("preparation-capacity".into());
            self.stall(FillStall::ArtifactCapacity);
            return Ok(());
        }
        match self.stage {
            FillJobStage::PrepareFixture => self.prepare_fixture_one(),
            FillJobStage::PrepareCatalogs => self.prepare_catalog_one(),
            FillJobStage::PrepareMeshes => self.prepare_mesh_one(),
            FillJobStage::PrepareEntries => self.prepare_entry_one(),
            FillJobStage::PrepareSpatial => return self.prepare_spatial_one(),
            FillJobStage::PrepareLookup => self.prepare_lookup_one(),
            FillJobStage::PrepareConfiguration => self.prepare_configuration_one(),
            _ => {}
        }
        Ok(())
    }

    fn prepare_fixture_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let fixture = &roots.scene.fixture;
        let value = match self.preparation_inner_cursor {
            0 => fixture.attractions.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.base.attractions.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::Attraction(owner));
                }
            }),
            1 => fixture.objects.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.base.objects.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::FixtureObject(owner));
                }
            }),
            _ => fixture.target_volumes.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.base.target_volumes.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::WorldVolume(owner));
                }
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
            0 => catalogs.and_then(|value| value.objects.get(self.preparation_cursor)).map(|value| {
                if let Err(owner) = self.catalogs.objects.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::ObjectKind(owner));
                }
            }),
            1 => catalogs.and_then(|value| value.vortices.get(self.preparation_cursor)).map(|value| {
                if let Err(owner) = self.catalogs.vortices.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::VortexKind(owner));
                }
            }),
            _ => catalogs.and_then(|value| value.cables.get(self.preparation_cursor)).map(|value| {
                if let Err(owner) = self.catalogs.cables.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::CableKind(owner));
                }
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
        let fixture = FillFixtureView { base: &self.base, appended: &self.appended_objects };
        let Some(mesh_url) = resolve_placed_object_mesh_url(object, &self.catalogs, &fixture) else {
            return;
        };
        if self.meshes.get(&mesh_url).is_none() {
            return;
        }
        self.placed.push(PlacedCollisionEntry { object_id: object.id.clone(), mesh_url, world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) });
    }

    fn prepare_spatial_one(&mut self) -> Result<(), StaleSpatialIndex> {
        let owner = self.collision_owner();
        if let Some(mutation) = self.preparation_spatial.as_mut() {
            match self.spatial_index.step_replacement(mutation, owner) {
                CollisionMutationStep::Pending => {}
                CollisionMutationStep::Complete => {
                    self.preparation_spatial = None;
                    self.preparation_cursor += 1;
                }
                CollisionMutationStep::Rejected(rejected) => {
                    self.fixed_rejection = Some(FillRetiredOwner::Spatial(rejected));
                    self.collection_over_capacity = true;
                }
                CollisionMutationStep::Stale => return Err(StaleSpatialIndex),
            }
            return Ok(());
        }
        let Some(entry) = self.placed.get(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareLookup;
            return Ok(());
        };
        let Some(body) = self.meshes.get(&entry.mesh_url) else {
            self.preparation_cursor += 1;
            return Ok(());
        };
        self.preparation_spatial = Some(self.spatial_index.begin_replacement(owner, entry.object_id.clone(), CollisionAabb::from_body(body, &entry.world)));
        Ok(())
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
            _ => roots.scene.kind_compatibility.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.kind_compatibility.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::Compat(owner));
                }
            }),
        };
        if value.is_some() {
            self.preparation_cursor += 1;
            return;
        }
        self.preparation_cursor = 0;
        self.preparation_inner_cursor += 1;
        if self.preparation_inner_cursor == 3 {
            self.host_rules = roots.scene.host_rules.clone();
            self.contact_tolerance = roots.scene.contact_tolerance;
            self.preparation_roots = None;
            self.preparation_inner_cursor = 0;
            self.stage = FillJobStage::PrepareTargets;
        }
    }
}

//#region 🧵️InteractiveFillJob
impl FillBuilder {
    /// 🌀️ Builds the free vortex pool one bounded unit per transition: forget the previous blocked set, block every vortex
    /// an attraction already occupies, then add every open vortex with its distribution weight.
    fn prepare_targets(&mut self) {
        match self.target_prepare_phase {
            TargetPreparePhase::Reset => {
                if let Some(value) = self.blocked_vortex_ids.pop_first() {
                    drop(value);
                } else {
                    self.target_attraction_cursor = 0;
                    self.target_prepare_phase = TargetPreparePhase::Blocked;
                }
            }
            TargetPreparePhase::Blocked => {
                if let Some((attracting, attracted)) = self.fixture_attraction(self.target_attraction_cursor).map(|attraction| (attraction.attracting.clone(), attraction.attracted.clone())) {
                    for id in [attracting, attracted] {
                        match self.blocked_vortex_ids.try_insert(id) {
                            Ok(FixedOwnerSetInsert::Inserted) => {}
                            Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                            Err(value) => {
                                self.fixed_rejection = Some(FillRetiredOwner::String(value));
                                return;
                            }
                        }
                    }
                    self.target_attraction_cursor += 1;
                } else {
                    self.target_object_cursor = 0;
                    self.target_vortex_cursor = 0;
                    self.target_prepare_phase = TargetPreparePhase::Enumerate;
                }
            }
            TargetPreparePhase::Enumerate => {
                let Some(object) = self.fixture_object(self.target_object_cursor) else {
                    self.targets_ready = true;
                    self.stage = FillJobStage::SelectTarget;
                    return;
                };
                let Some(vortex) = object.vortices.get(self.target_vortex_cursor) else {
                    self.target_object_cursor += 1;
                    self.target_vortex_cursor = 0;
                    return;
                };
                let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                let target = BrushFillVortexTarget { full_id, object_id: object.id.clone(), object_kind: object.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone(), vortex_index: self.target_vortex_cursor };
                self.target_vortex_cursor += 1;
                if !self.blocked_vortex_ids.contains(&target.full_id) {
                    self.push_target(target);
                }
            }
        }
    }

    /// ➕️ Adds one free vortex to the pool; a vortex kind the distribution weighs zero is never drawn, so it is not kept.
    fn push_target(&mut self, target: BrushFillVortexTarget) {
        let weight = retained_fill_vortex_target_weight(&target, &self.weights);
        if weight <= 0.0 {
            return;
        }
        if self.targets.len() >= DOCUMENT_VORTEX_SLOTS {
            self.collection_over_capacity = true;
            return;
        }
        self.targets.push(target);
        self.target_weights.push(weight);
        fenwick_push(&mut self.target_tree, weight);
    }

    /// 🚫️ Takes one vortex out of the draw for good (consumed by a placement, or marked).
    fn retire_target_weight(&mut self, index: usize) {
        if let Some(weight) = self.target_weights.get_mut(index) {
            let removed = std::mem::replace(weight, 0.0);
            fenwick_add(&mut self.target_tree, index, -removed);
        }
    }

    /// 🛑️ Why the pool ran dry before the requested count: no free vortex ever, no vortex with a compatible kind, or every
    /// tested pose collided.
    fn exhausted_targets_stall(&self) -> FillStall {
        if self.targets.is_empty() {
            FillStall::NoOpenVortex
        } else if self.ever_constructed {
            FillStall::NoFreePlacement
        } else {
            FillStall::NoCompatibleKind
        }
    }

    /// 🎲️ Draws one free vortex by its distribution weight; a pool whose every vortex is consumed or marked ends the plan.
    fn select_target(&mut self) {
        let Some(index) = weighted_draw(&self.target_weights, &self.target_tree, &mut self.rng_state) else {
            self.stall(self.exhausted_targets_stall());
            return;
        };
        self.current_target = self.targets.get(index).cloned();
        self.current_target_index = Some(index);
        self.reset_candidate_preparation();
        self.stage = FillJobStage::PrepareCandidates;
    }

    /// 🧩️ Lists the compatible vortices of the drawn vortex and orders them by weighted random draws without replacement,
    /// so testing them in order is the same as drawing a fresh random compatible vortex after every collision.
    fn prepare_candidates(&mut self) {
        let Some(target) = self.current_target.clone() else {
            self.mark_target("missing-target");
            return;
        };
        let target_context = AttractionVortexContext { object_kind: target.object_kind.clone(), vortex_kind: target.vortex_kind.clone() };
        match self.candidate_prepare_phase {
            CandidatePreparePhase::Reset => {
                if let Some(value) = self.candidate_seen.pop_first() {
                    drop(value);
                } else {
                    self.candidate_prepare_phase = CandidatePreparePhase::Enumerate;
                }
            }
            CandidatePreparePhase::Enumerate => {
                let Some(kind) = self.catalogs.objects.get(self.candidate_kind_cursor) else {
                    self.candidate_remaining = self.candidate_raw.len();
                    self.candidate_prepare_phase = CandidatePreparePhase::Order;
                    return;
                };
                if self.candidate_vortex_cursor >= kind.vortices.len() {
                    self.candidate_kind_cursor += 1;
                    self.candidate_vortex_cursor = 0;
                    return;
                }
                let vortex_index = self.candidate_vortex_cursor;
                self.candidate_vortex_cursor += 1;
                let Some((candidate, _)) = brush_fill_candidate_at(&target_context, &self.catalogs, self.kind_compatibility.as_slice(), &self.host_rules, self.candidate_kind_cursor, vortex_index) else { return };
                let weight = retained_candidate_suggestion_weight(&candidate, &self.weights, &self.catalogs);
                if weight <= 0.0 {
                    return;
                }
                let key = format!("{}\u{1}{}", candidate.object_kind_id, candidate.source_vortex_index);
                match self.candidate_seen.try_insert(key) {
                    Ok(FixedOwnerSetInsert::Inserted) => {
                        self.candidate_raw.push(candidate);
                        self.candidate_weights.push(weight);
                        fenwick_push(&mut self.candidate_tree, weight);
                    }
                    Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                    Err(key) => self.fixed_rejection = Some(FillRetiredOwner::String(key)),
                }
            }
            CandidatePreparePhase::Order => {
                if let Some(index) = weighted_pick(&mut self.candidate_weights, &mut self.candidate_tree, self.candidate_remaining, &mut self.rng_state) {
                    self.candidates.push(self.candidate_raw[index].clone());
                    self.candidate_remaining -= 1;
                } else {
                    self.candidate_prepare_phase = CandidatePreparePhase::Finish;
                }
            }
            CandidatePreparePhase::Finish => {
                self.candidate_cursor = 0;
                if self.candidates.is_empty() {
                    self.mark_target("no-compatible-candidate");
                } else {
                    self.stage = FillJobStage::SelectCandidate;
                }
            }
        }
    }

    fn select_candidate(&mut self) {
        if self.candidate_cursor >= self.candidates.len() {
            self.mark_target("candidates-exhausted");
            return;
        }
        self.stage = FillJobStage::ConstructPreview;
    }

    fn construct_preview(&mut self) {
        let Some(target) = &self.current_target else {
            self.mark_target("missing-target");
            return;
        };
        let Some(candidate) = self.candidates.get(self.candidate_cursor) else {
            self.mark_target("missing-candidate");
            return;
        };
        let Some(host) = self.base.objects.iter().chain(&self.appended_objects).find(|object| object.id == target.object_id) else {
            self.mark_target("missing-host");
            return;
        };
        let Some((position, direction)) = vortex_world_from_object(host, target.vortex_index) else {
            self.reject_candidate("invalid-target-pose");
            return;
        };
        let context = AttractionVortexContext { object_kind: target.object_kind.clone(), vortex_kind: target.vortex_kind.clone() };
        let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
        let fixture = self.fixture_view();
        let Some(preview) = brush_preview_from_candidate(&target.full_id, candidate, &context, world, &self.catalogs, &fixture) else {
            self.reject_candidate("preview-unavailable");
            return;
        };
        self.ever_constructed = true;
        if self.run_events.is_some() {
            self.run_candidate_live = true;
            self.run_event(FillRunEvent::Constructed { mesh_url: preview.mesh_url.clone(), origin: preview.origin, orientation: preview.orientation, scale: fill_run_scale(&preview.scale) });
        }
        let Some(body) = self.meshes.get(&preview.mesh_url) else {
            self.reject_candidate("mesh-unavailable");
            return;
        };
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let (min, max) = world_bounds(body, &preview_world);
        if !world_volumes_contain_aabb(self.base.target_volumes.as_slice(), min, max) {
            self.reject_candidate("outside-target-volume");
            return;
        }
        self.current_preview = Some(preview);
        self.last_rejection = None;
        self.stage = FillJobStage::QueryBroadPhase;
    }

    fn query_broad_phase(&mut self) {
        let Some(_target) = &self.current_target else {
            self.mark_target("missing-target");
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
        if self.broad_phase_query.is_none() {
            let world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
            let bounds = CollisionAabb::from_body(body, &world);
            self.broad_phase_bounds = Some(bounds);
            self.broad_phase_query = Some(self.spatial_index.begin_query(self.collision_owner(), bounds));
            self.broad_phase_cursor = 0;
            return;
        }
        let owner = self.collision_owner();
        let query = self.broad_phase_query.as_mut().expect("broad phase query");
        match self.spatial_index.step_query(query, owner) {
            CollisionQueryStep::Pending => return,
            CollisionQueryStep::Stale => {
                self.reject_candidate("stale-spatial-query");
                return;
            }
            CollisionQueryStep::Complete => {}
        }
        self.broad_phase_cursor = 0;
        self.collision = None;
        self.stage = FillJobStage::TestCollision;
    }

    fn test_collision<C: FillStepContext>(&mut self, context: &mut C) -> Option<StepOutcome> {
        let Some(pair_id) = self.broad_phase_query.as_ref().and_then(|query| query.candidate(self.broad_phase_cursor)).cloned() else {
            self.stage = FillJobStage::AcceptCandidate;
            return None;
        };
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
        let collision = self.collision.get_or_insert_with(|| CollisionPenetrationState::new(self.contact_tolerance));
        let result = collision.step(context, preview_body, &preview_world, other, &entry.world);
        match result {
            CollisionStepResult::Pending => {}
            CollisionStepResult::Cancelled => return Some(StepOutcome::Cancelled),
            CollisionStepResult::Complete { depth, .. } if depth > self.contact_tolerance => {
                self.reject_candidate("solid-overlap");
            }
            CollisionStepResult::Complete { .. } => {
                self.broad_phase_cursor += 1;
                self.collision = None;
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
                    scale: preview.scale,
                };
                let Some(kind) = self.catalogs.objects.iter().find(|kind| kind.id == payload.object_kind_id) else {
                    self.reject_candidate("placement-kind-missing");
                    return StepOutcome::Yield;
                };
                if kind.vortices.get(payload.source_vortex_index).is_none() {
                    self.reject_candidate("placement-vortex-missing");
                    return StepOutcome::Yield;
                }
                let fixture = self.fixture_view();
                let Some(mesh_url) = resolve_object_kind_mesh_url(&payload.object_kind_id, &self.catalogs, &fixture) else {
                    self.reject_candidate("placement-mesh-missing");
                    return StepOutcome::Yield;
                };
                let object_id = brush_object_id(&fixture, &payload);
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
                });
                self.pending_payload = Some(payload);
                self.accept_attraction_cursor = 0;
                self.accept_vortex_cursor = 0;
                self.accept_phase = AcceptPhase::CheckAttractions;
                StepOutcome::Yield
            }
            AcceptPhase::CheckAttractions => {
                let Some((pending_attracting, pending_attracted)) = self.pending_attraction.as_ref().map(|pending| (pending.attracting.clone(), pending.attracted.clone())) else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                if let Some(rejected) = self.fixture_attraction(self.accept_attraction_cursor).map(|attraction| attraction.attracting == pending_attracting || attraction.attracted == pending_attracted) {
                    self.accept_attraction_cursor += 1;
                    if rejected {
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
                self.accept_phase = AcceptPhase::BeginSpatial;
                StepOutcome::Yield
            }
            AcceptPhase::BeginSpatial => {
                let Some(object) = self.pending_object.as_ref() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(mesh_url) = object.mesh_url.as_ref() else {
                    self.accept_phase = AcceptPhase::Commit;
                    return StepOutcome::Yield;
                };
                let Some(body) = self.meshes.get(mesh_url) else {
                    self.reject_candidate("placement-mesh-missing");
                    return StepOutcome::Yield;
                };
                let world = pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale);
                self.pending_spatial = Some(self.spatial_index.begin_replacement(self.collision_owner(), object.id.clone(), CollisionAabb::from_body(body, &world)));
                self.accept_phase = AcceptPhase::StepSpatial;
                StepOutcome::Yield
            }
            AcceptPhase::StepSpatial => {
                let owner = self.collision_owner();
                let Some(mutation) = self.pending_spatial.as_mut() else {
                    self.reject_candidate("placement-spatial-state-missing");
                    return StepOutcome::Yield;
                };
                match self.spatial_index.step_replacement(mutation, owner) {
                    CollisionMutationStep::Pending => {}
                    CollisionMutationStep::Complete => {
                        self.pending_spatial = None;
                        self.accept_phase = AcceptPhase::InstallLookup;
                    }
                    CollisionMutationStep::Rejected(rejected) => self.fixed_rejection = Some(FillRetiredOwner::Spatial(rejected)),
                    CollisionMutationStep::Stale => self.reject_candidate("stale-spatial-mutation"),
                }
                StepOutcome::Yield
            }
            AcceptPhase::InstallLookup => {
                let Some(object) = self.pending_object.as_ref() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(mesh_url) = object.mesh_url.as_ref() else {
                    self.accept_phase = AcceptPhase::Commit;
                    return StepOutcome::Yield;
                };
                let index = self.placed.len();
                match self.placed_lookup.try_insert(object.id.clone(), index) {
                    Ok(FixedOwnerMapInsert::Inserted) => {}
                    Ok(FixedOwnerMapInsert::Occupied { input_key, input_value: _ }) | Err((input_key, _)) => {
                        self.fixed_rejection = Some(FillRetiredOwner::String(input_key));
                        return StepOutcome::Yield;
                    }
                }
                self.placed.push(PlacedCollisionEntry { object_id: object.id.clone(), mesh_url: mesh_url.clone(), world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) });
                self.accept_vortex_cursor = 0;
                self.accept_phase = AcceptPhase::AppendTargets;
                StepOutcome::Yield
            }
            AcceptPhase::AppendTargets => {
                let (Some(object), Some(payload)) = (self.pending_object.as_ref(), self.pending_payload.as_ref()) else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let index = self.accept_vortex_cursor;
                let Some(vortex) = object.vortices.get(index) else {
                    self.accept_phase = AcceptPhase::Commit;
                    return StepOutcome::Yield;
                };
                self.accept_vortex_cursor += 1;
                if index != payload.source_vortex_index {
                    let target = BrushFillVortexTarget { full_id: puzzle3d_vortex_full_id(&object.id, &vortex.id), object_id: object.id.clone(), object_kind: object.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone(), vortex_index: index };
                    self.push_target(target);
                }
                StepOutcome::Yield
            }
            AcceptPhase::Commit => {
                let Some(payload) = self.pending_payload.take() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(placed_object) = self.pending_object.take() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(attraction) = self.pending_attraction.take() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                self.sequence.push(payload);
                self.appended_objects.push(placed_object);
                self.appended_attractions.push(attraction);
                if std::mem::take(&mut self.run_candidate_live) {
                    self.run_event(FillRunEvent::Accepted);
                }
                if let Some(index) = self.current_target_index {
                    self.retire_target_weight(index);
                }
                self.reset_candidate();
                self.stage = self.next_round_stage();
                if matches!(self.stage, FillJobStage::Complete(_)) {
                    return self.complete();
                }
                StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: RetainedJobPayload::empty(JobPayloadStream::CheckpointState), applied_progress: self.applied_count as u64 })
            }
        }
    }

    /// 🛑️ Stops the planner below its requested count with a reason the HUD can name, instead of
    /// leaving the user with a silently finished plan.
    fn stall(&mut self, stall: FillStall) {
        if std::mem::take(&mut self.run_candidate_live) {
            self.run_event(FillRunEvent::Refused(stall.reason()));
        }
        self.stage = FillJobStage::Complete(FillPlanEnd::Stalled(stall));
    }

    fn reject_candidate(&mut self, reason: &str) {
        self.refuse_run_candidate(reason);
        self.last_rejection = Some(reason.to_string());
        self.rejected_count += 1;
        self.candidate_cursor += 1;
        self.reset_acceptance();
        self.reset_collision();
        self.stage = FillJobStage::SelectCandidate;
    }

    /// 🎯️ Marks the drawn vortex: every compatible candidate was refused (or it has none), so it leaves the draw for good,
    /// shows as a danger marker, and the next round draws another free vortex.
    fn mark_target(&mut self, reason: &str) {
        self.refuse_run_candidate(reason);
        self.last_rejection = Some(reason.to_string());
        self.rejected_count += 1;
        if let Some(index) = self.current_target_index.take() {
            self.retire_target_weight(index);
        }
        if self.run_events.is_some() {
            let host = self.current_target.as_ref().and_then(|target| self.base.objects.iter().chain(&self.appended_objects).find(|object| object.id == target.object_id).and_then(|host| vortex_world_from_object(host, target.vortex_index)));
            if let Some((position, _)) = host {
                self.run_event(FillRunEvent::VortexMarked { position });
            }
        }
        self.current_target = None;
        self.reset_candidate_preparation();
        self.reset_acceptance();
        self.reset_collision();
        self.stage = FillJobStage::SelectTarget;
    }

    fn reset_collision(&mut self) {
        self.current_preview = None;
        self.broad_phase_query = None;
        self.broad_phase_cursor = 0;
        self.broad_phase_bounds = None;
        self.collision = None;
    }

    /// 🔄️ Ends one vortex round: the drawn vortex, its candidates, the acceptance and the collision state. The pool stays.
    fn reset_candidate(&mut self) {
        self.current_target = None;
        self.current_target_index = None;
        self.reset_candidate_preparation();
        self.reset_acceptance();
        self.last_rejection = None;
        self.reset_collision();
    }

    /// 🌀️ Drops the free vortex pool so the next round rebuilds it from the (retracted) document; marks are forgotten
    /// because fewer placed objects can free a vortex again.
    fn reset_targets(&mut self) {
        self.targets.clear();
        self.target_weights.clear();
        self.target_tree = vec![0.0];
        self.targets_ready = false;
        self.target_prepare_phase = TargetPreparePhase::Reset;
        self.target_attraction_cursor = 0;
        self.target_object_cursor = 0;
        self.target_vortex_cursor = 0;
    }

    fn reset_candidate_preparation(&mut self) {
        self.candidates.clear();
        self.candidate_cursor = 0;
        self.candidate_prepare_phase = CandidatePreparePhase::Reset;
        self.candidate_kind_cursor = 0;
        self.candidate_vortex_cursor = 0;
        self.candidate_prepare_cursor = 0;
        self.candidate_raw.clear();
        self.candidate_weights.clear();
        self.candidate_tree = vec![0.0];
        self.candidate_remaining = 0;
    }

    fn reset_acceptance(&mut self) {
        self.accept_phase = AcceptPhase::Validate;
        self.accept_attraction_cursor = 0;
        self.accept_vortex_cursor = 0;
        self.pending_payload = None;
        self.pending_object = None;
        self.pending_attraction = None;
        self.pending_spatial = None;
    }

    /// 🛰️ Records one observation when a fill run job is listening.
    fn run_event(&mut self, event: FillRunEvent) {
        if let Some(events) = self.run_events.as_mut() {
            events.push(event);
        }
    }

    fn refuse_run_candidate(&mut self, reason: &str) {
        if std::mem::take(&mut self.run_candidate_live) {
            self.run_event(FillRunEvent::Refused(FillRunReason::of_refusal(reason)));
        }
    }

    /// 🫥️ A retarget or replan drops the candidate under test before it reached a verdict.
    fn abandon_run_candidate(&mut self) {
        if std::mem::take(&mut self.run_candidate_live) {
            self.run_event(FillRunEvent::Abandoned);
        }
    }

    /// 🛰️ Starts recording observations for a fill run job.
    pub(crate) fn observe_run(&mut self) {
        self.run_events.get_or_insert_with(|| Vec::with_capacity(4));
    }

    /// 📤️ Moves the recorded observations into `into` (cleared first), keeping both buffers' capacity.
    pub(crate) fn swap_run_events(&mut self, into: &mut Vec<FillRunEvent>) {
        into.clear();
        if let Some(events) = self.run_events.as_mut() {
            std::mem::swap(events, into);
        }
    }

    /// 🚧️ A document too large for the planner's fixed pages ends the run where the user can see it: the
    /// preparation preflight's refusal publishes one `danger` step through the run's tick writer and then
    /// faults the run (nothing was changed). Without a writer the refusal faults at once. A page that fills
    /// mid-plan is [`FillBuilder::stall_on_capacity`] instead.
    pub(crate) fn capacity_refusal<C: FillStepContext>(&mut self, context: &mut C, writer: Option<&mut ToolRunTickWriter>) -> Option<StepOutcome> {
        if let Some(refusal) = self.preparation_capacity_refusal.as_mut() {
            if let Some(writer) = writer.filter(|_| !refusal.published) {
                refusal.published = true;
                let _ = writer.step(ToolRunStepKind::Danger, FillRunStage::Prepare.index(), FillRunReason::ArtifactCapacity.code(), None, &[ToolRunStepArg::Unsigned(refusal.omitted_index as u64)]);
                return Some(StepOutcome::Yield);
            }
            return Some(StepOutcome::Fault(JobFault { detail: context.fault_payload(refusal.diagnostic().as_bytes()) }));
        }
        None
    }

    /// 📄️ A fixed page that refused an owner mid-plan stalls the plan as `artifact-capacity` over what was already
    /// placed; answers whether the plan is over capacity.
    pub(crate) fn stall_on_capacity(&mut self) -> bool {
        if self.collection_over_capacity || self.fixed_rejection.is_some() {
            if !matches!(self.stage, FillJobStage::Complete(_)) {
                self.stall(FillStall::ArtifactCapacity);
            }
            return true;
        }
        false
    }

    fn complete(&self) -> StepOutcome {
        StepOutcome::Complete(CommitCandidate {
            state: RetainedJobPayload::empty(JobPayloadStream::CommitState),
            output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput),
        })
    }

    pub(crate) fn stage_label(&self) -> &'static str {
        match self.stage {
            FillJobStage::RetractTail => "retract-tail",
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
            FillJobStage::Complete(_) => "complete",
        }
    }
}

impl FillBuilder {
    /// 🪜️ One bounded planner transition under any [`FillStepContext`].
    pub(crate) fn advance<C: FillStepContext>(&mut self, context: &mut C) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: context.fault_payload(b"stale-fill-operation") });
        }
        if let Some(outcome) = self.capacity_refusal(context, None) {
            return outcome;
        }
        if self.stall_on_capacity() {
            return self.complete();
        }
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        context.set_stage(self.stage_label());
        let stage = self.stage;
        let outcome = match stage {
            FillJobStage::RetractTail => {
                if self.discard_tail_one().is_err() {
                    return StepOutcome::Fault(JobFault { detail: context.fault_payload(b"stale-spatial-index") });
                }
                None
            }
            FillJobStage::PrepareFixture | FillJobStage::PrepareCatalogs | FillJobStage::PrepareMeshes | FillJobStage::PrepareEntries | FillJobStage::PrepareSpatial | FillJobStage::PrepareLookup | FillJobStage::PrepareConfiguration => {
                if self.prepare_one().is_err() {
                    return StepOutcome::Fault(JobFault { detail: context.fault_payload(b"stale-spatial-index") });
                }
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
            FillJobStage::Complete(_) => return self.complete(),
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
                FillJobStage::RetractTail
                    | FillJobStage::PrepareFixture
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
        outcome.unwrap_or(StepOutcome::Yield)
    }
}

impl InteractiveJob for FillBuilder {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        self.advance(context)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.closing = true;
        if maximum_items == 0 || maximum_bytes < FILL_BUILDER_OWNER_PAGE_BYTES {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.retire_one_close_owner() {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: FILL_BUILDER_OWNER_PAGE_BYTES }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.close_current.is_none() && self.terminal_owners_empty()
    }
}
//#endregion 🧵️InteractiveFillJob

//#region ⏯️FillRunJob
/// 🚰️ Estimated pending tick bytes past which a fill run job flushes: one tick must fit one
/// `JOB_PAYLOAD_PAGE_BYTES` job payload page with a placement's ops to spare.
pub(crate) const FILL_RUN_TICK_FLUSH_BYTES: usize = 8 * 1024;
/// 🧬️ Provisional ops one accepted placement appends: `create_object` then `connect_vortices`.
pub(crate) const FILL_RUN_OPS_PER_PLACEMENT: u32 = 2;
/// 🎯️ Trace scale of a marked vortex over the vortex marker mesh, so an exhausted vortex stands out from the open ones.
pub(crate) const FILL_RUN_MARKED_VORTEX_SCALE: f32 = 2.0;

/// 🆔️ Stable provisional entity of a placed object: the first eight little-endian bytes of its id digest.
pub(crate) fn fill_run_entity(object_id: &str) -> u64 {
    let digest = semio_framework_hash::hash(object_id.as_bytes());
    u64::from_le_bytes(digest.as_bytes()[..8].try_into().expect("eight digest bytes"))
}

/// 🧬️ The two `OpBinary` document ops one placement contributes, in append order.
pub(crate) fn fill_run_ops(object: &FixtureObject, attraction: &AttractionProps, peers: &[Puzzle3dObject], catalog_fixture: &Puzzle3dFixture) -> Option<[Vec<u8>; 2]> {
    use crate::standards::v1::subsets::any::schema::mutations::{binary::encode_op, connect_vortices, create_object};
    let mut document: crate::Puzzle3dObject = dsl::FromValue::from_value(dsl::ToValue::to_value(object)).ok()?;
    let kind_id = object.object_kind.as_deref().unwrap_or("object");
    if document.label.as_deref().map(str::is_empty).unwrap_or(true) {
        document.label = Some(puzzle3d_next_object_label(peers, catalog_fixture, kind_id));
    }
    let create = encode_op(&create_object(document, None)).ok()?;
    let connect = encode_op(&connect_vortices(attraction.id.clone(), attraction.attracting.clone(), attraction.attracted.clone(), attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y)).ok()?;
    Some([create, connect])
}

/// 🧱️ The placements a provisional op list holds, in op order (`create_object` then `connect_vortices`
/// per placement), keyed consecutively from `first_key` — a key range the run's trace key allocator handed out
/// (`ToolRunJobRequest::trace_keys`), since the run's own candidate keys are not recoverable from ops — with
/// subjects indexing `mesh_lane`; `None` when the list is not a whole sequence of fill placements.
pub(crate) fn fill_run_placements(provisional: &[crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation], mesh_lane: &[String], first_key: u64) -> Option<Vec<FillRunPlacement>> {
    use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
    provisional
        .chunks(FILL_RUN_OPS_PER_PLACEMENT as usize)
        .enumerate()
        .map(|(index, pair)| {
            let [Puzzle3dMutation::CreateObject(create), Puzzle3dMutation::ConnectVortices(connect)] = pair else { return None };
            let object = <FixtureObject as dsl::FromValue>::from_value(dsl::ToValue::to_value(&create.object)).ok()?;
            let mesh = object.mesh_url.as_ref().and_then(|url| mesh_lane.iter().position(|entry| entry == url)).unwrap_or(0) as u32;
            let subject = ToolRunTraceSubject::Instance3d { mesh, position: object.origin.map(|value| value as f32), rotation: object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]).map(|value| value as f32), scale: fill_run_scale(&object.scale) };
            let attraction = AttractionProps { id: connect.id.clone(), attracting: connect.attracting.clone(), attracted: connect.attracted.clone(), gap: connect.gap, shift: connect.shift, rise: connect.rise, rotation: connect.rotation, turn: connect.turn, tilt: connect.tilt, x: connect.x, y: connect.y };
            Some(FillRunPlacement { key: first_key + index as u64, subject, entity: fill_run_entity(&object.id), object, attraction })
        })
        .collect()
}

/// 🧱️ One provisional placement of a fill run: its trace key, entity, object and attraction.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FillRunPlacement {
    pub(crate) key: u64,
    pub(crate) subject: ToolRunTraceSubject,
    pub(crate) entity: u64,
    pub(crate) object: FixtureObject,
    pub(crate) attraction: AttractionProps,
}

impl FillRunStage {
    /// 🧭️ The run stage a planner stage belongs to.
    fn of(stage: FillJobStage) -> Option<Self> {
        match stage {
            FillJobStage::RetractTail => Some(Self::Retract),
            FillJobStage::PrepareFixture | FillJobStage::PrepareCatalogs | FillJobStage::PrepareMeshes | FillJobStage::PrepareEntries | FillJobStage::PrepareSpatial | FillJobStage::PrepareLookup | FillJobStage::PrepareConfiguration => Some(Self::Prepare),
            FillJobStage::PrepareTargets | FillJobStage::SelectTarget | FillJobStage::PrepareCandidates | FillJobStage::SelectCandidate => Some(Self::Search),
            FillJobStage::ConstructPreview | FillJobStage::QueryBroadPhase | FillJobStage::TestCollision => Some(Self::Test),
            FillJobStage::AcceptCandidate => Some(Self::Lock),
            FillJobStage::Complete(_) => None,
        }
    }
}

/// ⏳️ Planner transitions inside one run job step: the step's deadline and cancellation, but neither
/// its fuel — a run job's fuel counts candidates, never transitions or collision samples — nor its job
/// operation: the planner answers to its own `operation`, while the framework tool run ledger guards the
/// job by run identity and generation.
struct FillRunTransitionContext<'a, 'b> {
    outer: &'a mut StepContext<'b>,
    operation: Operation,
}

impl CollisionStepContext for FillRunTransitionContext<'_, '_> {
    fn is_cancelled(&self) -> bool {
        self.outer.is_cancelled()
    }

    fn should_yield(&self) -> bool {
        self.outer.deadline_exceeded()
    }

    fn consume_fuel(&mut self, _units: u64) {}
}

impl FillStepContext for FillRunTransitionContext<'_, '_> {
    fn operation(&self) -> OperationId {
        self.operation.operation
    }

    fn generation(&self) -> Generation {
        self.operation.generation
    }

    fn set_stage(&mut self, label: &'static str) {
        self.outer.set_stage(label);
    }

    fn fault_payload(&mut self, bytes: &[u8]) -> RetainedJobPayload {
        FillStepContext::fault_payload(self.outer, bytes)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillRunOwed {
    Checkpoint,
    Complete,
}

/// 🧾️ Test-only record of one verdict the run reached, with what the oracle needs to recompute it.
#[cfg(test)]
#[derive(Clone, Debug)]
pub(crate) struct FillRunVerdictRecord {
    pub(crate) key: u64,
    pub(crate) reason: FillRunReason,
    pub(crate) mesh_url: String,
    pub(crate) origin: [f64; 3],
    pub(crate) orientation: [f64; 4],
    pub(crate) host: Option<String>,
    pub(crate) placements_before: usize,
}

/// ⏯️ The puzzle 3d fill tool run job (`ToolRunDefinition.runJob`): drives [`FillBuilder`] and reports
/// every constructed candidate as a `testing` trace record that turns `danger` (collision), `warning`
/// (rule refusal) or `success` (placed); every placement appends its `create_object` +
/// `connect_vortices` ops and entity. One unit of fuel is one candidate verdict. Ticks travel only in
/// `PreviewReady` pages; `CheckpointReady` (after each placement) and `Complete` are returned by the
/// call after the tick that led to them. [`FillRunJob::resume`] with a new requested count continues the
/// same deterministic sequence (raise) or retracts the tail (lower); a job rebuilt after the framework
/// closed its predecessor reaches that checkpoint first by a silent replay ([`FillRunJob::replaying`]).
pub(crate) struct FillRunJob {
    builder: FillBuilder,
    writer: ToolRunTickWriter,
    inputs: [u8; 32],
    replay: Option<FillRunReplay>,
    mesh_lane: Vec<String>,
    mesh_index: HashMap<String, u32>,
    events: Vec<FillRunEvent>,
    deferred: Vec<FillRunEvent>,
    live: Option<(u64, ToolRunTraceSubject)>,
    /// 👁️ The one tested candidate whose verdict is still on screen. The run shows ONE candidate at a time: the next
    /// constructed candidate retires it, so a collision is visible only while it is the current attempt and a placement
    /// stays visible as its document instance (the `provisional` style), never as a pile of past trace records.
    shown: Option<u64>,
    next_key: u64,
    placement_keys: Vec<(u64, ToolRunTraceSubject)>,
    tested: u64,
    marked: u64,
    collisions: u64,
    rejected: u64,
    stage: FillRunStage,
    settled: bool,
    capped: bool,
    owed: Option<FillRunOwed>,
    progress_sequence: u64,
    #[cfg(test)]
    pub(crate) verdicts: Vec<FillRunVerdictRecord>,
    #[cfg(test)]
    live_record: Option<FillRunVerdictRecord>,
}

/// ⏪️ A rebuilt run job's silent way back to its predecessor's checkpoint: every planner transition runs,
/// no tick leaves the job, and on arrival the writer continues from the ledger's provisional length.
#[derive(Clone, Copy, Debug)]
struct FillRunReplay {
    target: FillRunCheckpoint,
    requested: usize,
    provisional: u32,
}

#[cfg(test)]
/// 🚫️ A checkpoint that does not belong to this resident run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FillRunResumeError {
    Malformed,
    Foreign,
}

impl FillRunJob {
    /// 🚦️ The one production construction path of a fill run: begins the cooperative preparation and picks
    /// replay (a `checkpoint` with the same `inputs`), restart (a predecessor left `provisional` ops or was
    /// reconfigured) or a fresh run.
    pub(crate) fn start(roots: FillPreparationRoots, operation: Operation, identity: ToolRunIdentity, mesh_lane: Vec<String>, inputs: [u8; 32], requested: usize, checkpoint: Option<FillRunCheckpoint>, provisional: u32) -> Self {
        match checkpoint.filter(|checkpoint| checkpoint.inputs == inputs) {
            Some(checkpoint) => Self::replaying(FillBuilder::begin_preparation(roots, operation, checkpoint.requested as usize), identity, mesh_lane, checkpoint, requested, provisional),
            None if provisional > 0 || identity.generation > 0 => Self::restarting(FillBuilder::begin_preparation(roots, operation, requested), identity, mesh_lane, inputs, provisional),
            None => Self::new(FillBuilder::begin_preparation(roots, operation, requested), identity, mesh_lane, inputs),
        }
    }

    /// 🎬️ Wraps a freshly begun planner; `mesh_lane` is the plugin mesh lane trace subjects index into
    /// (a url missing from it is appended and reported through [`FillRunJob::mesh_lane`]).
    pub(crate) fn new(builder: FillBuilder, identity: ToolRunIdentity, mesh_lane: Vec<String>, inputs: [u8; 32]) -> Self {
        Self::with_writer(builder, ToolRunTickWriter::new(identity), mesh_lane, inputs)
    }

    /// 🔁️ A fresh run that replaces a predecessor whose sequence it cannot reproduce (other inputs or no
    /// checkpoint): its first tick clears the trace and retracts all `provisional` ops the ledger holds.
    pub(crate) fn restarting(builder: FillBuilder, identity: ToolRunIdentity, mesh_lane: Vec<String>, inputs: [u8; 32], provisional: u32) -> Self {
        let mut job = Self::with_writer(builder, ToolRunTickWriter::with_provisional_base(identity, provisional), mesh_lane, inputs);
        job.writer.clear_trace();
        job.writer.retract_to(0);
        job
    }

    /// ⏪️ A run rebuilt from `checkpoint` (same `inputs`): `builder` must be begun with the checkpoint's own
    /// requested count; it replays silently to the checkpoint, retracts any `provisional` op the ledger holds
    /// past it, and then resumes toward `requested`. A replay that overshoots the checkpoint restarts.
    pub(crate) fn replaying(builder: FillBuilder, identity: ToolRunIdentity, mesh_lane: Vec<String>, checkpoint: FillRunCheckpoint, requested: usize, provisional: u32) -> Self {
        let mut job = Self::with_writer(builder, ToolRunTickWriter::new(identity), mesh_lane, checkpoint.inputs);
        job.replay = Some(FillRunReplay { target: checkpoint, requested, provisional });
        job
    }

    fn with_writer(mut builder: FillBuilder, writer: ToolRunTickWriter, mesh_lane: Vec<String>, inputs: [u8; 32]) -> Self {
        builder.observe_run();
        let mesh_index = mesh_lane.iter().enumerate().map(|(index, url)| (url.clone(), index as u32)).collect();
        Self {
            builder,
            writer,
            inputs,
            replay: None,
            mesh_lane,
            mesh_index,
            events: Vec::with_capacity(4),
            deferred: Vec::new(),
            live: None,
            shown: None,
            next_key: 0,
            placement_keys: Vec::new(),
            tested: 0,
            marked: 0,
            collisions: 0,
            rejected: 0,
            stage: FillRunStage::Prepare,
            settled: false,
            capped: false,
            owed: None,
            progress_sequence: 0,
            #[cfg(test)]
            verdicts: Vec::new(),
            #[cfg(test)]
            live_record: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn builder(&self) -> &FillBuilder {
        &self.builder
    }

    #[cfg(test)]
    pub(crate) fn operation(&self) -> Operation {
        self.builder.operation
    }

    #[cfg(test)]
    pub(crate) fn mesh_lane(&self) -> &[String] {
        &self.mesh_lane
    }

    /// 📟️ `[tested, locked, collisions, rejected, marked]`, in [`FillRunCounter::ALL`] order.
    pub(crate) fn counters(&self) -> [u64; 5] {
        [self.tested, self.placement_keys.len() as u64, self.collisions, self.rejected, self.marked]
    }

    /// 📸️ Where this run stands right now.
    pub(crate) fn checkpoint(&self) -> FillRunCheckpoint {
        FillRunCheckpoint { requested: self.builder.requested_count() as u64, placements: self.placement_keys.len() as u64, provisional_ops: self.writer.provisional_len(), tested: self.tested, next_key: self.next_key, inputs: self.inputs }
    }

    #[cfg(test)]
    /// 🧱️ The provisional placements in op order, for finalize revalidation.
    pub(crate) fn provisional_placements(&self) -> Vec<FillRunPlacement> {
        self.placement_keys
            .iter()
            .zip(self.builder.appended_objects.iter().zip(&self.builder.appended_attractions))
            .map(|((key, subject), (object, attraction))| FillRunPlacement { key: *key, subject: *subject, entity: fill_run_entity(&object.id), object: object.clone(), attraction: attraction.clone() })
            .collect()
    }

    #[cfg(test)]
    /// ⏩️ Resumes this resident run from one of its own checkpoints with a new requested count.
    pub(crate) fn resume(&mut self, checkpoint: &[u8], requested: usize) -> Result<(), FillRunResumeError> {
        let checkpoint = FillRunCheckpoint::decode(checkpoint).ok_or(FillRunResumeError::Malformed)?;
        let current = self.checkpoint();
        if checkpoint.inputs != self.inputs || checkpoint.next_key > current.next_key || checkpoint.tested > current.tested || checkpoint.placements > self.builder.sequence.len() as u64 {
            return Err(FillRunResumeError::Foreign);
        }
        self.retarget(self.writer.identity(), requested);
        Ok(())
    }

    /// 🪢️ Every later tick carries `identity` (the framework rebound the run after a base change).
    pub(crate) fn rebind(&mut self, identity: ToolRunIdentity) {
        self.writer.rebind(identity);
    }

    /// 🎯️ Retargets this resident run in place (framework `reconfigure: resume`): later ticks carry `identity` and
    /// the plan heads for `requested` — a raise continues the same deterministic sequence, a lower retracts its
    /// tail, and a completed run wakes up again; a run still replaying adopts `requested` on arrival.
    pub(crate) fn retarget(&mut self, identity: ToolRunIdentity, requested: usize) {
        self.writer.rebind(identity);
        if let Some(replay) = self.replay.as_mut() {
            replay.requested = requested;
            return;
        }
        self.builder.set_requested_count(requested);
        if !matches!(self.builder.stage, FillJobStage::Complete(_)) {
            self.settled = false;
            if self.owed == Some(FillRunOwed::Complete) {
                self.owed = None;
            }
        }
    }

    fn mesh(&mut self, url: &str) -> u32 {
        if let Some(index) = self.mesh_index.get(url) {
            return *index;
        }
        let index = self.mesh_lane.len() as u32;
        self.mesh_lane.push(url.to_string());
        self.mesh_index.insert(url.to_string(), index);
        index
    }

    fn progress_snapshot(&mut self) -> ToolRunProgress {
        self.progress_sequence += 1;
        let counters = self.counters();
        ToolRunProgress {
            identity: self.writer.identity(),
            sequence: self.progress_sequence,
            state: if matches!(self.builder.stage, FillJobStage::Complete(_)) { ToolRunState::Complete } else { ToolRunState::Running },
            stage: self.stage.index(),
            completed: self.placement_keys.len() as u64,
            total: Some(self.builder.requested_count() as u64),
            counters: FillRunCounter::ALL.iter().zip(counters).map(|(counter, value)| ToolRunCounter { counter: counter.index(), value }).collect(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::default(),
        }
    }

    fn verdict(&mut self, context: &mut StepContext<'_>, reason: FillRunReason, verdict: ToolRunVerdict, code: u16) -> Option<(u64, ToolRunTraceSubject)> {
        let (key, subject) = self.live.take()?;
        self.writer.upsert(key, verdict, code, subject);
        self.shown = Some(key);
        if self.replay.is_none() {
            context.consume_fuel(1);
        }
        #[cfg(test)]
        if let Some(mut record) = self.live_record.take() {
            record.reason = reason;
            self.verdicts.push(record);
        }
        let _ = reason;
        Some((key, subject))
    }

    /// 🔭️ Folds the planner's observations into the tick. A constructed candidate is one visible unit of fuel, like its
    /// verdict: when it exhausts the step's fuel, the observations after it wait in `deferred` for the next step, so a
    /// one-unit step (a paced run, a single step) publishes the candidate under test before the verdict that marks it.
    fn observe(&mut self, context: &mut StepContext<'_>) -> Result<bool, &'static [u8]> {
        if self.deferred.is_empty() {
            self.builder.swap_run_events(&mut self.events);
        } else {
            std::mem::swap(&mut self.events, &mut self.deferred);
            self.deferred.clear();
        }
        let mut events = std::mem::take(&mut self.events);
        let mut accepted = false;
        let mut cursor = 0;
        while let Some(event) = events.get(cursor) {
            cursor += 1;
            match event {
                FillRunEvent::Constructed { mesh_url, origin, orientation, scale } => {
                    if let Some(previous) = self.shown.take() {
                        self.writer.retire(previous);
                    }
                    let key = self.next_key;
                    self.next_key += 1;
                    self.tested += 1;
                    let subject = ToolRunTraceSubject::Instance3d { mesh: self.mesh(mesh_url), position: origin.map(|value| value as f32), rotation: orientation.map(|value| value as f32), scale: *scale };
                    self.writer.upsert(key, ToolRunVerdict::Testing, FillRunReason::Fits.code(), subject);
                    self.live = Some((key, subject));
                    #[cfg(test)]
                    {
                        self.live_record = Some(FillRunVerdictRecord { key, reason: FillRunReason::Fits, mesh_url: mesh_url.clone(), origin: *origin, orientation: *orientation, host: self.builder.current_target.as_ref().map(|target| target.object_id.clone()), placements_before: self.builder.sequence.len() });
                    }
                    if self.replay.is_none() {
                        context.consume_fuel(1);
                        if context.fuel_exhausted() && cursor < events.len() {
                            self.deferred = events.split_off(cursor);
                            break;
                        }
                    }
                }
                FillRunEvent::Refused(reason) => {
                    if self.verdict(context, *reason, reason.verdict(), reason.code()).is_some() {
                        if *reason == FillRunReason::SolidOverlap {
                            self.collisions += 1;
                        } else {
                            self.rejected += 1;
                        }
                    }
                }
                FillRunEvent::Accepted => {
                    let (Some(object), Some(attraction)) = (self.builder.appended_objects.last(), self.builder.appended_attractions.last()) else {
                        return Err(b"fill-run-placement-missing");
                    };
                    let peers = self.builder.label_peers();
                    let catalog_fixture = self.builder.label_catalog_fixture();
                    let Some(ops) = fill_run_ops(object, attraction, &peers, &catalog_fixture) else {
                        return Err(b"fill-run-op-encode");
                    };
                    let entity = fill_run_entity(&object.id);
                    if self.writer.provisional_len() + FILL_RUN_OPS_PER_PLACEMENT > TOOL_RUN_PROVISIONAL_OPS_MAX {
                        let _ = self.verdict(context, FillRunReason::Rejected, ToolRunVerdict::Warning, TOOL_RUN_REASON_PROVISIONAL_CAP);
                        self.rejected += 1;
                        self.capped = true;
                        let _ = self.writer.step(ToolRunStepKind::Warning, self.stage.index(), TOOL_RUN_REASON_PROVISIONAL_CAP, None, &[ToolRunStepArg::Unsigned(u64::from(TOOL_RUN_PROVISIONAL_OPS_MAX))]);
                        self.builder.set_requested_count(self.builder.sequence.len().saturating_sub(1));
                        continue;
                    }
                    let Some(placement) = self.verdict(context, FillRunReason::Fits, ToolRunVerdict::Success, FillRunReason::Fits.code()) else {
                        return Err(b"fill-run-placement-untested");
                    };
                    let [create, connect] = ops;
                    if self.writer.append_op(create).and_then(|()| self.writer.append_op(connect)).is_err() {
                        return Err(b"fill-run-provisional-ops");
                    }
                    self.writer.append_entity(entity);
                    self.placement_keys.push(placement);
                    accepted = true;
                }
                FillRunEvent::VortexMarked { position } => {
                    let key = self.next_key;
                    self.next_key += 1;
                    let subject = ToolRunTraceSubject::Instance3d { mesh: self.mesh(VORTEX_MARKER_MESH_KIND), position: position.map(|value| value as f32), rotation: [0.0, 0.0, 0.0, 1.0], scale: FILL_RUN_MARKED_VORTEX_SCALE };
                    self.writer.upsert(key, ToolRunVerdict::Danger, FillRunReason::VortexExhausted.code(), subject);
                    self.marked += 1;
                    if self.replay.is_none() {
                        context.consume_fuel(1);
                    }
                }
                FillRunEvent::Abandoned => {
                    if let Some((key, _)) = self.live.take() {
                        self.writer.retire(key);
                    }
                    #[cfg(test)]
                    {
                        self.live_record = None;
                    }
                }
                FillRunEvent::Discarded => {
                    while self.placement_keys.len() > self.builder.sequence.len() {
                        if let Some((key, _)) = self.placement_keys.pop().filter(|(key, _)| self.shown == Some(*key)) {
                            self.writer.retire(key);
                            self.shown = None;
                        }
                    }
                    self.writer.retract_to(self.placement_keys.len() as u32 * FILL_RUN_OPS_PER_PLACEMENT);
                    let _ = self.writer.step(ToolRunStepKind::Info, FillRunStage::Retract.index(), FillRunReason::Retracted.code(), None, &[]);
                }
            }
        }
        self.events = events;
        if let Some(stage) = FillRunStage::of(self.builder.stage) {
            self.stage = stage;
        }
        Ok(accepted)
    }

    /// 🏁️ The run's one terminal step, derived from how the plan ended: a declared stall's `warning`, or `success`
    /// `requested-reached` over a request the placements actually meet (a provisional-cap run already published its
    /// `warning`). A completed plan that satisfies neither is refused as `fill-run-end-undeclared` — the run faults
    /// visibly instead of ending without a reason.
    fn settle(&mut self) -> Result<(), &'static [u8]> {
        if std::mem::replace(&mut self.settled, true) {
            return Ok(());
        }
        let placements = self.placement_keys.len() as u64;
        let placed = [ToolRunStepArg::Unsigned(placements)];
        let _ = match self.builder.end() {
            Some(FillPlanEnd::Stalled(stall)) => self.writer.step(ToolRunStepKind::Warning, self.stage.index(), stall.reason().code(), None, &placed),
            Some(FillPlanEnd::Reached) if self.capped => Ok(()),
            Some(FillPlanEnd::Reached) if placements >= self.builder.requested_count() as u64 => self.writer.step(ToolRunStepKind::Success, self.stage.index(), FillRunReason::RequestedReached.code(), None, &placed),
            Some(FillPlanEnd::Reached) | None => return Err(b"fill-run-end-undeclared"),
        };
        Ok(())
    }

    fn flush(&mut self, context: &mut StepContext<'_>) -> Option<StepOutcome> {
        if self.writer.is_empty() {
            return None;
        }
        let progress = self.progress_snapshot();
        self.writer.progress(progress);
        let bytes = self.writer.finish()?.encode().ok();
        let page = bytes.as_deref().map(|bytes| context.payload_from_bytes(JobPayloadStream::Preview, bytes));
        Some(match page {
            Some(Ok(payload)) => StepOutcome::PreviewReady(payload),
            Some(Err(rejected)) => {
                drop(rejected.into_source());
                StepOutcome::Fault(JobFault { detail: FillStepContext::fault_payload(context, b"fill-run-tick-page") })
            }
            None => StepOutcome::Fault(JobFault { detail: FillStepContext::fault_payload(context, b"fill-run-tick-encode") }),
        })
    }

    fn flush_then(&mut self, context: &mut StepContext<'_>, owed: FillRunOwed) -> StepOutcome {
        match self.flush(context) {
            Some(preview @ StepOutcome::PreviewReady(_)) => {
                self.owed = Some(owed);
                preview
            }
            Some(fault) => fault,
            None => self.settle_owed(context, owed),
        }
    }

    fn settle_owed(&mut self, context: &mut StepContext<'_>, owed: FillRunOwed) -> StepOutcome {
        match owed {
            FillRunOwed::Complete => self.builder.complete(),
            FillRunOwed::Checkpoint => match context.payload_from_bytes(JobPayloadStream::CheckpointState, &self.checkpoint().encode()) {
                Ok(state) => StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state, applied_progress: self.placement_keys.len() as u64 }),
                Err(rejected) => {
                    drop(rejected.into_source());
                    StepOutcome::Yield
                }
            },
        }
    }
}

impl FillRunJob {
    /// ⏪️ Where a replay stands after one observed transition: arrived at its checkpoint (the writer now
    /// continues from the ledger's provisional length and the run resumes toward the requested count),
    /// overshot it (a restart over the ledger's provisional ops), or still on its way (pending bytes are
    /// dropped, never flushed).
    fn settle_replay(&mut self) {
        let Some(replay) = self.replay else { return };
        let target = replay.target;
        let placements = self.placement_keys.len() as u64;
        if placements == target.placements && self.tested == target.tested && self.next_key == target.next_key && self.writer.provisional_len() == target.provisional_ops {
            self.replay = None;
            self.writer = ToolRunTickWriter::with_provisional_base(self.writer.identity(), replay.provisional);
            if replay.provisional > target.provisional_ops {
                self.writer.retract_to(target.provisional_ops);
            }
            self.builder.set_requested_count(replay.requested);
            self.settled = false;
            return;
        }
        if placements > target.placements || self.tested > target.tested || self.next_key > target.next_key || matches!(self.builder.stage, FillJobStage::Complete(_)) {
            self.replay = None;
            self.writer = ToolRunTickWriter::with_provisional_base(self.writer.identity(), replay.provisional);
            self.writer.clear_trace();
            self.writer.retract_to(0);
            return;
        }
        if self.writer.pending_bytes() >= FILL_RUN_TICK_FLUSH_BYTES {
            let _ = self.writer.finish();
        }
    }
}

impl InteractiveJob for FillRunJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if let Some(owed) = self.owed.take() {
            return self.settle_owed(context, owed);
        }
        loop {
            if let Some(outcome) = self.builder.capacity_refusal(context, Some(&mut self.writer)) {
                return match outcome {
                    StepOutcome::Yield => self.flush(context).unwrap_or(StepOutcome::Yield),
                    outcome => outcome,
                };
            }
            if self.builder.stall_on_capacity() {
                if let Err(detail) = self.observe(context) {
                    return StepOutcome::Fault(JobFault { detail: FillStepContext::fault_payload(context, detail) });
                }
            }
            if self.replay.is_none() && self.deferred.is_empty() && matches!(self.builder.stage, FillJobStage::Complete(_)) {
                if let Err(detail) = self.settle() {
                    return StepOutcome::Fault(JobFault { detail: FillStepContext::fault_payload(context, detail) });
                }
                return self.flush_then(context, FillRunOwed::Complete);
            }
            if context.deadline_exceeded() || (self.replay.is_none() && (context.fuel_exhausted() || self.writer.pending_bytes() >= FILL_RUN_TICK_FLUSH_BYTES)) {
                return if self.replay.is_some() { StepOutcome::Yield } else { self.flush(context).unwrap_or(StepOutcome::Yield) };
            }
            let operation = self.builder.operation;
            let outcome = self.deferred.is_empty().then(|| self.builder.advance(&mut FillRunTransitionContext { outer: context, operation }));
            let accepted = match self.observe(context) {
                Ok(accepted) => accepted,
                Err(detail) => {
                    drop(outcome);
                    return StepOutcome::Fault(JobFault { detail: FillStepContext::fault_payload(context, detail) });
                }
            };
            match outcome {
                Some(StepOutcome::Cancelled) => return StepOutcome::Cancelled,
                Some(fault @ StepOutcome::Fault(_)) => return fault,
                _ => {}
            }
            if self.replay.is_some() {
                self.settle_replay();
                continue;
            }
            if accepted {
                return self.flush_then(context, FillRunOwed::Checkpoint);
            }
        }
    }

    fn begin_close(&mut self) {
        self.builder.begin_close();
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.builder.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.builder.terminal_is_empty()
    }
}

/// 🔍️ Where a finalize revalidation stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillRevalidatePhase {
    PrepareHead,
    Placement,
    Finish,
    Done,
}

/// 🔍️ The fill tool run revalidation job (`ToolRunDefinition.revalidateJob`): re-tests every provisional
/// placement against the head document — its host vortex must still exist, its id must be free and its
/// surface must not reach into any head object, its docking host included, deeper than the head's contact tolerance. Each placement is one unit of
/// fuel and ends as a `success` (`fits`) or `danger` (`TOOL_RUN_REASON_CONFLICT`) trace record. The last
/// tick retracts to the first conflict and re-appends every later survivor's ops and entity, with one
/// `danger` conflict step carrying the conflict count; `Complete` follows on the next call.
pub(crate) struct FillRevalidateJob {
    operation: Operation,
    identity: ToolRunIdentity,
    scene: Arc<SceneConfig>,
    meshes: Arc<HashMap<String, CollisionBody>>,
    placements: Vec<FillRunPlacement>,
    head: Vec<PlacedCollisionEntry>,
    head_ids: std::collections::HashSet<String>,
    vortex_owners: HashMap<String, String>,
    head_cursor: usize,
    pair_cursor: usize,
    collision: Option<CollisionPenetrationState>,
    conflicts: Vec<bool>,
    cursor: usize,
    phase: FillRevalidatePhase,
    ops: Vec<ToolRunTraceOp>,
    /// 👁️ The one placement under revalidation still on screen: the next placement's test retires it and the finish retires
    /// the last, so a finalize never leaves a verdict mesh over every committed object (the one-candidate rule of
    /// [`FillRunJob`]).
    shown: Option<u64>,
    steps: Vec<ToolRunStep>,
    sequence: u64,
    page: u32,
    closed: bool,
}

impl FillRevalidateJob {
    pub(crate) fn new(operation: Operation, identity: ToolRunIdentity, head: FillPreparationRoots, placements: Vec<FillRunPlacement>, first_sequence: u64) -> Self {
        Self {
            operation,
            identity,
            scene: head.scene,
            meshes: head.meshes,
            conflicts: Vec::with_capacity(placements.len()),
            placements,
            head: Vec::new(),
            head_ids: std::collections::HashSet::new(),
            vortex_owners: HashMap::new(),
            head_cursor: 0,
            pair_cursor: 0,
            collision: None,
            cursor: 0,
            phase: FillRevalidatePhase::PrepareHead,
            ops: Vec::new(),
            shown: None,
            steps: Vec::new(),
            sequence: first_sequence,
            page: 0,
            closed: false,
        }
    }

    /// 🪢️ Every later tick carries `identity`.
    pub(crate) fn rebind(&mut self, identity: ToolRunIdentity) {
        self.identity = identity;
    }

    #[cfg(test)]
    /// ⚖️ `true` per placement that conflicts with the head, in op order.
    pub(crate) fn conflicts(&self) -> &[bool] {
        &self.conflicts
    }

    fn prepare_head_one(&mut self) {
        let Some(object) = self.scene.fixture.objects.get(self.head_cursor) else {
            self.phase = FillRevalidatePhase::Placement;
            return;
        };
        self.head_cursor += 1;
        self.head_ids.insert(object.id.clone());
        for vortex in &object.vortices {
            self.vortex_owners.insert(puzzle3d_vortex_full_id(&object.id, &vortex.id), object.id.clone());
        }
        let empty = KindCatalogBundle::default();
        let mesh_url = resolve_placed_object_mesh_url(object, self.scene.kind_catalogs.as_ref().unwrap_or(&empty), &self.scene.fixture);
        if let Some(mesh_url) = mesh_url.filter(|url| self.meshes.contains_key(url)) {
            self.head.push(PlacedCollisionEntry { object_id: object.id.clone(), mesh_url, world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) });
        }
    }

    fn host_of(&self, index: usize) -> Option<String> {
        let attracting = &self.placements[index].attraction.attracting;
        self.vortex_owners.get(attracting).cloned().or_else(|| {
            self.placements[..index].iter().zip(&self.conflicts).filter(|(_, conflict)| !**conflict).find(|(placement, _)| placement.object.vortices.iter().any(|vortex| puzzle3d_vortex_full_id(&placement.object.id, &vortex.id) == *attracting)).map(|(placement, _)| placement.object.id.clone())
        })
    }

    /// ⚖️ One bounded unit of the current placement's test: `Some(conflict)` once it is decided.
    fn test_placement_unit(&mut self, context: &mut StepContext<'_>) -> Option<bool> {
        let index = self.cursor;
        let placement = &self.placements[index];
        if self.pair_cursor == 0 && self.collision.is_none() {
            if self.head_ids.contains(&placement.object.id) {
                return Some(true);
            }
            if self.host_of(index).is_none() {
                return Some(true);
            }
        }
        let placement = &self.placements[index];
        let Some(body) = placement.object.mesh_url.as_ref().and_then(|url| self.meshes.get(url)) else {
            return Some(false);
        };
        let world = pose_isometry(placement.object.origin, placement.object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &placement.object.scale);
        let Some(entry) = self.head.get(self.pair_cursor) else {
            return Some(false);
        };
        let Some(other) = self.meshes.get(&entry.mesh_url) else {
            self.pair_cursor += 1;
            return None;
        };
        if self.collision.is_none() {
            if !CollisionAabb::from_body(other, &entry.world).intersects(&CollisionAabb::from_body(body, &world)) {
                self.pair_cursor += 1;
                return None;
            }
        }
        let budget = self.scene.contact_tolerance;
        let collision = self.collision.get_or_insert_with(|| CollisionPenetrationState::new(budget));
        match collision.step(&mut FillRunTransitionContext { outer: context, operation: self.operation }, body, &world, other, &entry.world) {
            CollisionStepResult::Pending | CollisionStepResult::Cancelled => None,
            CollisionStepResult::Complete { depth, .. } => {
                self.collision = None;
                self.pair_cursor += 1;
                (depth > budget).then_some(true)
            }
        }
    }

    fn flush(&mut self, context: &mut StepContext<'_>, final_ops: Option<(u32, Vec<Vec<u8>>, Vec<u64>)>) -> StepOutcome {
        let (retract_to, append_ops, append_entities) = match final_ops {
            Some((retract_to, ops, entities)) => (Some(retract_to), ops, entities),
            None => (None, Vec::new(), Vec::new()),
        };
        let trace = if self.ops.is_empty() { Vec::new() } else { vec![ToolRunTracePage { identity: self.identity, page: self.page, ops: std::mem::take(&mut self.ops) }] };
        self.page += u32::from(!trace.is_empty());
        let conflicts = self.conflicts.iter().filter(|conflict| **conflict).count() as u64;
        let progress = ToolRunProgress {
            identity: self.identity,
            sequence: self.sequence,
            state: ToolRunState::Finalizing,
            stage: FillRunStage::Lock.index(),
            completed: self.conflicts.len() as u64,
            total: Some(self.placements.len() as u64),
            counters: vec![ToolRunCounter { counter: FillRunCounter::Collisions.index(), value: conflicts }],
            units_per_second: 0.0,
            conflicts: conflicts as u32,
            steps: ToolRunStepRing::default(),
        };
        let tick = ToolRunTick { identity: self.identity, sequence: self.sequence, progress: Some(progress), steps: std::mem::take(&mut self.steps), trace, append_ops, append_entities, retract_to, payload: None };
        self.sequence += 1;
        match tick.encode().ok().map(|bytes| context.payload_from_bytes(JobPayloadStream::Preview, &bytes)) {
            Some(Ok(payload)) => StepOutcome::PreviewReady(payload),
            Some(Err(rejected)) => {
                drop(rejected.into_source());
                StepOutcome::Fault(JobFault { detail: FillStepContext::fault_payload(context, b"fill-revalidate-tick-page") })
            }
            None => StepOutcome::Fault(JobFault { detail: FillStepContext::fault_payload(context, b"fill-revalidate-tick-encode") }),
        }
    }

    fn finish(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        self.phase = FillRevalidatePhase::Done;
        if let Some(previous) = self.shown.take() {
            self.ops.push(ToolRunTraceOp::Retire { key: previous });
        }
        let Some(first) = self.conflicts.iter().position(|conflict| *conflict) else {
            return self.flush(context, None);
        };
        let mut catalog_fixture = empty_fixture();
        if let Some(catalogs) = self.scene.kind_catalogs.as_ref() {
            catalog_fixture.meta.kind_catalogs = Some(dsl::ToValue::to_value(catalogs));
        }
        let mut peers: Vec<Puzzle3dObject> = self
            .scene
            .fixture
            .objects
            .iter()
            .filter_map(|object| dsl::FromValue::from_value(dsl::ToValue::to_value(object)).ok())
            .collect();
        let mut ops = Vec::new();
        let mut entities = Vec::new();
        for (placement, _) in self.placements.iter().zip(&self.conflicts).skip(first).filter(|(_, conflict)| !**conflict) {
            let Some([create, connect]) = fill_run_ops(&placement.object, &placement.attraction, &peers, &catalog_fixture) else {
                return StepOutcome::Fault(JobFault { detail: FillStepContext::fault_payload(context, b"fill-revalidate-op-encode") });
            };
            if let Ok(mut peer) = <Puzzle3dObject as dsl::FromValue>::from_value(dsl::ToValue::to_value(&placement.object)) {
                let kind_id = placement.object.object_kind.as_deref().unwrap_or("object");
                peer.label = Some(puzzle3d_next_object_label(&peers, &catalog_fixture, kind_id));
                peers.push(peer);
            }
            ops.extend([create, connect]);
            entities.push(placement.entity);
        }
        let conflicts = self.conflicts.iter().filter(|conflict| **conflict).count() as u64;
        self.steps.push(ToolRunStep { sequence: 0, kind: ToolRunStepKind::Danger, stage: FillRunStage::Lock.index(), reason: TOOL_RUN_REASON_CONFLICT, subject: None, repeat: 1, args: vec![ToolRunStepArg::Unsigned(conflicts)] });
        self.flush(context, Some((first as u32 * FILL_RUN_OPS_PER_PLACEMENT, ops, entities)))
    }
}

impl InteractiveJob for FillRevalidateJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        loop {
            match self.phase {
                FillRevalidatePhase::Done => {
                    return StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) });
                }
                FillRevalidatePhase::Finish => return self.finish(context),
                _ if context.fuel_exhausted() || context.deadline_exceeded() => {
                    return if self.ops.is_empty() { StepOutcome::Yield } else { self.flush(context, None) };
                }
                FillRevalidatePhase::PrepareHead => self.prepare_head_one(),
                FillRevalidatePhase::Placement => {
                    let Some(placement) = self.placements.get(self.cursor) else {
                        self.phase = FillRevalidatePhase::Finish;
                        continue;
                    };
                    let (key, subject) = (placement.key, placement.subject);
                    if self.pair_cursor == 0 && self.collision.is_none() {
                        if let Some(previous) = self.shown.take().filter(|previous| *previous != key) {
                            self.ops.push(ToolRunTraceOp::Retire { key: previous });
                        }
                        self.ops.push(ToolRunTraceOp::Upsert { key, verdict: ToolRunVerdict::Testing, reason: FillRunReason::Fits.code(), subject });
                    }
                    if let Some(conflict) = self.test_placement_unit(context) {
                        let (verdict, reason) = if conflict { (ToolRunVerdict::Danger, TOOL_RUN_REASON_CONFLICT) } else { (ToolRunVerdict::Success, FillRunReason::Fits.code()) };
                        self.ops.push(ToolRunTraceOp::Upsert { key, verdict, reason, subject });
                        self.shown = Some(key);
                        self.conflicts.push(conflict);
                        self.cursor += 1;
                        self.pair_cursor = 0;
                        self.collision = None;
                        context.consume_fuel(1);
                    }
                }
            }
        }
    }

    fn begin_close(&mut self) {
        self.closed = true;
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.closed = true;
        self.placements = Vec::new();
        self.head = Vec::new();
        self.head_ids = std::collections::HashSet::new();
        self.vortex_owners = HashMap::new();
        self.collision = None;
        self.ops = Vec::new();
        self.steps = Vec::new();
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closed && self.placements.is_empty() && self.head.is_empty() && self.ops.is_empty()
    }
}
//#endregion ⏯️FillRunJob

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
