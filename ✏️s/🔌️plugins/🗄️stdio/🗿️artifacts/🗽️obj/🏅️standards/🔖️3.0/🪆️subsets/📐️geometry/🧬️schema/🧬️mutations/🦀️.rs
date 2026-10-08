//! 🧬️ ObjMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `ObjDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index/name-aware, reading the pre-state it needs from `base`.
//! `SetVertex`/`SetTexcoord`/`SetNormal`/`SetFace` each set a WHOLE item at an index (not a
//! single sub-field) — their `diff()` still constructs a sparse per-field patch by comparing
//! against `base`'s current value, never a full-item replace.
//!
//! ↩️ Three kinds need MORE than one step to undo, which is why `inverse()` returns `Vec<Self>`.
//! `RemoveFace` closes the face-index space that `g`/`o` membership is keyed on, and `InsertFace`
//! carries a face value with no membership at all, so the row alone comes back into no band and no
//! object — measured as `$.vertexCount` 8577 against the real `pattern-sphere` mesh's own 8576
//! (ticket `26/08/23/END-TO-END-TESTING-REFACTOR`, `inverse-remove-face`). `RemoveGroup`/
//! `RemoveObject` have the same shape one level up: `SetGroup`/`SetObject` on a name the document
//! no longer carries APPENDS, so a single-step undo restores the membership but not the position
//! the entry held. See the `InverseRestoration` region for the three repairs.
//!
//! 🧪️ F6: `#[derive(dsl::DslOps)]` — DERIVE path (ticket `f6-recon-report.md` §3's decision rule:
//! the Mutation side only cares whether the Snapshot type tree contains a data-carrying enum
//! ANYWHERE; `obj`'s whole model is
//! plain structs/`Vec`/`Option<T>`, zero enums, confirmed by `cargo check` — no compile error).
//! `OpText`/`OpBinary` are still handcrafted (P6: `DslOps` emits `DslVariants` only, never the two
//! op-codec traits themselves) using the exact boilerplate wrapper every derived-`DslOps` mutation
//! in the repo uses (`GifMutation`, `FlowMutationDsl`, `SpaceMutation`) — see `f6-recon-report.md`
//! §2. `#[derive(dsl::DslOps)]` is now kept ALONGSIDE `#[derive(dsl::Mutations)]`: every variant
//! below is a single-field newtype wrapping its own mutation leaf, and `dsl_variants_codegen`'s
//! "single-field tuple variant" branch (`✨️derive/🦀️.rs`) delegates `DslVariants`
//! straight through to that leaf's own `#[derive(dsl::DslRecord)]`-provided `DslField` impl — the
//! SAME `record_codegen` output the fields produced when they lived inline in the enum, so the
//! committed `crate::standards::v3_0::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO`/`crate::standards::v3_0::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO`
//! facets and this `OpText`/`OpBinary` pair are unaffected by the leaf split.

use crate::schema::diff::{
    diff_insert_face, diff_insert_normal, diff_insert_texcoord, diff_insert_vertex, diff_remove_face, diff_remove_group, diff_remove_normal, diff_remove_object, diff_remove_texcoord, diff_remove_vertex, diff_set_face, diff_set_group,
    diff_set_mtllib, diff_set_normal, diff_set_object, diff_set_smoothing_groups, diff_set_texcoord, diff_set_unknown_statements, diff_set_usemtl, diff_set_vertex, face_field_changes, normal_field_changes, texcoord_field_changes,
    vertex_field_changes, ObjDiff};
use crate::schema::snapshot::{ObjFace, ObjNormal, ObjSmoothingRange, ObjTexCoord, ObjUnknownStatement, ObjUsemtlRange, ObjVertex};
#[cfg(test)]
use crate::schema::snapshot::{ObjFaceVertex, ObjGroup, ObjObject};
use crate::ObjSnapshot;
use protocol::{Mutation, MutationDiff};


