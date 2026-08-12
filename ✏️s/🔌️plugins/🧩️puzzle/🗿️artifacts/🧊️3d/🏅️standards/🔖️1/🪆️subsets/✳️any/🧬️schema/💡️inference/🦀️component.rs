//! 💡️ Puzzle3d inference schema — `flatPosition` (plane + center), the fourth schema family
//! alongside snapshot/diff/mutations (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
//! Canonical worked example: cached per-object flatten via two independent dependency-hash chains
//! (plane, center) per the closed-ticket merkle design (26/04/17/OPTIMIZE-FLATTEN-DESIGN-WITH-MERKLE-HASH-CACHE)
//! — an object's flatPosition entry changes iff its own vortex, an ancestor's plane/center, or the
//! connecting attraction's params change; nothing else. The result TYPES (`FlattenPlane`/
//! `FlattenPose`) and the low-level per-edge math stay owned by `⚙️engine/📐️geometry/🎛flatten` —
//! this facet only assembles/declares the schema and drives incremental per-entity caching over it.

use crate::artifacts::puzzle3d::standards::v1::engine::geometry::flatten::{
    compute_child_plane, diagram_center, find_vortex, flatten_objects_with_assignment, flatten_snapshot, orientation_to_plane, vortex_geom, FlattenParent, FlattenPlane, FlattenPose,
};
use crate::artifacts::puzzle3d::{Puzzle3dObjectAnchor, Puzzle3dSnapshot};
use artifact_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️Inference
/// 💡️ Everything inferable from a puzzle3d snapshot: absolute flatten pose per object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle3d.inference")]
pub struct Puzzle3dInference {
    #[state(inferred)]
    pub flat_positions: BTreeMap<String, FlattenPose>,
}

impl protocol::Inference<Puzzle3dSnapshot> for Puzzle3dInference {
    fn infer(snapshot: &Puzzle3dSnapshot) -> Self {
        Self { flat_positions: flatten_snapshot(snapshot).into_iter().collect() }
    }
}

