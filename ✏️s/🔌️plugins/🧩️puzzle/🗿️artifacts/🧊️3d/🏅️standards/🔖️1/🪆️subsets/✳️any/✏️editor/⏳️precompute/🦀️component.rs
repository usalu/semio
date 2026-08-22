//! ⏳️ Puzzle 3d play app — the precompute session: the scene the host syncs in, the registered
//! collision meshes, the two independent background lanes (brush-candidate caching and fill
//! planning), and `dispatch`, which drives `Puzzle3dEngineCommand`/`Puzzle3dEngineOutcome` (schema
//! types, `crate::artifacts::puzzle3d::schema`) through the session. The rules the lanes consult live
//! in `🖌️brush/🦀️component.rs`, the geometry in `📐️geometry/🦀️component.rs`, the fill plan's own state
//! in `🪣️fill/🦀️component.rs`. Rehomed from the former `⚙️engine/⏳️session` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a puzzle-3d artifact is a schema plus an io
//! system, never an engine — this interactive brush/fill session is the app's own state machine over
//! that schema, not artifact behaviour.

//#region 🔖️Reexports
pub use crate::editor::puzzle3d::precompute::brush::apply_brush_placement_to_fixture;
//#endregion 🔖️Reexports

//#region 🔖️Constants
/// ⏳️ Default cap on how many objects one fill session may plan — was `⚙️engine`'s own
/// `FILL_COUNT_MAX`; distinct from (and not to be confused with) the UI-facing
/// `crate::editor::puzzle3d::PUZZLE3D_FILL_COUNT_MAX` slider clamp.
pub(crate) const FILL_COUNT_MAX: usize = 1000;
//#endregion 🔖️Constants

use crate::artifacts::puzzle3d::schema::{
    puzzle3d_vortex_full_id, BrushCollisionFreeResult, BrushCompatibleCandidate, BrushPlacePayload, BrushPreviewState, FillBuildProgress, FillProgressSummary, Fixture, KindCatalogBundle, PrecomputeLane, Puzzle3dEngineCommand, Puzzle3dEngineOutcome,
    SceneConfig,
};
use crate::artifacts::puzzle3d::Puzzle3dError;
use crate::editor::puzzle3d::precompute::brush::{
    brush_candidate_suggestion_weight, brush_compatible_candidates, brush_preview_from_candidate, brush_target_vortex_allows_suggestion, enumerate_brush_fill_vortex_targets, resolve_object_kind_mesh_url, vortex_world_from_object,
    AttractionVortexContext, TargetVortexWorld,
};
use crate::editor::puzzle3d::precompute::fill::{FillBuilder, PlacedCollisionEntry};
use crate::editor::puzzle3d::precompute::geometry::{pose_isometry, world_bounds, CollisionBody, CollisionOverlapState, CollisionStepContext, CollisionStepResult};
use semio_framework_job::{default_now_ms, drive_step, root_cancel_token, CancelToken, Generation, InteractiveStage, Operation, RevisionId, StepBudget, StepOutcome};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::pin::Pin;