//#region 🔖️Mutations
//#region 🔖️Leaves
#[path = "🔷insert-face/🦀️.rs"]
pub mod insert_face;
#[path = "📐insert-normal/🦀️.rs"]
pub mod insert_normal;
#[path = "🧷insert-texcoord/🦀️.rs"]
pub mod insert_tex_coord;
#[path = "➕insert-vertex/🦀️.rs"]
pub mod insert_vertex;
#[path = "🗑️remove-face/🦀️.rs"]
pub mod remove_face;
#[path = "🪓remove-group/🦀️.rs"]
pub mod remove_group;
#[path = "🚫remove-normal/🦀️.rs"]
pub mod remove_normal;
#[path = "🗃️remove-object/🦀️.rs"]
pub mod remove_object;
#[path = "🚮remove-texcoord/🦀️.rs"]
pub mod remove_tex_coord;
#[path = "➖remove-vertex/🦀️.rs"]
pub mod remove_vertex;
#[path = "🔶set-face/🦀️.rs"]
pub mod set_face;
#[path = "🏷️set-group/🦀️.rs"]
pub mod set_group;
#[path = "🎨set-mtllib/🦀️.rs"]
pub mod set_mtllib;
#[path = "🧲set-normal/🦀️.rs"]
pub mod set_normal;
#[path = "📦set-object/🦀️.rs"]
pub mod set_object;
#[path = "🧵set-smoothing-groups/🦀️.rs"]
pub mod set_smoothing_groups;
#[path = "🧭set-texcoord/🦀️.rs"]
pub mod set_tex_coord;
#[path = "🕳️set-unknown-statements/🦀️.rs"]
pub mod set_unknown_statements;
#[path = "🖌️set-usemtl/🦀️.rs"]
pub mod set_usemtl;
#[path = "📍set-vertex/🦀️.rs"]
pub mod set_vertex;
//#endregion 🔖️Leaves

/// 📐️ Typed content mutation for `stdio.obj`. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none, and `no`
/// is not an approved semantic verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = ObjSnapshot, diff = ObjDiff, schema = "ObjMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum ObjMutation {

    /// ➕️ Inserts a whole `v` row at `index` (clamped to the end on apply).
    InsertVertex(insert_vertex::InsertVertex),
    /// ➖️ Removes the `v` row at `index`.
    RemoveVertex(remove_vertex::RemoveVertex),
    /// ✏️ Replaces the WHOLE `v` row at `index` (diff is still a sparse per-field patch).
    SetVertex(set_vertex::SetVertex),

    /// ➕️ Inserts a whole `vt` row at `index`.
    InsertTexcoord(insert_tex_coord::InsertTexcoord),
    /// ➖️ Removes the `vt` row at `index`.
    RemoveTexcoord(remove_tex_coord::RemoveTexcoord),
    /// ✏️ Replaces the WHOLE `vt` row at `index`.
    SetTexcoord(set_tex_coord::SetTexcoord),

    /// ➕️ Inserts a whole `vn` row at `index`.
    InsertNormal(insert_normal::InsertNormal),
    /// ➖️ Removes the `vn` row at `index`.
    RemoveNormal(remove_normal::RemoveNormal),
    /// ✏️ Replaces the WHOLE `vn` row at `index`.
    SetNormal(set_normal::SetNormal),

    /// ➕️ Inserts a whole `f` row at `index`.
    InsertFace(insert_face::InsertFace),
    /// ➖️ Removes the `f` row at `index`.
    RemoveFace(remove_face::RemoveFace),
    /// ✏️ Replaces the WHOLE `f` row at `index`.
    SetFace(set_face::SetFace),

    /// 🏷️ Creates or replaces a named `g` group's face-index membership.
    SetGroup(set_group::SetGroup),
    /// ➖️ Removes a named `g` group.
    RemoveGroup(remove_group::RemoveGroup),
    /// 🏷️ Creates or replaces a named `o` object's face-index membership.
    SetObject(set_object::SetObject),
    /// ➖️ Removes a named `o` object.
    RemoveObject(remove_object::RemoveObject),

    /// 🎨️ Sets or clears the `mtllib` reference.
    SetMtllib(set_mtllib::SetMtllib),
    /// 🎨️ Replaces the whole `usemtl` range list.
    SetUsemtl(set_usemtl::SetUsemtl),
    /// 🧵️ Replaces the whole `s` smoothing-range list.
    SetSmoothingGroups(set_smoothing_groups::SetSmoothingGroups),
    /// 🕳️ Replaces the whole retained unknown-statement list.
    SetUnknownStatements(set_unknown_statements::SetUnknownStatements),
}

