//! 🏗️ Builder state: the job stages and phases, the fixed owners, the scene and catalog views, the preparation capacity refusal, `FillBuilder` and its retained brush weights.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillJobStage {
    RetractTail,
    PrepareScene,
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
pub(super) enum TargetPreparePhase {
    Reset,
    Blocked,
    Enumerate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CandidatePreparePhase {
    Reset,
    Enumerate,
    Order,
    Finish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AcceptPhase {
    Validate,
    CheckAttractions,
    BuildVortices,
    BeginSpatial,
    StepSpatial,
    InstallLookup,
    AppendTargets,
    Commit,
}

pub type FillMeshMap = semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, CollisionBody>;

#[derive(semio_framework_value::RetireOwned)]
pub struct FillPreparationRoots {
    pub scene: Arc<SceneConfig>,
    pub meshes: Arc<FillMeshMap>,
}

impl FillPreparationRoots {
    pub fn new(scene: Arc<SceneConfig>, meshes: Arc<FillMeshMap>) -> Self {
        Self { scene, meshes }
    }
}

#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct FixedSceneOwner {
    pub objects: FixedOwnerVec<EngineSceneObject, DOCUMENT_OBJECT_SLOTS>,
    pub attractions: FixedOwnerVec<AttractionProps, DOCUMENT_ATTRACTION_SLOTS>,
    pub target_volumes: FixedOwnerVec<WorldVolumeProps, DOCUMENT_VOLUME_SLOTS>,
}

impl FixedSceneOwner {
    pub fn new() -> Self {
        Self { objects: FixedOwnerVec::new(), attractions: FixedOwnerVec::new(), target_volumes: FixedOwnerVec::new() }
    }

}

#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct FixedCatalogOwner {
    pub objects: FixedOwnerVec<ObjectKind, DOCUMENT_KIND_SLOTS>,
    pub vortices: FixedOwnerVec<VortexKindCatalog, DOCUMENT_KIND_SLOTS>,
    pub cables: FixedOwnerVec<CableKindCatalog, DOCUMENT_KIND_SLOTS>,
}

impl FixedCatalogOwner {
    pub fn new() -> Self {
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

pub(super) struct FillSceneView<'a> {
    pub base: &'a FixedSceneOwner,
    pub(super) appended: &'a [EngineSceneObject],
}

impl BrushSceneView for FillSceneView<'_> {
    fn object_count(&self) -> usize {
        self.base.objects.len() + self.appended.len()
    }

