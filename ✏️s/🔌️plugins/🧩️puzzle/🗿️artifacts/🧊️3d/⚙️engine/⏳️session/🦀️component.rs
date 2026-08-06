//! ⚙️ Puzzle 3d artifact engine — the precompute session: the scene the host syncs in, the registered
//! collision meshes, the two independent background lanes (brush-candidate caching and fill
//! planning), and the typed `Puzzle3dEngineCommand`/`Puzzle3dEngineOutcome` dispatch envelope every
//! mutating engine action crosses. The rules the lanes consult live in `🦀️brush.rs`, the geometry in
//! `🦀️geometry.rs`, the fill plan's own state in `🦀️fill.rs`.

use crate::artifacts::puzzle3d::engine::brush::{
    brush_candidate_suggestion_weight, brush_compatible_candidates, brush_preview_from_candidate, brush_target_vortex_allows_suggestion, enumerate_brush_fill_vortex_targets, order_brush_fill_compatible_candidates,
    resolve_object_kind_mesh_url, vortex_world_from_object, weighted_order_fill_vortex_targets, AttractionVortexContext, TargetVortexWorld,
};
use crate::artifacts::puzzle3d::engine::fill::{FillBuilder, PlacedCollisionEntry};
use crate::artifacts::puzzle3d::engine::geometry::{pose_isometry, solid_overlap_volume, world_bounds, world_volumes_contain_aabb, CollisionBody};
use crate::artifacts::puzzle3d::engine::{
    apply_brush_placement_to_fixture, collision_body_from_buffers, puzzle3d_vortex_full_id, BrushCollisionFreeResult, BrushCompatibleCandidate, BrushPlacePayload, BrushPreviewState, FillBuildProgress, FillProgressSummary, Fixture,
    KindCatalogBundle, PrecomputeLane, SceneConfig, FILL_COUNT_MAX,
};
use crate::artifacts::puzzle3d::Puzzle3dError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

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
const PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS: f64 = 12.0;
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
}

impl Puzzle3dCollision {
    pub(crate) fn new() -> Self {
        Self { scene: None, scene_json: None, meshes: HashMap::new(), mesh_is_fallback: HashMap::new(), brush_cache: HashMap::new(), brush_queue: VecDeque::new(), fill_steps_remaining: 0, fill: None }
    }

