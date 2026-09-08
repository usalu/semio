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

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive};

//#region 🔖️Value
/// 📦 One primitive's axis-aligned bounding box. `SemioAabb::default()` (`min`/`max` both the
/// origin) is the honest "no geometry" value for a primitive with an empty `positions` buffer —
/// same convention brep's `validationReport` uses an empty `Vec` for "nothing to report".
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioAabb {
    pub min: SemioPoint3,
    pub max: SemioPoint3,
}
//#endregion 🔖️Value

//#region 🌉️SerdeBridge
/// 🌉 Hand-written, not dual-derived: `SemioAabb`'s own field type `SemioPoint3` has already
/// dropped `serde` entirely in favor of `ToValue`/`FromValue` — a `#[derive(Serialize,
/// Deserialize)]` here would need it to grow serde back. `store::InferredField::Value` still
/// bounds on `Serialize + DeserializeOwned` (a genuine byte-cache codec, not a stale requirement),
/// so this bridges through the value this type's own `ToValue`/`FromValue` already compute — same
/// bridge shape as `🎛flattened-scene`'s `FlattenedNode` (see its docstring for the full rationale)
/// and, mirrored, the store's `🌉️SerdeValueBridge` (`🏪️store/🧬️schema/🧬️mutations/🦀️.rs`).
impl serde::Serialize for SemioAabb {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde_json::Value::from(&<Self as store::ToValue>::to_value(self)).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for SemioAabb {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = serde_json::Value::deserialize(deserializer)?;
        <Self as store::FromValue>::from_value(store::DslValue::from(json)).map_err(serde::de::Error::custom)
    }
}
//#endregion 🌉️SerdeBridge

//#region 🔖️Lookup
/// 🔎 Composite key = `"{mesh_id}:{primitive_id}"`. Looked up by RECONSTRUCTING the same key for
/// comparison (never by splitting the key string apart) — a mesh or primitive id may itself
/// contain `:`, so parsing the key back into two ids would be ambiguous; comparing constructed
/// keys is not. O(meshes × primitives) per lookup — first-cut correctness over performance, same
/// documented tradeoff the proven `🎛flat-position` pilot's own `assignment_for` makes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_primitive_by_key<'a>(snapshot: &'a SemioMeshSnapshot, key: &str) -> Option<(&'a SemioMesh, &'a SemioPrimitive)> {
    snapshot.meshes.iter().find_map(|mesh| mesh.primitives.iter().find(|p| aabb_key(&mesh.id, &p.id) == key).map(|p| (mesh, p)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
        snapshot.meshes.iter().flat_map(|mesh| mesh.primitives.iter().map(move |p| store::InferenceStep { key: aabb_key(&mesh.id, &p.id), parents: Vec::new() })).collect()
    }

    /// 🔑 Canonical dependency-input bytes — EXACTLY `positions` (the only field `compute` reads),
    /// nothing else (not `normals`/`uvs`/`colors`/`indices`/`material_id`, none of which affect an
    /// AABB) — an unrelated field touch on the SAME primitive must still hit the cache, proven by
    /// the incrementality-law test below.
    fn dep_input(snapshot: &SemioMeshSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        match find_primitive_by_key(snapshot, key) {
            Some((_, primitive)) => pack::to_json_string(&primitive.positions).into_bytes(),
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