    fn find_object_kind(&self, kind_id: &str) -> Option<&EngineSceneObject> {
        self.base.objects.iter().chain(self.appended).find(|object| object.object_kind.as_deref() == Some(kind_id))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PreparationCapacityBranch {
    EngineSceneObjects,
    SceneAttractions,
    SceneTargetVolumes,
    Meshes,
    CatalogObjects,
    CatalogVortices,
    CatalogCables,
    KindCompatibility,
    ObjectWeights,
    VortexWeights,
}

impl PreparationCapacityBranch {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::EngineSceneObjects => "scene_snapshot-objects",
            Self::SceneAttractions => "scene-attractions",
            Self::SceneTargetVolumes => "scene-target-volumes",
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
pub(super) struct PreparationCapacityRefusal {
    pub(super) branch: PreparationCapacityBranch,
    pub(super) omitted_index: usize,
    pub(super) published: bool,
}

impl PreparationCapacityRefusal {
    /// 🏷️ `preparation-capacity:<branch>:<limit>` — the refused document-scale branch and the exact
    /// capacity its fixed page declares, so the published diagnostic never hides which limit bound.
    pub(super) fn diagnostic(self) -> String {
        format!("preparation-capacity:{}:{}", self.branch.label(), self.omitted_index)
    }
}

/// 🚧️ Preflights every document-scale owner the preparation stages install into, each against the
/// capacity its own fixed page declares — objects and attractions at `DOCUMENT_OBJECT_SLOTS` /
/// `DOCUMENT_ATTRACTION_SLOTS` (the scene plus the room a plan can still claim), catalogs, compatibility
/// rows, mesh urls and kind weights at `DOCUMENT_KIND_SLOTS`. The refusal carries that capacity so
/// the published diagnostic names the branch and the limit it exceeded.
pub fn preparation_capacity_refusal(roots: &FillPreparationRoots) -> Option<PreparationCapacityRefusal> {
    let catalogs = roots.scene.kind_catalogs.as_ref();
    let (branch, capacity) = [
        (PreparationCapacityBranch::EngineSceneObjects, roots.scene.scene_snapshot.objects.len(), DOCUMENT_OBJECT_SLOTS),
        (PreparationCapacityBranch::SceneAttractions, roots.scene.scene_snapshot.attractions.len(), DOCUMENT_ATTRACTION_SLOTS),
        (PreparationCapacityBranch::SceneTargetVolumes, roots.scene.scene_snapshot.target_volumes.len(), DOCUMENT_VOLUME_SLOTS),
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

pub struct FillBuilder {
    pub base: FixedSceneOwner,
    pub(super) preparation_roots: Option<FillPreparationRoots>,
    pub(super) preparation_cursor: usize,
    pub(super) preparation_inner_cursor: usize,
    pub(super) preparation_spatial: Option<CollisionIndexMutation>,
    /// 🎚️ Live withdrawal of ONE unapplied placement during a weight-only replan — the same
    /// resumable index mutation shape as `preparation_spatial`, so the discard never scans.
    pub(super) tail_removal: Option<CollisionIndexRemoval>,
    pub preparation_capacity_refusal: Option<PreparationCapacityRefusal>,
    pub applied_count: usize,
    pub sequence: Vec<BrushPlacePayload>,
    pub appended_objects: Vec<EngineSceneObject>,
    pub appended_attractions: Vec<AttractionProps>,
    pub placed: Vec<PlacedCollisionEntry>,
    pub placed_lookup: FixedOwnerMap<String, usize, DOCUMENT_OBJECT_SLOTS>,
    /// 🗄️ Keeps the bookkeeping page width: no stage installs a per-target-vortex candidate list
    /// yet, so this owner exists for the census/retirement walk only and never reaches document
    /// scale. Wiring a real cache means widening it to `DOCUMENT_VORTEX_SLOTS` in the same move.
    pub candidate_cache: FixedOwnerMap<String, Vec<BrushCompatibleCandidate>>,
    pub rng_state: u32,
    pub max_count: usize,
    /// 🪜️ Where a running tail discard stops: the applied prefix for a weight replan, the newly
    /// requested count for a lowered ask.
    pub(super) tail_floor: usize,
    /// 🔎️ Whether the plan ever built a pose, which tells a bare document apart from one whose every
    /// candidate was refused.
    pub(super) ever_constructed: bool,
    pub operation: Operation,
    pub stage: FillJobStage,
    pub catalogs: FixedCatalogOwner,
    pub weights: RetainedBrushKindWeights,
    pub kind_compatibility: FixedOwnerVec<KindCompatEntry, DOCUMENT_KIND_SLOTS>,
    pub(super) host_rules: BrushHostRules,
    pub contact_tolerance: f64,
    pub meshes: FixedOwnerMap<String, CollisionBody, DOCUMENT_KIND_SLOTS>,
    pub(super) spatial_index: CollisionSpatialIndex,
    /// 🌀️ The pool of free vortices, drawn from by `target_weights` (a vortex kind's distribution weight, zero once the
    /// vortex is consumed by a placement or marked) through the Fenwick tree `target_tree`.
    pub targets: Vec<BrushFillVortexTarget>,
    pub target_weights: Vec<f64>,
    pub(super) target_tree: Vec<f64>,
    pub(super) targets_ready: bool,
    pub(super) current_target_index: Option<usize>,
    pub(super) target_prepare_phase: TargetPreparePhase,
    pub(super) blocked_vortex_ids: FixedOwnerSet<String, DOCUMENT_VORTEX_SLOTS>,
    pub(super) target_attraction_cursor: usize,
    pub(super) target_object_cursor: usize,
    pub(super) target_vortex_cursor: usize,
    pub current_target: Option<BrushFillVortexTarget>,
    pub(super) candidates: Vec<BrushCompatibleCandidate>,
    pub(super) candidate_cursor: usize,
    pub(super) candidate_prepare_phase: CandidatePreparePhase,
    pub(super) candidate_kind_cursor: usize,
    pub(super) candidate_vortex_cursor: usize,
    pub(super) candidate_prepare_cursor: usize,
    pub(super) candidate_seen: FixedOwnerSet<String, DOCUMENT_CANDIDATE_SLOTS>,
    pub(super) candidate_raw: Vec<BrushCompatibleCandidate>,
    pub(super) candidate_weights: Vec<f64>,
    pub(super) candidate_tree: Vec<f64>,
    pub(super) candidate_remaining: usize,
    pub current_preview: Option<BrushPreviewState>,
    pub(super) broad_phase_query: Option<CollisionQueryCursor>,
    pub(super) broad_phase_cursor: usize,
    pub(super) broad_phase_bounds: Option<CollisionAabb>,
    pub(super) collision: Option<CollisionPenetrationState>,
    pub(super) accept_phase: AcceptPhase,
    pub(super) accept_attraction_cursor: usize,
    pub(super) accept_vortex_cursor: usize,
    pub(super) pending_payload: Option<BrushPlacePayload>,
    pub(super) pending_object: Option<EngineSceneObject>,
    pub(super) pending_attraction: Option<AttractionProps>,
    pub(super) pending_spatial: Option<CollisionIndexMutation>,
    pub last_rejection: Option<String>,
    pub(super) fixed_rejection: Option<FillRetiredOwner>,
    pub collection_over_capacity: bool,
    pub transition_count: u64,
    pub(super) rejected_count: u64,
    close_started: bool,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<FillClosingOwners>,
    pub(super) closing: bool,
    /// 🛰️ Observations for a fill run job; `None` for the retained session lane, which never reads them.
    pub(super) run_events: Option<Vec<FillRunEvent>>,
    /// 👻️ A constructed candidate still owes the run its verdict.
    pub(super) run_candidate_live: bool,
}

#[derive(semio_framework_value::RetireOwned)]
struct FillClosingOwners {
    base: FixedSceneOwner,
    preparation_roots: Option<FillPreparationRoots>,
    preparation_spatial: Option<CollisionIndexMutation>,
    tail_removal: Option<CollisionIndexRemoval>,
    sequence: Vec<BrushPlacePayload>,
    appended_objects: Vec<EngineSceneObject>,
    appended_attractions: Vec<AttractionProps>,
    placed: Vec<PlacedCollisionEntry>,
    placed_lookup: FixedOwnerMap<String, usize, DOCUMENT_OBJECT_SLOTS>,
    candidate_cache: FixedOwnerMap<String, Vec<BrushCompatibleCandidate>>,
    catalogs: FixedCatalogOwner,
    weights: RetainedBrushKindWeights,
    kind_compatibility: FixedOwnerVec<KindCompatEntry, DOCUMENT_KIND_SLOTS>,
    meshes: FixedOwnerMap<String, CollisionBody, DOCUMENT_KIND_SLOTS>,
    spatial_index: CollisionSpatialIndex,
    targets: Vec<BrushFillVortexTarget>,
    target_weights: Vec<f64>,
    target_tree: Vec<f64>,
    blocked_vortex_ids: FixedOwnerSet<String, DOCUMENT_VORTEX_SLOTS>,
    current_target: Option<BrushFillVortexTarget>,
    candidates: Vec<BrushCompatibleCandidate>,
    candidate_seen: FixedOwnerSet<String, DOCUMENT_CANDIDATE_SLOTS>,
    candidate_raw: Vec<BrushCompatibleCandidate>,
    candidate_weights: Vec<f64>,
    candidate_tree: Vec<f64>,
    current_preview: Option<BrushPreviewState>,
    broad_phase_query: Option<CollisionQueryCursor>,
    collision: Option<CollisionPenetrationState>,
    pending_payload: Option<BrushPlacePayload>,
    pending_object: Option<EngineSceneObject>,
    pending_attraction: Option<AttractionProps>,
    pending_spatial: Option<CollisionIndexMutation>,
    last_rejection: Option<String>,
    fixed_rejection: Option<FillRetiredOwner>,
    run_events: Option<Vec<FillRunEvent>>,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct RetainedBrushKindWeights {
    pub object_weights: FixedOwnerMap<String, f64, DOCUMENT_KIND_SLOTS>,
    pub vortex_weights: FixedOwnerMap<String, f64, DOCUMENT_KIND_SLOTS>,
}

impl RetainedBrushKindWeights {
    pub fn new() -> Self {
        Self { object_weights: FixedOwnerMap::new(), vortex_weights: FixedOwnerMap::new() }
    }

    fn object_value(&self, id: &str) -> f64 {
        self.object_weights.get(id).copied().unwrap_or(1.0)
    }

    fn vortex_value(&self, id: &str) -> f64 {
        self.vortex_weights.get(id).copied().unwrap_or(1.0)
    }
}

pub(super) fn retained_fill_vortex_target_weight(target: &BrushFillVortexTarget, weights: &RetainedBrushKindWeights) -> f64 {
    let object_kind = target.object_kind.as_deref().unwrap_or("");
    let vortex_kind = target.vortex_kind.as_deref().unwrap_or("");
    retained_joint_weight(weights, object_kind, vortex_kind)
}

pub(super) fn retained_candidate_suggestion_weight(candidate: &BrushCompatibleCandidate, weights: &RetainedBrushKindWeights, catalogs: &impl BrushCatalogView) -> f64 {
    let vortex_kind = catalogs.objects().iter().find(|kind| kind.id == candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|template| template.vortex_kind.as_deref()).unwrap_or("");
    retained_joint_weight(weights, &candidate.object_kind_id, vortex_kind)
}

fn retained_joint_weight(weights: &RetainedBrushKindWeights, object_kind_id: &str, vortex_kind_id: &str) -> f64 {
    let sep = semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_JOINT_WEIGHT_SEP;
    let composite = format!("{object_kind_id}{sep}{vortex_kind_id}");
    if let Some(weight) = weights.vortex_weights.get(&composite) {
        return *weight;
    }
    weights.object_value(object_kind_id) * weights.vortex_value(vortex_kind_id)
}