    fn fill_lane_active(&self) -> bool {
        self.fill.as_ref().is_some_and(|fill| !fill.stalled && fill.sequence.len() < fill.max_count && self.fill_steps_remaining > 0)
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
        self.brush_queue.clear();
        self.brush_cache.clear();
        self.fill_steps_remaining = 0;
        if let Some(scene) = &self.scene {
            for target in enumerate_brush_fill_vortex_targets(&scene.fixture) {
                self.brush_queue.push_back(target.full_id);
            }
            self.fill_steps_remaining = FILL_COUNT_MAX;
            let catalogs = scene.kind_catalogs.clone().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
            self.fill = Some(FillBuilder::new(scene.fixture.clone(), scene.seed, &self.meshes, &catalogs));
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
        let retained_ids: std::collections::HashSet<&str> = fill.fixture.objects.iter().map(|object| object.id.as_str()).collect();
        fill.placed.retain(|entry| retained_ids.contains(entry.object_id.as_str()));
        fill.candidate_cache.clear();
        fill.stalled = false;
        self.fill_steps_remaining = fill.max_count.saturating_sub(applied);
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
        let Some(body) = collision_body_from_buffers(positions, indices) else {
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
        let result = self.compute_brush_cache_entry(vortex_full_id);
        self.brush_cache.insert(vortex_full_id.to_string(), result);
    }

    fn preview_collides(meshes: &HashMap<String, CollisionBody>, preview: &BrushPreviewState, placed: &[PlacedCollisionEntry], overlap_budget: f64, sample_count: usize) -> Option<bool> {
        let preview_body = meshes.get(&preview.mesh_url)?;
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let (pmin, pmax) = world_bounds(preview_body, &preview_world);
        for entry in placed {
            let other = meshes.get(&entry.mesh_url)?;
            let (omin, omax) = world_bounds(other, &entry.world);
            if pmax.x() < omin.x() || pmin.x() > omax.x() || pmax.y() < omin.y() || pmin.y() > omax.y() || pmax.z() < omin.z() || pmin.z() > omax.z() {
                continue;
            }
            let vol = solid_overlap_volume(preview_body, &preview_world, other, &entry.world, sample_count, overlap_budget);
            if vol > overlap_budget {
                return Some(true);
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
            match Self::preview_collides(&self.meshes, &preview, &placed, overlap_budget, 1024) {
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
                    if !self.fill_step_one() {
                        self.fill_steps_remaining = 0;
                    } else {
                        self.fill_steps_remaining = self.fill_steps_remaining.saturating_sub(1);
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

    fn fill_step_one(&mut self) -> bool {
        let Some(scene) = &self.scene else {
            return false;
        };
        let Some(fill) = &mut self.fill else {
            return false;
        };
        if fill.stalled || fill.sequence.len() >= fill.max_count {
            fill.stalled = true;
            return false;
        }
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let overlap_budget = scene.overlap_budget;
        let weights = scene.weights.clone();
        let kind_compatibility = scene.kind_compatibility.clone();
        let host_rules = scene.host_rules.clone();
        let free_targets = enumerate_brush_fill_vortex_targets(&fill.fixture);
        if free_targets.is_empty() {
            fill.stalled = true;
            return false;
        }
        let seed_targets: Vec<_> = free_targets.iter().filter(|t| fill.seed_object_ids.contains(&t.object_id)).cloned().collect();
        let frontier_targets: Vec<_> = free_targets.iter().filter(|t| !fill.seed_object_ids.contains(&t.object_id)).cloned().collect();
        let ordered_targets: Vec<_> = weighted_order_fill_vortex_targets(&seed_targets, &weights, &mut fill.rng_state).into_iter().chain(weighted_order_fill_vortex_targets(&frontier_targets, &weights, &mut fill.rng_state)).collect();
        if ordered_targets.is_empty() {
            fill.stalled = true;
            return false;
        }
        let target_start = fill.sequence.len() % ordered_targets.len();
        for target_offset in 0..ordered_targets.len() {
            let target = &ordered_targets[(target_start + target_offset) % ordered_targets.len()];
            let Some(host) = fill.fixture.objects.iter().find(|o| o.id == target.object_id) else {
                continue;
            };
            let Some((position, direction)) = vortex_world_from_object(host, target.vortex_index) else {
                continue;
            };
            let target_ctx = AttractionVortexContext { object_kind: target.object_kind.clone(), vortex_kind: target.vortex_kind.clone() };
            let key = format!("{}\u{1}{}", target.object_kind.as_deref().unwrap_or(""), target.vortex_kind.as_deref().unwrap_or(""));
            let compatible = fill.candidate_cache.entry(key).or_insert_with(|| brush_compatible_candidates(&target_ctx, &catalogs, &kind_compatibility, &host_rules)).clone();
            if compatible.is_empty() {
                continue;
            }
            let ordered_candidates = order_brush_fill_compatible_candidates(&compatible, target.vortex_kind.as_deref(), target.vortex_index, target.object_kind.as_deref(), &catalogs, &weights, &mut fill.rng_state);
            if ordered_candidates.is_empty() {
                continue;
            }
            for candidate in &ordered_candidates {
                let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
                let Some(preview) = brush_preview_from_candidate(&target.full_id, candidate, &target_ctx, world, &catalogs, &fill.fixture) else {
                    continue;
                };
                if !self.meshes.contains_key(&preview.mesh_url) {
                    continue;
                }
                let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
                if let Some(body) = self.meshes.get(&preview.mesh_url) {
                    let (min, max) = world_bounds(body, &preview_world);
                    if !world_volumes_contain_aabb(&scene.fixture.target_volumes, min, max) {
                        continue;
                    }
                }
                let placed_snapshot: Vec<PlacedCollisionEntry> = fill.placed.iter().filter(|entry| entry.object_id != target.object_id).cloned().collect();
                match Self::preview_collides(&self.meshes, &preview, &placed_snapshot, overlap_budget, 512) {
                    None | Some(true) => continue,
                    Some(false) => {}
                }
                let payload = BrushPlacePayload {
                    target_vortex_full_id: preview.target_vortex_full_id.clone(),
                    object_kind_id: preview.object_kind_id.clone(),
                    source_vortex_index: preview.source_vortex_index,
                    origin: preview.origin,
                    orientation: preview.orientation,
                    scale: preview.scale.clone(),
                };
                let next_fixture = apply_brush_placement_to_fixture(&fill.fixture, &payload, &catalogs);
                if next_fixture.objects.len() == fill.fixture.objects.len() {
                    continue;
                }
                // 🔒️ Infallible: the length check above proves `apply_brush_placement_to_fixture` actually
                // appended (rather than returning `fixture.clone()` unchanged), and it only ever appends
                // exactly one object together with exactly one attraction, never one without the other.
                let mut placed_object = next_fixture.objects.last().cloned().expect("objects grew, so last() is Some");
                if let Some(mesh_url) = resolve_object_kind_mesh_url(placed_object.object_kind.as_deref().unwrap_or(""), &catalogs, &next_fixture) {
                    if self.meshes.contains_key(&mesh_url) {
                        fill.placed.push(PlacedCollisionEntry { object_id: placed_object.id.clone(), mesh_url, world: pose_isometry(placed_object.origin, placed_object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &placed_object.scale) });
                    }
                }
                let new_attraction = next_fixture.attractions.last().cloned().expect("attractions grew alongside objects, so last() is Some");
                fill.fixture = next_fixture;
                fill.sequence.push(payload);
                // 🪣️ Tag with its sequence position so `compose_fill_display` can expose it as `revealIndex`.
                placed_object.reveal_index = Some(fill.appended_objects.len());
                fill.appended_objects.push(placed_object);
                fill.appended_attractions.push(new_attraction);
                return true;
            }
        }
        fill.stalled = true;
        false
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

//#region 🔖️Dispatch
/// 🎯️ Typed command envelope for `Puzzle3dPrecomputeSession::dispatch` — the headless replacement for
/// the old per-action JSON-string wasm-bindgen methods. Derived (not hand-written `OpText`/`OpBinary`)
/// here — not in `📡️spr` — because the derive's generated code needs `SceneConfig`/`BrushPlacePayload`
/// in scope by value; `📡️spr` re-exports this type and wraps `encode_op`/`decode_op`, exactly like it
/// already does for `🔧️op`'s `Puzzle3dOperation`. Field shapes mirror the exact payload each old
/// JSON-string method parsed: `SetScene` mirrors `set_scene`'s `SceneConfig` JSON body,
/// `ApplyBrushPlacement` mirrors `apply_brush_placement_json`'s `BrushPlacePayload` body,
/// `UpdateKindWeights` mirrors `update_kind_weights`'s two JSON map bodies.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Puzzle3dEngineCommand {
    #[dsl(key = "set-scene")]
    SetScene { scene: SceneConfig },
    #[dsl(key = "apply-brush-placement")]
    ApplyBrushPlacement { payload: BrushPlacePayload },
    #[dsl(key = "apply-fill-count")]
    ApplyFillCount { count: u32 },
    #[dsl(key = "compose-fill-display")]
    ComposeFillDisplay { count: u32 },
    #[dsl(key = "update-kind-weights")]
    UpdateKindWeights { object_weights: std::collections::BTreeMap<String, f64>, vortex_weights: std::collections::BTreeMap<String, f64> },
    #[dsl(key = "brush-preview")]
    BrushPreview { vortex_full_id: String, candidate_index: u32 },
}

/// 📬️ What `dispatch` hands back — the typed counterpart of what each old JSON-string method
/// returned (a `Fixture` JSON string, a `BrushPreviewState` JSON string, or nothing). Plain Rust, no
/// DSL/wasm-bindgen requirement — this only ever crosses the artifact <-> app boundary in-process.
#[derive(Debug, Clone, PartialEq)]
pub enum Puzzle3dEngineOutcome {
    Unit,
    Fixture(Fixture),
    BrushPreview(Option<BrushPreviewState>),
}
//#endregion 🔖️Dispatch

//#region 🔖️Session
pub struct Puzzle3dPrecomputeSession {
    engine: Puzzle3dCollision,
}

impl Default for Puzzle3dPrecomputeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Puzzle3dPrecomputeSession {
    pub fn new() -> Self {
        Self { engine: Puzzle3dCollision::new() }
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
        self.engine.fill.as_ref().map_or(FillBuildProgress { count: 0, applied_count: 0, max_count: FILL_COUNT_MAX, done: true, appended_objects: vec![], appended_attractions: vec![], sequence: vec![] }, |f| f.progress())
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle3d::engine::testkit::*;
    use crate::artifacts::puzzle3d::engine::{BrushHostRules, BrushKindWeights, CableKindCatalog, FixtureObject, KindCompatEntry, ObjectKind, ObjectKindVortexTemplate, VortexKindCatalog, VortexProps};

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
                    mesh_url: Some("/test/preview.glb".to_string()),
                    scale: None,
                    vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                }],
                vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None }],
                cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None }],
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
        println!("[DEBUG] apply_fill_count(5): {count_ms:.3}ms");
        assert!(count_ms < 5.0, "fill count apply took {count_ms}ms");
        assert_eq!(engine.fill.as_ref().expect("fill").applied_count, 5);

        let queue_before = engine.work_pending_for_test();
        let weight_start = std::time::Instant::now();
        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Placed".to_string(), 1.0);
        let mut vortex_weights = std::collections::BTreeMap::new();
        vortex_weights.insert("c-b".to_string(), 0.5);
        vortex_weights.insert("b-s".to_string(), 0.5);
        engine.update_kind_weights(object_weights, vortex_weights);
        let weight_ms = weight_start.elapsed().as_secs_f64() * 1000.0;
        println!("[DEBUG] update_kind_weights: {weight_ms:.3}ms queue_before={queue_before} queue_after={}", engine.work_pending_for_test());
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
        assert!(fill.stalled, "apply_fill_count never touches stalled — only actual planning (fill_step_one) does");
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
    fn engine_precompute_step_and_fill_step_false_with_no_scene() {
        let mut engine = Puzzle3dCollision::new();
        assert!(!engine.precompute_step(10));
        assert!(!engine.fill_step_one());
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
        println!("[DEBUG] fill_lane planning count before={before} after={after}");
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
}
//#endregion 🧪️Tests