//#region 💼️FillJobBridge
pub(crate) const FILL_JOB_KIND: &str = "semio.puzzle3d.fill";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct FillJobRequest {
    job: u64,
    operation: u64,
    generation: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FillObservation {
    generation: u64,
    sequence: u64,
    available: u32,
    done: bool,
}

struct FillJobSlice {
    progress: Option<Vec<u8>>,
    checkpoint: Option<Vec<u8>>,
    done: bool,
}
//#endregion 💼️FillJobBridge

//#region 🔖️Clock
/// ⏱️ Monotonic-enough wall clock in milliseconds for precompute step budgeting. WASI P2 program
/// components (`target_env = "p2"`, this artifact's real deployment target) and native (tests) share
/// the `Instant`-based path below. Plain `wasm32-unknown-unknown` has no OS clock and this headless
/// engine node must not depend on `js-sys`/`wasm-bindgen` to bridge to `Date.now()` — it freezes at
/// 0.0 instead, degrading step budgeting to the step-count budget alone. The one real
/// `wasm32-unknown-unknown` build of this crate (a Storybook DSL-text-parsing wasm bundle) never
/// drives the precompute session, so this never actually matters at runtime.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
fn puzzle3d_now_ms() -> f64 {
    0.0
}

#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
fn puzzle3d_now_ms() -> f64 {
    use std::sync::OnceLock;
    static START: OnceLock<std::time::Instant> = OnceLock::new();
    START.get_or_init(std::time::Instant::now).elapsed().as_secs_f64() * 1000.0
}

/// 🪫️ Soft wall-clock ceiling for a single `precompute_step` call — a `FillStep` task's own collision
/// search cost is otherwise unbounded per call, so this only caps how many *additional* tasks beyond
/// the first are attempted once time runs out; the first task in a call always runs so a tick always
/// makes forward progress.
const PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS: f64 = 2.0;
//#endregion 🔖️Clock

//#region 🔖️Engine
pub(crate) struct Puzzle3dCollision {
    pub(crate) scene: Option<SceneConfig>,
    /// 🧊️ Raw JSON of the last `set_scene` call, so a resync with byte-identical config (every action
    /// re-syncs the session, see the app's `sync_precompute_session`) can skip `rebuild_queue` instead
    /// of wiping `brush_cache`/`fill`/`queue` and restarting suggestion+fill precompute from zero.
    scene_json: Option<String>,
    meshes: HashMap<String, CollisionBody>,
    mesh_is_fallback: HashMap<String, bool>,
    pub(crate) brush_cache: HashMap<String, BrushCollisionFreeResult>,
    pub(crate) brush_queue: VecDeque<String>,
    fill_steps_remaining: usize,
    pub(crate) fill: Option<FillBuilder>,
    fill_cancel: CancelToken,
    fill_revision: u64,
    fill_generation: u64,
    fill_preview_sequence: u64,
}

impl Puzzle3dCollision {
    pub(crate) fn new() -> Self {
        Self {
            scene: None,
            scene_json: None,
            meshes: HashMap::new(),
            mesh_is_fallback: HashMap::new(),
            brush_cache: HashMap::new(),
            brush_queue: VecDeque::new(),
            fill_steps_remaining: 0,
            fill: None,
            fill_cancel: root_cancel_token(),
            fill_revision: 0,
            fill_generation: 0,
            fill_preview_sequence: 0,
        }
    }

    fn fill_lane_active(&self) -> bool {
        self.fill.is_some() && self.fill_steps_remaining > 0
    }

    fn brush_lane_active(&self) -> bool {
        !self.brush_queue.is_empty()
    }

    fn re_enqueue_brush_targets(&mut self) {
        let Some(scene) = &self.scene else {
            return;
        };
        for target in enumerate_brush_fill_vortex_targets(&scene.fixture) {
            if !self.brush_queue.iter().any(|id| id == &target.full_id) {
                self.brush_queue.push_back(target.full_id);
            }
        }
    }

    fn rebuild_queue(&mut self) {
        self.fill_cancel.cancel_now();
        self.fill_cancel = root_cancel_token();
        self.fill_revision = self.fill_revision.wrapping_add(1);
        self.fill_generation = self.fill_generation.wrapping_add(1);
        self.fill_preview_sequence = 0;
        self.brush_queue.clear();
        self.brush_cache.clear();
        self.fill_steps_remaining = 0;
        if let Some(scene) = &self.scene {
            for target in enumerate_brush_fill_vortex_targets(&scene.fixture) {
                self.brush_queue.push_back(target.full_id);
            }
            self.fill_steps_remaining = FILL_COUNT_MAX;
            let catalogs = scene.kind_catalogs.clone().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
            let operation = Operation::new(semio_framework_job::allocate_operation_id(), RevisionId(self.fill_revision), Generation(self.fill_generation), scene.seed as u64);
            let mut fill = FillBuilder::new(scene.fixture.clone(), scene.seed, &self.meshes, &catalogs);
            fill.configure(operation, scene.weights.clone(), scene.kind_compatibility.clone(), scene.host_rules.clone(), scene.fixture.target_volumes.clone(), scene.overlap_budget);
            self.fill = Some(fill);
        } else {
            self.fill = None;
        }
    }

    /// 🎚️ Distribution-weight edits must not `rebuild_queue()` — applied fill objects stay, only the
    /// unapplied planning tail is discarded and re-enqueued for background `fillBuildTick` planning.
    fn soft_replan_fill_tail(&mut self) {
        let Some(fill) = &mut self.fill else {
            return;
        };
        let applied = fill.applied_count;
        fill.sequence.truncate(applied);
        fill.appended_objects.truncate(applied);
        fill.appended_attractions.truncate(applied);
        fill.fixture = fill.base.clone();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());
        fill.rebuild_collision_index();
        fill.candidate_cache.clear();
        fill.stalled = false;
        self.fill_steps_remaining = fill.max_count.saturating_sub(applied);
    }

    fn refresh_fill_job(&mut self, refresh_meshes: bool) {
        let Some(scene) = self.scene.clone() else { return };
        let meshes = refresh_meshes.then(|| self.meshes.clone());
        self.fill_cancel.cancel_now();
        self.fill_cancel = root_cancel_token();
        self.fill_generation = self.fill_generation.wrapping_add(1);
        self.fill_preview_sequence = 0;
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), RevisionId(self.fill_revision), Generation(self.fill_generation), scene.seed as u64);
        if let Some(fill) = &mut self.fill {
            if let Some(meshes) = &meshes {
                fill.refresh_meshes(meshes);
            } else {
                fill.restart_search();
            }
            fill.configure(operation, scene.weights, scene.kind_compatibility, scene.host_rules, scene.fixture.target_volumes, scene.overlap_budget);
        }
    }

    pub(crate) fn update_kind_weights(&mut self, object_weights: std::collections::BTreeMap<String, f64>, vortex_weights: std::collections::BTreeMap<String, f64>) {
        if let Some(scene) = &mut self.scene {
            scene.weights.object_weights = object_weights;
            scene.weights.vortex_weights = vortex_weights;
            if let Ok(normalized) = serde_json::to_string(scene) {
                self.scene_json = Some(normalized);
            }
        }
        self.brush_cache.clear();
        self.soft_replan_fill_tail();
        self.refresh_fill_job(false);
    }

    /// 🪣️ True when `fixture` is the fill plan's base plus zero-or-more applied fill objects — i.e. the
    /// live document after `setFillCount`, which must NOT rebuild the precompute session or the slider
    /// loses its ability to remove/replan those objects.
    fn is_fill_applied_projection(fixture: &Fixture, fill: &FillBuilder) -> bool {
        let plan_objects: std::collections::HashSet<&str> = fill.appended_objects.iter().map(|object| object.id.as_str()).collect();
        let plan_attractions: std::collections::HashSet<&str> = fill.appended_attractions.iter().map(|attraction| attraction.id.as_str()).collect();
        let base_objects: std::collections::HashSet<&str> = fill.base.objects.iter().map(|object| object.id.as_str()).collect();
        let base_attractions: std::collections::HashSet<&str> = fill.base.attractions.iter().map(|attraction| attraction.id.as_str()).collect();
        let base_volumes: std::collections::HashSet<&str> = fill.base.target_volumes.iter().map(|volume| volume.id.as_str()).collect();
        let incoming_objects: std::collections::HashSet<&str> = fixture.objects.iter().map(|object| object.id.as_str()).filter(|id| !plan_objects.contains(id)).collect();
        let incoming_attractions: std::collections::HashSet<&str> = fixture.attractions.iter().map(|attraction| attraction.id.as_str()).filter(|id| !plan_attractions.contains(id)).collect();
        let incoming_volumes: std::collections::HashSet<&str> = fixture.target_volumes.iter().map(|volume| volume.id.as_str()).collect();
        incoming_objects == base_objects && incoming_attractions == base_attractions && incoming_volumes == base_volumes
    }

    fn strip_fill_plan_from_fixture(fixture: &mut Fixture, fill: &FillBuilder) {
        let plan_objects: std::collections::HashSet<&str> = fill.appended_objects.iter().map(|object| object.id.as_str()).collect();
        let plan_attractions: std::collections::HashSet<&str> = fill.appended_attractions.iter().map(|attraction| attraction.id.as_str()).collect();
        fixture.objects.retain(|object| !plan_objects.contains(object.id.as_str()));
        fixture.attractions.retain(|attraction| !plan_attractions.contains(attraction.id.as_str()));
    }

    pub(crate) fn set_scene(&mut self, json: &str) -> Result<(), Puzzle3dError> {
        let mut scene: SceneConfig = serde_json::from_str(json)?;
        // 🪣️ After the fill slider materializes objects into the document, every incidental action
        // (hover, pick, mesh register sync, …) re-feeds that applied projection here. Treating it as a
        // brand-new scene used to `rebuild_queue()` and bake the filled objects into `fill.base`, after
        // which the slider could neither remove them nor replan a fresh tail.
        if self.fill.as_ref().is_some_and(|fill| Self::is_fill_applied_projection(&scene.fixture, fill)) {
            if let Some(fill) = &self.fill {
                Self::strip_fill_plan_from_fixture(&mut scene.fixture, fill);
            }
            let normalized = serde_json::to_string(&scene)?;
            if let Some(current) = &mut self.scene {
                current.overlap_budget = scene.overlap_budget;
                current.seed = scene.seed;
                current.weights = scene.weights;
                current.kind_catalogs = scene.kind_catalogs;
                current.kind_compatibility = scene.kind_compatibility;
                current.host_rules = scene.host_rules;
            }
            self.scene_json = Some(normalized);
            return Ok(());
        }
        let normalized = serde_json::to_string(&scene)?;
        if self.scene_json.as_deref() == Some(normalized.as_str()) {
            return Ok(());
        }
        self.scene = Some(scene);
        self.scene_json = Some(normalized);
        self.rebuild_queue();
        Ok(())
    }

    fn install_collision_mesh(&mut self, url: String, positions: &[f32], indices: &[u32], is_fallback: bool) {
        let Some(body) = crate::editor::puzzle3d::precompute::geometry::collision_body_from_buffers(positions, indices) else {
            return;
        };
        if !is_fallback && self.mesh_is_fallback.get(&url) == Some(&false) {
            return;
        }
        if is_fallback && self.mesh_is_fallback.get(&url) == Some(&false) {
            return;
        }
        self.meshes.insert(url.clone(), body);
        self.mesh_is_fallback.insert(url, is_fallback);
        self.brush_cache.clear();
        self.soft_replan_fill_tail();
        self.refresh_fill_job(true);
        self.re_enqueue_brush_targets();
    }

    pub(crate) fn register_mesh_fallback(&mut self, url: String, positions: &[f32], indices: &[u32]) {
        self.install_collision_mesh(url, positions, indices, true);
    }

    pub(crate) fn register_mesh(&mut self, url: String, positions: &[f32], indices: &[u32]) {
        self.install_collision_mesh(url, positions, indices, false);
    }

    pub(crate) fn has_mesh(&self, url: &str) -> bool {
        self.meshes.contains_key(url)
    }

    /// 🧊️ Drops a cached brush-candidate entry and re-queues that vortex at the front so a just-opened
    /// suggestion popup is not stuck on a stale empty / pending result.
    pub(crate) fn invalidate_brush_target(&mut self, vortex_full_id: &str) {
        self.brush_cache.remove(vortex_full_id);
        self.brush_queue.retain(|id| id != vortex_full_id);
        self.brush_queue.push_front(vortex_full_id.to_string());
    }

    pub(crate) fn enqueue_brush_target(&mut self, vortex_full_id: &str) {
        if !self.brush_queue.iter().any(|id| id == vortex_full_id) {
            self.brush_queue.push_back(vortex_full_id.to_string());
        }
    }

    /// 🧊️ Recomputes and caches brush candidates for one vortex immediately (used when opening / accepting
    /// the suggestion popup so the UI does not wait on the background queue).
    pub(crate) fn refresh_brush_candidates(&mut self, vortex_full_id: &str) {
        let prior = self.brush_cache.get(vortex_full_id).cloned();
        let resume_from = prior.as_ref().map_or(0, |entry| entry.resume_candidate_index);
        let prior_free = prior.map(|entry| entry.free).unwrap_or_default();
        let deadline = puzzle3d_now_ms() + PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS;
        let result = self.compute_brush_cache_entry_partial(vortex_full_id, resume_from, prior_free, deadline);
        if result.unknown_pending && result.resume_candidate_index > 0 && !self.brush_queue.iter().any(|id| id == vortex_full_id) {
            self.brush_queue.push_front(vortex_full_id.to_string());
        }
        self.brush_cache.insert(vortex_full_id.to_string(), result);
    }

    fn preview_collides(meshes: &HashMap<String, CollisionBody>, preview: &BrushPreviewState, placed: &[PlacedCollisionEntry], overlap_budget: f64, sample_count: usize, deadline_ms: f64) -> Option<bool> {
        struct BrushCollisionContext {
            deadline_ms: f64,
        }
        impl CollisionStepContext for BrushCollisionContext {
            fn is_cancelled(&self) -> bool {
                false
            }
            fn should_yield(&self) -> bool {
                puzzle3d_now_ms() >= self.deadline_ms
            }
            fn consume_fuel(&mut self, _units: u64) {}
        }
        let preview_body = meshes.get(&preview.mesh_url)?;
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let (pmin, pmax) = world_bounds(preview_body, &preview_world);
        let mut context = BrushCollisionContext { deadline_ms };
        for entry in placed {
            let other = meshes.get(&entry.mesh_url)?;
            let (omin, omax) = world_bounds(other, &entry.world);
            if pmax.x() < omin.x() || pmin.x() > omax.x() || pmax.y() < omin.y() || pmin.y() > omax.y() || pmax.z() < omin.z() || pmin.z() > omax.z() {
                continue;
            }
            let mut collision = CollisionOverlapState::new(sample_count, 8, overlap_budget);
            loop {
                match collision.step(&mut context, preview_body, &preview_world, other, &entry.world) {
                    CollisionStepResult::Pending if context.should_yield() => return None,
                    CollisionStepResult::Pending => {}
                    CollisionStepResult::Cancelled => return None,
                    CollisionStepResult::Complete { overlap, .. } if overlap > overlap_budget => return Some(true),
                    CollisionStepResult::Complete { .. } => break,
                }
            }
        }
        Some(false)
    }

    fn brush_collision_free_until(&self, target_full_id: &str, candidates: &[BrushCompatibleCandidate], overlap_budget: f64, resume_from: usize, mut free: Vec<BrushCompatibleCandidate>, deadline_ms: f64) -> BrushCollisionFreeResult {
        let Some(scene) = &self.scene else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: resume_from };
        };
        let empty_catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let catalogs = scene.kind_catalogs.as_ref().unwrap_or(&empty_catalogs);
        let target_obj = scene.fixture.objects.iter().find_map(|o| {
            o.vortices.iter().enumerate().find_map(|(i, v)| {
                let full_id = puzzle3d_vortex_full_id(&o.id, &v.id);
                if full_id == target_full_id {
                    Some((o, i, v))
                } else {
                    None
                }
            })
        });
        let Some((host, vortex_index, _)) = target_obj else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        };
        let Some((position, direction)) = vortex_world_from_object(host, vortex_index) else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        };
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: host.vortices[vortex_index].vortex_kind.clone() };
        let host_id = host.id.clone();
        let placed: Vec<PlacedCollisionEntry> = scene
            .fixture
            .objects
            .iter()
            .filter(|obj| obj.id != host_id)
            .filter_map(|obj| {
                let mesh_url = resolve_object_kind_mesh_url(obj.object_kind.as_deref().unwrap_or(""), catalogs, &scene.fixture)?;
                if !self.meshes.contains_key(&mesh_url) {
                    return None;
                }
                Some(PlacedCollisionEntry { object_id: obj.id.clone(), mesh_url, world: pose_isometry(obj.origin, obj.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &obj.scale) })
            })
            .collect();
        let mut unknown_pending = false;
        for (index, candidate) in candidates.iter().enumerate().skip(resume_from) {
            if puzzle3d_now_ms() >= deadline_ms {
                return BrushCollisionFreeResult { free, unknown_pending: true, resume_candidate_index: index };
            }
            let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
            let Some(preview) = brush_preview_from_candidate(target_full_id, candidate, &target_ctx, world, catalogs, &scene.fixture) else {
                continue;
            };
            if !self.meshes.contains_key(&preview.mesh_url) {
                unknown_pending = true;
                continue;
            }
            match Self::preview_collides(&self.meshes, &preview, &placed, overlap_budget, 1024, deadline_ms) {
                None => unknown_pending = true,
                Some(true) => {}
                Some(false) => free.push(candidate.clone()),
            }
        }
        BrushCollisionFreeResult { free, unknown_pending, resume_candidate_index: 0 }
    }

    fn brush_collision_free(&self, target_full_id: &str, candidates: &[BrushCompatibleCandidate], overlap_budget: f64) -> BrushCollisionFreeResult {
        let deadline = puzzle3d_now_ms() + PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS * 8.0;
        self.brush_collision_free_until(target_full_id, candidates, overlap_budget, 0, Vec::new(), deadline)
    }

    fn compute_brush_cache_entry(&self, target_full_id: &str) -> BrushCollisionFreeResult {
        let Some(scene) = &self.scene else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: 0 };
        };
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let target_obj = scene.fixture.objects.iter().find_map(|o| {
            o.vortices.iter().enumerate().find_map(|(i, v)| {
                let full_id = puzzle3d_vortex_full_id(&o.id, &v.id);
                if full_id == target_full_id {
                    Some((o, i, v))
                } else {
                    None
                }
            })
        });
        let Some((host, _, vortex)) = target_obj else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        };
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone() };
        if !brush_target_vortex_allows_suggestion(vortex.vortex_kind.as_deref(), &scene.weights) {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        }
        let compatible = brush_compatible_candidates(&target_ctx, &catalogs, &scene.kind_compatibility, &scene.host_rules);
        let compatible: Vec<BrushCompatibleCandidate> = compatible.into_iter().filter(|candidate| brush_candidate_suggestion_weight(candidate, &scene.weights, &catalogs) > 0.0).collect();
        self.brush_collision_free(target_full_id, &compatible, scene.overlap_budget)
    }

    pub(crate) fn brush_preview(&self, target_full_id: &str, candidate_index: usize) -> Option<BrushPreviewState> {
        let scene = self.scene.as_ref()?;
        let result = self.brush_cache.get(target_full_id)?;
        if result.unknown_pending && result.free.is_empty() {
            return None;
        }
        if result.free.is_empty() {
            return None;
        }
        let candidate = &result.free[candidate_index % result.free.len()];
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let target_obj = scene.fixture.objects.iter().find_map(|object| {
            object.vortices.iter().enumerate().find_map(|(index, vortex)| {
                let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                if full_id == target_full_id {
                    Some((object, index))
                } else {
                    None
                }
            })
        })?;
        let (host, vortex_index) = target_obj;
        let (position, direction) = vortex_world_from_object(host, vortex_index)?;
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: host.vortices[vortex_index].vortex_kind.clone() };
        let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
        brush_preview_from_candidate(target_full_id, candidate, &target_ctx, world, &catalogs, &scene.fixture)
    }

    pub(crate) fn precompute_step_lane(&mut self, lane: PrecomputeLane, budget: u32) -> bool {
        let start = puzzle3d_now_ms();
        let mut remaining = budget as usize;
        let mut steps_done = 0usize;
        while remaining > 0 {
            if steps_done > 0 && puzzle3d_now_ms() - start >= PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS {
                break;
            }
            match lane {
                PrecomputeLane::Brush => {
                    let Some(full_id) = self.brush_queue.pop_front() else {
                        break;
                    };
                    let prior = self.brush_cache.get(&full_id).cloned();
                    let resume_from = prior.as_ref().map_or(0, |entry| entry.resume_candidate_index);
                    let prior_free = prior.map(|entry| entry.free).unwrap_or_default();
                    let deadline = puzzle3d_now_ms() + PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS;
                    let result = self.compute_brush_cache_entry_partial(&full_id, resume_from, prior_free, deadline);
                    let needs_resume = result.unknown_pending && result.resume_candidate_index > 0;
                    if needs_resume {
                        self.brush_queue.push_front(full_id.clone());
                    }
                    self.brush_cache.insert(full_id, result);
                }
                PrecomputeLane::Fill => {
                    if self.fill_steps_remaining == 0 {
                        break;
                    }
                    let Some(fill) = &mut self.fill else {
                        self.fill_steps_remaining = 0;
                        break;
                    };
                    let operation = fill.operation;
                    let outcome = drive_step(
                        fill,
                        "puzzle3d.fill.step",
                        operation.operation,
                        operation.generation,
                        InteractiveStage::BackgroundStep,
                        StepBudget::new(32, default_now_ms().saturating_add(2)),
                        self.fill_cancel.clone(),
                        default_now_ms,
                        &mut self.fill_preview_sequence,
                    );
                    match outcome {
                        StepOutcome::CheckpointReady(_) => self.fill_steps_remaining = self.fill_steps_remaining.saturating_sub(1),
                        StepOutcome::Complete(_) | StepOutcome::Cancelled | StepOutcome::Fault(_) => self.fill_steps_remaining = 0,
                        StepOutcome::Yield | StepOutcome::PreviewReady(_) => {}
                    }
                }
            }
            steps_done += 1;
            remaining -= 1;
        }
        match lane {
            PrecomputeLane::Brush => self.brush_lane_active(),
            PrecomputeLane::Fill => self.fill_lane_active(),
        }
    }

    fn compute_brush_cache_entry_partial(&self, target_full_id: &str, resume_from: usize, prior_free: Vec<BrushCompatibleCandidate>, deadline_ms: f64) -> BrushCollisionFreeResult {
        let Some(scene) = &self.scene else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: resume_from };
        };
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let target_obj = scene.fixture.objects.iter().find_map(|o| {
            o.vortices.iter().enumerate().find_map(|(i, v)| {
                let full_id = puzzle3d_vortex_full_id(&o.id, &v.id);
                if full_id == target_full_id {
                    Some((o, i, v))
                } else {
                    None
                }
            })
        });
        let Some((host, _, vortex)) = target_obj else {
            return BrushCollisionFreeResult { free: prior_free, unknown_pending: false, resume_candidate_index: 0 };
        };
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone() };
        if !brush_target_vortex_allows_suggestion(vortex.vortex_kind.as_deref(), &scene.weights) {
            return BrushCollisionFreeResult { free: prior_free, unknown_pending: false, resume_candidate_index: 0 };
        }
        let compatible = brush_compatible_candidates(&target_ctx, &catalogs, &scene.kind_compatibility, &scene.host_rules);
        let compatible: Vec<BrushCompatibleCandidate> = compatible.into_iter().filter(|candidate| brush_candidate_suggestion_weight(candidate, &scene.weights, &catalogs) > 0.0).collect();
        self.brush_collision_free_until(target_full_id, &compatible, scene.overlap_budget, resume_from, prior_free, deadline_ms)
    }

    pub(crate) fn precompute_step(&mut self, budget: u32) -> bool {
        let half = (budget / 2).max(1);
        let fill = self.precompute_step_lane(PrecomputeLane::Fill, half);
        let brush = self.precompute_step_lane(PrecomputeLane::Brush, budget.saturating_sub(half));
        fill || brush || self.fill_lane_active() || self.brush_lane_active()
    }

    pub(crate) fn fill_progress_summary(&self) -> FillProgressSummary {
        self.fill.as_ref().map_or(FillProgressSummary { count: 0, applied_count: 0, max_count: FILL_COUNT_MAX, done: true }, |fill| FillProgressSummary {
            count: fill.sequence.len(),
            applied_count: fill.applied_count,
            max_count: fill.max_count,
            done: fill.stalled || fill.sequence.len() >= fill.max_count,
        })
    }

    #[cfg(test)]
    pub(crate) fn work_pending_for_test(&self) -> usize {
        self.brush_queue.len() + self.fill_steps_remaining
    }

    #[cfg(test)]
    pub(crate) fn fill_steps_pending_for_test(&self) -> usize {
        self.fill_steps_remaining
    }

    /// 🔽️ Moving the count down (or up) only changes which prefix of the already-planned sequence is
    /// applied to the document — the plan (`sequence`/`appended_*`/`placed`/`fixture`) is prefix-stable
    /// and is never discarded here, so a jittery drag can never force expensive replanning.
    pub(crate) fn apply_fill_count(&mut self, count: usize) -> Option<Fixture> {
        let fill = self.fill.as_mut()?;
        let count = count.min(fill.sequence.len());
        fill.applied_count = count;
        let mut fixture = fill.base.clone();
        // 🪣️ `revealIndex` is a live-viewport-only hint (see `compose_fill_display`) — never persist it
        // to the committed document projection.
        fixture.objects.extend(fill.appended_objects.iter().take(count).cloned().map(|mut object| {
            object.reveal_index = None;
            object
        }));
        fixture.attractions.extend(fill.appended_attractions.iter().take(count).cloned());
        Some(fixture)
    }

    /// 🪣️ Read-only prefix of the precomputed fill plan for live viewport show/hide — does not mutate
    /// `applied_count`, the queue, or the document projection.
    pub(crate) fn compose_fill_display(&self, count: usize) -> Option<Fixture> {
        let fill = self.fill.as_ref()?;
        let visible = count.min(fill.sequence.len());
        let mut fixture = fill.base.clone();
        fixture.objects.extend(fill.appended_objects.iter().take(visible).cloned());
        fixture.attractions.extend(fill.appended_attractions.iter().take(visible).cloned());
        Some(fixture)
    }

    pub(crate) fn apply_brush_placement(&mut self, payload: &BrushPlacePayload) -> Option<Fixture> {
        let catalogs = self.scene.as_ref()?.kind_catalogs.as_ref()?.clone();
        let fixture = &self.scene.as_ref()?.fixture;
        let next = apply_brush_placement_to_fixture(fixture, payload, &catalogs);
        if next.objects.len() == fixture.objects.len() {
            return None;
        }
        if let Some(scene) = &mut self.scene {
            scene.fixture = next.clone();
        }
        self.rebuild_queue();
        Some(next)
    }
}
//#endregion 🔖️Engine