/// 🏷️ Kebab-case spelling of every `ObjMutation` variant, in declaration order — the vocabulary the
/// `obj-3-0-any` mutation catalog (`../../🔣️oracle.json`) declares and the exhaustive
/// mutate/inverse test case measures itself against. `kinds_cover_every_variant` below is what keeps
/// this list honest against the enum it names, since the framework never parses Rust.
pub const KINDS: &[&str] = &[
    "insert-vertex",
    "remove-vertex",
    "set-vertex",
    "insert-texcoord",
    "remove-texcoord",
    "set-texcoord",
    "insert-normal",
    "remove-normal",
    "set-normal",
    "insert-face",
    "remove-face",
    "set-face",
    "set-group",
    "remove-group",
    "set-object",
    "remove-object",
    "set-mtllib",
    "set-usemtl",
    "set-smoothing-groups",
    "set-unknown-statements",
];
//#endregion 🔖️Mutations


//#endregion 🔖️Apply



//#region 🔖️InverseRestoration
/// ↩️ The undo of a positional `f` removal at `index`. `InsertFace` puts the row back BY VALUE and
/// carries no `g`/`o` membership, so on its own it restores geometry into no band and no object:
/// the real `pattern-sphere` fixture reads back as a fourth `tobj` model, `$.vertexCount` 8577
/// against the mesh's own 8576 (ticket `26/08/23/END-TO-END-TESTING-REFACTOR`, scenario
/// `inverse-remove-face`). Removing face `index` also closes the whole face-index space up by one,
/// so every membership list naming a face AT OR AFTER `index` — the removed face's own bands
/// included — is set back to the exact list `base` carries, after the row is back in place.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn restore_face_at(index: usize, face: &ObjFace, base: &ObjSnapshot) -> Vec<ObjMutation> {
    let disturbed = |faces: &[u64]| u64::try_from(index).is_ok_and(|index| faces.iter().any(|member| *member >= index));
    let mut undo: Vec<ObjMutation> = base.groups.iter().filter(|group| disturbed(&group.faces)).map(|group| ObjMutation::SetGroup(set_group::SetGroup { name: group.name.clone(), faces: group.faces.clone(), index: None })).collect();
    undo.extend(base.objects.iter().filter(|object| disturbed(&object.faces)).map(|object| ObjMutation::SetObject(set_object::SetObject { name: object.name.clone(), faces: object.faces.clone(), index: None })));
    undo.push(ObjMutation::InsertFace(insert_face::InsertFace { index, face: face.clone() }));
    undo
}

//#endregion 🔖️InverseRestoration

//#endregion 🔖️MutationTrait

//#region OpCodecs



