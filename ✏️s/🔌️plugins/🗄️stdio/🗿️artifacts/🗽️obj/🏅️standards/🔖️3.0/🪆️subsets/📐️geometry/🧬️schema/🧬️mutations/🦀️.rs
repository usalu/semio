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
//! ANYWHERE, since `SetSnapshot` always carries the whole `ObjSnapshot`; `obj`'s whole model is
//! plain structs/`Vec`/`Option<T>`, zero enums, confirmed by `cargo check` — no compile error).
//! `OpText`/`OpBinary` are still handcrafted (P6: `DslOps` emits `DslVariants` only, never the two
//! op-codec traits themselves) using the exact boilerplate wrapper every derived-`DslOps` mutation
//! in the repo uses (`GifMutation`, `FlowMutationDsl`, `SpaceMutation`) — see `f6-recon-report.md`
//! §2. `#[derive(dsl::DslOps)]` is now kept ALONGSIDE `#[derive(dsl::Mutations)]`: every variant
//! below is a single-field newtype wrapping its own mutation leaf, and `dsl_variants_codegen`'s
//! "single-field tuple variant" branch (`✨️derive/🦀️.rs`) delegates `DslVariants`
//! straight through to that leaf's own `#[derive(dsl::DslRecord)]`-provided `DslField` impl — the
//! SAME `record_codegen` output the fields produced when they lived inline in the enum, so the
//! committed `mutations::text::COMPONENT_GRAMMAR_SEMIO`/`mutations::binary::COMPONENT_PROTOCOL_SEMIO`
//! facets and this `OpText`/`OpBinary` pair are unaffected by the leaf split.

use crate::artifacts::obj::schema::diff::{
    diff_insert_face, diff_insert_normal, diff_insert_texcoord, diff_insert_vertex, diff_remove_face, diff_remove_group, diff_remove_normal, diff_remove_object, diff_remove_texcoord, diff_remove_vertex, diff_set_face, diff_set_group,
    diff_set_mtllib, diff_set_normal, diff_set_object, diff_set_smoothing_groups, diff_set_snapshot, diff_set_texcoord, diff_set_unknown_statements, diff_set_usemtl, diff_set_vertex, face_diff_between, normal_diff_between, texcoord_diff_between,
    vertex_diff_between, ObjDiff,
};
use crate::artifacts::obj::schema::snapshot::{ObjFace, ObjNormal, ObjSmoothingRange, ObjTexCoord, ObjUnknownStatement, ObjUsemtlRange, ObjVertex};
#[cfg(test)]
use crate::artifacts::obj::schema::snapshot::{ObjFaceVertex, ObjGroup, ObjObject};
use crate::artifacts::obj::ObjSnapshot;
use protocol::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};

//#region 🔖️Mutations
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "➕insert-vertex/🦀️.rs"]
pub mod insert_vertex;
#[path = "➖remove-vertex/🦀️.rs"]
pub mod remove_vertex;
#[path = "📍set-vertex/🦀️.rs"]
pub mod set_vertex;
#[path = "🧷insert-texcoord/🦀️.rs"]
pub mod insert_tex_coord;
#[path = "🚮remove-texcoord/🦀️.rs"]
pub mod remove_tex_coord;
#[path = "🧭set-texcoord/🦀️.rs"]
pub mod set_tex_coord;
#[path = "📐insert-normal/🦀️.rs"]
pub mod insert_normal;
#[path = "🚫remove-normal/🦀️.rs"]
pub mod remove_normal;
#[path = "🧲set-normal/🦀️.rs"]
pub mod set_normal;
#[path = "🔷insert-face/🦀️.rs"]
pub mod insert_face;
#[path = "🗑️remove-face/🦀️.rs"]
pub mod remove_face;
#[path = "🔶set-face/🦀️.rs"]
pub mod set_face;
#[path = "🏷️set-group/🦀️.rs"]
pub mod set_group;
#[path = "🪓remove-group/🦀️.rs"]
pub mod remove_group;
#[path = "📦set-object/🦀️.rs"]
pub mod set_object;
#[path = "🗃️remove-object/🦀️.rs"]
pub mod remove_object;
#[path = "🎨set-mtllib/🦀️.rs"]
pub mod set_mtllib;
#[path = "🖌️set-usemtl/🦀️.rs"]
pub mod set_usemtl;
#[path = "🧵set-smoothing-groups/🦀️.rs"]
pub mod set_smoothing_groups;
#[path = "🕳️set-unknown-statements/🦀️.rs"]
pub mod set_unknown_statements;
//#endregion 🔖️Leaves