//#region 🔖️Session
pub struct Puzzle3dPrecomputeSession {
    engine: Puzzle3dCollision,
    fill_job: Option<FillJobRequest>,
    fill_observation: FillObservation,
    last_emitted_fill_checkpoint: RefCell<Vec<u8>>,
}

impl Default for Puzzle3dPrecomputeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Puzzle3dPrecomputeSession {
    pub fn new() -> Self {
        Self { engine: Puzzle3dCollision::new(), fill_job: None, fill_observation: FillObservation::default(), last_emitted_fill_checkpoint: RefCell::new(Vec::new()) }
    }

    pub fn set_scene(&mut self, json: &str) -> Result<(), Puzzle3dError> {
        self.engine.set_scene(json)
    }

    pub fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.engine.register_mesh(url.to_string(), positions, indices);
    }

    pub fn register_mesh_fallback(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.engine.register_mesh_fallback(url.to_string(), positions, indices);
    }

    pub fn has_mesh(&self, url: &str) -> bool {
        self.engine.has_mesh(url)
    }

    pub fn precompute_step(&mut self, budget: u32) -> bool {
        self.engine.precompute_step(budget)
    }

    pub fn precompute_step_lane(&mut self, lane: PrecomputeLane, budget: u32) -> bool {
        self.engine.precompute_step_lane(lane, budget)
    }

    pub fn enqueue_brush_target(&mut self, vortex_full_id: &str) {
        self.engine.enqueue_brush_target(vortex_full_id);
    }

    pub fn invalidate_brush_target(&mut self, vortex_full_id: &str) {
        self.engine.invalidate_brush_target(vortex_full_id);
    }

    pub fn refresh_brush_candidates(&mut self, vortex_full_id: &str) {
        self.engine.refresh_brush_candidates(vortex_full_id);
    }

    /// 🎯️ Typed readout — was a JSON string before the headless-engine-law fix; the app now reads
    /// `.free`/`.unknown_pending` directly.
    pub fn brush_candidates(&self, vortex_full_id: &str) -> BrushCollisionFreeResult {
        self.engine.brush_cache.get(vortex_full_id).cloned().unwrap_or(BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: 0 })
    }

    pub fn brush_preview(&self, vortex_full_id: &str, candidate_index: usize) -> Option<BrushPreviewState> {
        self.engine.brush_preview(vortex_full_id, candidate_index)
    }

    pub fn fill_progress(&self) -> FillBuildProgress {
        self.engine.fill.as_ref().map_or(FillBuildProgress { count: 0, applied_count: 0, max_count: FILL_COUNT_MAX, done: true, appended_objects: vec![], appended_attractions: vec![], sequence: vec![], preview: None }, |f| f.progress())
    }

    pub fn fill_progress_summary(&self) -> FillProgressSummary {
        self.engine.fill_progress_summary()
    }

    /// 🪣️ O(1) planned-count readout for the render/tick hot path — avoids a `fill_progress` round
    /// trip just to read `sequence.len()`.
    pub fn fill_available_count(&self) -> u32 {
        self.engine.fill.as_ref().map_or(0, |fill| fill.sequence.len() as u32)
    }

    pub fn fill_is_done(&self) -> bool {
        self.engine.fill.as_ref().is_none_or(|fill| fill.stalled || fill.sequence.len() >= fill.max_count)
    }

    pub fn fill_checkpoint_bytes(&self) -> Vec<u8> {
        let checkpoint = self.engine.fill.as_ref().map_or_else(Vec::new, FillBuilder::checkpoint_bytes);
        *self.last_emitted_fill_checkpoint.borrow_mut() = checkpoint.clone();
        checkpoint
    }

    pub fn restore_persisted_fill(&mut self, checkpoint: &[u8]) -> bool {
        if checkpoint.is_empty() {
            return false;
        }
        if self.last_emitted_fill_checkpoint.borrow().as_slice() == checkpoint {
            return true;
        }
        let Some(fixture) = self.engine.scene.as_ref().map(|scene| scene.fixture.clone()) else {
            return false;
        };
        let restored = self.engine.fill.as_mut().is_some_and(|fill| fill.restore_checkpoint_for_fixture(checkpoint, &fixture).unwrap_or(false));
        if restored {
            self.engine.fill_steps_remaining = if self.fill_is_done() { 0 } else { FILL_COUNT_MAX };
            *self.last_emitted_fill_checkpoint.borrow_mut() = checkpoint.to_vec();
        }
        restored
    }

    //#region 💼️FillJobBridge
    pub fn enqueue_fill_job(&mut self) -> Option<(u64, Vec<u8>)> {
        if self.fill_job.is_some() || !self.engine.fill_lane_active() {
            return None;
        }
        let operation = self.engine.fill.as_ref()?.operation;
        let request = FillJobRequest { job: semio_framework_job::allocate_operation_id().0, operation: operation.operation.0, generation: operation.generation.0 };
        let input = serde_json::to_vec(&request).expect("fill job request is serializable");
        self.fill_job = Some(request.clone());
        Some((request.job, input))
    }

    pub fn poll_fill_job(&mut self) -> bool {
        let preview = self.engine.fill.as_ref().map(|fill| &fill.preview);
        let current = FillObservation { generation: preview.map_or(0, |value| value.generation), sequence: preview.map_or(0, |value| value.sequence), available: self.fill_available_count(), done: self.fill_is_done() };
        let changed = current != self.fill_observation;
        self.fill_observation = current;
        changed
    }

    fn drive_fill_job(&mut self, request: &FillJobRequest) -> Option<FillJobSlice> {
        let current = self.fill_job.as_ref()?;
        let operation = self.engine.fill.as_ref()?.operation;
        if current.job != request.job || operation.operation.0 != request.operation || operation.generation.0 != request.generation {
            if current.job == request.job {
                self.fill_job = None;
            }
            return None;
        }
        let prior_sequence = self.engine.fill.as_ref().map_or(0, |fill| fill.preview.sequence);
        let prior_available = self.fill_available_count();
        self.engine.precompute_step_lane(PrecomputeLane::Fill, 1);
        let done = !self.engine.fill_lane_active();
        let current_sequence = self.engine.fill.as_ref().map_or(0, |fill| fill.preview.sequence);
        let current_available = self.fill_available_count();
        let visible_change = prior_sequence != current_sequence || prior_available != current_available || done;
        let stable_change = prior_available != current_available || done;
        let progress = visible_change.then(|| serde_json::to_vec(&self.fill_progress()).expect("fill progress is serializable"));
        let checkpoint = stable_change.then(|| self.engine.fill.as_ref().map_or_else(Vec::new, FillBuilder::checkpoint_bytes));
        if done {
            self.fill_job = None;
        }
        Some(FillJobSlice { progress, checkpoint, done })
    }

    #[cfg(test)]
    pub(crate) fn drive_enqueued_fill_job_for_test(&mut self, slices: usize) {
        for _ in 0..slices {
            let Some(request) = self.fill_job.clone() else { break };
            if self.drive_fill_job(&request).is_none() {
                break;
            }
        }
    }

    fn restore_fill_job(&mut self, request: &FillJobRequest, checkpoint: &[u8]) -> bool {
        let Some(current) = &self.fill_job else { return false };
        let Some(fill) = &mut self.engine.fill else { return false };
        if current.job != request.job || fill.operation.operation.0 != request.operation || fill.operation.generation.0 != request.generation {
            return false;
        }
        fill.restore_checkpoint(checkpoint).is_ok()
    }

    //#endregion 💼️FillJobBridge

    /// 🪣️ Read-only prefix of the precomputed fill plan for live viewport show/hide — a query, so it
    /// stays a plain `&self` method rather than routing through `dispatch` (which is `&mut self`,
    /// uniform for the small number of genuinely mutating actions). `Puzzle3dEngineCommand::
    /// ComposeFillDisplay` still exists as a `dispatch`-able alias of this same call for command-log/
    /// wasm-bindgen-wrapper callers that only ever hold `&mut Puzzle3dPrecomputeSession`.
    pub fn compose_fill_display(&self, count: u32) -> Option<Fixture> {
        self.engine.compose_fill_display(count as usize)
    }

    /// 🎯️ Single typed entry point for every mutating (or JSON-carrying-before-this-fix) engine
    /// action — the headless replacement for the old per-action `apply_brush_placement_json`/
    /// `apply_fill_count`/`compose_fill_display`/`update_kind_weights`/`brush_preview_json`
    /// wasm-bindgen methods. Each arm calls the SAME underlying typed `Puzzle3dCollision` method those
    /// JSON wrappers always delegated to — no reimplementation.
    pub fn dispatch(&mut self, command: Puzzle3dEngineCommand) -> Result<Puzzle3dEngineOutcome, Puzzle3dError> {
        match command {
            Puzzle3dEngineCommand::SetScene { scene } => {
                let json = serde_json::to_string(&scene)?;
                self.engine.set_scene(&json)?;
                Ok(Puzzle3dEngineOutcome::Unit)
            }
            Puzzle3dEngineCommand::ApplyBrushPlacement { payload } => {
                let fixture = self.engine.apply_brush_placement(&payload).ok_or(Puzzle3dError::BrushPlacementRejected)?;
                Ok(Puzzle3dEngineOutcome::Fixture(fixture))
            }
            Puzzle3dEngineCommand::ApplyFillCount { count } => {
                let fixture = self.engine.apply_fill_count(count as usize).ok_or(Puzzle3dError::FillSessionUnavailable)?;
                Ok(Puzzle3dEngineOutcome::Fixture(fixture))
            }
            Puzzle3dEngineCommand::ComposeFillDisplay { count } => {
                let fixture = self.engine.compose_fill_display(count as usize).ok_or(Puzzle3dError::FillSessionUnavailable)?;
                Ok(Puzzle3dEngineOutcome::Fixture(fixture))
            }
            Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights } => {
                self.engine.update_kind_weights(object_weights, vortex_weights);
                Ok(Puzzle3dEngineOutcome::Unit)
            }
            Puzzle3dEngineCommand::BrushPreview { vortex_full_id, candidate_index } => Ok(Puzzle3dEngineOutcome::BrushPreview(self.engine.brush_preview(&vortex_full_id, candidate_index as usize))),
        }
    }
}
//#endregion 🔖️Session

