//! 🔺️ SemioMeshDiff — handcrafted sparse diff over `SemioMeshSnapshot`. No
//! `snapshot: Option<SemioMeshSnapshot>` full-replace slot — even a whole-document replace
//! diffs as the sparse field-by-field `SemioMeshDiff::between(base, next)`.
//!
//! `meshes`/`materials`/`textures` (id-keyed) and, within a modified mesh, `primitives` (also
//! id-keyed) are diffed via the shared generic `engine::triples::NamedTripleDiff<K, D, T>` —
//! reusing the SAME type bcf/docx hand-rolled their own copy of (f6-final-summary.md §4.4: no
//! `DslField` bridge exists for generic collection-diff wrappers, so every subset hand-writes its
//! own `between`/`apply`/`inverse`/`absorb` algorithm over the shared struct rather than
//! reinventing the struct itself — see `w1b-type-ownership.md`'s "🧰️triples" entry).

use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::standards::v1::subsets::base::schema::triples::{NamedModified, NamedTripleDiff};



use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️NamedAdded
/// 🏷️ Local wrapper carrying the real target position for a `NamedTripleDiff<K,D,T>.added` entry.
/// The shared engine's bare `added: Vec<T>` loses position for name/id-keyed collections (unlike
/// its indexed sibling `IndexedTripleDiff<D,T>.added: Vec<IndexAdded<T>>`, which already carries
/// one) — `apply_named` could previously only ever append at the end, silently reordering the
/// reconstructed snapshot whenever a remove+re-add happened together in the same `between()`.
/// Fixed locally (shared `⚙️engine/🧰️triples` is out of this subset's write scope) — same fix,
/// same shape, as `value`'s own `NamedAdded<T>` (`w2a-verify-report.md`'s mesh finding).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedAdded<T> {
    pub index: usize,
    pub item: T,
}
//#endregion 🔖️NamedAdded

//#region 🔖️GenericNamedEngine
/// 🏷️ Name/id-keyed `between`/`apply`/`absorb` over the shared `NamedTripleDiff<K,D,T>` struct,
/// with `T` instantiated as [`NamedAdded<Item>`] for the `added` field so re-added entries land at
/// their real target position instead of always appending at the end. Ported verbatim (same
/// algorithm, generic over key/item/diff) from bcf/docx's own hand-rolled copies — this subset's
/// own instance since no shared generic ALGORITHM exists yet (only the shared struct does; see
/// module doc comment / `w1b-type-ownership.md`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_named<K, T, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, NamedAdded<T>>>
where
    K: PartialEq + Clone,
    T: Clone + PartialEq,
{
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        let bk = key_of(b);
        match other.iter().find(|o| key_of(o) == bk) {
            None => removed.push(bk),
            Some(o) if o != b => {
                if let Some(d) = diff_item(b, o) {
                    modified.push(NamedModified { key: bk, diff: d });
                }
            }
            Some(_) => {}
        }
    }
    let mut added = Vec::new();
    for (idx, o) in other.iter().enumerate() {
        let ok = key_of(o);
        if !base.iter().any(|b| key_of(b) == ok) {
            added.push(NamedAdded { index: idx, item: o.clone() });
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added })
    }
}

/// ▶️ Apply semantics (normative, mirrors `IndexedTripleDiff`'s own `added` handling):
/// `removed`/`modified` resolve by key; `added` entries carry their FINAL-state target position
/// and are inserted ascending at `min(index, len)`, exactly like `value`'s `apply_map_diff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
where
    K: PartialEq + Clone,
    T: Clone,
{
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        if let Some(item) = items.iter_mut().find(|i| key_of(i) == m.key) {
            apply_item(item, &m.diff);
        }
    }
    let mut added_sorted: Vec<&NamedAdded<T>> = diff.added.iter().collect();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        let idx = a.index.min(items.len());
        items.insert(idx, a.item.clone());
    }
}