impl protocol::InferenceSpec<Puzzle3dSnapshot> for Puzzle3dInference {
    fn inference_schema_id() -> &'static str {
        "s.puzzle.puzzle3d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.puzzle.puzzle3d.inference.flatPosition.plane", reads: &["objects", "attractions"] },
            protocol::InferenceFieldSpec { id: "s.puzzle.puzzle3d.inference.flatPosition.center", reads: &["objects", "attractions"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️DependencyHashChains
// 🎯️ First-cut correctness-over-performance: `assignment_for` re-runs the O(objects+attractions)
// BFS on every `dep_input`/`compute` call, so a full `infer_field` pass here is O(n²) rather than
// O(n) — acceptable at pilot/example scale. A later wave can thread the assignment through once per
// `infer_field` call (e.g. by widening `InferredField::plan`'s contract) instead of re-deriving it.
fn assignment_for(snapshot: &Puzzle3dSnapshot) -> HashMap<String, FlattenParent> {
    flatten_objects_with_assignment(&snapshot.objects, &snapshot.attractions, None).2
}

fn push_numbers(bytes: &mut Vec<u8>, values: impl IntoIterator<Item = f64>) {
    for value in values {
        bytes.extend(semio_framework_hash::format_number_for_hash(value).as_bytes());
        bytes.push(0x1f);
    }
}

/// 🎛️ `flatPosition.plane` — root dep = fixed plane (anchor + origin + orientation); chain dep =
/// parent PlaneHash (folded in by the driver via `parents`) + both connectors' point/direction +
/// gap/shift/rise/rotation/turn/tilt. Matches the merkle-hash ticket's `PlaneHash` chain exactly.
pub struct Puzzle3dFlatPlane;

impl store::InferredField<Puzzle3dSnapshot> for Puzzle3dFlatPlane {
    type Key = String;
    type Value = FlattenPlane;
    const FIELD_ID: &'static str = "s.puzzle.puzzle3d.inference.flatPosition.plane";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["objects", "attractions"]
    }

    fn plan(snapshot: &Puzzle3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        let (_, order, assignment) = flatten_objects_with_assignment(&snapshot.objects, &snapshot.attractions, None);
        order
            .into_iter()
            .map(|id| {
                let parents = match assignment.get(&id) {
                    Some(FlattenParent::Child { parent_id, .. }) => vec![parent_id.clone()],
                    _ => Vec::new(),
                };
                store::InferenceStep { key: id, parents }
            })
            .collect()
    }

    fn dep_input(snapshot: &Puzzle3dSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let assignment = assignment_for(snapshot);
        let mut bytes = Vec::new();
        match assignment.get(key) {
            Some(FlattenParent::Child { parent_id, attraction_index, parent_vortex_id, child_vortex_id }) => {
                let parent_object = snapshot.objects.iter().find(|o| &o.id == parent_id);
                let child_object = snapshot.objects.iter().find(|o| &o.id == key);
                let edge = parent_object
                    .and_then(|p| find_vortex(p, parent_vortex_id))
                    .zip(child_object.and_then(|c| find_vortex(c, child_vortex_id)))
                    .zip(snapshot.attractions.get(*attraction_index));
                if let Some(((parent_vortex, child_vortex), attraction)) = edge {
                    let (pp, pd, _) = vortex_geom(parent_vortex);
                    let (cp, cd, _) = vortex_geom(child_vortex);
                    push_numbers(&mut bytes, pp);
                    push_numbers(&mut bytes, pd);
                    push_numbers(&mut bytes, cp);
                    push_numbers(&mut bytes, cd);
                    push_numbers(&mut bytes, [attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt]);
                }
            }
            _ => {
                if let Some(object) = snapshot.objects.iter().find(|o| &o.id == key) {
                    bytes.push(matches!(object.anchor, Puzzle3dObjectAnchor::Fixed) as u8);
                    push_numbers(&mut bytes, object.origin);
                    if let Some(orientation) = object.orientation {
                        push_numbers(&mut bytes, orientation);
                    }
                }
            }
        }
        bytes
    }

    fn compute(snapshot: &Puzzle3dSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        let assignment = assignment_for(snapshot);
        match assignment.get(key) {
            Some(FlattenParent::Child { parent_id, attraction_index, parent_vortex_id, child_vortex_id }) => {
                let parent_plane = parents.first().copied().unwrap_or_default();
                let parent_object = snapshot.objects.iter().find(|o| &o.id == parent_id);
                let child_object = snapshot.objects.iter().find(|o| &o.id == key);
                let edge = parent_object
                    .and_then(|p| find_vortex(p, parent_vortex_id))
                    .zip(child_object.and_then(|c| find_vortex(c, child_vortex_id)))
                    .zip(snapshot.attractions.get(*attraction_index));
                match edge {
                    Some(((parent_vortex, child_vortex), attraction)) => {
                        let (pp, pd, _) = vortex_geom(parent_vortex);
                        let (cp, cd, _) = vortex_geom(child_vortex);
                        compute_child_plane(parent_plane, pp, pd, cp, cd, attraction)
                    }
                    None => FlattenPlane::default(),
                }
            }
            _ => match snapshot.objects.iter().find(|o| &o.id == key) {
                Some(object) if matches!(object.anchor, Puzzle3dObjectAnchor::Fixed) => orientation_to_plane(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])),
                _ => FlattenPlane::default(),
            },
        }
    }
}

/// 🎛️ `flatPosition.center` — root dep = fixed center (always `[0, 0]` today, `flatten_snapshot`
/// never seeds centers); chain dep = parent CenterHash (folded in via `parents`) + parent connector
/// `direction.z`/`t` + attraction `x`/`y`. Deliberately a SEPARATE chain from `Puzzle3dFlatPlane`: a
/// center-only change (attraction `x`/`y`) must never invalidate the plane chain, and vice versa.
pub struct Puzzle3dFlatCenter;

