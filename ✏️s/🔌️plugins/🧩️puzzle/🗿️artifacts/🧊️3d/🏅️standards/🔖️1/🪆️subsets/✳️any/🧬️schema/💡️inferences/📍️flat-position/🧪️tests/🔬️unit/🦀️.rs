
use super::*;
use crate::{Puzzle3dAttraction, Puzzle3dObject, Puzzle3dVortex};
use store::{InferenceCache, InferenceCacheConfig};

//#region 🧸️Fixtures
fn vortex(id: &str, position: [f64; 3], direction: [f64; 3]) -> Puzzle3dVortex {
    Puzzle3dVortex { id: id.into(), vortex_kind: None, label: None, position, direction: Some(direction), radius: None, hidden: false, locked: false }
}

fn object(id: &str, origin: [f64; 3], anchor: Puzzle3dObjectAnchor, vortices: Vec<Puzzle3dVortex>) -> Puzzle3dObject {
    Puzzle3dObject { id: id.into(), label: None, object_kind: None, anchor, origin, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, mesh_url: None, vortices, hidden: false, locked: false }
}

pub(crate) fn chain_snapshot() -> Puzzle3dSnapshot {
    // root -A- mid -B- leaf: a 3-object chain so an ancestor change propagates to a grandchild.
    let root = object("root", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Fixed, vec![vortex("top", [0.0, 0.0, 1.0], [0.0, 0.0, 1.0])]);
    let mut mid = object("mid", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Derived, vec![vortex("bottom", [0.0, 0.0, -1.0], [0.0, 0.0, -1.0]), vortex("top", [0.0, 0.0, 1.0], [0.0, 0.0, 1.0])]);
    mid.anchor = Puzzle3dObjectAnchor::Derived;
    let leaf = object("leaf", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Derived, vec![vortex("bottom", [0.0, 0.0, -1.0], [0.0, 0.0, -1.0])]);
    let attraction_a = Puzzle3dAttraction { id: "a1".into(), attracting: "root:top".into(), attracted: "mid:bottom".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 1.0, y: 0.0 };
    let attraction_b = Puzzle3dAttraction { id: "a2".into(), attracting: "mid:top".into(), attracted: "leaf:bottom".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 1.0 };
    Puzzle3dSnapshot {
        schema: crate::PUZZLE_3D_SCHEMA.to_string(),
        domain: "architecture".into(),
        meta: Default::default(),
        objects: vec![root, mid, leaf],
        attractions: vec![attraction_a, attraction_b],
        target_volumes: Vec::new(),
        references: Vec::new(),
    }
}
//#endregion 🧸️Fixtures

//#region 🧪️IncrementalityLaw
#[test]
fn changing_a_leaf_own_vortex_does_not_recompute_ancestors() {
    let mut cache = semio_framework::io::resolve_ready(InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }));
    let base = chain_snapshot();
    let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.objects[2].vortices[0].position = [0.0, 0.0, -5.0]; // leaf's own vortex moves
    let before = semio_framework::io::resolve_ready(cache.stats());
    let planes = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&changed, Some(&mut cache));
    let after = semio_framework::io::resolve_ready(cache.stats());

    assert_eq!(after.misses - before.misses, 1, "only the leaf itself may miss when its own vortex changes");
    assert_eq!(planes.get("root"), Some(&orientation_to_plane([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0])));
}

#[test]
fn changing_the_root_position_recomputes_the_whole_chain() {
    let mut cache = semio_framework::io::resolve_ready(InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }));
    let base = chain_snapshot();
    let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.objects[0].origin = [9.0, 9.0, 9.0]; // root moves
    let before = semio_framework::io::resolve_ready(cache.stats());
    let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&changed, Some(&mut cache));
    let after = semio_framework::io::resolve_ready(cache.stats());

    assert_eq!(after.misses - before.misses, 3, "root + mid + leaf must all miss when the root's own plane changes");
}

#[test]
fn changing_an_attraction_center_param_never_touches_the_plane_chain() {
    let mut plane_cache = semio_framework::io::resolve_ready(InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }));
    let base = chain_snapshot();
    let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&base, Some(&mut plane_cache));

    let mut changed = base.clone();
    changed.attractions[0].x = 42.0; // center-only param
    let before = semio_framework::io::resolve_ready(plane_cache.stats());
    let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&changed, Some(&mut plane_cache));
    let after = semio_framework::io::resolve_ready(plane_cache.stats());
    assert_eq!(after.misses, before.misses, "the plane chain must be cache-hit-only when only a center param (x/y) changes");

    let mut center_cache = semio_framework::io::resolve_ready(InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }));
    let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatCenter>(&base, Some(&mut center_cache));
    let before = semio_framework::io::resolve_ready(center_cache.stats());
    let _ = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatCenter>(&changed, Some(&mut center_cache));
    let after = semio_framework::io::resolve_ready(center_cache.stats());
    assert!(after.misses > before.misses, "the center chain must miss when a center param changes");
}
//#endregion 🧪️IncrementalityLaw

//#region 🧪️CacheTransparencyLaw
#[test]
fn disabled_cache_matches_pure_recompute() {
    let snapshot = chain_snapshot();
    let mut disabled = semio_framework::io::resolve_ready(InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }));
    let pure_planes = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&snapshot, None);
    let via_disabled = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(&snapshot, Some(&mut disabled));
    assert_eq!(pure_planes, via_disabled);
}
//#endregion 🧪️CacheTransparencyLaw