/// 🧮️ Key-identity absorb: a `d2`-removal of a `d1`-added key annihilates the add; a `d2`-modify
/// of a `d1`-added key patches into the carried payload; everything else composes on the shared
/// key space (canonical cases in `absorb_law` below).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
    let d1_added_keys: Vec<K> = d1.added.iter().map(&key_of).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<T> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(a))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(a) == m2.key) {
            apply_item(added, &m2.diff);
            continue;
        }
        if removed.contains(&m2.key) {
            continue;
        }
        match modified.iter_mut().find(|m| m.key == m2.key) {
            Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
            None => modified.push(NamedModified { key: m2.key.clone(), diff: m2.diff.clone() }),
        }
    }
    for a2 in &d2.added {
        let k2 = key_of(a2);
        match working_added.iter_mut().find(|a| key_of(a) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    NamedTripleDiff { removed, modified, added: working_added }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️DiffTypes
pub type SemioMeshesDiff = NamedTripleDiff<String, SemioMeshItemDiff, NamedAdded<SemioMesh>>;
pub type SemioPrimitivesDiff = NamedTripleDiff<String, SemioPrimitiveDiff, NamedAdded<SemioPrimitive>>;
pub type SemioMaterialsDiff = NamedTripleDiff<String, SemioMaterialDiff, NamedAdded<SemioMaterial>>;
pub type SemioTexturesDiff = NamedTripleDiff<String, SemioTextureDiff, NamedAdded<SemioTexture>>;

/// 🔺️ Diff for `s.stdio.semio.mesh`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioMeshDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub meshes: Option<SemioMeshesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub materials: Option<SemioMaterialsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub textures: Option<SemioTexturesDiff>,
}

/// 🔺️ Per-mesh sparse diff — `id` is the key, so only `primitives` (a nested id-keyed triple)
/// can change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioMeshItemDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub primitives: Option<SemioPrimitivesDiff>,
}

/// 🔺️ Per-primitive sparse diff. `positions`/`normals`/`uvs`/`colors`/`indices` are weak
/// parallel-buffer fields (whole-value replaced, per the recipe — never sub-diffed per vertex).
/// `material_id` is tri-state (`Some(None)` = material reference cleared).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioPrimitiveDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub topology: Option<SemioTopology>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<SemioPoint3>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub normals: Option<Vec<SemioPoint3>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub uvs: Option<Vec<SemioUv>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<SemioRgba>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<u32>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material_id: Option<Option<String>>,
}

/// 🔺️ Per-material sparse diff — `id` is the key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioMaterialDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_color: Option<SemioRgba>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metallic: Option<f32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub roughness: Option<f32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metallic_roughness_texture: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<Option<String>>,
}

/// 🔺️ Per-texture sparse diff — `id` is the key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioTextureDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}
//#endregion 🔖️DiffTypes

//#region 🔖️WrapHelpers
/// 🧭️ Lowers a per-mesh leaf diff into a full `SemioMeshDiff` (mirrors bcf's `wrap_topic_diff` /
/// docx's per-mutation `diff_*` helpers, specialized to this artifact's fixed two-level id
/// nesting — meshes never nest deeper than mesh -> primitives).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wrap_mesh_diff(mesh_id: &str, diff: SemioMeshItemDiff) -> SemioMeshDiff {
    SemioMeshDiff { meshes: Some(SemioMeshesDiff { removed: Vec::new(), modified: vec![NamedModified { key: mesh_id.to_string(), diff }], added: Vec::new() }), materials: None, textures: None }
}

/// 🧭️ Lowers a per-primitive leaf diff (inside mesh `mesh_id`) into a full `SemioMeshDiff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wrap_primitive_diff(mesh_id: &str, primitive_id: &str, diff: SemioPrimitiveDiff) -> SemioMeshDiff {
    wrap_mesh_diff(mesh_id, SemioMeshItemDiff { primitives: Some(SemioPrimitivesDiff { removed: Vec::new(), modified: vec![NamedModified { key: primitive_id.to_string(), diff }], added: Vec::new() }) })
}
//#endregion 🔖️WrapHelpers

