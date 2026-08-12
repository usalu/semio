//! 📦 `aabb` — real per-primitive axis-aligned bounding box, computed as a genuine
//! `InferredField<SemioMeshSnapshot>` (not a bare pass-through): a real `DepHash` chain keyed per
//! (`mesh_id`,`primitive_id`), one step per primitive, NO parents — each primitive's AABB depends
//! only on its OWN `positions`, never on any other primitive's, so this is the honest per-object
//! chain shape (proven puzzle3d `🎛flat-position` pilot's own convention), stronger than a
//! whole-document single-key fallback: the incrementality-law tests below prove touching one
//! primitive misses ONLY that primitive's cache entry.
//!
//! `computed-normals`/`tessellation-preview` are DELIBERATELY OMITTED from this facet — not
//! forgotten, but because a real chain cannot be authored honestly for them without colliding with
//! this subset's own authored data or duplicating a rendering-engine-owned convention:
//! - `computed-normals` would infer a SECOND, competing definition of `normals` that might
//!   disagree with the snapshot's own already-persisted `SemioPrimitive.normals` field (custom
//!   sculpted/stylized normals are a legitimate authored value this format explicitly supports) —
//!   inferring a shadow value for a field the mutation vocabulary already owns as tier-(b)
//!   authored state would blur the tier-(b)/(c) boundary this whole ticket exists to keep sharp.
//!   It also has no single canonical algorithm (flat vs. smooth vs. area/angle-weighted averaging,
//!   and strip/fan connectivity differs from a plain triangle list) — picking one at the stdio
//!   pure-value layer risks diverging from whatever the real renderer/engine expects, the same
//!   duplication risk brep's own `tessellation`/`mass-properties` omission flags for NURBS math.
//! - `tessellation-preview` is not a genuine derivation for THIS subset: `positions`/`indices`/
//!   `topology` already ARE the tessellated render buffers (unlike brep, whose topological B-rep
//!   needs real curve/surface evaluation to produce a renderable mesh at all). A "preview" that
//!   merely copies already-authoritative snapshot data is not an honest inference, and canonicalizing
//!   `TriangleStrip`/`TriangleFan`/`Lines`/`Points` into a uniform triangle list is again a
//!   rendering-engine-owned convention with no single canonical answer at this layer.
//!
//! Neither omission is a silently dropped field — both are the sanctioned outcome per
//! `📌️important.md`'s "if a real dependency chain cannot be authored honestly for a field, omit
//! that field and say why rather than faking one", mirroring brep's own two omissions in shape.

use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive};
use serde::{Deserialize, Serialize};

//#region 🔖️Value
/// 📦 One primitive's axis-aligned bounding box. `SemioAabb::default()` (`min`/`max` both the
/// origin) is the honest "no geometry" value for a primitive with an empty `positions` buffer —
/// same convention brep's `validationReport` uses an empty `Vec` for "nothing to report".
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioAabb {
    pub min: SemioPoint3,
    pub max: SemioPoint3,
}
//#endregion 🔖️Value

//#region 🔖️Lookup
/// 🔎 Composite key = `"{mesh_id}:{primitive_id}"`. Looked up by RECONSTRUCTING the same key for
/// comparison (never by splitting the key string apart) — a mesh or primitive id may itself
/// contain `:`, so parsing the key back into two ids would be ambiguous; comparing constructed
/// keys is not. O(meshes × primitives) per lookup — first-cut correctness over performance, same
/// documented tradeoff the proven `🎛flat-position` pilot's own `assignment_for` makes.
fn find_primitive_by_key<'a>(snapshot: &'a SemioMeshSnapshot, key: &str) -> Option<(&'a SemioMesh, &'a SemioPrimitive)> {
    snapshot.meshes.iter().find_map(|mesh| mesh.primitives.iter().find(|p| aabb_key(&mesh.id, &p.id) == key).map(|p| (mesh, p)))
}
pub(crate) fn aabb_key(mesh_id: &str, primitive_id: &str) -> String {
    format!("{mesh_id}:{primitive_id}")
}
//#endregion 🔖️Lookup

//#region 🔖️DependencyHashChain
pub struct MeshAabb;

