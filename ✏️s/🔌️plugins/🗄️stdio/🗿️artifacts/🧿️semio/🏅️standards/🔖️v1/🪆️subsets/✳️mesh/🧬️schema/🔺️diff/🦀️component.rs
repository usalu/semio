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

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{dec_named_triple, enc_named_triple, split_top_level, strip_brackets, NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology};
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
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
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
    fn apply(&self, base: &SemioMeshSnapshot) -> protocol::MutationApplyResult<SemioMeshSnapshot> {
        let mut next = base.clone();
        if let Some(md) = &self.meshes {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.meshes, md, |item| item.id.clone(), |added| added.item.id.clone(), ["meshes"])?;
            apply_named(&mut next.meshes, md, |m| m.id.clone(), apply_mesh);
        }
        if let Some(md) = &self.materials {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.materials, md, |item| item.id.clone(), |added| added.item.id.clone(), ["materials"])?;
            apply_named(&mut next.materials, md, |m| m.id.clone(), apply_material);
        }
        if let Some(td) = &self.textures {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.textures, td, |item| item.id.clone(), |added| added.item.id.clone(), ["textures"])?;
            apply_named(&mut next.textures, td, |t| t.id.clone(), apply_texture);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.meshes = match (self.meshes.take(), other.meshes) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |m: &NamedAdded<SemioMesh>| m.item.id.clone(), absorb_mesh_diff, apply_mesh_added)),
        };
        self.materials = match (self.materials.take(), other.materials) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |m: &NamedAdded<SemioMaterial>| m.item.id.clone(), absorb_material_diff, apply_material_added)),
        };
        self.textures = match (self.textures.take(), other.textures) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |t: &NamedAdded<SemioTexture>| t.item.id.clone(), absorb_texture_diff, apply_texture_added)),
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
    let base_color = if base.base_color != other.base_color { Some(other.base_color) } else { None };
    let metallic = if base.metallic != other.metallic { Some(other.metallic) } else { None };
    let roughness = if base.roughness != other.roughness { Some(other.roughness) } else { None };
    if base_color.is_none() && metallic.is_none() && roughness.is_none() {
        None
    } else {
        Some(SemioMaterialDiff { base_color, metallic, roughness })
    }
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
        (Some(x), Some(y)) => Some(absorb_named(x, y, |p: &NamedAdded<SemioPrimitive>| p.item.id.clone(), absorb_primitive_diff, apply_primitive_added)),
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
fn absorb_material_diff(mut a: SemioMaterialDiff, b: SemioMaterialDiff) -> SemioMaterialDiff {
    if b.base_color.is_some() {
        a.base_color = b.base_color;
    }
    if b.metallic.is_some() {
        a.metallic = b.metallic;
    }
    if b.roughness.is_some() {
        a.roughness = b.roughness;
    }
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bytes(b: &[u8]) -> String {
    hex_encode(b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    hex_decode(s)
}
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`) backing the real `DiffCodec::encode_diff`/`decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec();
    String::from_utf8(bytes).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f32(s: &str) -> Result<f32, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_uv(v: &SemioUv) -> String {
    format!("[{},{}]", v.u, v.v)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_uv(s: &str) -> Result<SemioUv, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [u, v] = parts.as_slice() else { return Err(format!("uv: expected 2 fields, got {}", parts.len())) };
    Ok(SemioUv { u: parse_f64(u)?, v: parse_f64(v)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rgba(c: &SemioRgba) -> String {
    format!("[{},{},{},{}]", c.r, c.g, c.b, c.a)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rgba(s: &str) -> Result<SemioRgba, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b, a] = parts.as_slice() else { return Err(format!("rgba: expected 4 fields, got {}", parts.len())) };
    Ok(SemioRgba { r: parse_f32(r)?, g: parse_f32(g)?, b: parse_f32(b)?, a: parse_f32(a)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topology(t: &SemioTopology) -> String {
    match t {
        SemioTopology::Points => "P".to_string(),
        SemioTopology::Lines => "L".to_string(),
        SemioTopology::LineStrip => "S".to_string(),
        SemioTopology::Triangles => "T".to_string(),
        SemioTopology::TriangleStrip => "X".to_string(),
        SemioTopology::TriangleFan => "F".to_string(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topology(s: &str) -> Result<SemioTopology, String> {
    match s {
        "P" => Ok(SemioTopology::Points),
        "L" => Ok(SemioTopology::Lines),
        "S" => Ok(SemioTopology::LineStrip),
        "T" => Ok(SemioTopology::Triangles),
        "X" => Ok(SemioTopology::TriangleStrip),
        "F" => Ok(SemioTopology::TriangleFan),
        other => Err(format!("topology: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_primitive(p: &SemioPrimitive) -> String {
    format!(
        "[{},{},{},{},{},{},{},{}]",
        enc_str(&p.id),
        enc_topology(&p.topology),
        enc_list(&p.positions, enc_point3),
        enc_list(&p.normals, enc_point3),
        enc_list(&p.uvs, enc_uv),
        enc_list(&p.colors, enc_rgba),
        enc_list(&p.indices, |v: &u32| v.to_string()),
        encode_option(&p.material_id, |v: &String| enc_str(v)),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_primitive(s: &str) -> Result<SemioPrimitive, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, topology, positions, normals, uvs, colors, indices, material_id] = parts.as_slice() else {
        return Err(format!("primitive: expected 8 fields, got {}", parts.len()));
    };
    Ok(SemioPrimitive {
        id: dec_str(id)?,
        topology: dec_topology(topology)?,
        positions: dec_list(positions, dec_point3)?,
        normals: dec_list(normals, dec_point3)?,
        uvs: dec_list(uvs, dec_uv)?,
        colors: dec_list(colors, dec_rgba)?,
        indices: dec_list(indices, parse_u32)?,
        material_id: decode_option(material_id, dec_str)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mesh(m: &SemioMesh) -> String {
    format!("[{},{}]", enc_str(&m.id), enc_list(&m.primitives, enc_primitive))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mesh(s: &str) -> Result<SemioMesh, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, primitives] = parts.as_slice() else { return Err(format!("mesh: expected 2 fields, got {}", parts.len())) };
    Ok(SemioMesh { id: dec_str(id)?, primitives: dec_list(primitives, dec_primitive)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_material(m: &SemioMaterial) -> String {
    format!("[{},{},{},{}]", enc_str(&m.id), enc_rgba(&m.base_color), m.metallic, m.roughness)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_material(s: &str) -> Result<SemioMaterial, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, base_color, metallic, roughness] = parts.as_slice() else { return Err(format!("material: expected 4 fields, got {}", parts.len())) };
    Ok(SemioMaterial { id: dec_str(id)?, base_color: dec_rgba(base_color)?, metallic: parse_f32(metallic)?, roughness: parse_f32(roughness)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texture(t: &SemioTexture) -> String {
    format!("[{},{},{}]", enc_str(&t.id), enc_str(&t.mime), enc_bytes(&t.bytes))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texture(s: &str) -> Result<SemioTexture, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, mime, bytes] = parts.as_slice() else { return Err(format!("texture: expected 3 fields, got {}", parts.len())) };
    Ok(SemioTexture { id: dec_str(id)?, mime: dec_str(mime)?, bytes: dec_bytes(bytes)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️NamedAddedCodecs
/// 🧷️ `NamedAdded<T>`-wrapping encoders/decoders — `index:item` prefix, same convention
/// `engine::triples::enc_indexed_triple`'s own `IndexAdded<T>` handling uses — used ONLY for a
/// diff's own `added` list (see [`NamedAdded`]'s doc comment); the plain (unwrapped) `enc_*`/
/// `dec_*` above remain the snapshot-level codec for the real entity.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_mesh(a: &NamedAdded<SemioMesh>) -> String {
    format!("{}:{}", a.index, enc_mesh(&a.item))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_mesh(s: &str) -> Result<NamedAdded<SemioMesh>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added mesh: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_mesh(rest)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_primitive(a: &NamedAdded<SemioPrimitive>) -> String {
    format!("{}:{}", a.index, enc_primitive(&a.item))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_primitive(s: &str) -> Result<NamedAdded<SemioPrimitive>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added primitive: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_primitive(rest)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_material(a: &NamedAdded<SemioMaterial>) -> String {
    format!("{}:{}", a.index, enc_material(&a.item))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_material(s: &str) -> Result<NamedAdded<SemioMaterial>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added material: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_material(rest)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_texture(a: &NamedAdded<SemioTexture>) -> String {
    format!("{}:{}", a.index, enc_texture(&a.item))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_texture(s: &str) -> Result<NamedAdded<SemioTexture>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added texture: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_texture(rest)? })
}
//#endregion 🔖️NamedAddedCodecs

//#region 🔖️DiffValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_primitive_diff(d: &SemioPrimitiveDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        encode_option(&d.topology, |v: &SemioTopology| enc_topology(v)),
        encode_option(&d.positions, |v: &Vec<SemioPoint3>| enc_list(v, enc_point3)),
        encode_option(&d.normals, |v: &Vec<SemioPoint3>| enc_list(v, enc_point3)),
        encode_option(&d.uvs, |v: &Vec<SemioUv>| enc_list(v, enc_uv)),
        encode_option(&d.colors, |v: &Vec<SemioRgba>| enc_list(v, enc_rgba)),
        encode_option(&d.indices, |v: &Vec<u32>| enc_list(v, |x: &u32| x.to_string())),
        encode_option(&d.material_id, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_primitive_diff(s: &str) -> Result<SemioPrimitiveDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [topology, positions, normals, uvs, colors, indices, material_id] = parts.as_slice() else {
        return Err(format!("primitive diff: expected 7 fields, got {}", parts.len()));
    };
    Ok(SemioPrimitiveDiff {
        topology: decode_option(topology, dec_topology)?,
        positions: decode_option(positions, |s| dec_list(s, dec_point3))?,
        normals: decode_option(normals, |s| dec_list(s, dec_point3))?,
        uvs: decode_option(uvs, |s| dec_list(s, dec_uv))?,
        colors: decode_option(colors, |s| dec_list(s, dec_rgba))?,
        indices: decode_option(indices, |s| dec_list(s, parse_u32))?,
        material_id: decode_option(material_id, |s| decode_option(s, dec_str))?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mesh_item_diff(d: &SemioMeshItemDiff) -> String {
    format!("[{}]", encode_option(&d.primitives, |v: &SemioPrimitivesDiff| enc_named_triple(v, |k: &String| enc_str(k), enc_primitive_diff, enc_named_added_primitive)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mesh_item_diff(s: &str) -> Result<SemioMeshItemDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(SemioMeshItemDiff { primitives: decode_option(inner, |s| dec_named_triple(s, dec_str, dec_primitive_diff, dec_named_added_primitive))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_material_diff(d: &SemioMaterialDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.base_color, |v: &SemioRgba| enc_rgba(v)), encode_option(&d.metallic, |v: &f32| v.to_string()), encode_option(&d.roughness, |v: &f32| v.to_string()),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_material_diff(s: &str) -> Result<SemioMaterialDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [base_color, metallic, roughness] = parts.as_slice() else { return Err(format!("material diff: expected 3 fields, got {}", parts.len())) };
    Ok(SemioMaterialDiff { base_color: decode_option(base_color, dec_rgba)?, metallic: decode_option(metallic, parse_f32)?, roughness: decode_option(roughness, parse_f32)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texture_diff(d: &SemioTextureDiff) -> String {
    format!("[{},{}]", encode_option(&d.mime, |v: &String| enc_str(v)), encode_option(&d.bytes, |v: &Vec<u8>| enc_bytes(v)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texture_diff(s: &str) -> Result<SemioTextureDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [mime, bytes] = parts.as_slice() else { return Err(format!("texture diff: expected 2 fields, got {}", parts.len())) };
    Ok(SemioTextureDiff { mime: decode_option(mime, dec_str)?, bytes: decode_option(bytes, dec_bytes)? })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_meshes_diff(d: &SemioMeshesDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_mesh_item_diff, enc_named_added_mesh)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_meshes_diff(s: &str) -> Result<SemioMeshesDiff, String> {
    dec_named_triple(s, dec_str, dec_mesh_item_diff, dec_named_added_mesh)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_materials_diff(d: &SemioMaterialsDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_material_diff, enc_named_added_material)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_materials_diff(s: &str) -> Result<SemioMaterialsDiff, String> {
    dec_named_triple(s, dec_str, dec_material_diff, dec_named_added_material)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_textures_diff(d: &SemioTexturesDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_texture_diff, enc_named_added_texture)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_textures_diff(s: &str) -> Result<SemioTexturesDiff, String> {
    dec_named_triple(s, dec_str, dec_texture_diff, dec_named_added_texture)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_mesh_diff(d: &SemioMeshDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.meshes {
        tokens.push(format!("meshes={}", enc_meshes_diff(v)));
    }
    if let Some(v) = &d.materials {
        tokens.push(format!("materials={}", enc_materials_diff(v)));
    }
    if let Some(v) = &d.textures {
        tokens.push(format!("textures={}", enc_textures_diff(v)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_mesh_diff(line: &str) -> Result<SemioMeshDiff, String> {
    let mut d = SemioMeshDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("meshes=") {
            d.meshes = Some(dec_meshes_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("materials=") {
            d.materials = Some(dec_materials_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("textures=") {
            d.textures = Some(dec_textures_diff(rest)?);
        } else {
            return Err(format!("semio mesh diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for SemioMeshDiff {
    fn print_diff(&self) -> String {
        print_mesh_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_mesh_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
    /// shortcut (per this wave's brief item 5). `format u8` + `presence u8` (bit0 = `meshes`
    /// present, bit1 = `materials` present, bit2 = `textures` present) are two REAL fixed fields;
    /// each present collection then follows as its own varint-length-prefixed opaque blob (the
    /// same `enc_meshes_diff`/`enc_materials_diff`/`enc_textures_diff` bracket/hex text
    /// `print_diff` already produces) — independently-delimited segments rather than one bare
    /// trailing `bytes` because there can be 0-3 of them (chaining a `Cond` per-segment hits the
    /// `protocol-cond-cannot-chain` gap: a second `if`-guard on a field that was itself only
    /// conditionally decoded hard-errors `eval_cond` — see `✳️flow`'s pilot report).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut presence = 0u8;
        if self.meshes.is_some() {
            presence |= 0b001;
        }
        if self.materials.is_some() {
            presence |= 0b010;
        }
        if self.textures.is_some() {
            presence |= 0b100;
        }
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(v) = &self.meshes {
            write_str_lp(&mut out, &enc_meshes_diff(v));
        }
        if let Some(v) = &self.materials {
            write_str_lp(&mut out, &enc_materials_diff(v));
        }
        if let Some(v) = &self.textures {
            write_str_lp(&mut out, &enc_textures_diff(v));
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
        }
        if bytes[0] != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
        }
        let presence = bytes[1];
        let mut reader = store::ByteReader::new(&bytes[2..]);
        let meshes = if presence & 0b001 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff meshes blob", offset: 2, detail: e })?;
            Some(dec_meshes_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff meshes text", offset: 2, detail: e })?)
        } else {
            None
        };
        let materials = if presence & 0b010 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff materials blob", offset: 2, detail: e })?;
            Some(dec_materials_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff materials text", offset: 2, detail: e })?)
        } else {
            None
        };
        let textures = if presence & 0b100 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff textures blob", offset: 2, detail: e })?;
            Some(dec_textures_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff textures text", offset: 2, detail: e })?)
        } else {
            None
        };
        Ok(SemioMeshDiff { meshes, materials, textures })
    }
}
//#endregion 🔖️TopLevel

//#region 🔖️Demo
/// 🌱 Representative `SemioMeshDiff` cases (empty/no-op, a full meshes+materials+textures sweep
/// both directions incl. the nested primitives triple, a bare mesh/texture insert) — single
/// source of truth for `grammar_conformance_law`/`protocol_walk_law` in
/// `🎹️composer/🦀️component.rs`. Local snapshot fixtures (not imported from `schema::mutations`,
/// which itself depends ON `schema::diff` — see this file's own module doc comment on why
/// `diff`/`snapshot`/`mutations` avoid reverse dependencies on each other).
#[cfg(test)]
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
        materials: vec![SemioMaterial { id: "mat-a".into(), base_color: SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, metallic: 0.0, roughness: 1.0 }],
        textures: vec![SemioTexture { id: "tex-a".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] }],
        ..Default::default()
    }
}
#[cfg(test)]
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
        materials: vec![SemioMaterial { id: "mat-a".into(), base_color: SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }, metallic: 1.0, roughness: 0.0 }],
        textures: vec![SemioTexture { id: "tex-a".into(), mime: "image/jpeg".into(), bytes: vec![4, 5] }],
        ..Default::default()
    }
}
#[cfg(test)]
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
    use protocol::DiffCodec;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot_a() -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            meshes: vec![SemioMesh {
                id: "m1".into(),
                primitives: vec![SemioPrimitive {
                    id: "p1".into(),
                    topology: SemioTopology::Triangles,
                    positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }],
                    normals: vec![],
                    uvs: vec![],
                    colors: vec![],
                    indices: vec![0],
                    material_id: Some("mat1".into()),
                }],
            }],
            materials: vec![SemioMaterial { id: "mat1".into(), base_color: SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, metallic: 0.0, roughness: 1.0 }],
            textures: vec![SemioTexture { id: "tex1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] }],
            ..Default::default()
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot_b() -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            meshes: vec![SemioMesh {
                id: "m1".into(),
                primitives: vec![SemioPrimitive {
                    id: "p1".into(),
                    topology: SemioTopology::Lines,
                    positions: vec![SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }],
                    normals: vec![SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }],
                    uvs: vec![SemioUv { u: 0.5, v: 0.5 }],
                    colors: vec![SemioRgba { r: 0.5, g: 0.5, b: 0.5, a: 1.0 }],
                    indices: vec![0, 1],
                    material_id: None,
                }],
            }],
            materials: vec![SemioMaterial { id: "mat1".into(), base_color: SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }, metallic: 1.0, roughness: 0.0 }],
            textures: vec![SemioTexture { id: "tex1".into(), mime: "image/jpeg".into(), bytes: vec![4, 5] }],
            ..Default::default()
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn between_apply_and_inverse_round_trip() {
        let a = snapshot_a();
        let b = snapshot_b();
        let d = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &b);
        assert_eq!(d.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&d.apply(&a).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture"), a);
        assert!(<SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_composes_two_sequential_diffs() {
        let a = snapshot_a();
        let mid = snapshot_b();
        let mut after = mid.clone();
        after.materials[0].metallic = 0.42;
        let mut d1 = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &mid);
        let d2 = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&mid, &after);
        let applied_before_absorb = d1.apply(&a).expect("apply must succeed for a well-formed fixture");
        d1.absorb(d2.clone());
        assert_eq!(d1.apply(&a).expect("apply must succeed for a well-formed fixture"), d2.apply(&applied_before_absorb).expect("apply must succeed for a well-formed fixture"));
        assert_eq!(d1.apply(&a).expect("apply must succeed for a well-formed fixture"), after);
    }

    /// 🧪️ diff_codec_text_binary_roundtrip_law: hand-rolled `DiffCodec` round-trips through both
    /// `print_diff`/`parse_diff` and `encode_diff`/`decode_diff`, over a real `between()` result
    /// exercising the nested mesh -> primitive triple plus materials/textures.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = snapshot_a();
        let b = snapshot_b();
        let cases =
            vec![SemioMeshDiff::default(), <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &b), <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&b, &a), <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioMeshDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioMeshDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }

        // Confirm nested + tri-state coverage genuinely got exercised above.
        let diff_ab = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &b);
        let meshes = diff_ab.meshes.as_ref().expect("meshes diff present");
        let mesh_mod = meshes.modified.iter().find(|m| m.key == "m1").expect("m1 modified");
        let prims = mesh_mod.diff.primitives.as_ref().expect("primitives diff present");
        let prim_mod = prims.modified.iter().find(|p| p.key == "p1").expect("p1 modified");
        assert_eq!(prim_mod.diff.material_id, Some(None), "material_id tri-state Some(None) not exercised");
        let diff_ba = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&b, &a);
        let prim_mod_ba = diff_ba.meshes.as_ref().unwrap().modified[0].diff.primitives.as_ref().unwrap().modified.iter().find(|p| p.key == "p1").unwrap();
        assert_eq!(prim_mod_ba.diff.material_id, Some(Some("mat1".to_string())), "material_id tri-state Some(Some(_)) not exercised");
    }
}
//#endregion 🔖️Tests