//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `ObjSnapshot`/`ObjMutation` fixtures — the single source of truth
/// reused by `op_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️.rs`'s
/// `ops_grammar_conformance_law`/`protocol_walk_law` conformance tests (same convention P2-P1's
/// json/zip pilots established: `mutations::demo_mutation_cases()`/`diff::demo_diff_cases()`).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn base_snapshot() -> ObjSnapshot {
    ObjSnapshot {
        schema: "stdio.obj".into(),
        vertices: vec![ObjVertex { x: 0.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 1.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 0.0, y: 1.0, z: 0.0, w: None }],
        texcoords: vec![ObjTexCoord { u: 0.0, v: 0.0, w: None }],
        normals: vec![ObjNormal { x: 0.0, y: 0.0, z: 1.0 }],
        faces: vec![ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: None, normal: None }, ObjFaceVertex { vertex: 1, texcoord: None, normal: None }, ObjFaceVertex { vertex: 2, texcoord: None, normal: None }] }],
        groups: vec![ObjGroup { name: "Base".into(), faces: vec![0] }],
        objects: vec![ObjObject { name: "Obj".into(), faces: vec![0] }],
        mtllib: Some("m.mtl".into()),
        usemtl: vec![ObjUsemtlRange { face_index_from: 0, material: "Red".into() }],
        smoothing_groups: vec![ObjSmoothingRange { face_index_from: 0, group: Some(1) }],
        unknown_statements: vec![ObjUnknownStatement { line_index: 0, raw: "# c".into() }],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<ObjMutation> {
    vec![
        ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: 1, vertex: ObjVertex { x: 9.0, y: 9.0, z: 9.0, w: Some(1.0) } }),
        ObjMutation::RemoveVertex(remove_vertex::RemoveVertex { index: 0 }),
        ObjMutation::SetVertex(set_vertex::SetVertex { index: 0, vertex: ObjVertex { x: 2.0, y: 2.0, z: 2.0, w: None } }),
        ObjMutation::InsertTexcoord(insert_tex_coord::InsertTexcoord { index: 0, texcoord: ObjTexCoord { u: 9.0, v: 9.0, w: Some(1.0) } }),
        ObjMutation::RemoveTexcoord(remove_tex_coord::RemoveTexcoord { index: 0 }),
        ObjMutation::SetTexcoord(set_tex_coord::SetTexcoord { index: 0, texcoord: ObjTexCoord { u: 5.0, v: 5.0, w: None } }),
        ObjMutation::InsertNormal(insert_normal::InsertNormal { index: 0, normal: ObjNormal { x: 1.0, y: 0.0, z: 0.0 } }),
        ObjMutation::RemoveNormal(remove_normal::RemoveNormal { index: 0 }),
        ObjMutation::SetNormal(set_normal::SetNormal { index: 0, normal: ObjNormal { x: -1.0, y: 0.0, z: 0.0 } }),
        ObjMutation::InsertFace(insert_face::InsertFace {
            index: 0,
            face: ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: None, normal: None }, ObjFaceVertex { vertex: 1, texcoord: None, normal: None }, ObjFaceVertex { vertex: 2, texcoord: None, normal: None }] },
        }),
        ObjMutation::RemoveFace(remove_face::RemoveFace { index: 0 }),
        ObjMutation::SetFace(set_face::SetFace {
            index: 0,
            face: ObjFace { vertices: vec![ObjFaceVertex { vertex: 2, texcoord: None, normal: None }, ObjFaceVertex { vertex: 1, texcoord: None, normal: None }, ObjFaceVertex { vertex: 0, texcoord: None, normal: None }] },
        }),
        ObjMutation::SetGroup(set_group::SetGroup { name: "Base".into(), faces: vec![0, 0], index: None }),
        ObjMutation::SetGroup(set_group::SetGroup { name: "New".into(), faces: vec![0], index: None }),
        ObjMutation::RemoveGroup(remove_group::RemoveGroup { name: "Base".into() }),
        ObjMutation::SetObject(set_object::SetObject { name: "Obj".into(), faces: vec![0], index: None }),
        ObjMutation::SetObject(set_object::SetObject { name: "NewObj".into(), faces: vec![0], index: None }),
        ObjMutation::RemoveObject(remove_object::RemoveObject { name: "Obj".into() }),
        ObjMutation::SetMtllib(set_mtllib::SetMtllib { mtllib: None }),
        ObjMutation::SetMtllib(set_mtllib::SetMtllib { mtllib: Some("new.mtl".into()) }),
        ObjMutation::SetUsemtl(set_usemtl::SetUsemtl { usemtl: vec![ObjUsemtlRange { face_index_from: 0, material: "Blue".into() }] }),
        ObjMutation::SetSmoothingGroups(set_smoothing_groups::SetSmoothingGroups { smoothing_groups: vec![] }),
        ObjMutation::SetUnknownStatements(set_unknown_statements::SetUnknownStatements { unknown_statements: vec![] }),
    ]
}