impl store::InferredField<SemioMeshSnapshot> for MeshAabb {
    type Key = String;
    type Value = SemioAabb;
    const FIELD_ID: &'static str = "s.stdio.semio.mesh.inference.aabb";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["meshes"]
    }

    fn plan(snapshot: &SemioMeshSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        snapshot
            .meshes
            .iter()
            .flat_map(|mesh| mesh.primitives.iter().map(move |p| store::InferenceStep { key: aabb_key(&mesh.id, &p.id), parents: Vec::new() }))
            .collect()
    }

    /// 🔑 Canonical dependency-input bytes — EXACTLY `positions` (the only field `compute` reads),
    /// nothing else (not `normals`/`uvs`/`colors`/`indices`/`material_id`, none of which affect an
    /// AABB) — an unrelated field touch on the SAME primitive must still hit the cache, proven by
    /// the incrementality-law test below.
    fn dep_input(snapshot: &SemioMeshSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        match find_primitive_by_key(snapshot, key) {
            Some((_, primitive)) => serde_json::to_vec(&primitive.positions).unwrap_or_default(),
            None => Vec::new(),
        }
    }

    fn compute(snapshot: &SemioMeshSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let Some((_, primitive)) = find_primitive_by_key(snapshot, key) else {
            return SemioAabb::default();
        };
        let Some(first) = primitive.positions.first().copied() else {
            return SemioAabb::default();
        };
        let mut min = first;
        let mut max = first;
        for p in &primitive.positions[1..] {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }
        SemioAabb { min, max }
    }
}
//#endregion 🔖️DependencyHashChain

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};
    use store::{InferenceCache, InferenceCacheConfig, InferredField};

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
    #[test]
    fn aabb_of_a_populated_primitive_is_the_real_componentwise_extent() {
        let values = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&two_primitive_snapshot(), None);
        let aabb = values.get(&aabb_key("mesh-a", "prim-1")).expect("prim-1 aabb present");
        assert_eq!(aabb.min, SemioPoint3 { x: -1.0, y: 0.0, z: 0.0 });
        assert_eq!(aabb.max, SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 });
    }

    #[test]
    fn aabb_of_an_empty_primitive_is_the_honest_default_not_a_faked_extent() {
        let snapshot = SemioMeshSnapshot { meshes: vec![SemioMesh { id: "mesh-a".into(), primitives: vec![SemioPrimitive { id: "empty".into(), ..Default::default() }] }], ..Default::default() };
        let values = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&snapshot, None);
        assert_eq!(values.get(&aabb_key("mesh-a", "empty")), Some(&SemioAabb::default()));
    }
    //#endregion 🧪️Honesty

    //#region 🧪️CacheTransparencyLaw
    #[test]
    fn disabled_cache_matches_pure_recompute() {
        let snapshot = two_primitive_snapshot();
        let pure = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&snapshot, None);
        let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() });
        let via_disabled = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&snapshot, Some(&mut disabled));
        assert_eq!(pure, via_disabled);
    }
    //#endregion 🧪️CacheTransparencyLaw

    //#region 🧪️IncrementalityLaw
    #[test]
    fn identical_snapshot_recompute_is_a_cache_hit() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_primitive_snapshot();
        let _ = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&base, Some(&mut cache));
        let before = cache.stats();
        let _ = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&base, Some(&mut cache));
        let after = cache.stats();
        assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
        assert_eq!(after.hits - before.hits, 2, "both primitives must be cache hits");
    }

    #[test]
    fn changing_one_primitives_positions_misses_only_that_primitives_cache_entry() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_primitive_snapshot();
        let _ = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.meshes[0].primitives[0].positions[0] = SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 };
        let before = cache.stats();
        let values = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&changed, Some(&mut cache));
        let after = cache.stats();

        assert_eq!(after.misses - before.misses, 1, "only prim-1's own entry may miss when its own positions change");
        assert_eq!(values.get(&aabb_key("mesh-a", "prim-2")), Some(&SemioAabb { min: SemioPoint3 { x: 5.0, y: 5.0, z: 5.0 }, max: SemioPoint3 { x: 5.0, y: 5.0, z: 5.0 } }), "prim-2's aabb must be untouched");
    }

    #[test]
    fn changing_an_unrelated_field_on_the_same_primitive_does_not_miss() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_primitive_snapshot();
        let _ = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.meshes[0].primitives[0].material_id = Some("some-material".into());
        let before = cache.stats();
        let _ = store::infer_field::<SemioMeshSnapshot, MeshAabb>(&changed, Some(&mut cache));
        let after = cache.stats();
        assert_eq!(after.misses, before.misses, "material_id has no bearing on the aabb dep chain");
    }
    //#endregion 🧪️IncrementalityLaw
}
//#endregion 🧪️Tests