impl store::InferredField<Puzzle3dSnapshot> for Puzzle3dFlatCenter {
    type Key = String;
    type Value = [f64; 2];
    const FIELD_ID: &'static str = "s.puzzle.puzzle3d.inference.flatPosition.center";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["objects", "attractions"]
    }

    fn plan(snapshot: &Puzzle3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        Puzzle3dFlatPlane::plan(snapshot)
    }

    fn dep_input(snapshot: &Puzzle3dSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let assignment = assignment_for(snapshot);
        let mut bytes = Vec::new();
        match assignment.get(key) {
            Some(FlattenParent::Child { parent_id, attraction_index, parent_vortex_id, .. }) => {
                let parent_object = snapshot.objects.iter().find(|o| &o.id == parent_id);
                let edge = parent_object.and_then(|p| find_vortex(p, parent_vortex_id)).zip(snapshot.attractions.get(*attraction_index));
                if let Some((parent_vortex, attraction)) = edge {
                    let (_, pd, pt) = vortex_geom(parent_vortex);
                    push_numbers(&mut bytes, pd);
                    push_numbers(&mut bytes, [pt, attraction.x, attraction.y]);
                }
            }
            _ => bytes.push(0),
        }
        bytes
    }

    fn compute(snapshot: &Puzzle3dSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        let assignment = assignment_for(snapshot);
        match assignment.get(key) {
            Some(FlattenParent::Child { parent_id, attraction_index, parent_vortex_id, .. }) => {
                let parent_center = parents.first().copied().unwrap_or([0.0, 0.0]);
                let parent_object = snapshot.objects.iter().find(|o| &o.id == parent_id);
                let edge = parent_object.and_then(|p| find_vortex(p, parent_vortex_id)).zip(snapshot.attractions.get(*attraction_index));
                match edge {
                    Some((parent_vortex, attraction)) => {
                        let (_, pd, pt) = vortex_geom(parent_vortex);
                        diagram_center(parent_center, pd, pt, attraction)
                    }
                    None => [0.0, 0.0],
                }
            }
            _ => [0.0, 0.0],
        }
    }
}
//#endregion 🔖️DependencyHashChains

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::puzzle3d::standards::v1::subsets::any::builder::Puzzle3dBuilder {
    type Snapshot = Puzzle3dSnapshot;
    type Inference = Puzzle3dInference;

    fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = session;
        let planes = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(snapshot, Some(cache));
        let centers = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatCenter>(snapshot, Some(cache));
        let flat_positions = planes
            .into_iter()
            .map(|(id, plane)| {
                let center = centers.get(&id).copied().unwrap_or([0.0, 0.0]);
                let orientation = crate::artifacts::puzzle3d::standards::v1::engine::geometry::flatten::plane_to_orientation(plane);
                (id, FlattenPose { plane, center, orientation })
            })
            .collect();
        Puzzle3dInference { flat_positions }
    }
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.puzzle.puzzle3d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `puzzle3d_artifact_schema_descriptor`'s registration.
pub fn puzzle3d_artifact_inference_descriptor() -> artifact_schema::ArtifactInferenceDescriptor {
    artifact_schema::ArtifactInferenceDescriptor {
        id: "s.puzzle.puzzle3d.inference",
        inference: artifact_schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dObject, Puzzle3dVortex};
    use protocol::Inference;
    use store::{InferenceCache, InferenceCacheConfig, InferredField};

    //#region 🧸️Fixtures
    fn vortex(id: &str, position: [f64; 3], direction: [f64; 3]) -> Puzzle3dVortex {
        Puzzle3dVortex { id: id.into(), vortex_kind: None, label: None, position, direction: Some(direction), radius: None, hidden: false, locked: false }
    }

    fn object(id: &str, origin: [f64; 3], anchor: Puzzle3dObjectAnchor, vortices: Vec<Puzzle3dVortex>) -> Puzzle3dObject {
        Puzzle3dObject { id: id.into(), label: None, object_kind: None, anchor, origin, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, mesh_url: None, vortices, hidden: false, locked: false }
    }

    fn chain_snapshot() -> Puzzle3dSnapshot {
        // root -A- mid -B- leaf: a 3-object chain so an ancestor change propagates to a grandchild.
        let root = object("root", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Fixed, vec![vortex("top", [0.0, 0.0, 1.0], [0.0, 0.0, 1.0])]);
        let mut mid = object("mid", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Derived, vec![vortex("bottom", [0.0, 0.0, -1.0], [0.0, 0.0, -1.0]), vortex("top", [0.0, 0.0, 1.0], [0.0, 0.0, 1.0])]);
        mid.anchor = Puzzle3dObjectAnchor::Derived;
        let leaf = object("leaf", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Derived, vec![vortex("bottom", [0.0, 0.0, -1.0], [0.0, 0.0, -1.0])]);
        let attraction_a = Puzzle3dAttraction { id: "a1".into(), attracting: "root:top".into(), attracted: "mid:bottom".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 1.0, y: 0.0 };
        let attraction_b = Puzzle3dAttraction { id: "a2".into(), attracting: "mid:top".into(), attracted: "leaf:bottom".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 1.0 };
        Puzzle3dSnapshot { schema: crate::artifacts::puzzle3d::PUZZLE_3D_SCHEMA.to_string(), domain: "architecture".into(), meta: Default::default(), objects: vec![root, mid, leaf], attractions: vec![attraction_a, attraction_b], target_volumes: Vec::new(), references: Vec::new() }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = chain_snapshot();
        assert_eq!(Puzzle3dInference::infer(&snapshot), Puzzle3dInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Puzzle3dInference::infer(&Puzzle3dSnapshot::default()), Puzzle3dInference::default());
    }

    #[test]
    fn inference_matches_flatten_snapshot_directly() {
        let snapshot = chain_snapshot();
        let inferred = Puzzle3dInference::infer(&snapshot);
        let direct = flatten_snapshot(&snapshot);
        for (id, pose) in &direct {
            assert_eq!(inferred.flat_positions.get(id), Some(pose), "inference must match flatten_snapshot exactly for {id}");
        }
    }
    //#endregion 🧪️InferenceLaws

    //#region 🧪️IncrementalityLaw
    #[test]
    fn changing_a_leaf_own_vortex_does_not_recompute_ancestors() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = chain_snapshot();
        let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.objects[2].vortices[0].position = [0.0, 0.0, -5.0]; // leaf's own vortex moves
        let before = cache.stats();
        let planes = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&changed, Some(&mut cache));
        let after = cache.stats();

        assert_eq!(after.misses - before.misses, 1, "only the leaf itself may miss when its own vortex changes");
        assert_eq!(planes.get("root"), Some(&orientation_to_plane([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0])));
    }

    #[test]
    fn changing_the_root_position_recomputes_the_whole_chain() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = chain_snapshot();
        let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.objects[0].origin = [9.0, 9.0, 9.0]; // root moves
        let before = cache.stats();
        let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&changed, Some(&mut cache));
        let after = cache.stats();

        assert_eq!(after.misses - before.misses, 3, "root + mid + leaf must all miss when the root's own plane changes");
    }

    #[test]
    fn changing_an_attraction_center_param_never_touches_the_plane_chain() {
        let mut plane_cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = chain_snapshot();
        let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&base, Some(&mut plane_cache));

        let mut changed = base.clone();
        changed.attractions[0].x = 42.0; // center-only param
        let before = plane_cache.stats();
        let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&changed, Some(&mut plane_cache));
        let after = plane_cache.stats();
        assert_eq!(after.misses, before.misses, "the plane chain must be cache-hit-only when only a center param (x/y) changes");

        let mut center_cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatCenter>(&base, Some(&mut center_cache));
        let before = center_cache.stats();
        let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatCenter>(&changed, Some(&mut center_cache));
        let after = center_cache.stats();
        assert!(after.misses > before.misses, "the center chain must miss when a center param changes");
    }
    //#endregion 🧪️IncrementalityLaw

    //#region 🧪️CacheTransparencyLaw
    #[test]
    fn disabled_cache_matches_pure_infer() {
        let snapshot = chain_snapshot();
        let pure = Puzzle3dInference::infer(&snapshot);
        let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() });
        let planes = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&snapshot, Some(&mut disabled));
        let centers = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatCenter>(&snapshot, Some(&mut disabled));
        for (id, pose) in &pure.flat_positions {
            assert_eq!(planes.get(id), Some(&pose.plane));
            assert_eq!(centers.get(id), Some(&pose.center));
        }
    }
    //#endregion 🧪️CacheTransparencyLaw
}
//#endregion 🧪️Tests