/// 📐️ Typed content mutation for `stdio.obj`. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none, and `no`
/// is not an approved semantic verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = ObjSnapshot, diff = ObjDiff, schema = "ObjMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum ObjMutation {
    SetSnapshot(set_snapshot::SetSnapshot),

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
    "set-snapshot",
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

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_obj_mutation(snapshot: &mut ObjSnapshot, mutation: &ObjMutation) -> protocol::MutationOutcome<ObjDiff> {
    let outcome = <ObjMutation as Mutation<ObjSnapshot>>::diff(mutation, snapshot);
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
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
    let disturbed = |faces: &[usize]| faces.iter().any(|member| *member >= index);
    let mut undo = vec![ObjMutation::InsertFace(insert_face::InsertFace { index, face: face.clone() })];
    undo.extend(base.groups.iter().filter(|group| disturbed(&group.faces)).map(|group| ObjMutation::SetGroup(set_group::SetGroup { name: group.name.clone(), faces: group.faces.clone() })));
    undo.extend(base.objects.iter().filter(|object| disturbed(&object.faces)).map(|object| ObjMutation::SetObject(set_object::SetObject { name: object.name.clone(), faces: object.faces.clone() })));
    undo
}

/// ↩️ The undo of removing the `g` entry at position `at`. `SetGroup` on a name the document no
/// longer carries APPENDS, so a lone `SetGroup` restores the membership but moves the band to the
/// end of the list — and the list's order is what decides the token order of a `g a b` line for a
/// face two bands share. The tail after `at` is therefore lifted off and re-declared in its own
/// order, which puts every entry back at the exact position `base` gave it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn restore_group_at(at: usize, base: &ObjSnapshot) -> Vec<ObjMutation> {
    let mut undo: Vec<ObjMutation> = base.groups[at + 1..].iter().map(|group| ObjMutation::RemoveGroup(remove_group::RemoveGroup { name: group.name.clone() })).collect();
    undo.extend(base.groups[at..].iter().map(|group| ObjMutation::SetGroup(set_group::SetGroup { name: group.name.clone(), faces: group.faces.clone() })));
    undo
}