//#region 💼️SharedPluginJob
pub(crate) fn fill_job(context: semio_framework_plugin::reactor::jobs::JobCtx, input: Vec<u8>, restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
    Box::pin(async move {
        let request: FillJobRequest = serde_json::from_slice(&input).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("puzzle3d.fill-job.decode"), error.to_string()))?;
        if let Some(checkpoint) = restored {
            crate::editor::puzzle3d::with_puzzle3d_app_mut(|app| app.precompute.borrow_mut().restore_fill_job(&request, &checkpoint));
        }
        loop {
            context.tick().await;
            let Some(slice) = crate::editor::puzzle3d::with_puzzle3d_app_mut(|app| app.precompute.borrow_mut().drive_fill_job(&request)) else {
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("puzzle3d.fill-job.stale"), "fill job no longer matches the live operation"));
            };
            if let Some(progress) = slice.progress.clone() {
                context.progress(progress).await;
            }
            if let Some(checkpoint) = slice.checkpoint {
                context.checkpoint(checkpoint).await;
            }
            if slice.done {
                return Ok(slice.progress.expect("completed fill slice has progress"));
            }
        }
    })
}
//#endregion 💼️SharedPluginJob

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle3d::schema::testkit::*;
    use crate::artifacts::puzzle3d::schema::{BrushHostRules, BrushKindWeights, CableKindCatalog, FixtureObject, KindCompatEntry, ObjectKind, ObjectKindRepresentation, ObjectKindVortexTemplate, VortexKindCatalog, VortexProps};
    use semio_framework_job::{BatchDriveConfig, BatchJobParams, InteractiveStage, StepOutcome};
    use std::time::{Duration, Instant};

    fn fill_capable_engine() -> Puzzle3dCollision {
        let mut engine = Puzzle3dCollision::new();
        let (positions, indices) = unit_cube_mesh_buffers();
        engine.register_mesh("/test/host.glb".to_string(), &positions, &indices);
        engine.register_mesh("/test/candidate.glb".to_string(), &positions, &indices);
        let scene = SceneConfig {
            fixture: Fixture {
                attractions: vec![],
                target_volumes: vec![],
                objects: vec![FixtureObject {
                    id: "host".to_string(),
                    object_kind: Some("Host".to_string()),
                    anchor: Default::default(),
                    mesh_url: Some("/test/host.glb".to_string()),
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [4.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                    reveal_index: None,
                }],
            },
            kind_catalogs: Some(KindCatalogBundle {
                objects: vec![
                    ObjectKind {
                        id: "Host".to_string(),
                        representations: vec![ObjectKindRepresentation { id: "host".into(), name: String::new(), url: "/test/host.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                        scale: None,
                        vortices: vec![],
                    },
                    ObjectKind {
                        id: "Candidate".to_string(),
                        representations: vec![ObjectKindRepresentation { id: "candidate".into(), name: String::new(), url: "/test/candidate.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                        scale: None,
                        vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, 1.0]), ..Default::default() }],
                    },
                ],
                vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None, ..Default::default() }],
                cables: vec![],
            }),
            kind_compatibility: vec![KindCompatEntry { source: "port-b".to_string(), target: "port-a".to_string(), bidirectional: true, important: false, specificity: Some("vortex".to_string()) }],
            overlap_budget: DEFAULT_OVERLAP_BUDGET,
            seed: 1,
            host_rules: BrushHostRules::default(),
            weights: BrushKindWeights::default(),
        };
        engine.set_scene(&serde_json::to_string(&scene).expect("fill scene")).expect("set fill scene");
        engine.fill.as_mut().expect("fill").max_count = 1;
        engine
    }

    #[test]
    fn brush_candidates_allow_separated_boxes() {
        let mut engine = Puzzle3dCollision::new();
        let positions: Vec<f32> = vec![-4.0, -4.0, -4.0, 4.0, -4.0, -4.0, 4.0, 4.0, -4.0, -4.0, 4.0, -4.0, -4.0, -4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0];
        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
        engine.register_mesh("/test/obstacle.glb".to_string(), &positions, &indices);
        engine.register_mesh("/test/preview.glb".to_string(), &positions, &indices);
        let scene = SceneConfig {
            fixture: Fixture {
                attractions: vec![],
                target_volumes: vec![],
                objects: vec![
                    FixtureObject {
                        id: "obstacle".to_string(),
                        object_kind: Some("Kind".to_string()),
                        anchor: Default::default(),
                        mesh_url: Some("/test/obstacle.glb".to_string()),
                        origin: [0.0, 0.0, 0.0],
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                        vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                        reveal_index: None,
                    },
                    FixtureObject {
                        id: "host".to_string(),
                        object_kind: Some("Host".to_string()),
                        anchor: Default::default(),
                        mesh_url: Some("/test/unregistered.glb".to_string()),
                        origin: [12.0, 0.0, 0.0],
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                        vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                        reveal_index: None,
                    },
                ],
            },
            kind_catalogs: Some(KindCatalogBundle {
                objects: vec![ObjectKind {
                    id: "Kind".to_string(),
                    representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/preview.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                    scale: None,
                    vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() }],
                }],
                vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None, ..Default::default() }],
                cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None, ..Default::default() }],
            }),
            kind_compatibility: vec![KindCompatEntry { source: "port-b".to_string(), target: "port-a".to_string(), bidirectional: true, important: false, specificity: Some("vortex".to_string()) }],
            overlap_budget: DEFAULT_OVERLAP_BUDGET,
            seed: 1,
            host_rules: BrushHostRules::default(),
            weights: BrushKindWeights::default(),
        };
        engine.scene = Some(scene);
        let result = engine.compute_brush_cache_entry("host:v0");
        assert!(!result.unknown_pending, "expected mesh-ready result");
        assert_eq!(result.free.len(), 1, "expected one collision-free candidate");
    }

    /// 🪪️ Regression: `set_scene` used to unconditionally `rebuild_queue()`, wiping `brush_cache`/`fill`
    /// progress on every resync — the app's `sync_precompute_session` calls `set_scene` on *every*
    /// action, so this made suggestion/fill precompute restart from zero on every single tick, freezing
    /// the UI. A resync with byte-identical scene JSON must be a no-operation.
    #[test]
    fn compose_fill_display_is_read_only_and_matches_apply_prefix() {
        let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = FillBuilder::new(base, 7, &HashMap::new(), &catalogs);
        fill.applied_count = 2;
        fill.sequence = (0..5).map(fill_plan_payload).collect();
        fill.appended_objects = (0..5).map(|index| fill_plan_object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..5).map(fill_plan_attraction).collect();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());
        let mut engine = Puzzle3dCollision::new();
        engine.fill = Some(fill);

        let display = engine.compose_fill_display(4).expect("semio_compose_rs display");
        assert_eq!(display.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0", "p1", "p2", "p3"]);
        assert_eq!(engine.fill.as_ref().expect("fill").applied_count, 2, "semio_compose_rs must not mutate applied_count");

        let applied = engine.apply_fill_count(4).expect("apply fill count");
        assert_eq!(applied.objects.len(), display.objects.len());
        assert_eq!(engine.fill.as_ref().expect("fill").applied_count, 4);
    }

    #[test]
    fn fill_options_paths_are_millisecond_scale() {
        let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = FillBuilder::new(base.clone(), 7, &HashMap::new(), &catalogs);
        fill.applied_count = 0;
        fill.sequence = (0..10).map(fill_plan_payload).collect();
        fill.appended_objects = (0..10).map(|index| fill_plan_object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..10).map(fill_plan_attraction).collect();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());

        let mut engine = Puzzle3dCollision::new();
        let base_scene = SceneConfig { fixture: base, kind_catalogs: Some(catalogs), kind_compatibility: vec![], overlap_budget: 0.0, seed: 7, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
        engine.set_scene(&serde_json::to_string(&base_scene).unwrap()).expect("seed");
        engine.fill = Some(fill);

        let count_start = std::time::Instant::now();
        let _ = engine.apply_fill_count(5).expect("apply fill count");
        let count_ms = count_start.elapsed().as_secs_f64() * 1000.0;
        assert!(count_ms < 5.0, "fill count apply took {count_ms}ms");
        assert_eq!(engine.fill.as_ref().expect("fill").applied_count, 5);

        let weight_start = std::time::Instant::now();
        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Placed".to_string(), 1.0);
        let mut vortex_weights = std::collections::BTreeMap::new();
        vortex_weights.insert("c-b".to_string(), 0.5);
        vortex_weights.insert("b-s".to_string(), 0.5);
        engine.update_kind_weights(object_weights, vortex_weights);
        let weight_ms = weight_start.elapsed().as_secs_f64() * 1000.0;
        assert!(weight_ms < 50.0, "weight update took {weight_ms}ms");
        let fill = engine.fill.as_ref().expect("fill");
        let fill_steps = engine.fill_steps_pending_for_test();
        assert_eq!(fill_steps, fill.max_count - fill.applied_count, "weight update must soft-replan the tail without a full queue wipe");
        assert_eq!(fill.applied_count, 5, "applied fill objects must survive weight edits");
    }

    #[test]
    fn apply_fill_count_downward_move_keeps_the_plan_intact() {
        // 🔽️ Moving the count DOWN must never discard the already-planned sequence/appended objects/
        // placed entries or re-enqueue FillSteps — only `applied_count` (and the returned document-prefix
        // fixture) may change. Otherwise a jittery drag forces expensive replanning on every dip.
        let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = FillBuilder::new(base.clone(), 7, &HashMap::new(), &catalogs);
        fill.applied_count = 0;
        fill.sequence = (0..10).map(fill_plan_payload).collect();
        fill.appended_objects = (0..10).map(|index| fill_plan_object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..10).map(fill_plan_attraction).collect();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());
        fill.placed = fill
            .appended_objects
            .iter()
            .map(|object| PlacedCollisionEntry { object_id: object.id.clone(), mesh_url: "/test/placed.glb".into(), world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) })
            .collect();

        let mut engine = Puzzle3dCollision::new();
        let base_scene = SceneConfig { fixture: base.clone(), kind_catalogs: Some(catalogs), kind_compatibility: vec![], overlap_budget: 0.0, seed: 7, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
        engine.set_scene(&serde_json::to_string(&base_scene).unwrap()).expect("seed");
        engine.fill = Some(fill);

        engine.apply_fill_count(8).expect("apply up to 8");
        let queue_before = engine.work_pending_for_test();
        let placed_before = engine.fill.as_ref().unwrap().placed.len();
        let sequence_before = engine.fill.as_ref().unwrap().sequence.len();

        engine.apply_fill_count(3).expect("apply down to 3");
        let fill = engine.fill.as_ref().expect("fill");
        assert_eq!(fill.applied_count, 3);
        assert_eq!(fill.sequence.len(), sequence_before, "the plan is prefix-stable — downward moves never truncate it");
        assert_eq!(fill.appended_objects.len(), sequence_before);
        assert_eq!(fill.appended_attractions.len(), sequence_before);
        assert_eq!(fill.placed.len(), placed_before, "placed collision entries survive a downward move");
        assert_eq!(engine.work_pending_for_test(), queue_before, "no FillSteps get re-enqueued on a downward move");

        let fixture = engine.apply_fill_count(7).expect("apply back up to 7");
        assert_eq!(fixture.objects.len(), base.objects.len() + 7, "moving back up is instant — the plan was never discarded");
    }

    #[test]
    fn update_kind_weights_soft_replans_tail_without_rebuilding_queue() {
        let mut engine = Puzzle3dCollision::new();
        let json = single_object_scene_json();
        engine.set_scene(&json).expect("seed scene");
        let queue_len_after_seed = engine.work_pending_for_test();
        engine.precompute_step(8);
        let queue_len_after_step = engine.work_pending_for_test();
        assert!(queue_len_after_step < queue_len_after_seed);

        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Host".to_string(), 0.25);
        object_weights.insert("Placed".to_string(), 0.75);
        let mut vortex_weights = std::collections::BTreeMap::new();
        vortex_weights.insert("c-b".to_string(), 0.5);
        vortex_weights.insert("b-s".to_string(), 0.5);
        engine.update_kind_weights(object_weights, vortex_weights);

        assert_eq!(engine.fill.as_ref().map_or(0, |fill| fill.applied_count), 0, "weight-only edits must not change applied count");
        assert_eq!(engine.fill.as_ref().map_or(0, |fill| fill.sequence.len()), 0, "planned tail must be discarded for replanning");
        assert!(engine.work_pending_for_test() >= queue_len_after_step, "fill steps must be re-enqueued without a full queue wipe");
        assert!(engine.fill_steps_pending_for_test() > 0, "fill planning must continue after weight edits");
    }

    #[test]
    fn set_scene_with_identical_json_preserves_precompute_progress() {
        let mut engine = Puzzle3dCollision::new();
        let json = single_object_scene_json();
        engine.set_scene(&json).expect("first set_scene should succeed");
        let queue_len_before = engine.work_pending_for_test();
        assert!(queue_len_before > 0, "rebuild_queue should have enqueued at least the fill steps");
        engine.precompute_step(4);
        let queue_len_after_step = engine.work_pending_for_test();
        assert!(queue_len_after_step < queue_len_before, "precompute_step should have drained some queue items");

        engine.set_scene(&json).expect("resync with identical json should succeed");
        assert_eq!(engine.work_pending_for_test(), queue_len_after_step, "identical scene JSON must not rebuild (wipe) the queue");

        // A genuinely different scene (different object count) must still rebuild.
        let mut scene: serde_json::Value = serde_json::from_str(&json).unwrap();
        scene["fixture"]["objects"].as_array_mut().unwrap().push(serde_json::json!({ "id": "extra", "objectKind": "Host", "meshUrl": "/test/host.glb", "origin": [5.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0], "vortices": [] }));
        let changed_json = serde_json::to_string(&scene).unwrap();
        engine.set_scene(&changed_json).expect("set_scene with a genuinely different scene should succeed");
        assert_ne!(engine.work_pending_for_test(), queue_len_after_step, "a changed scene must rebuild the queue");
    }

    #[test]
    fn decreasing_fill_count_keeps_the_plan_intact_and_does_not_replan() {
        // 🔽️ Downward moves are prefix-stable (see `apply_fill_count`) — the plan/sequence/appended
        // objects/queue must never be discarded or re-enqueued just because the applied prefix shrank;
        // that used to force expensive replanning on every jittery drag dip.
        let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = FillBuilder::new(base, 7, &HashMap::new(), &catalogs);
        fill.applied_count = 3;
        fill.sequence = (0..3).map(fill_plan_payload).collect();
        fill.appended_objects = (0..3).map(|index| fill_plan_object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..3).map(fill_plan_attraction).collect();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());
        fill.stalled = true;
        let rng_state = fill.rng_state;
        let mut engine = Puzzle3dCollision::new();
        engine.fill = Some(fill);

        let fixture = engine.apply_fill_count(1).expect("fill session");
        assert_eq!(fixture.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0"], "the returned document prefix reflects the new applied count");
        let fill = engine.fill.as_ref().expect("fill builder");
        assert_eq!(fill.appended_objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["p0", "p1", "p2"], "the full plan survives — a downward move never discards the tail");
        assert_eq!(fill.sequence.len(), 3, "the planned sequence is never truncated by a downward move");
        assert_eq!(fill.applied_count, 1);
        assert!(fill.stalled, "apply_fill_count never touches stalled — only actual planning does");
        assert_eq!(fill.rng_state, rng_state, "no replanning happens, so the random stream is untouched");
        assert_eq!(engine.fill_steps_pending_for_test(), 0, "no FillSteps get enqueued by a downward move");

        let fixture = engine.apply_fill_count(0).expect("zero fill count");
        assert_eq!(fixture.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"], "zero applies nothing to the document");
        assert_eq!(engine.fill.as_ref().expect("fill builder").sequence.len(), 3, "even at count 0, the plan is preserved for instant re-apply");
    }

    #[test]
    fn set_scene_with_applied_fill_projection_preserves_slider_session() {
        let base = Fixture { objects: vec![fill_plan_object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = FillBuilder::new(base.clone(), 7, &HashMap::new(), &catalogs);
        fill.applied_count = 3;
        fill.sequence = (0..3).map(fill_plan_payload).collect();
        fill.appended_objects = (0..3).map(|index| fill_plan_object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..3).map(fill_plan_attraction).collect();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());
        fill.stalled = true;

        let mut engine = Puzzle3dCollision::new();
        let base_scene = SceneConfig { fixture: base, kind_catalogs: Some(catalogs), kind_compatibility: vec![], overlap_budget: 0.0, seed: 7, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
        let base_json = serde_json::to_string(&base_scene).unwrap();
        engine.set_scene(&base_json).expect("seed base scene");
        // 🪣️ Replace the fresh FillBuilder from rebuild_queue with the already-applied session under test.
        engine.fill = Some(fill);

        let mut applied_scene = base_scene;
        applied_scene.fixture.objects.extend((0..3).map(|index| fill_plan_object(&format!("p{index}"))));
        applied_scene.fixture.attractions.extend((0..3).map(fill_plan_attraction));
        // 🪪️ Pose drift on the base object (attraction rederive) must not count as a new scene.
        applied_scene.fixture.objects[0].origin = [1.0, 2.0, 3.0];
        let applied_json = serde_json::to_string(&applied_scene).unwrap();
        engine.set_scene(&applied_json).expect("re-syncing the applied fill projection must succeed");

        let fill = engine.fill.as_ref().expect("fill session must survive the applied-projection re-sync");
        assert_eq!(fill.applied_count, 3, "applied fill count must survive incidental set_scene syncs");
        assert_eq!(fill.sequence.len(), 3, "planned fill sequence must survive incidental set_scene syncs");
        assert_eq!(fill.base.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"]);

        let reduced = engine.apply_fill_count(1).expect("decreasing after sync");
        assert_eq!(reduced.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0"], "slider must still be able to remove fill objects after a document re-sync");
        let cleared = engine.apply_fill_count(0).expect("clear after sync");
        assert_eq!(cleared.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"]);
    }

    /// 🪪️ Regression: registering a mesh must invalidate any cached brush candidates computed against a
    /// different (e.g. fallback-box) body for the same url, but a no-operation re-registration must not matter
    /// once the cache already reflects the current mesh set (the everyday case: every action re-seeds the
    /// fallback body, and the app's `sync_precompute_session` already guards that with `has_mesh`).
    #[test]
    fn register_mesh_invalidates_cached_precompute_state() {
        let mut engine = Puzzle3dCollision::new();
        engine.set_scene(&single_object_scene_json()).expect("set_scene should succeed");
        let applied_before = engine.fill.as_ref().map_or(0, |fill| fill.applied_count);
        let positions: Vec<f32> = vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0];
        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
        engine.register_mesh("/test/host.glb".to_string(), &positions, &indices);
        assert!(engine.brush_cache.is_empty(), "mesh registration must invalidate stale brush cache entries");
        assert_eq!(engine.fill.as_ref().map(|fill| fill.applied_count), Some(applied_before), "mesh registration must not reset applied fill count");
    }

    #[test]
    fn engine_precompute_step_is_false_with_no_scene() {
        let mut engine = Puzzle3dCollision::new();
        assert!(!engine.precompute_step(10));
        assert!(engine.fill.is_none());
    }

    #[test]
    fn engine_apply_brush_placement_none_without_scene_or_catalogs() {
        let mut engine = Puzzle3dCollision::new();
        let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert!(engine.apply_brush_placement(&payload).is_none(), "no scene means no placement");

        engine.set_scene(&single_object_scene_json()).expect("seed");
        if let Some(scene) = &mut engine.scene {
            scene.kind_catalogs = None;
        }
        assert!(engine.apply_brush_placement(&payload).is_none(), "no catalogs means no placement");
    }

    #[test]
    fn engine_has_mesh_invalidate_and_refresh_brush_candidates() {
        let mut engine = Puzzle3dCollision::new();
        engine.set_scene(&single_object_scene_json()).expect("seed");
        assert!(!engine.has_mesh("/test/host.glb"));
        let (positions, indices) = unit_cube_mesh_buffers();
        engine.register_mesh("/test/host.glb".to_string(), &positions, &indices);
        assert!(engine.has_mesh("/test/host.glb"));

        engine.invalidate_brush_target("host:v0");
        assert_eq!(engine.brush_queue.front().map(String::as_str), Some("host:v0"), "invalidated brush target must be requeued at the front");
        assert!(!engine.brush_cache.contains_key("host:v0"));

        engine.refresh_brush_candidates("host:v0");
        assert!(engine.brush_cache.contains_key("host:v0"));
        assert_eq!(engine.brush_preview("host:v0", 0), None, "the catalog's Host kind has no vortices, so there are no free candidates");
    }

    #[test]
    fn precompute_session_native_wrapper_exercises_public_methods() {
        let mut session = Puzzle3dPrecomputeSession::default();
        session.set_scene(&single_object_scene_json()).expect("set_scene");
        assert!(!session.has_mesh("/test/host.glb"));
        let (positions, indices) = unit_cube_mesh_buffers();
        session.register_mesh("/test/host.glb", &positions, &indices);
        assert!(session.has_mesh("/test/host.glb"));
        assert!(!session.fill_is_done(), "a freshly (re)seeded fill session has not stalled or hit max_count yet");

        session.precompute_step(50);
        session.invalidate_brush_target("host:v0");
        session.refresh_brush_candidates("host:v0");
        let _candidates: BrushCollisionFreeResult = session.brush_candidates("host:v0");
        assert!(session.brush_preview("host:v0", 0).is_none());

        assert_eq!(session.fill_progress().max_count, FILL_COUNT_MAX);
        assert_eq!(session.fill_available_count(), 0);

        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Host".to_string(), 1.0);
        session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() }).expect("update kind weights");

        let missing_payload = BrushPlacePayload { target_vortex_full_id: "missing:v0".to_string(), object_kind_id: "Nonexistent".to_string(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert!(session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload: missing_payload }).is_err());

        let outcome = session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).expect("fill session available");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"));
        let outcome = session.dispatch(Puzzle3dEngineCommand::ComposeFillDisplay { count: 0 }).expect("fill session available");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"));
    }

    #[test]
    fn precompute_session_native_wrapper_errors_without_scene() {
        let mut session = Puzzle3dPrecomputeSession::new();
        assert!(session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).is_err());
        assert!(session.dispatch(Puzzle3dEngineCommand::ComposeFillDisplay { count: 0 }).is_err());
        let payload = BrushPlacePayload { target_vortex_full_id: "a:v0".to_string(), object_kind_id: "b".to_string(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert!(session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload }).is_err());
        assert!(session.fill_is_done());
        assert_eq!(session.fill_available_count(), 0);
    }

    #[test]
    fn fill_lane_advances_while_brush_targets_remain_queued() {
        let mut engine = Puzzle3dCollision::new();
        engine.set_scene(&single_object_scene_json()).expect("seed");
        assert!(engine.fill_steps_pending_for_test() > 0, "seed scene must schedule fill steps");
        assert!(!engine.brush_queue.is_empty(), "seed scene must schedule brush targets");
        let before = engine.fill_progress_summary().count;
        for _ in 0..24 {
            engine.precompute_step_lane(PrecomputeLane::Fill, 4);
        }
        let after = engine.fill_progress_summary().count;
        assert!(after > before || engine.fill_progress_summary().done, "fill lane must make planning progress without draining brush first");
    }

    #[test]
    fn brush_candidates_cold_cache_returns_pending_without_populating_cache() {
        let mut session = Puzzle3dPrecomputeSession::new();
        session.set_scene(&single_object_scene_json()).expect("seed");
        let result = session.brush_candidates("host:v0");
        assert!(result.unknown_pending, "cold cache must surface pending state: {result:?}");
        assert!(session.brush_preview("host:v0", 0).is_none());
    }

    /// 🧰️ `enqueue_brush_target` is the app-facing append (vs. `invalidate_brush_target`'s
    /// front-of-queue jump) — appending an already-queued id must be a no-operation.
    #[test]
    fn enqueue_brush_target_appends_once() {
        let mut engine = Puzzle3dCollision::new();
        engine.enqueue_brush_target("host:v0");
        engine.enqueue_brush_target("host:v0");
        assert_eq!(engine.brush_queue.len(), 1);
    }

    /// 🖐️ Compile-guard for the 🖐️5d app, which builds its own `Puzzle5dPrecomputeSession` on top of
    /// this one (relocated from the former `⚙️engine` root's `the_5d_facing_engine_surface_stays_public`,
    /// ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): every item it names must stay
    /// publicly reachable — pure data under `crate::artifacts::puzzle3d::schema::…`, the session/dispatch
    /// surface under `crate::editor::puzzle3d::precompute::…`. A rename or a visibility narrowing breaks
    /// this test long before it breaks 5d.
    #[test]
    fn the_5d_facing_precompute_surface_stays_public() {
        use crate::artifacts::puzzle3d::Puzzle3dError as GuardError;

        let mut session = Puzzle3dPrecomputeSession::new();
        assert!(session.set_scene("{ not json").is_err(), "set_scene surfaces a Puzzle3dError");
        session.register_mesh("/probe.glb", &[], &[]);
        assert!(!session.has_mesh("/probe.glb"));
        assert!(!session.precompute_step(1));
        let _: BrushCollisionFreeResult = session.brush_candidates("probe:v0");
        let _: Option<BrushPreviewState> = session.brush_preview("probe:v0", 0);
        let _: FillBuildProgress = session.fill_progress();
        assert!(session.precompute_step_lane(PrecomputeLane::Brush, 1) || true);
        let payload = BrushPlacePayload { target_vortex_full_id: "probe:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let rejected: Result<Puzzle3dEngineOutcome, GuardError> = session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload });
        assert!(matches!(rejected, Err(GuardError::BrushPlacementRejected)));
        assert!(session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).is_err());
        let _: fn(&Fixture, &BrushPlacePayload, &KindCatalogBundle) -> Fixture = apply_brush_placement_to_fixture;
    }

    /// 🔗️ Minimal scene JSON matching `SceneConfig`'s real wire shape (camelCase, per its
    /// `#[serde(rename = ...)]` attrs) — relocated from `🧬️mutations/💾️binary/🦀️component.rs`'s own
    /// `sample_scene_config` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): that file's
    /// own copy stays for the pure-data wire-format-guard tests that need no session, and this copy feeds
    /// the two dispatch tests below, since a schema-side test module must not depend on the app.
    fn sample_scene_config() -> SceneConfig {
        let json = r#"{
            "fixture": {
                "objects": [{"id": "host", "objectKind": "Host", "meshUrl": "/test/host.glb", "origin": [0,0,0], "orientation": [0,0,0,1], "vortices": [{"id": "v0", "vortexKind": "port-a", "position": [0,0,0], "direction": [0,0,-1]}]}],
                "attractions": [],
                "targetVolumes": []
            },
            "kindCatalogs": {"objects": [{"id": "Host", "representations": [{"id": "r0", "name": "default", "url": "/test/host.glb"}], "vortices": []}], "vortices": [{"id": "port-a"}], "cables": []},
            "kindCompatibility": [],
            "overlapBudget": 0.02,
            "seed": 1
        }"#;
        serde_json::from_str(json).expect("sample scene config parses")
    }

    /// 🎯️ Behavioral parity: `dispatch` must reach the exact same engine logic the old JSON-string
    /// wasm-bindgen methods delegated to — `SetScene` seeds a fill session, `ApplyFillCount`/
    /// `ComposeFillDisplay` read/apply its prefix, matching what this module's own
    /// `precompute_session_native_wrapper_exercises_public_methods` test already asserts for the
    /// pre-dispatch API. Relocated from `🧬️mutations/💾️binary/🦀️component.rs`'s
    /// `dispatch_set_scene_then_apply_and_compose_fill_count_round_trip` — that test constructed
    /// `Puzzle3dPrecomputeSession` directly, which is now an app type a schema test file must not reach.
    #[test]
    fn dispatch_set_scene_then_apply_and_compose_fill_count_round_trip() {
        let mut session = Puzzle3dPrecomputeSession::new();
        session.dispatch(Puzzle3dEngineCommand::SetScene { scene: sample_scene_config() }).expect("set scene");
        assert!(!session.fill_is_done(), "a freshly seeded fill session has not stalled or hit max_count yet");

        session.precompute_step(50);

        let outcome = session.dispatch(Puzzle3dEngineCommand::ComposeFillDisplay { count: 0 }).expect("compose fill display");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"), "the base scene's host object must survive compose_fill_display(0)");

        let outcome = session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).expect("apply fill count");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"));
    }

    /// 🎯️ Relocated from `🧬️mutations/💾️binary/🦀️component.rs`'s
    /// `dispatch_brush_preview_without_scene_returns_none` (same reason as the test above).
    #[test]
    fn dispatch_brush_preview_without_scene_returns_none() {
        let mut session = Puzzle3dPrecomputeSession::new();
        let outcome = session.dispatch(Puzzle3dEngineCommand::BrushPreview { vortex_full_id: "host:v0".to_string(), candidate_index: 0 }).expect("brush preview never errors");
        assert_eq!(outcome, Puzzle3dEngineOutcome::BrushPreview(None), "no scene means no cached brush candidates yet");
    }

    fn finish_fill_with_budget(step_budget: u32) -> Vec<u8> {
        let mut engine = fill_capable_engine();
        for _ in 0..10_000 {
            if !engine.precompute_step_lane(PrecomputeLane::Fill, step_budget) {
                return engine.fill.as_ref().expect("fill").checkpoint_bytes();
            }
        }
        let fill = engine.fill.as_ref().expect("fill");
        panic!("fill job did not terminate: stage={:?} count={} rejected={}", fill.stage, fill.sequence.len(), fill.preview.rejected_count);
    }

    fn normalized_fill_checkpoint(bytes: &[u8]) -> Vec<u8> {
        FillBuilder::normalized_checkpoint_bytes(bytes)
    }

    #[test]
    fn fill_job_is_deterministic_across_drive_batch_sizes() {
        let checkpoints = [1, 2, 4, 8].map(finish_fill_with_budget).map(|checkpoint| normalized_fill_checkpoint(&checkpoint));
        assert_eq!(checkpoints[0], checkpoints[1]);
        assert_eq!(checkpoints[1], checkpoints[2]);
        assert_eq!(checkpoints[2], checkpoints[3]);
    }

    #[test]
    fn fill_job_checkpoint_resume_matches_uninterrupted_execution() {
        let mut uninterrupted = fill_capable_engine();
        uninterrupted.precompute_step_lane(PrecomputeLane::Fill, 3);
        let checkpoint = uninterrupted.fill.as_ref().expect("fill").checkpoint_bytes();

        let mut resumed = fill_capable_engine();
        resumed.fill.as_mut().expect("fill").restore_checkpoint(&checkpoint).expect("restore");
        resumed.fill_preview_sequence = uninterrupted.fill_preview_sequence;

        for _ in 0..10_000 {
            let left = uninterrupted.precompute_step_lane(PrecomputeLane::Fill, 1);
            let right = resumed.precompute_step_lane(PrecomputeLane::Fill, 1);
            assert_eq!(left, right);
            if !left {
                break;
            }
        }
        assert_eq!(uninterrupted.fill.as_ref().expect("fill").checkpoint_bytes(), resumed.fill.as_ref().expect("fill").checkpoint_bytes());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn fill_job_commit_is_byte_identical_at_one_two_four_and_default_workers() {
        let template = fill_capable_engine();
        let initial = template.fill.as_ref().expect("fill").checkpoint_bytes();
        let default_workers = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        let mut outputs = Vec::new();
        for worker_count in [1usize, 2, 4, default_workers] {
            let mut engine = fill_capable_engine();
            let mut job = engine.fill.take().expect("fill");
            job.restore_checkpoint(&initial).expect("initial checkpoint");
            let operation = job.operation;
            let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, worker_count));
            let params = BatchJobParams {
                operation: operation.operation,
                generation: operation.generation,
                cancel: semio_framework_job::root_cancel_token(),
                config: BatchDriveConfig { site: "puzzle3d.fill.workers", stage: InteractiveStage::BackgroundStep, fuel_per_step: 1, step_budget_ms: 7 },
                now_ms: semio_framework_job::default_now_ms,
            };
            let receiver = semio_framework_job::run_on_worker(&pool, semio_framework_async::Lane::Background, job, params);
            let outcome = receiver.recv_timeout(Duration::from_secs(20)).expect("fill worker did not finish");
            pool.shutdown();
            match outcome {
                StepOutcome::Complete(candidate) => {
                    eprintln!("[DEBUG] puzzle3d-fill-worker-parity workers={worker_count} commit-bytes={}", candidate.state.len());
                    outputs.push(candidate.state);
                }
                other => panic!("worker_count={worker_count} ended with {other:?}"),
            }
        }
        assert!(outputs.windows(2).all(|pair| pair[0] == pair[1]), "fill commit diverged across 1/2/4/default workers");
        let mut verifier = fill_capable_engine();
        verifier.fill.as_mut().expect("fill").restore_checkpoint(&outputs[0]).expect("worker commit checkpoint");
        assert_eq!(verifier.fill.as_ref().expect("fill").sequence.len(), 1, "worker parity fixture must contain an accepted placement");
    }

    #[test]
    fn fill_first_substantive_preview_arrives_below_fifty_ms_and_every_step_below_eight_ms() {
        let mut engine = fill_capable_engine();
        let started = Instant::now();
        let mut first_preview = None;
        let mut completed = false;
        let mut max_step = Duration::ZERO;
        for _ in 0..10_000 {
            let step_started = Instant::now();
            let active = engine.precompute_step_lane(PrecomputeLane::Fill, 1);
            let step_elapsed = step_started.elapsed();
            max_step = max_step.max(step_elapsed);
            assert!(step_elapsed < Duration::from_millis(8), "fill resume step reached the 8ms ceiling");
            if first_preview.is_none() && engine.fill.as_ref().is_some_and(|fill| fill.preview.candidate_ghost.is_some()) {
                first_preview = Some(started.elapsed());
            }
            if !active {
                completed = true;
                break;
            }
        }
        assert!(
            first_preview.is_some_and(|elapsed| elapsed < Duration::from_millis(50)),
            "first substantive fill preview exceeded 50ms: {first_preview:?}; stage={:?}; rejected={}",
            engine.fill.as_ref().map(|fill| fill.stage),
            engine.fill.as_ref().map_or(0, |fill| fill.preview.rejected_count)
        );
        assert!(completed, "fill did not complete within the bounded resume budget");
        eprintln!("[DEBUG] puzzle3d-fill-preview first-preview-us={} max-step-us={}", first_preview.expect("first preview").as_micros(), max_step.as_micros());
    }
}
//#endregion 🧪️Tests