/// 🧬️ Canonical "differs in every mutable field" snapshot A — every index-keyed collection
/// has 2 items (a stable prefix item + one that will be modified); every name-keyed
/// collection has 2 named entries (one that will be removed, one that will be modified).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_a() -> ObjSnapshot {
    ObjSnapshot {
        schema: "stdio.obj".into(),
        vertices: vec![ObjVertex { x: 0.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 1.0, y: 1.0, z: 1.0, w: None }],
        texcoords: vec![ObjTexCoord { u: 0.0, v: 0.0, w: None }, ObjTexCoord { u: 1.0, v: 1.0, w: Some(5.0) }],
        normals: vec![ObjNormal { x: 0.0, y: 0.0, z: 1.0 }, ObjNormal { x: 1.0, y: 1.0, z: 1.0 }],
        faces: vec![ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: None, normal: None }] }, ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: None, normal: None }] }],
        groups: vec![ObjGroup { name: "G1".into(), faces: vec![0] }, ObjGroup { name: "G2".into(), faces: vec![1] }],
        objects: vec![ObjObject { name: "O1".into(), faces: vec![0] }, ObjObject { name: "O2".into(), faces: vec![1] }],
        mtllib: Some("a.mtl".into()),
        usemtl: vec![ObjUsemtlRange { face_index_from: 0, material: "Red".into() }],
        smoothing_groups: vec![ObjSmoothingRange { face_index_from: 0, group: Some(1) }],
        unknown_statements: vec![ObjUnknownStatement { line_index: 0, raw: "# a".into() }],
    }
}
/// 🧬️ Sweep B: every index-keyed collection keeps its stable-prefix item at index 0
/// UNCHANGED, has its index-1 item MODIFIED in every field (including a tri-state
/// `Some(None)` on `texcoords[1].w`), and gains a brand-new item at index 2 (ADDED — proven
/// via `between(a,b)`, since `b` is the longer side). `between(b,a)` then proves REMOVED
/// (the same extra item, from `b`'s perspective). Name-keyed `groups`/`objects` show
/// removed+modified+added simultaneously from ONE `between(a,b)` call (name-keyed
/// collections aren't subject to the flat/positional "only one tail" limitation).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_b() -> ObjSnapshot {
    ObjSnapshot {
        schema: "stdio.obj".into(),
        vertices: vec![ObjVertex { x: 0.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 9.0, y: 9.0, z: 9.0, w: Some(0.5) }, ObjVertex { x: 5.0, y: 5.0, z: 5.0, w: Some(1.0) }],
        texcoords: vec![ObjTexCoord { u: 0.0, v: 0.0, w: None }, ObjTexCoord { u: 2.0, v: 2.0, w: None }, ObjTexCoord { u: 5.0, v: 5.0, w: None }],
        normals: vec![ObjNormal { x: 0.0, y: 0.0, z: 1.0 }, ObjNormal { x: -1.0, y: -1.0, z: -1.0 }, ObjNormal { x: 0.0, y: 1.0, z: 0.0 }],
        faces: vec![
            ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: None, normal: None }] },
            ObjFace { vertices: vec![ObjFaceVertex { vertex: 1, texcoord: Some(0), normal: Some(0) }] },
            ObjFace { vertices: vec![ObjFaceVertex { vertex: 2, texcoord: None, normal: None }] },
        ],
        groups: vec![ObjGroup { name: "G2".into(), faces: vec![1, 2] }, ObjGroup { name: "G3".into(), faces: vec![3] }],
        objects: vec![ObjObject { name: "O2".into(), faces: vec![1, 2] }, ObjObject { name: "O3".into(), faces: vec![3] }],
        mtllib: None,
        usemtl: vec![ObjUsemtlRange { face_index_from: 0, material: "Blue".into() }, ObjUsemtlRange { face_index_from: 2, material: "Green".into() }],
        smoothing_groups: vec![ObjSmoothingRange { face_index_from: 0, group: None }],
        unknown_statements: vec![ObjUnknownStatement { line_index: 5, raw: "# b".into() }, ObjUnknownStatement { line_index: 6, raw: "weird".into() }],
    }
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureCases
//#endregion 🧪️FixtureCases

#[cfg(test)]
use protocol::{OpBinary,OpText};
