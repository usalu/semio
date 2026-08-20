//! 🧬️ ObjMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `ObjDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index/name-aware, reading the pre-state it needs from `base`.
//! `SetVertex`/`SetTexCoord`/`SetNormal`/`SetFace` each set a WHOLE item at an index (not a
//! single sub-field) — their `diff()` still constructs a sparse per-field patch by comparing
//! against `base`'s current value, never a full-item replace.
//!
//! 🧪️ F6: `#[derive(dsl::DslOps)]` — DERIVE path (ticket `f6-recon-report.md` §3's decision rule:
//! the Mutation side only cares whether the Snapshot type tree contains a data-carrying enum
//! ANYWHERE, since `SetSnapshot` always carries the whole `ObjSnapshot`; `obj`'s whole model is
//! plain structs/`Vec`/`Option<T>`, zero enums, confirmed by `cargo check` — no compile error).
//! `OpText`/`OpBinary` are still handcrafted (P6: `DslOps` emits `DslVariants` only, never the two
//! op-codec traits themselves) using the exact boilerplate wrapper every derived-`DslOps` mutation
//! in the repo uses (`GifMutation`, `FlowMutationDsl`, `SpaceMutation`) — see `f6-recon-report.md`
//! §2.

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
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.obj`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum ObjMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
        snapshot: ObjSnapshot,
    },

    /// ➕️ Inserts a whole `v` row at `index` (clamped to the end on apply).
    InsertVertex {
        index: usize,
        #[dsl(block)]
        vertex: ObjVertex,
    },
    /// ➖️ Removes the `v` row at `index`.
    RemoveVertex { index: usize },
    /// ✏️ Replaces the WHOLE `v` row at `index` (diff is still a sparse per-field patch).
    SetVertex {
        index: usize,
        #[dsl(block)]
        vertex: ObjVertex,
    },

    /// ➕️ Inserts a whole `vt` row at `index`.
    InsertTexCoord {
        index: usize,
        #[dsl(block)]
        texcoord: ObjTexCoord,
    },
    /// ➖️ Removes the `vt` row at `index`.
    RemoveTexCoord { index: usize },
    /// ✏️ Replaces the WHOLE `vt` row at `index`.
    SetTexCoord {
        index: usize,
        #[dsl(block)]
        texcoord: ObjTexCoord,
    },

    /// ➕️ Inserts a whole `vn` row at `index`.
    InsertNormal {
        index: usize,
        #[dsl(block)]
        normal: ObjNormal,
    },
    /// ➖️ Removes the `vn` row at `index`.
    RemoveNormal { index: usize },
    /// ✏️ Replaces the WHOLE `vn` row at `index`.
    SetNormal {
        index: usize,
        #[dsl(block)]
        normal: ObjNormal,
    },

    /// ➕️ Inserts a whole `f` row at `index`.
    InsertFace {
        index: usize,
        #[dsl(block)]
        face: ObjFace,
    },
    /// ➖️ Removes the `f` row at `index`.
    RemoveFace { index: usize },
    /// ✏️ Replaces the WHOLE `f` row at `index`.
    SetFace {
        index: usize,
        #[dsl(block)]
        face: ObjFace,
    },

    /// 🏷️ Creates or replaces a named `g` group's face-index membership.
    SetGroup { name: String, faces: Vec<usize> },
    /// ➖️ Removes a named `g` group.
    RemoveGroup { name: String },
    /// 🏷️ Creates or replaces a named `o` object's face-index membership.
    SetObject { name: String, faces: Vec<usize> },
    /// ➖️ Removes a named `o` object.
    RemoveObject { name: String },

    /// 🎨️ Sets or clears the `mtllib` reference.
    SetMtllib { mtllib: Option<String> },
    /// 🎨️ Replaces the whole `usemtl` range list.
    SetUsemtl { usemtl: Vec<ObjUsemtlRange> },
    /// 🧵️ Replaces the whole `s` smoothing-range list.
    SetSmoothingGroups { smoothing_groups: Vec<ObjSmoothingRange> },
    /// 🕳️ Replaces the whole retained unknown-statement list.
    SetUnknownStatements { unknown_statements: Vec<ObjUnknownStatement> },
}
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

//#region 🔖️MutationTrait
impl Mutation<ObjSnapshot> for ObjMutation {
    type Diff = ObjDiff;

    async fn diff(&self, base: &ObjSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            ObjMutation::NoMutation => ObjDiff::default(),
            ObjMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),

            ObjMutation::InsertVertex { index, vertex } => diff_insert_vertex(*index, vertex.clone()),
            ObjMutation::RemoveVertex { index } => diff_remove_vertex(*index),
            ObjMutation::SetVertex { index, vertex } => {
                let old = base.vertices.get(*index).cloned().unwrap_or_default();
                diff_set_vertex(*index, vertex_diff_between(&old, vertex))
            }

            ObjMutation::InsertTexCoord { index, texcoord } => diff_insert_texcoord(*index, texcoord.clone()),
            ObjMutation::RemoveTexCoord { index } => diff_remove_texcoord(*index),
            ObjMutation::SetTexCoord { index, texcoord } => {
                let old = base.texcoords.get(*index).cloned().unwrap_or_default();
                diff_set_texcoord(*index, texcoord_diff_between(&old, texcoord))
            }

            ObjMutation::InsertNormal { index, normal } => diff_insert_normal(*index, normal.clone()),
            ObjMutation::RemoveNormal { index } => diff_remove_normal(*index),
            ObjMutation::SetNormal { index, normal } => {
                let old = base.normals.get(*index).cloned().unwrap_or_default();
                diff_set_normal(*index, normal_diff_between(&old, normal))
            }

            ObjMutation::InsertFace { index, face } => diff_insert_face(*index, face.clone()),
            ObjMutation::RemoveFace { index } => diff_remove_face(*index),
            ObjMutation::SetFace { index, face } => {
                let old = base.faces.get(*index).cloned().unwrap_or_default();
                diff_set_face(*index, face_diff_between(&old, face))
            }

            ObjMutation::SetGroup { name, faces } => {
                let existed = base.groups.iter().any(|g| &g.name == name);
                diff_set_group(base.groups.len(), name, faces.clone(), existed)
            }
            ObjMutation::RemoveGroup { name } => diff_remove_group(name),
            ObjMutation::SetObject { name, faces } => {
                let existed = base.objects.iter().any(|o| &o.name == name);
                diff_set_object(base.objects.len(), name, faces.clone(), existed)
            }
            ObjMutation::RemoveObject { name } => diff_remove_object(name),

            ObjMutation::SetMtllib { mtllib } => diff_set_mtllib(mtllib.clone()),
            ObjMutation::SetUsemtl { usemtl } => diff_set_usemtl(usemtl.clone()),
            ObjMutation::SetSmoothingGroups { smoothing_groups } => diff_set_smoothing_groups(smoothing_groups.clone()),
            ObjMutation::SetUnknownStatements { unknown_statements } => diff_set_unknown_statements(unknown_statements.clone()),
        })
    }

    async fn inverse(&self, base: &ObjSnapshot) -> Vec<Self> {
        match self {
            ObjMutation::NoMutation => vec![ObjMutation::NoMutation],
            ObjMutation::SetSnapshot { .. } => vec![ObjMutation::SetSnapshot { snapshot: base.clone() }],

            ObjMutation::InsertVertex { index, .. } => vec![ObjMutation::RemoveVertex { index: *index }],
            ObjMutation::RemoveVertex { index } => match base.vertices.get(*index) {
                Some(v) => vec![ObjMutation::InsertVertex { index: *index, vertex: v.clone() }],
                None => vec![ObjMutation::NoMutation],
            },
            ObjMutation::SetVertex { index, .. } => match base.vertices.get(*index) {
                Some(v) => vec![ObjMutation::SetVertex { index: *index, vertex: v.clone() }],
                None => vec![ObjMutation::NoMutation],
            },

            ObjMutation::InsertTexCoord { index, .. } => vec![ObjMutation::RemoveTexCoord { index: *index }],
            ObjMutation::RemoveTexCoord { index } => match base.texcoords.get(*index) {
                Some(v) => vec![ObjMutation::InsertTexCoord { index: *index, texcoord: v.clone() }],
                None => vec![ObjMutation::NoMutation],
            },
            ObjMutation::SetTexCoord { index, .. } => match base.texcoords.get(*index) {
                Some(v) => vec![ObjMutation::SetTexCoord { index: *index, texcoord: v.clone() }],
                None => vec![ObjMutation::NoMutation],
            },

            ObjMutation::InsertNormal { index, .. } => vec![ObjMutation::RemoveNormal { index: *index }],
            ObjMutation::RemoveNormal { index } => match base.normals.get(*index) {
                Some(v) => vec![ObjMutation::InsertNormal { index: *index, normal: v.clone() }],
                None => vec![ObjMutation::NoMutation],
            },
            ObjMutation::SetNormal { index, .. } => match base.normals.get(*index) {
                Some(v) => vec![ObjMutation::SetNormal { index: *index, normal: v.clone() }],
                None => vec![ObjMutation::NoMutation],
            },

            ObjMutation::InsertFace { index, .. } => vec![ObjMutation::RemoveFace { index: *index }],
            ObjMutation::RemoveFace { index } => match base.faces.get(*index) {
                Some(v) => vec![ObjMutation::InsertFace { index: *index, face: v.clone() }],
                None => vec![ObjMutation::NoMutation],
            },
            ObjMutation::SetFace { index, .. } => match base.faces.get(*index) {
                Some(v) => vec![ObjMutation::SetFace { index: *index, face: v.clone() }],
                None => vec![ObjMutation::NoMutation],
            },

            ObjMutation::SetGroup { name, .. } => match base.groups.iter().find(|g| &g.name == name) {
                Some(g) => vec![ObjMutation::SetGroup { name: name.clone(), faces: g.faces.clone() }],
                None => vec![ObjMutation::RemoveGroup { name: name.clone() }],
            },
            ObjMutation::RemoveGroup { name } => match base.groups.iter().find(|g| &g.name == name) {
                Some(g) => vec![ObjMutation::SetGroup { name: name.clone(), faces: g.faces.clone() }],
                None => vec![ObjMutation::NoMutation],
            },
            ObjMutation::SetObject { name, .. } => match base.objects.iter().find(|o| &o.name == name) {
                Some(o) => vec![ObjMutation::SetObject { name: name.clone(), faces: o.faces.clone() }],
                None => vec![ObjMutation::RemoveObject { name: name.clone() }],
            },
            ObjMutation::RemoveObject { name } => match base.objects.iter().find(|o| &o.name == name) {
                Some(o) => vec![ObjMutation::SetObject { name: name.clone(), faces: o.faces.clone() }],
                None => vec![ObjMutation::NoMutation],
            },

            ObjMutation::SetMtllib { .. } => vec![ObjMutation::SetMtllib { mtllib: base.mtllib.clone() }],
            ObjMutation::SetUsemtl { .. } => vec![ObjMutation::SetUsemtl { usemtl: base.usemtl.clone() }],
            ObjMutation::SetSmoothingGroups { .. } => vec![ObjMutation::SetSmoothingGroups { smoothing_groups: base.smoothing_groups.clone() }],
            ObjMutation::SetUnknownStatements { .. } => vec![ObjMutation::SetUnknownStatements { unknown_statements: base.unknown_statements.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only) — the same ~15-line body
/// every `DslOps`-derived enum's `OpText` impl uses (`GifMutation`, `FlowMutationDsl`,
/// `SpaceMutation`; see `f6-recon-report.md` §2).
impl OpText for ObjMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline }).await?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record).await;
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self).await;
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline).await
    }
}

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`.
impl OpBinary for ObjMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self).await
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes).await
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `ObjSnapshot`/`ObjMutation` fixtures — the single source of truth
/// reused by `op_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️component.rs`'s
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
        ObjMutation::NoMutation,
        ObjMutation::SetSnapshot { snapshot: sweep_b() },
        ObjMutation::InsertVertex { index: 1, vertex: ObjVertex { x: 9.0, y: 9.0, z: 9.0, w: Some(1.0) } },
        ObjMutation::RemoveVertex { index: 0 },
        ObjMutation::SetVertex { index: 0, vertex: ObjVertex { x: 2.0, y: 2.0, z: 2.0, w: None } },
        ObjMutation::InsertTexCoord { index: 0, texcoord: ObjTexCoord { u: 9.0, v: 9.0, w: Some(1.0) } },
        ObjMutation::RemoveTexCoord { index: 0 },
        ObjMutation::SetTexCoord { index: 0, texcoord: ObjTexCoord { u: 5.0, v: 5.0, w: None } },
        ObjMutation::InsertNormal { index: 0, normal: ObjNormal { x: 1.0, y: 0.0, z: 0.0 } },
        ObjMutation::RemoveNormal { index: 0 },
        ObjMutation::SetNormal { index: 0, normal: ObjNormal { x: -1.0, y: 0.0, z: 0.0 } },
        ObjMutation::InsertFace {
            index: 0,
            face: ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: None, normal: None }, ObjFaceVertex { vertex: 1, texcoord: None, normal: None }, ObjFaceVertex { vertex: 2, texcoord: None, normal: None }] },
        },
        ObjMutation::RemoveFace { index: 0 },
        ObjMutation::SetFace {
            index: 0,
            face: ObjFace { vertices: vec![ObjFaceVertex { vertex: 2, texcoord: None, normal: None }, ObjFaceVertex { vertex: 1, texcoord: None, normal: None }, ObjFaceVertex { vertex: 0, texcoord: None, normal: None }] },
        },
        ObjMutation::SetGroup { name: "Base".into(), faces: vec![0, 0] },
        ObjMutation::SetGroup { name: "New".into(), faces: vec![0] },
        ObjMutation::RemoveGroup { name: "Base".into() },
        ObjMutation::SetObject { name: "Obj".into(), faces: vec![0] },
        ObjMutation::SetObject { name: "NewObj".into(), faces: vec![0] },
        ObjMutation::RemoveObject { name: "Obj".into() },
        ObjMutation::SetMtllib { mtllib: None },
        ObjMutation::SetMtllib { mtllib: Some("new.mtl".into()) },
        ObjMutation::SetUsemtl { usemtl: vec![ObjUsemtlRange { face_index_from: 0, material: "Blue".into() }] },
        ObjMutation::SetSmoothingGroups { smoothing_groups: vec![] },
        ObjMutation::SetUnknownStatements { unknown_statements: vec![] },
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
        let d1 = ObjMutation::InsertVertex { index: 2, vertex: ObjVertex { x: 8.0, y: 8.0, z: 8.0, w: None } }.diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::RemoveVertex { index: 0 }.diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Insert+Remove-before absorb mismatch");

        // 🧩 Insert(2,f) + Insert(2,g): both must survive.
        let d1 = ObjMutation::InsertVertex { index: 2, vertex: ObjVertex { x: 1.0, y: 0.0, z: 0.0, w: None } }.diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::InsertVertex { index: 2, vertex: ObjVertex { x: 2.0, y: 0.0, z: 0.0, w: None } }.diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Insert+Insert-same-index absorb mismatch");
        assert_eq!(after.vertices.len(), base.vertices.len() + 2, "both inserts must survive");

        // 🧩 Add + SetField (SetVertex): patch into the added payload.
        let d1 = ObjMutation::InsertVertex { index: 1, vertex: ObjVertex { x: 0.0, y: 0.0, z: 0.0, w: None } }.diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::SetVertex { index: 1, vertex: ObjVertex { x: 42.0, y: 0.0, z: 0.0, w: Some(1.0) } }.diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+SetField absorb mismatch");
        assert_eq!(after.vertices[1].x, 42.0);

        // 🧩 Modify + Remove: modifying then removing the same vertex collapses to a removal.
        let d1 = ObjMutation::SetVertex { index: 1, vertex: ObjVertex { x: 7.0, y: 0.0, z: 0.0, w: None } }.diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::RemoveVertex { index: 1 }.diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Modify+Remove absorb mismatch");

        // 🧩 Name-keyed: Add group + Rename-shaped remove-of-added annihilates the add.
        let d1 = ObjMutation::SetGroup { name: "Fresh".into(), faces: vec![0] }.diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::RemoveGroup { name: "Fresh".into() }.diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+Remove(name-keyed) absorb mismatch");
        assert_eq!(after.groups, base.groups, "add-then-remove of the same name must be a full no-op");

        // 🧩 Associativity over a triple.
        let base = base_snapshot();
        let d1 = ObjMutation::InsertVertex { index: 0, vertex: ObjVertex { x: 1.0, y: 0.0, z: 0.0, w: None } }.diff(&base);
        let s1 = d1.diff().apply(&base).expect("valid first diff");
        let d2 = ObjMutation::SetVertex { index: 0, vertex: ObjVertex { x: 2.0, y: 0.0, z: 0.0, w: Some(1.0) } }.diff(&s1);
        let s2 = d2.diff().apply(&s1).expect("valid second diff");
        let d3 = ObjMutation::RemoveVertex { index: 2 }.diff(&s2);
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
}
//#endregion 🧪️Tests
