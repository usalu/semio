//! 🪣️ Puzzle 3d artifact engine — the fill planner's own state: the running `FillBuilder` (base
//! scene, the growing plan sequence and its appended objects/attractions, the placed collision
//! entries the next step tests against, the per-session RNG stream) plus its progress readout. The
//! stepping itself lives in `🦀️session.rs`, which owns the two precompute lanes.

use crate::artifacts::puzzle3d::engine::brush::resolve_object_kind_mesh_url;
use crate::artifacts::puzzle3d::engine::geometry::{pose_isometry, Pose3d};
use crate::artifacts::puzzle3d::engine::{AttractionProps, BrushCompatibleCandidate, BrushPlacePayload, CollisionBody, FillBuildProgress, Fixture, FixtureObject, KindCatalogBundle, FILL_COUNT_MAX};
use std::collections::HashMap;

/// 🧱️ One already-placed object's collision footprint, kept alongside the plan so each new fill step
/// only has to test the candidate against bodies it can actually hit.
#[derive(Clone)]
pub(crate) struct PlacedCollisionEntry {
    pub(crate) object_id: String,
    pub(crate) mesh_url: String,
    pub(crate) world: Pose3d,
}

pub(crate) struct FillBuilder {
    pub(crate) base: Fixture,
    pub(crate) fixture: Fixture,
    pub(crate) applied_count: usize,
    pub(crate) sequence: Vec<BrushPlacePayload>,
    pub(crate) appended_objects: Vec<FixtureObject>,
    pub(crate) appended_attractions: Vec<AttractionProps>,
    pub(crate) placed: Vec<PlacedCollisionEntry>,
    pub(crate) candidate_cache: HashMap<String, Vec<BrushCompatibleCandidate>>,
    pub(crate) seed_object_ids: std::collections::HashSet<String>,
    pub(crate) rng_state: u32,
    pub(crate) stalled: bool,
    pub(crate) max_count: usize,
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
        Self {
            base: base.clone(),
            fixture: base,
            applied_count: 0,
            sequence: Vec::new(),
            appended_objects: Vec::new(),
            appended_attractions: Vec::new(),
            placed,
            candidate_cache: HashMap::new(),
            seed_object_ids,
            rng_state: seed,
            stalled: false,
            max_count: FILL_COUNT_MAX,
        }
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
        }
    }
}