//#region 🔖️Apply
impl MutationDiff<SemioMeshSnapshot> for SemioMeshDiff {
    fn apply(&self, base: &SemioMeshSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioMeshSnapshot> {
        let mut next = base.clone();
        if let Some(md) = &self.meshes {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.meshes, md, |item| item.id.clone(), |added| added.item.id.clone(), ["meshes"])?;
            apply_named(&mut next.meshes, md, |m| m.id.clone(), apply_mesh);
        }
        if let Some(md) = &self.materials {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.materials, md, |item| item.id.clone(), |added| added.item.id.clone(), ["materials"])?;
            apply_named(&mut next.materials, md, |m| m.id.clone(), apply_material);
        }
        if let Some(td) = &self.textures {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.textures, td, |item| item.id.clone(), |added| added.item.id.clone(), ["textures"])?;
            apply_named(&mut next.textures, td, |t| t.id.clone(), apply_texture);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.meshes = match (self.meshes.take(), other.meshes) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |m: &NamedAdded<SemioMesh>| m.item.id.clone(), absorb_mesh_diff, apply_mesh_added)),
        };
        self.materials = match (self.materials.take(), other.materials) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |m: &NamedAdded<SemioMaterial>| m.item.id.clone(), |a, b| absorb_material_diff(a, &b), apply_material_added)),
        };
        self.textures = match (self.textures.take(), other.textures) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |t: &NamedAdded<SemioTexture>| t.item.id.clone(), absorb_texture_diff, apply_texture_added)),
        };
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_mesh(mesh: &mut SemioMesh, diff: &SemioMeshItemDiff) {
    if let Some(pd) = &diff.primitives {
        apply_named(&mut mesh.primitives, pd, |p| p.id.clone(), apply_primitive);
    }
}

