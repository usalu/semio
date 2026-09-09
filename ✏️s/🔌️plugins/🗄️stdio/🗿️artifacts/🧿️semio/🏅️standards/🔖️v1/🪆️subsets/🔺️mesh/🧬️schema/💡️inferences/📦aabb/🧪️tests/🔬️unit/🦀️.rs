use super::*;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};
use store::{InferenceCache, InferenceCacheConfig};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn two_primitive_snapshot() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        meshes: vec![SemioMesh {
            id: "mesh-a".into(),
            primitives: vec![
                SemioPrimitive { id: "prim-1".into(), positions: vec![SemioPoint3 { x: -1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 }], ..Default::default() },
                SemioPrimitive { id: "prim-2".into(), positions: vec![SemioPoint3 { x: 5.0, y: 5.0, z: 5.0 }], ..Default::default() },
            ],
        }],
        ..Default::default()
    }
}

//#region 🧪️Honesty
#[semio_framework_async_macros::async_test]
async fn aabb_of_a_populated_primitive_is_the_real_componentwise_extent() {
    let values = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&two_primitive_snapshot(), None);
    let aabb = values.get(&aabb_key("mesh-a", "prim-1")).expect("prim-1 aabb present");
    assert_eq!(aabb.min, SemioPoint3 { x: -1.0, y: 0.0, z: 0.0 });
    assert_eq!(aabb.max, SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 });
}

#[semio_framework_async_macros::async_test]
async fn aabb_of_an_empty_primitive_is_the_honest_default_not_a_faked_extent() {
    let snapshot = SemioMeshSnapshot { meshes: vec![SemioMesh { id: "mesh-a".into(), primitives: vec![SemioPrimitive { id: "empty".into(), ..Default::default() }] }], ..Default::default() };
    let values = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&snapshot, None);
    assert_eq!(values.get(&aabb_key("mesh-a", "empty")), Some(&SemioAabb::default()));
}
//#endregion 🧪️Honesty

//#region 🧪️CacheTransparencyLaw
#[semio_framework_async_macros::async_test]
async fn disabled_cache_matches_pure_recompute() {
    let snapshot = two_primitive_snapshot();
    let pure = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&snapshot, None);
    let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
    let via_disabled = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&snapshot, Some(&mut disabled));
    assert_eq!(pure, via_disabled);
}
//#endregion 🧪️CacheTransparencyLaw

//#region 🧪️IncrementalityLaw
#[semio_framework_async_macros::async_test]
async fn identical_snapshot_recompute_is_a_cache_hit() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_primitive_snapshot();
    let _ = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&base, Some(&mut cache));
    let before = cache.stats().await;
    let _ = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&base, Some(&mut cache));
    let after = cache.stats().await;
    assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
    assert_eq!(after.hits - before.hits, 2, "both primitives must be cache hits");
}

#[semio_framework_async_macros::async_test]
async fn changing_one_primitives_positions_misses_only_that_primitives_cache_entry() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_primitive_snapshot();
    let _ = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.meshes[0].primitives[0].positions[0] = SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 };
    let before = cache.stats().await;
    let values = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&changed, Some(&mut cache));
    let after = cache.stats().await;

    assert_eq!(after.misses - before.misses, 1, "only prim-1's own entry may miss when its own positions change");
    assert_eq!(values.get(&aabb_key("mesh-a", "prim-2")), Some(&SemioAabb { min: SemioPoint3 { x: 5.0, y: 5.0, z: 5.0 }, max: SemioPoint3 { x: 5.0, y: 5.0, z: 5.0 } }), "prim-2's aabb must be untouched");
}

#[semio_framework_async_macros::async_test]
async fn changing_an_unrelated_field_on_the_same_primitive_does_not_miss() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_primitive_snapshot();
    let _ = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.meshes[0].primitives[0].material_id = Some("some-material".into());
    let before = cache.stats().await;
    let _ = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&changed, Some(&mut cache));
    let after = cache.stats().await;
    assert_eq!(after.misses, before.misses, "material_id has no bearing on the aabb dep chain");
}
//#endregion 🧪️IncrementalityLaw