/// ↩️ The `o` mirror of [`restore_group_at`] — same append-loses-position defect, same repair, kept
/// separate because `groups` and `objects` are distinct name spaces rather than one list.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn restore_object_at(at: usize, base: &ObjSnapshot) -> Vec<ObjMutation> {
    let mut undo: Vec<ObjMutation> = base.objects[at + 1..].iter().map(|object| ObjMutation::RemoveObject(remove_object::RemoveObject { name: object.name.clone() })).collect();
    undo.extend(base.objects[at..].iter().map(|object| ObjMutation::SetObject(set_object::SetObject { name: object.name.clone(), faces: object.faces.clone() })));
    undo
}
//#endregion 🔖️InverseRestoration

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &ObjMutation, base: &ObjSnapshot) -> protocol::MutationOutcome<ObjDiff> {
    protocol::MutationOutcome::new(match this {
        ObjMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),

        ObjMutation::InsertVertex(insert_vertex::InsertVertex { index, vertex }) => diff_insert_vertex(*index, vertex.clone()),
        ObjMutation::RemoveVertex(remove_vertex::RemoveVertex { index }) => diff_remove_vertex(*index),
        ObjMutation::SetVertex(set_vertex::SetVertex { index, vertex }) => {
            let old = base.vertices.get(*index).cloned().unwrap_or_default();
            diff_set_vertex(*index, vertex_diff_between(&old, vertex))
        }

        ObjMutation::InsertTexcoord(insert_tex_coord::InsertTexcoord { index, texcoord }) => diff_insert_texcoord(*index, texcoord.clone()),
        ObjMutation::RemoveTexcoord(remove_tex_coord::RemoveTexcoord { index }) => diff_remove_texcoord(*index),
        ObjMutation::SetTexcoord(set_tex_coord::SetTexcoord { index, texcoord }) => {
            let old = base.texcoords.get(*index).cloned().unwrap_or_default();
            diff_set_texcoord(*index, texcoord_diff_between(&old, texcoord))
        }

        ObjMutation::InsertNormal(insert_normal::InsertNormal { index, normal }) => diff_insert_normal(*index, normal.clone()),
        ObjMutation::RemoveNormal(remove_normal::RemoveNormal { index }) => diff_remove_normal(*index),
        ObjMutation::SetNormal(set_normal::SetNormal { index, normal }) => {
            let old = base.normals.get(*index).cloned().unwrap_or_default();
            diff_set_normal(*index, normal_diff_between(&old, normal))
        }

        ObjMutation::InsertFace(insert_face::InsertFace { index, face }) => diff_insert_face(*index, face.clone()),
        ObjMutation::RemoveFace(remove_face::RemoveFace { index }) => diff_remove_face(*index),
        ObjMutation::SetFace(set_face::SetFace { index, face }) => {
            let old = base.faces.get(*index).cloned().unwrap_or_default();
            diff_set_face(*index, face_diff_between(&old, face))
        }

        ObjMutation::SetGroup(set_group::SetGroup { name, faces }) => {
            let existed = base.groups.iter().any(|g| &g.name == name);
            diff_set_group(base.groups.len(), name, faces.clone(), existed)
        }
        ObjMutation::RemoveGroup(remove_group::RemoveGroup { name }) => diff_remove_group(name),
        ObjMutation::SetObject(set_object::SetObject { name, faces }) => {
            let existed = base.objects.iter().any(|o| &o.name == name);
            diff_set_object(base.objects.len(), name, faces.clone(), existed)
        }
        ObjMutation::RemoveObject(remove_object::RemoveObject { name }) => diff_remove_object(name),

        ObjMutation::SetMtllib(set_mtllib::SetMtllib { mtllib }) => diff_set_mtllib(mtllib.clone()),
        ObjMutation::SetUsemtl(set_usemtl::SetUsemtl { usemtl }) => diff_set_usemtl(usemtl.clone()),
        ObjMutation::SetSmoothingGroups(set_smoothing_groups::SetSmoothingGroups { smoothing_groups }) => diff_set_smoothing_groups(smoothing_groups.clone()),
        ObjMutation::SetUnknownStatements(set_unknown_statements::SetUnknownStatements { unknown_statements }) => diff_set_unknown_statements(unknown_statements.clone()),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &ObjMutation, base: &ObjSnapshot) -> Vec<ObjMutation> {
    match this {
        ObjMutation::SetSnapshot(_) => vec![ObjMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],

        ObjMutation::InsertVertex(insert_vertex::InsertVertex { index, .. }) => vec![ObjMutation::RemoveVertex(remove_vertex::RemoveVertex { index: (*index).min(base.vertices.len()) })],
        ObjMutation::RemoveVertex(remove_vertex::RemoveVertex { index }) => match base.vertices.get(*index) {
            Some(v) => vec![ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: *index, vertex: v.clone() })],
            None => Vec::new(),
        },
        ObjMutation::SetVertex(set_vertex::SetVertex { index, .. }) => match base.vertices.get(*index) {
            Some(v) => vec![ObjMutation::SetVertex(set_vertex::SetVertex { index: *index, vertex: v.clone() })],
            None => Vec::new(),
        },

        ObjMutation::InsertTexcoord(insert_tex_coord::InsertTexcoord { index, .. }) => vec![ObjMutation::RemoveTexcoord(remove_tex_coord::RemoveTexcoord { index: (*index).min(base.texcoords.len()) })],
        ObjMutation::RemoveTexcoord(remove_tex_coord::RemoveTexcoord { index }) => match base.texcoords.get(*index) {
            Some(v) => vec![ObjMutation::InsertTexcoord(insert_tex_coord::InsertTexcoord { index: *index, texcoord: v.clone() })],
            None => Vec::new(),
        },
        ObjMutation::SetTexcoord(set_tex_coord::SetTexcoord { index, .. }) => match base.texcoords.get(*index) {
            Some(v) => vec![ObjMutation::SetTexcoord(set_tex_coord::SetTexcoord { index: *index, texcoord: v.clone() })],
            None => Vec::new(),
        },

        ObjMutation::InsertNormal(insert_normal::InsertNormal { index, .. }) => vec![ObjMutation::RemoveNormal(remove_normal::RemoveNormal { index: (*index).min(base.normals.len()) })],
        ObjMutation::RemoveNormal(remove_normal::RemoveNormal { index }) => match base.normals.get(*index) {
            Some(v) => vec![ObjMutation::InsertNormal(insert_normal::InsertNormal { index: *index, normal: v.clone() })],
            None => Vec::new(),
        },
        ObjMutation::SetNormal(set_normal::SetNormal { index, .. }) => match base.normals.get(*index) {
            Some(v) => vec![ObjMutation::SetNormal(set_normal::SetNormal { index: *index, normal: v.clone() })],
            None => Vec::new(),
        },

        ObjMutation::InsertFace(insert_face::InsertFace { index, .. }) => vec![ObjMutation::RemoveFace(remove_face::RemoveFace { index: (*index).min(base.faces.len()) })],
        ObjMutation::RemoveFace(remove_face::RemoveFace { index }) => match base.faces.get(*index) {
            Some(v) => restore_face_at(*index, v, base),
            None => Vec::new(),
        },
        ObjMutation::SetFace(set_face::SetFace { index, .. }) => match base.faces.get(*index) {
            Some(v) => vec![ObjMutation::SetFace(set_face::SetFace { index: *index, face: v.clone() })],
            None => Vec::new(),
        },

        ObjMutation::SetGroup(set_group::SetGroup { name, .. }) => match base.groups.iter().find(|g| &g.name == name) {
            Some(g) => vec![ObjMutation::SetGroup(set_group::SetGroup { name: name.clone(), faces: g.faces.clone() })],
            None => vec![ObjMutation::RemoveGroup(remove_group::RemoveGroup { name: name.clone() })],
        },
        ObjMutation::RemoveGroup(remove_group::RemoveGroup { name }) => match base.groups.iter().position(|g| &g.name == name) {
            Some(at) => restore_group_at(at, base),
            None => Vec::new(),
        },
        ObjMutation::SetObject(set_object::SetObject { name, .. }) => match base.objects.iter().find(|o| &o.name == name) {
            Some(o) => vec![ObjMutation::SetObject(set_object::SetObject { name: name.clone(), faces: o.faces.clone() })],
            None => vec![ObjMutation::RemoveObject(remove_object::RemoveObject { name: name.clone() })],
        },
        ObjMutation::RemoveObject(remove_object::RemoveObject { name }) => match base.objects.iter().position(|o| &o.name == name) {
            Some(at) => restore_object_at(at, base),
            None => Vec::new(),
        },

        ObjMutation::SetMtllib(_) => vec![ObjMutation::SetMtllib(set_mtllib::SetMtllib { mtllib: base.mtllib.clone() })],
        ObjMutation::SetUsemtl(_) => vec![ObjMutation::SetUsemtl(set_usemtl::SetUsemtl { usemtl: base.usemtl.clone() })],
        ObjMutation::SetSmoothingGroups(_) => vec![ObjMutation::SetSmoothingGroups(set_smoothing_groups::SetSmoothingGroups { smoothing_groups: base.smoothing_groups.clone() })],
        ObjMutation::SetUnknownStatements(_) => vec![ObjMutation::SetUnknownStatements(set_unknown_statements::SetUnknownStatements { unknown_statements: base.unknown_statements.clone() })],
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only) — the same ~15-line body
/// every `DslOps`-derived enum's `OpText` impl uses (`GifMutation`, `FlowMutationDsl`,
/// `SpaceMutation`; see `f6-recon-report.md` §2).
impl OpText for ObjMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`.
impl OpBinary for ObjMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
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
        ObjMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
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
        ObjMutation::SetGroup(set_group::SetGroup { name: "Base".into(), faces: vec![0, 0] }),
        ObjMutation::SetGroup(set_group::SetGroup { name: "New".into(), faces: vec![0] }),
        ObjMutation::RemoveGroup(remove_group::RemoveGroup { name: "Base".into() }),
        ObjMutation::SetObject(set_object::SetObject { name: "Obj".into(), faces: vec![0] }),
        ObjMutation::SetObject(set_object::SetObject { name: "NewObj".into(), faces: vec![0] }),
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
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;

    //#region 🔖️MutationDiffLaw
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = base_snapshot();
        for m in demo_mutation_cases() {
            let diff = m.diff(&base);
            let expected = diff.diff().apply(&base).expect("valid mutation diff");

            let mut via_apply = base.clone();
            let returned_diff = apply_obj_mutation(&mut via_apply, &m);

            assert_eq!(via_apply, expected, "apply_obj_mutation mismatch for {m:?}");
            assert_eq!(returned_diff, diff, "returned diff mismatch for {m:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base = base_snapshot();
        for m in demo_mutation_cases() {
            let mut forward = base.clone();
            apply_obj_mutation(&mut forward, &m);
            for inv in m.inverse(&base) {
                apply_obj_mutation(&mut forward, &inv);
            }
            assert_eq!(forward, base, "mutation-level inverse round trip failed for {m:?}");

            let d = m.diff(&base);
            let mid = d.diff().apply(&base).expect("valid forward diff");
            let back = d.diff().inverse(&base).apply(&mid).expect("valid inverse diff");
            assert_eq!(back, base, "diff-level inverse round trip failed for {m:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        let base = base_snapshot();

        // 🧩 Insert(2) + Remove(0): the two-op sequence base → mid → after.
        let d1 = ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: 2, vertex: ObjVertex { x: 8.0, y: 8.0, z: 8.0, w: None } }).diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::RemoveVertex(remove_vertex::RemoveVertex { index: 0 }).diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Insert+Remove-before absorb mismatch");

        // 🧩 Insert(2,f) + Insert(2,g): both must survive.
        let d1 = ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: 2, vertex: ObjVertex { x: 1.0, y: 0.0, z: 0.0, w: None } }).diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: 2, vertex: ObjVertex { x: 2.0, y: 0.0, z: 0.0, w: None } }).diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Insert+Insert-same-index absorb mismatch");
        assert_eq!(after.vertices.len(), base.vertices.len() + 2, "both inserts must survive");

        // 🧩 Add + SetField (SetVertex): patch into the added payload.
        let d1 = ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: 1, vertex: ObjVertex { x: 0.0, y: 0.0, z: 0.0, w: None } }).diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::SetVertex(set_vertex::SetVertex { index: 1, vertex: ObjVertex { x: 42.0, y: 0.0, z: 0.0, w: Some(1.0) } }).diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+SetField absorb mismatch");
        assert_eq!(after.vertices[1].x, 42.0);

        // 🧩 Modify + Remove: modifying then removing the same vertex collapses to a removal.
        let d1 = ObjMutation::SetVertex(set_vertex::SetVertex { index: 1, vertex: ObjVertex { x: 7.0, y: 0.0, z: 0.0, w: None } }).diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::RemoveVertex(remove_vertex::RemoveVertex { index: 1 }).diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Modify+Remove absorb mismatch");

        // 🧩 Name-keyed: Add group + Rename-shaped remove-of-added annihilates the add.
        let d1 = ObjMutation::SetGroup(set_group::SetGroup { name: "Fresh".into(), faces: vec![0] }).diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::RemoveGroup(remove_group::RemoveGroup { name: "Fresh".into() }).diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+Remove(name-keyed) absorb mismatch");
        assert_eq!(after.groups, base.groups, "add-then-remove of the same name must be a full no-op");

        // 🧩 Associativity over a triple.
        let base = base_snapshot();
        let d1 = ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: 0, vertex: ObjVertex { x: 1.0, y: 0.0, z: 0.0, w: None } }).diff(&base);
        let s1 = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::SetVertex(set_vertex::SetVertex { index: 0, vertex: ObjVertex { x: 2.0, y: 0.0, z: 0.0, w: Some(1.0) } }).diff(&s1);
        let s2 = d2.diff().apply(&s1).expect("valid second diff");
        let d3 = ObjMutation::RemoveVertex(remove_vertex::RemoveVertex { index: 2 }).diff(&s2);
        let s3 = d3.diff().apply(&s2).expect("valid third diff");

        let mut left = d1.diff().clone();
        left.absorb(d2.diff().clone());
        left.absorb(d3.diff().clone());

        let mut d23 = d2.diff().clone();
        d23.absorb(d3.diff().clone());
        let mut right = d1.diff().clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base).expect("valid left diff"), s3);
        assert_eq!(right.apply(&base).expect("valid right diff"), s3);
        assert_eq!(left.apply(&base).expect("valid left diff"), right.apply(&base).expect("valid right diff"), "absorb must be associative");
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(ObjDiff::between(&a, &b).apply(&a).expect("valid forward diff"), b);
        assert_eq!(ObjDiff::between(&b, &a).apply(&b).expect("valid backward diff"), a);
        assert!(ObjDiff::between(&a, &a).is_empty());
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️FieldSweep
    #[semio_framework_async_macros::async_test]
    async fn field_sweep_every_mutable_field_changes() {
        let a = sweep_a();
        let b = sweep_b();

        let d_ab = ObjDiff::between(&a, &b);
        assert_eq!(d_ab.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) == b");
        let d_ba = ObjDiff::between(&b, &a);
        assert_eq!(d_ba.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) == a");
        assert!(ObjDiff::between(&a, &a).is_empty());

        // 🔍 Index-keyed collections: `between(a,b)` (b longer) proves modified+added;
        // `between(b,a)` (b longer, now the base) proves modified+removed. Combined, every
        // triple kind is exercised for every one of the four index-keyed collections.
        let vd_ab = d_ab.vertices.as_ref().expect("vertices diff populated (a->b)");
        assert!(vd_ab.removed.is_empty() && !vd_ab.modified.is_empty() && !vd_ab.added.is_empty());
        let vm = &vd_ab.modified[0].diff;
        assert!(vm.x.is_some() && vm.y.is_some() && vm.z.is_some() && vm.w.is_some(), "every ObjVertexDiff field must be patched");
        let vd_ba = d_ba.vertices.as_ref().expect("vertices diff populated (b->a)");
        assert!(!vd_ba.removed.is_empty() && !vd_ba.modified.is_empty() && vd_ba.added.is_empty());

        let td_ab = d_ab.texcoords.as_ref().expect("texcoords diff populated");
        assert!(!td_ab.modified.is_empty() && !td_ab.added.is_empty());
        let tm = &td_ab.modified[0].diff;
        assert!(tm.u.is_some() && tm.v.is_some(), "u/v must be patched");
        assert_eq!(tm.w, Some(None), "w tri-state must exercise Some(None) (source had w, target doesn't)");

        let nd_ab = d_ab.normals.as_ref().expect("normals diff populated");
        assert!(!nd_ab.modified.is_empty() && !nd_ab.added.is_empty());
        let nm = &nd_ab.modified[0].diff;
        assert!(nm.x.is_some() && nm.y.is_some() && nm.z.is_some());

        let fd_ab = d_ab.faces.as_ref().expect("faces diff populated");
        assert!(!fd_ab.modified.is_empty() && !fd_ab.added.is_empty());
        assert!(fd_ab.modified[0].diff.vertices.is_some());

        // 🔍 Name-keyed collections: all three kinds from ONE `between(a,b)` call.
        let gd = d_ab.groups.as_ref().expect("groups diff populated");
        assert!(!gd.removed.is_empty(), "removed must be non-empty (G1 dropped)");
        assert!(!gd.modified.is_empty(), "modified must be non-empty (G2's faces changed)");
        assert!(!gd.added.is_empty(), "added must be non-empty (G3 is new)");
        assert!(gd.modified[0].diff.faces.is_some());

        let od = d_ab.objects.as_ref().expect("objects diff populated");
        assert!(!od.removed.is_empty() && !od.modified.is_empty() && !od.added.is_empty());

        // 🔍 Scalars.
        assert_eq!(d_ab.mtllib, Some(None), "mtllib tri-state must exercise Some(None)");
        assert!(d_ab.usemtl.is_some());
        assert!(d_ab.smoothing_groups.is_some());
        assert!(d_ab.unknown_statements.is_some());
    }
    //#endregion 🔖️FieldSweep

    //#region 🔖️OpTextBinaryRoundtripLaw
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws over every `ObjMutation` variant (handcrafted
    /// impls over the `dsl::DslOps`-derived `DslVariants` — ticket `f6-recon-report.md` §2/§3;
    /// `demo_mutation_cases()` already covers every variant, incl. `SetSnapshot`'s whole nested
    /// `ObjSnapshot` tree and every index-/name-keyed leaf payload type).
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for m in demo_mutation_cases() {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = ObjMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = ObjMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw

    //#region 🔖️KindsCoverageLaw
    /// 🏷️ `KINDS` must name exactly the enum's variants (kebab-case), one entry each — an
    /// exhaustive `match` so the compiler itself fails the moment a variant is added, renamed or
    /// removed without this list being updated alongside it. The manifest side of the same claim
    /// (`../../🔣️oracle.json`'s `obj-3-0-any` catalog `kinds`) is checked by the
    /// mutate/inverse test case's own contract gate, which fails if the two lists ever diverge.
    #[semio_framework_async_macros::async_test]
    async fn kinds_cover_every_variant() {
        fn kind_of(mutation: &ObjMutation) -> &'static str {
            match mutation {
                ObjMutation::SetSnapshot(_) => "set-snapshot",
                ObjMutation::InsertVertex(_) => "insert-vertex",
                ObjMutation::RemoveVertex(_) => "remove-vertex",
                ObjMutation::SetVertex(_) => "set-vertex",
                ObjMutation::InsertTexcoord(_) => "insert-texcoord",
                ObjMutation::RemoveTexcoord(_) => "remove-texcoord",
                ObjMutation::SetTexcoord(_) => "set-texcoord",
                ObjMutation::InsertNormal(_) => "insert-normal",
                ObjMutation::RemoveNormal(_) => "remove-normal",
                ObjMutation::SetNormal(_) => "set-normal",
                ObjMutation::InsertFace(_) => "insert-face",
                ObjMutation::RemoveFace(_) => "remove-face",
                ObjMutation::SetFace(_) => "set-face",
                ObjMutation::SetGroup(_) => "set-group",
                ObjMutation::RemoveGroup(_) => "remove-group",
                ObjMutation::SetObject(_) => "set-object",
                ObjMutation::RemoveObject(_) => "remove-object",
                ObjMutation::SetMtllib(_) => "set-mtllib",
                ObjMutation::SetUsemtl(_) => "set-usemtl",
                ObjMutation::SetSmoothingGroups(_) => "set-smoothing-groups",
                ObjMutation::SetUnknownStatements(_) => "set-unknown-statements",
            }
        }
        let mut exercised: Vec<&str> = demo_mutation_cases().iter().map(kind_of).collect();
        exercised.sort_unstable();
        exercised.dedup();
        let mut declared: Vec<&str> = KINDS.to_vec();
        declared.sort_unstable();
        assert_eq!(exercised, declared, "KINDS must name exactly the variants demo_mutation_cases() exercises");
        assert_eq!(KINDS.len(), 21, "obj-3-0-any declares 21 ObjMutation variants");
    }
    //#endregion 🔖️KindsCoverageLaw

    //#region 🔖️IndexSpaceInverseLaw
    /// 🧊️ Three faces, two bands and one object over them — the smallest mesh on which removing a
    /// face disturbs a membership list that another entry sits after.
    // 🚫️async: E1 pure test-fixture builder, no I/O — see R9
    fn banded_snapshot() -> ObjSnapshot {
        let corner = |vertex: u32| ObjFaceVertex { vertex, texcoord: None, normal: None };
        let face = |a: u32, b: u32, c: u32| ObjFace { vertices: vec![corner(a), corner(b), corner(c)] };
        ObjSnapshot {
            schema: "stdio.obj".into(),
            vertices: vec![ObjVertex { x: 0.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 1.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 0.0, y: 1.0, z: 0.0, w: None }],
            texcoords: vec![],
            normals: vec![],
            faces: vec![face(0, 1, 2), face(1, 2, 0), face(2, 0, 1)],
            groups: vec![ObjGroup { name: "front".into(), faces: vec![0, 1] }, ObjGroup { name: "back".into(), faces: vec![2] }],
            objects: vec![ObjObject { name: "shell".into(), faces: vec![0, 1, 2] }],
            mtllib: None,
            usemtl: vec![],
            smoothing_groups: vec![],
            unknown_statements: vec![],
        }
    }

    /// ↩️ Removing a face that BELONGS to a band must invert through the band, not through the row
    /// alone: the undo names every membership list the positional removal disturbed and puts each
    /// back to the exact list the pre-mutation document declared.
    #[test]
    fn remove_face_inverts_through_the_membership_it_disturbed() {
        let base = banded_snapshot();
        let removal = ObjMutation::RemoveFace(remove_face::RemoveFace { index: 1 });
        let undo = removal.inverse(&base);
        assert!(matches!(undo.first(), Some(ObjMutation::InsertFace(insert_face::InsertFace { index: 1, .. }))), "the row itself comes back first, at its own position: {undo:?}");
        assert!(undo.iter().any(|step| matches!(step, ObjMutation::SetGroup(set_group::SetGroup { name, faces }) if name == "front" && faces == &vec![0, 1])), "the removed face's own band must be re-declared: {undo:?}");
        assert!(undo.iter().any(|step| matches!(step, ObjMutation::SetGroup(set_group::SetGroup { name, faces }) if name == "back" && faces == &vec![2])), "so must the band the removal shifted: {undo:?}");
        assert!(undo.iter().any(|step| matches!(step, ObjMutation::SetObject(set_object::SetObject { name, faces }) if name == "shell" && faces == &vec![0, 1, 2])), "and the object over all three: {undo:?}");

        let mut restored = base.clone();
        apply_obj_mutation(&mut restored, &removal);
        assert_ne!(restored.faces, base.faces, "the removal has to move the mesh, or the undo proves nothing");
        for step in &undo {
            apply_obj_mutation(&mut restored, step);
        }
        assert_eq!(restored, base, "forward then inverse must return the whole snapshot, membership included");
    }

    /// ↩️ Removing the FIRST of several bands must invert back to that band's own position. A single
    /// `SetGroup` appends instead, which the second half of this test states outright rather than
    /// leaving as folklore.
    #[test]
    fn remove_group_inverts_back_to_its_own_position() {
        let base = banded_snapshot();
        let removal = ObjMutation::RemoveGroup(remove_group::RemoveGroup { name: "front".into() });
        let mut restored = base.clone();
        apply_obj_mutation(&mut restored, &removal);
        assert_eq!(restored.groups.len(), 1, "the removal has to move the document");

        let mut naive = restored.clone();
        apply_obj_mutation(&mut naive, &ObjMutation::SetGroup(set_group::SetGroup { name: "front".into(), faces: vec![0, 1] }));
        assert_eq!(naive.groups.iter().map(|group| group.name.as_str()).collect::<Vec<_>>(), vec!["back", "front"], "a lone SetGroup appends — this is the position loss the sequenced inverse repairs");

        for step in removal.inverse(&base) {
            apply_obj_mutation(&mut restored, &step);
        }
        assert_eq!(restored, base, "the sequenced inverse must restore both the membership and the order");
    }

    /// ↩️ The `o` mirror, kept separate because `groups` and `objects` are distinct name spaces.
    #[test]
    fn remove_object_inverts_back_to_its_own_position() {
        let mut base = banded_snapshot();
        base.objects = vec![ObjObject { name: "shell".into(), faces: vec![0, 1] }, ObjObject { name: "cap".into(), faces: vec![2] }];
        let removal = ObjMutation::RemoveObject(remove_object::RemoveObject { name: "shell".into() });
        let mut restored = base.clone();
        apply_obj_mutation(&mut restored, &removal);
        assert_eq!(restored.objects.len(), 1, "the removal has to move the document");
        for step in removal.inverse(&base) {
            apply_obj_mutation(&mut restored, &step);
        }
        assert_eq!(restored, base, "the sequenced inverse must restore both the membership and the order");
    }
    //#endregion 🔖️IndexSpaceInverseLaw
}
//#endregion 🧪️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/🏗️lifts-the-third-vertex-and-gives-it-an-explicit-w/🦀️.rs"]
mod set_snapshot_lifts_the_third_vertex_and_gives_it_an_explicit_w;
//#endregion 🧪️FixtureCases
