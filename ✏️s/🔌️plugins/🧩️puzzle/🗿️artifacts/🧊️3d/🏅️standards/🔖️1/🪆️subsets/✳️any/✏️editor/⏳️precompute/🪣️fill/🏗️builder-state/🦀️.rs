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
pub enum TargetPreparePhase {
    Reset,
    Blocked,
    Enumerate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CandidatePreparePhase {
    Reset,
    Enumerate,
    Order,
    Finish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcceptPhase {
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

pub struct FillPreparationRoots {
    pub scene: Arc<SceneConfig>,
    pub meshes: Arc<FillMeshMap>,
}

impl FillPreparationRoots {
    pub fn new(scene: Arc<SceneConfig>, meshes: Arc<FillMeshMap>) -> Self {
        Self { scene, meshes }
    }

    pub fn into_lease(self) -> FillPreparationLease {
        FillPreparationLease { scene: semio_framework_value::retirement::arc_custody::ArcLease::new(self.scene), meshes: semio_framework_value::retirement::arc_custody::ArcLease::new(self.meshes) }
    }
}

/// 🔗️ The shared roots of a closing fill builder: read leases that retire their Arc backing before the last payload.
#[derive(semio_framework_value::RetireOwned)]
pub struct FillPreparationLease {
    pub scene: semio_framework_value::retirement::arc_custody::ArcLease<SceneConfig>,
    pub meshes: semio_framework_value::retirement::arc_custody::ArcLease<FillMeshMap>,
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
pub struct FixedCatalogOwner {
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

pub struct FillSceneView<'a> {
    pub base: &'a FixedSceneOwner,
    pub appended: &'a [EngineSceneObject],
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
pub enum PreparationCapacityBranch {
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
    pub fn label(self) -> &'static str {
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
pub struct PreparationCapacityRefusal {
    pub branch: PreparationCapacityBranch,
    pub omitted_index: usize,
    pub published: bool,
}

impl PreparationCapacityRefusal {
    /// 🏷️ `preparation-capacity:<branch>:<limit>` — the refused document-scale branch and the exact
    /// capacity its fixed page declares, so the published diagnostic never hides which limit bound.
    pub fn diagnostic(self) -> String {
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
    pub preparation_roots: Option<FillPreparationRoots>,
    pub preparation_cursor: usize,
    pub preparation_inner_cursor: usize,
    pub preparation_spatial: Option<CollisionIndexMutation>,
    /// 🎚️ Live withdrawal of ONE unapplied placement during a weight-only replan — the same
    /// resumable index mutation shape as `preparation_spatial`, so the discard never scans.
    pub tail_removal: Option<CollisionIndexRemoval>,
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
    pub tail_floor: usize,
    /// 🔎️ Whether the plan ever built a pose, which tells a bare document apart from one whose every
    /// candidate was refused.
    pub ever_constructed: bool,
    pub operation: Operation,
    pub stage: FillJobStage,
    pub catalogs: FixedCatalogOwner,
    pub weights: RetainedBrushKindWeights,
    pub kind_compatibility: FixedOwnerVec<KindCompatEntry, DOCUMENT_KIND_SLOTS>,
    pub host_rules: BrushHostRules,
    pub contact_tolerance: f64,
    pub meshes: FixedOwnerMap<String, CollisionBody, DOCUMENT_KIND_SLOTS>,
    pub spatial_index: CollisionSpatialIndex,
    /// 🌀️ The pool of free vortices, drawn from by `target_weights` (a vortex kind's distribution weight, zero once the
    /// vortex is consumed by a placement or marked) through the Fenwick tree `target_tree`.
    pub targets: Vec<BrushFillVortexTarget>,
    pub target_weights: Vec<f64>,
    pub target_tree: Vec<f64>,
    pub targets_ready: bool,
    pub current_target_index: Option<usize>,
    pub target_prepare_phase: TargetPreparePhase,
    pub blocked_vortex_ids: FixedOwnerSet<String, DOCUMENT_VORTEX_SLOTS>,
    pub target_attraction_cursor: usize,
    pub target_object_cursor: usize,
    pub target_vortex_cursor: usize,
    pub current_target: Option<BrushFillVortexTarget>,
    pub candidates: Vec<BrushCompatibleCandidate>,
    pub candidate_cursor: usize,
    pub candidate_prepare_phase: CandidatePreparePhase,
    pub candidate_kind_cursor: usize,
    pub candidate_vortex_cursor: usize,
    pub candidate_prepare_cursor: usize,
    pub candidate_seen: FixedOwnerSet<String, DOCUMENT_CANDIDATE_SLOTS>,
    pub candidate_raw: Vec<BrushCompatibleCandidate>,
    pub candidate_weights: Vec<f64>,
    pub candidate_tree: Vec<f64>,
    pub candidate_remaining: usize,
    pub current_preview: Option<BrushPreviewState>,
    pub broad_phase_query: Option<CollisionQueryCursor>,
    pub broad_phase_cursor: usize,
    pub broad_phase_bounds: Option<CollisionAabb>,
    pub collision: Option<CollisionPenetrationState>,
    pub accept_phase: AcceptPhase,
    pub accept_attraction_cursor: usize,
    pub accept_vortex_cursor: usize,
    pub pending_payload: Option<BrushPlacePayload>,
    pub pending_object: Option<EngineSceneObject>,
    pub pending_attraction: Option<AttractionProps>,
    pub pending_spatial: Option<CollisionIndexMutation>,
    pub last_rejection: Option<String>,
    pub fixed_rejection: Option<FillRetiredOwner>,
    pub collection_over_capacity: bool,
    pub transition_count: u64,
    pub rejected_count: u64,
    pub close_started: bool,
    pub close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<FillClosingOwners>,
    pub closing: bool,
    /// 🛰️ Observations for a fill run job; `None` for the retained session lane, which never reads them.
    pub run_events: Option<Vec<FillRunEvent>>,
    /// 👻️ A constructed candidate still owes the run its verdict.
    pub run_candidate_live: bool,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct FillClosingOwners {
    pub base: FixedSceneOwner,
    pub preparation_roots: Option<FillPreparationLease>,
    pub preparation_spatial: Option<CollisionIndexMutation>,
    pub tail_removal: Option<CollisionIndexRemoval>,
    pub sequence: Vec<BrushPlacePayload>,
    pub appended_objects: Vec<EngineSceneObject>,
    pub appended_attractions: Vec<AttractionProps>,
    pub placed: Vec<PlacedCollisionEntry>,
    pub placed_lookup: FixedOwnerMap<String, usize, DOCUMENT_OBJECT_SLOTS>,
    pub candidate_cache: FixedOwnerMap<String, Vec<BrushCompatibleCandidate>>,
    pub catalogs: FixedCatalogOwner,
    pub weights: RetainedBrushKindWeights,
    pub kind_compatibility: FixedOwnerVec<KindCompatEntry, DOCUMENT_KIND_SLOTS>,
    pub meshes: FixedOwnerMap<String, CollisionBody, DOCUMENT_KIND_SLOTS>,
    pub spatial_index: CollisionSpatialIndex,
    pub targets: Vec<BrushFillVortexTarget>,
    pub target_weights: Vec<f64>,
    pub target_tree: Vec<f64>,
    pub blocked_vortex_ids: FixedOwnerSet<String, DOCUMENT_VORTEX_SLOTS>,
    pub current_target: Option<BrushFillVortexTarget>,
    pub candidates: Vec<BrushCompatibleCandidate>,
    pub candidate_seen: FixedOwnerSet<String, DOCUMENT_CANDIDATE_SLOTS>,
    pub candidate_raw: Vec<BrushCompatibleCandidate>,
    pub candidate_weights: Vec<f64>,
    pub candidate_tree: Vec<f64>,
    pub current_preview: Option<BrushPreviewState>,
    pub broad_phase_query: Option<CollisionQueryCursor>,
    pub collision: Option<CollisionPenetrationState>,
    pub pending_payload: Option<BrushPlacePayload>,
    pub pending_object: Option<EngineSceneObject>,
    pub pending_attraction: Option<AttractionProps>,
    pub pending_spatial: Option<CollisionIndexMutation>,
    pub last_rejection: Option<String>,
    pub fixed_rejection: Option<FillRetiredOwner>,
    pub run_events: Option<Vec<FillRunEvent>>,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct RetainedBrushKindWeights {
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

pub fn retained_fill_vortex_target_weight(target: &BrushFillVortexTarget, weights: &RetainedBrushKindWeights) -> f64 {
    let object_kind = target.object_kind.as_deref().unwrap_or("");
    let vortex_kind = target.vortex_kind.as_deref().unwrap_or("");
    retained_joint_weight(weights, object_kind, vortex_kind)
}

pub fn retained_candidate_suggestion_weight(candidate: &BrushCompatibleCandidate, weights: &RetainedBrushKindWeights, catalogs: &impl BrushCatalogView) -> f64 {
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