/// 🧭️ `NamedAdded<T>`-preserving apply wrappers — used ONLY by `absorb_named`'s `apply_item`
/// (patching a `d1`-added item's payload with a `d2`-modify, index untouched); see
/// [`NamedAdded`]'s doc comment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_mesh_added(a: &mut NamedAdded<SemioMesh>, d: &SemioMeshItemDiff) {
    apply_mesh(&mut a.item, d);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_material_added(a: &mut NamedAdded<SemioMaterial>, d: &SemioMaterialDiff) {
    apply_material(&mut a.item, d);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_texture_added(a: &mut NamedAdded<SemioTexture>, d: &SemioTextureDiff) {
    apply_texture(&mut a.item, d);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_primitive_added(a: &mut NamedAdded<SemioPrimitive>, d: &SemioPrimitiveDiff) {
    apply_primitive(&mut a.item, d);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_primitive(prim: &mut SemioPrimitive, diff: &SemioPrimitiveDiff) {
    if let Some(v) = &diff.topology {
        prim.topology = *v;
    }
    if let Some(v) = &diff.positions {
        prim.positions = v.clone();
    }
    if let Some(v) = &diff.normals {
        prim.normals = v.clone();
    }
    if let Some(v) = &diff.uvs {
        prim.uvs = v.clone();
    }
    if let Some(v) = &diff.colors {
        prim.colors = v.clone();
    }
    if let Some(v) = &diff.indices {
        prim.indices = v.clone();
    }
    if let Some(v) = &diff.material_id {
        prim.material_id = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_material(mat: &mut SemioMaterial, diff: &SemioMaterialDiff) {
    if let Some(v) = &diff.base_color {
        mat.base_color = *v;
    }
    if let Some(v) = diff.metallic {
        mat.metallic = v;
    }
    if let Some(v) = diff.roughness {
        mat.roughness = v;
    }
    if let Some(value) = &diff.base_color_texture { mat.base_color_texture = value.clone(); }
    if let Some(value) = &diff.metallic_roughness_texture { mat.metallic_roughness_texture = value.clone(); }
    if let Some(value) = &diff.normal_texture { mat.normal_texture = value.clone(); }
    if let Some(value) = &diff.occlusion_texture { mat.occlusion_texture = value.clone(); }
    if let Some(value) = &diff.emissive_texture { mat.emissive_texture = value.clone(); }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_texture(tex: &mut SemioTexture, diff: &SemioTextureDiff) {
    if let Some(v) = &diff.mime {
        tex.mime = v.clone();
    }
    if let Some(v) = &diff.bytes {
        tex.bytes = v.clone();
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SemioMeshSnapshot> for SemioMeshDiff {
    /// 🔁️ Diff-level undo, derived generically from `between` (same accepted technique `value`'s
    /// own `DiffAlgebra::inverse` uses — recomputing via a real `between()` call sidesteps having
    /// to hand-derive `NamedAdded<T>` position math for the undo direction): `mid = self.apply(base)`,
    /// then `between(mid, base)` is exactly the diff that restores `base` when applied to `mid`.
    fn inverse(&self, base: &SemioMeshSnapshot) -> Self {
        let mid = self.apply(base).unwrap();
        Self::between(&mid, base)
    }

    fn between(base: &SemioMeshSnapshot, other: &SemioMeshSnapshot) -> Self {
        SemioMeshDiff {
            meshes: between_named(&base.meshes, &other.meshes, |m| m.id.clone(), between_mesh),
            materials: between_named(&base.materials, &other.materials, |m| m.id.clone(), between_material),
            textures: between_named(&base.textures, &other.textures, |t| t.id.clone(), between_texture),
        }
    }

    fn is_empty(&self) -> bool {
        self.meshes.is_none() && self.materials.is_none() && self.textures.is_none()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_mesh(base: &SemioMesh, other: &SemioMesh) -> Option<SemioMeshItemDiff> {
    let primitives = between_named(&base.primitives, &other.primitives, |p| p.id.clone(), between_primitive);
    primitives.map(|primitives| SemioMeshItemDiff { primitives: Some(primitives) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_primitive(base: &SemioPrimitive, other: &SemioPrimitive) -> Option<SemioPrimitiveDiff> {
    let topology = if base.topology != other.topology { Some(other.topology) } else { None };
    let positions = if base.positions != other.positions { Some(other.positions.clone()) } else { None };
    let normals = if base.normals != other.normals { Some(other.normals.clone()) } else { None };
    let uvs = if base.uvs != other.uvs { Some(other.uvs.clone()) } else { None };
    let colors = if base.colors != other.colors { Some(other.colors.clone()) } else { None };
    let indices = if base.indices != other.indices { Some(other.indices.clone()) } else { None };
    let material_id = if base.material_id != other.material_id { Some(other.material_id.clone()) } else { None };
    if topology.is_none() && positions.is_none() && normals.is_none() && uvs.is_none() && colors.is_none() && indices.is_none() && material_id.is_none() {
        None
    } else {
        Some(SemioPrimitiveDiff { topology, positions, normals, uvs, colors, indices, material_id })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_material(base: &SemioMaterial, other: &SemioMaterial) -> Option<SemioMaterialDiff> {
    let diff = SemioMaterialDiff {
        base_color: (base.base_color != other.base_color).then_some(other.base_color),
        metallic: (base.metallic != other.metallic).then_some(other.metallic),
        roughness: (base.roughness != other.roughness).then_some(other.roughness),
        base_color_texture: (base.base_color_texture != other.base_color_texture).then(|| other.base_color_texture.clone()),
        metallic_roughness_texture: (base.metallic_roughness_texture != other.metallic_roughness_texture).then(|| other.metallic_roughness_texture.clone()),
        normal_texture: (base.normal_texture != other.normal_texture).then(|| other.normal_texture.clone()),
        occlusion_texture: (base.occlusion_texture != other.occlusion_texture).then(|| other.occlusion_texture.clone()),
        emissive_texture: (base.emissive_texture != other.emissive_texture).then(|| other.emissive_texture.clone()),
    };
    (diff != SemioMaterialDiff::default()).then_some(diff)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_texture(base: &SemioTexture, other: &SemioTexture) -> Option<SemioTextureDiff> {
    let mime = if base.mime != other.mime { Some(other.mime.clone()) } else { None };
    let bytes = if base.bytes != other.bytes { Some(other.bytes.clone()) } else { None };
    if mime.is_none() && bytes.is_none() {
        None
    } else {
        Some(SemioTextureDiff { mime, bytes })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_mesh_diff(mut a: SemioMeshItemDiff, b: SemioMeshItemDiff) -> SemioMeshItemDiff {
    a.primitives = match (a.primitives.take(), b.primitives) {
        (None, x) => x,
        (x, None) => x,
        (Some(x), Some(y)) => Some(absorb_named(x, &y, |p: &NamedAdded<SemioPrimitive>| p.item.id.clone(), absorb_primitive_diff, apply_primitive_added)),
    };
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_primitive_diff(mut a: SemioPrimitiveDiff, b: SemioPrimitiveDiff) -> SemioPrimitiveDiff {
    if b.topology.is_some() {
        a.topology = b.topology;
    }
    if b.positions.is_some() {
        a.positions = b.positions;
    }
    if b.normals.is_some() {
        a.normals = b.normals;
    }
    if b.uvs.is_some() {
        a.uvs = b.uvs;
    }
    if b.colors.is_some() {
        a.colors = b.colors;
    }
    if b.indices.is_some() {
        a.indices = b.indices;
    }
    if b.material_id.is_some() {
        a.material_id = b.material_id;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_material_diff(mut a: SemioMaterialDiff, b: &SemioMaterialDiff) -> SemioMaterialDiff {
    if b.base_color.is_some() {
        a.base_color = b.base_color;
    }
    if b.metallic.is_some() {
        a.metallic = b.metallic;
    }
    if b.roughness.is_some() {
        a.roughness = b.roughness;
    }
    if b.base_color_texture.is_some() { a.base_color_texture = b.base_color_texture.clone(); }
    if b.metallic_roughness_texture.is_some() { a.metallic_roughness_texture = b.metallic_roughness_texture.clone(); }
    if b.normal_texture.is_some() { a.normal_texture = b.normal_texture.clone(); }
    if b.occlusion_texture.is_some() { a.occlusion_texture = b.occlusion_texture.clone(); }
    if b.emissive_texture.is_some() { a.emissive_texture = b.emissive_texture.clone(); }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_texture_diff(mut a: SemioTextureDiff, b: SemioTextureDiff) -> SemioTextureDiff {
    if b.mime.is_some() {
        a.mime = b.mime;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️EntityLookup
/// 🔎 Shared id-lookup helpers — moved here (from the now-deleted hand-rolled dispatch) so every
/// triad leaf's `diff`/`inverse` (17 of them) can reuse one copy instead of re-deriving the same
/// four one-line finders (`if code is repeated it must be close together`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn mesh_at<'a>(base: &'a SemioMeshSnapshot, mesh_id: &str) -> Option<&'a SemioMesh> {
    base.meshes.iter().find(|m| m.id == mesh_id)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn primitive_at<'a>(base: &'a SemioMeshSnapshot, mesh_id: &str, primitive_id: &str) -> Option<&'a SemioPrimitive> {
    mesh_at(base, mesh_id)?.primitives.iter().find(|p| p.id == primitive_id)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn material_at<'a>(base: &'a SemioMeshSnapshot, id: &str) -> Option<&'a SemioMaterial> {
    base.materials.iter().find(|m| m.id == id)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn texture_at<'a>(base: &'a SemioMeshSnapshot, id: &str) -> Option<&'a SemioTexture> {
    base.textures.iter().find(|t| t.id == id)
}
//#endregion 🔖️EntityLookup

//#region 🔖️MutationDiffHelpers
/// 🧩️ Per-mutation-variant sparse diff constructors — each triad leaf's `diff(payload, base)`
/// calls exactly one of these (never apply-and-capture). Mirrors docx's
/// `diff_insert_block`/`diff_remove_block`/... precedent.
/// ⚠️ Every constructor now takes `base` and checks presence/duplication FIRST, returning
/// `SemioMeshDiff::default()` on a missing target or a duplicate-id create — brep's law-testing
/// wave caught the identical bug class here (an unconditional `removed`/`modified` entry made
/// `is_empty()` lie for an absent target), so these are authored already-fixed rather than
/// reproducing that defect.
/// ➕️ `base` also supplies the real target position for a new entry's `NamedAdded<T>.index` (its
/// natural append position — the current collection length — same convention `value`'s own
/// `SetMapEntry`/`SetNode` diff constructors use).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_add_mesh(base: &SemioMeshSnapshot, mesh: SemioMesh) -> SemioMeshDiff {
    if mesh_at(base, &mesh.id).is_some() {
        return SemioMeshDiff::default();
    }
    SemioMeshDiff { meshes: Some(SemioMeshesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: base.meshes.len(), item: mesh }] }), materials: None, textures: None }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_mesh(base: &SemioMeshSnapshot, id: &str) -> SemioMeshDiff {
    if mesh_at(base, id).is_none() {
        return SemioMeshDiff::default();
    }
    SemioMeshDiff { meshes: Some(SemioMeshesDiff { removed: vec![id.to_string()], modified: Vec::new(), added: Vec::new() }), materials: None, textures: None }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_add_primitive(base: &SemioMeshSnapshot, mesh_id: &str, primitive: SemioPrimitive) -> SemioMeshDiff {
    let Some(mesh) = mesh_at(base, mesh_id) else { return SemioMeshDiff::default() };
    if mesh.primitives.iter().any(|p| p.id == primitive.id) {
        return SemioMeshDiff::default();
    }
    wrap_mesh_diff(mesh_id, SemioMeshItemDiff { primitives: Some(SemioPrimitivesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: mesh.primitives.len(), item: primitive }] }) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_primitive(base: &SemioMeshSnapshot, mesh_id: &str, primitive_id: &str) -> SemioMeshDiff {
    if primitive_at(base, mesh_id, primitive_id).is_none() {
        return SemioMeshDiff::default();
    }
    wrap_mesh_diff(mesh_id, SemioMeshItemDiff { primitives: Some(SemioPrimitivesDiff { removed: vec![primitive_id.to_string()], modified: Vec::new(), added: Vec::new() }) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_primitive_topology(base: &SemioMeshSnapshot, mesh_id: &str, primitive_id: &str, topology: SemioTopology) -> SemioMeshDiff {
    if primitive_at(base, mesh_id, primitive_id).is_none() {
        return SemioMeshDiff::default();
    }
    wrap_primitive_diff(mesh_id, primitive_id, SemioPrimitiveDiff { topology: Some(topology), ..Default::default() })
}
/// 📐 `replace-primitive-geometry` — SMO-approved rename of the old `set-primitive-geometry`
/// (a positions/normals/uvs/colors/indices blob is a structured sub-payload, so `set` was the
/// wrong verb; `replace` is a whole-value swap of it). SMO approved the reasoning and reserved the
/// edit; SMO wound down without doing it; DKM completes it here.
#[allow(clippy::too_many_arguments)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_replace_primitive_geometry(base: &SemioMeshSnapshot, mesh_id: &str, primitive_id: &str, positions: Vec<SemioPoint3>, normals: Vec<SemioPoint3>, uvs: Vec<SemioUv>, colors: Vec<SemioRgba>, indices: Vec<u32>) -> SemioMeshDiff {
    if primitive_at(base, mesh_id, primitive_id).is_none() {
        return SemioMeshDiff::default();
    }
    wrap_primitive_diff(mesh_id, primitive_id, SemioPrimitiveDiff { positions: Some(positions), normals: Some(normals), uvs: Some(uvs), colors: Some(colors), indices: Some(indices), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_primitive_material(base: &SemioMeshSnapshot, mesh_id: &str, primitive_id: &str, material_id: Option<String>) -> SemioMeshDiff {
    if primitive_at(base, mesh_id, primitive_id).is_none() {
        return SemioMeshDiff::default();
    }
    wrap_primitive_diff(mesh_id, primitive_id, SemioPrimitiveDiff { material_id: Some(material_id), ..Default::default() })
}
/// 📍 `move-vertex` — repositions ONE element of `positions` by BASE-state index, leaving every
/// other element (and `normals`/`uvs`/`colors`/`indices`) untouched. `SemioPrimitiveDiff.positions`
/// only expresses a whole-array replace, so this reads the primitive's CURRENT positions from
/// `base`, clones, and patches just `vertex_index` — a real diff built from `(payload, base)`,
/// never apply-then-capture.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_move_vertex(base: &SemioMeshSnapshot, mesh_id: &str, primitive_id: &str, vertex_index: usize, new_point: SemioPoint3) -> SemioMeshDiff {
    let Some(primitive) = primitive_at(base, mesh_id, primitive_id) else { return SemioMeshDiff::default() };
    if vertex_index >= primitive.positions.len() {
        return SemioMeshDiff::default();
    }
    let mut positions = primitive.positions.clone();
    positions[vertex_index] = new_point;
    wrap_primitive_diff(mesh_id, primitive_id, SemioPrimitiveDiff { positions: Some(positions), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_add_material(base: &SemioMeshSnapshot, material: SemioMaterial) -> SemioMeshDiff {
    if material_at(base, &material.id).is_some() {
        return SemioMeshDiff::default();
    }
    SemioMeshDiff { meshes: None, materials: Some(SemioMaterialsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: base.materials.len(), item: material }] }), textures: None }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_material(base: &SemioMeshSnapshot, id: &str) -> SemioMeshDiff {
    if material_at(base, id).is_none() {
        return SemioMeshDiff::default();
    }
    SemioMeshDiff { meshes: None, materials: Some(SemioMaterialsDiff { removed: vec![id.to_string()], modified: Vec::new(), added: Vec::new() }), textures: None }
}
/// 🌈 `change-material-base-color` — SMO's stroke-color precedent: a color is treated as ONE
/// cohesive value field (never edited channel-by-channel from outside), so `change`, not `replace`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_change_material_base_color(base: &SemioMeshSnapshot, id: &str, new_base_color: SemioRgba) -> SemioMeshDiff {
    if material_at(base, id).is_none() {
        return SemioMeshDiff::default();
    }
    SemioMeshDiff {
        meshes: None,
        materials: Some(SemioMaterialsDiff { removed: Vec::new(), modified: vec![NamedModified { key: id.to_string(), diff: SemioMaterialDiff { base_color: Some(new_base_color), ..Default::default() } }], added: Vec::new() }),
        textures: None,
    }
}
/// ⚙️ `change-material-metallic` — decomposed from the old bundled `SetMaterialPbr{metallic,
/// roughness}`: `SemioMaterial.metallic`/`.roughness` are two independent top-level scalar fields
/// (not grouped into one value type the way `base_color` is `SemioRgba`), and every real
/// metallic/roughness PBR editor sets them via two independent sliders — same decompose test SMO's
/// `StrokeStyle` ruling already applies (`change-stroke-width`/`change-stroke-color`/… kept
/// separate because the editor sets fields one at a time).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_change_material_metallic(base: &SemioMeshSnapshot, id: &str, new_metallic: f32) -> SemioMeshDiff {
    if material_at(base, id).is_none() {
        return SemioMeshDiff::default();
    }
    SemioMeshDiff {
        meshes: None,
        materials: Some(SemioMaterialsDiff { removed: Vec::new(), modified: vec![NamedModified { key: id.to_string(), diff: SemioMaterialDiff { metallic: Some(new_metallic), ..Default::default() } }], added: Vec::new() }),
        textures: None,
    }
}
/// 🧱 `change-material-roughness` — see [`diff_change_material_metallic`]'s doc comment; the same
/// decompose reasoning applies symmetrically to `roughness`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_change_material_roughness(base: &SemioMeshSnapshot, id: &str, new_roughness: f32) -> SemioMeshDiff {
    if material_at(base, id).is_none() {
        return SemioMeshDiff::default();
    }
    SemioMeshDiff {
        meshes: None,
        materials: Some(SemioMaterialsDiff { removed: Vec::new(), modified: vec![NamedModified { key: id.to_string(), diff: SemioMaterialDiff { roughness: Some(new_roughness), ..Default::default() } }], added: Vec::new() }),
        textures: None,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_add_texture(base: &SemioMeshSnapshot, texture: SemioTexture) -> SemioMeshDiff {
    if texture_at(base, &texture.id).is_some() {
        return SemioMeshDiff::default();
    }
    SemioMeshDiff { meshes: None, materials: None, textures: Some(SemioTexturesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![NamedAdded { index: base.textures.len(), item: texture }] }) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_texture(base: &SemioMeshSnapshot, id: &str) -> SemioMeshDiff {
    if texture_at(base, id).is_none() {
        return SemioMeshDiff::default();
    }
    SemioMeshDiff { meshes: None, materials: None, textures: Some(SemioTexturesDiff { removed: vec![id.to_string()], modified: Vec::new(), added: Vec::new() }) }
}
/// 🏷️ `change-texture-mime` — decomposed from the old bundled `SetTextureBytes{mime, bytes}`:
/// `SemioTexture.mime`/`.bytes` are two independent top-level fields (rule 2's "per remaining
/// scalar" for `mime`, "per large structured field" for `bytes`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_change_texture_mime(base: &SemioMeshSnapshot, id: &str, new_mime: String) -> SemioMeshDiff {
    if texture_at(base, id).is_none() {
        return SemioMeshDiff::default();
    }
    SemioMeshDiff {
        meshes: None,
        materials: None,
        textures: Some(SemioTexturesDiff { removed: Vec::new(), modified: vec![NamedModified { key: id.to_string(), diff: SemioTextureDiff { mime: Some(new_mime), ..Default::default() } }], added: Vec::new() }),
    }
}
/// 📀 `replace-texture-bytes` — see [`diff_change_texture_mime`]'s doc comment; raw image bytes
/// are the "large" swapped payload (matches `replace-primitive-geometry`'s exact rename rationale),
/// never edited byte-by-byte from outside, so `replace`, not `change`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_replace_texture_bytes(base: &SemioMeshSnapshot, id: &str, new_bytes: Vec<u8>) -> SemioMeshDiff {
    if texture_at(base, id).is_none() {
        return SemioMeshDiff::default();
    }
    SemioMeshDiff {
        meshes: None,
        materials: None,
        textures: Some(SemioTexturesDiff { removed: Vec::new(), modified: vec![NamedModified { key: id.to_string(), diff: SemioTextureDiff { bytes: Some(new_bytes), ..Default::default() } }], added: Vec::new() }),
    }
}
//#endregion 🔖️MutationDiffHelpers

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` for `SemioMeshDiff` — same grammar style as bcf/docx
/// (bracket-depth-aware split, hex for strings/bytes, `[0]`/`[1,x]` for `Option<T>`, single-letter
/// tag prefixes for data-carrying enums). `split_top_level`/`strip_brackets`/`enc_named_triple`/
/// `dec_named_triple` come from the shared `engine::triples` module (not re-derived); the small
/// hex/option/list primitives below are this subset's own copy (no shared "hand-roll primitives"
/// module exists yet — same as every other hand-rolled artifact in the repo).
//#region 🔖️Primitives
















//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs





















//#endregion 🔖️ValueCodecs

//#region 🔖️NamedAddedCodecs








//#endregion 🔖️NamedAddedCodecs

//#region 🔖️DiffValueCodecs











//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel











//#endregion 🔖️TopLevel

//#region 🔖️Demo
/// 🌱 Representative `SemioMeshDiff` cases (empty/no-op, a full meshes+materials+textures sweep
/// both directions incl. the nested primitives triple, a bare mesh/texture insert) — single
/// source of truth for `grammar_conformance_law`/`protocol_walk_law` in
/// `🎹️composer/🦀️.rs`. Local snapshot fixtures (not imported from `schema::mutations`,
/// which itself depends ON `schema::diff` — see this file's own module doc comment on why
/// `diff`/`snapshot`/`mutations` avoid reverse dependencies on each other).
#[cfg(all(test, feature = "conversion-mesh"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn demo_snapshot_a() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        meshes: vec![SemioMesh {
            id: "keep".into(),
            primitives: vec![
                SemioPrimitive { id: "toRemove".into(), topology: SemioTopology::Points, ..Default::default() },
                SemioPrimitive { id: "toModify".into(), topology: SemioTopology::Triangles, positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }], material_id: Some("mat-a".into()), ..Default::default() },
            ],
        }],
        materials: vec![SemioMaterial { id: "mat-a".into(), base_color: SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, metallic: 0.0, roughness: 1.0, ..Default::default() }],
        textures: vec![SemioTexture { id: "tex-a".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] }],
        ..Default::default()
    }
}
#[cfg(all(test, feature = "conversion-mesh"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn demo_snapshot_b() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        meshes: vec![SemioMesh {
            id: "keep".into(),
            primitives: vec![
                SemioPrimitive { id: "toModify".into(), topology: SemioTopology::Lines, positions: vec![SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 }], material_id: None, ..Default::default() },
                SemioPrimitive { id: "added".into(), topology: SemioTopology::Points, ..Default::default() },
            ],
        }],
        materials: vec![SemioMaterial { id: "mat-a".into(), base_color: SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }, metallic: 1.0, roughness: 0.0, ..Default::default() }],
        textures: vec![SemioTexture { id: "tex-a".into(), mime: "image/jpeg".into(), bytes: vec![4, 5] }],
        ..Default::default()
    }
}
#[cfg(all(test, feature = "conversion-mesh"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioMeshDiff> {
    let a = demo_snapshot_a();
    let b = demo_snapshot_b();
    vec![
        SemioMeshDiff::default(),
        <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &b),
        <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&b, &a),
        diff_add_mesh(&a, SemioMesh { id: "extra".into(), primitives: vec![] }),
        diff_add_texture(&a, SemioTexture { id: "extra-tex".into(), mime: "image/gif".into(), bytes: vec![7, 7] }),
    ]
}
//#endregion 🔖️Demo
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
