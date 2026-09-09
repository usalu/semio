use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex};
use store::{InferenceCache, InferenceCacheConfig};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn valid_snapshot() -> SemioBrepSnapshot {
    let mut s = SemioBrepSnapshot::default();
    s.vertices = vec![BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, tol: 0.0 }];
    s.edges = vec![BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Circle { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 1.0 }, tol: 0.0 }];
    s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] }];
    s.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true, tol: 0.0 }];
    s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
    s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];
    s
}

//#region 🧪️Honesty
#[semio_framework_async_macros::async_test]
async fn valid_snapshot_has_no_findings() {
    let values = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&valid_snapshot(), None);
    assert!(values["document"].is_empty());
}

#[semio_framework_async_macros::async_test]
async fn dangling_reference_is_a_real_finding_not_a_faked_one() {
    let mut broken = valid_snapshot();
    broken.edges[0].end_vertex = "v-missing".into();
    let values = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&broken, None);
    let findings = &values["document"];
    assert!(findings.iter().any(|d| d.code == "stdio.semio_brep.dangling-edge-end-vertex"), "findings: {findings:?}");
}
//#endregion 🧪️Honesty

//#region 🧪️CacheTransparencyLaw
#[semio_framework_async_macros::async_test]
async fn disabled_cache_matches_pure_recompute() {
    let snapshot = valid_snapshot();
    let pure = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&snapshot, None);
    let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
    let via_disabled = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&snapshot, Some(&mut disabled));
    assert_eq!(pure, via_disabled);
}
//#endregion 🧪️CacheTransparencyLaw

//#region 🧪️IncrementalityLaw
#[semio_framework_async_macros::async_test]
async fn identical_snapshot_recompute_is_a_cache_hit() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = valid_snapshot();
    let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&base, Some(&mut cache));
    let before = cache.stats().await;
    let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&base, Some(&mut cache));
    let after = cache.stats().await;
    assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
    assert_eq!(after.hits - before.hits, 1);
}

#[semio_framework_async_macros::async_test]
async fn changing_any_collection_misses_the_cache() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = valid_snapshot();
    let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&base, Some(&mut cache));
    let mut changed = base.clone();
    changed.vertices[0].point = SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 };
    let before = cache.stats().await;
    let _ = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(&changed, Some(&mut cache));
    let after = cache.stats().await;
    assert_eq!(after.misses - before.misses, 1, "a real change to a covered collection must miss");
}
//#endregion 🧪️IncrementalityLaw
