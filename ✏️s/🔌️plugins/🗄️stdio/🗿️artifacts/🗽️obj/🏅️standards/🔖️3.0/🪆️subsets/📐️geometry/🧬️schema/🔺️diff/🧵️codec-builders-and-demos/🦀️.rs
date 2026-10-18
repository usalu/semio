//! 🧵️ The hand-rolled `ObjDiff` text and binary codec, the mutation diff builders and the demo diff cases.

use super::*;

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` for `ObjDiff` — see the module doc comment at the top
/// of this file for the confirmed 3b (tri-state) compile-error citation. **Grammar** (real, not
/// `serde_json`, mirrors gif89a/svg's §5 template): one space-separated token per changed
/// top-level field (absent token = unchanged); the four index-keyed collections
/// (`vertices`/`texcoords`/`normals`/`faces`) and the two name-keyed collections
/// (`groups`/`objects`) print as `name{[removed];[modified];[added]}`; strings (group/object/
/// material names, `mtllib`, retained raw source lines) are lowercase hex (no external base64
/// dep, no escaping needed — this artifact's `unknown_statements` can contain arbitrary source
/// text); `f64` fields use Rust's own round-trippable `to_string`/`parse` (no external float-fmt
/// dep); `Option<T>` values (both real optional snapshot fields like `ObjVertex::w` AND diff
/// tri-states like `ObjDiff::mtllib`) use the uniform `[0]`=None / `[1,<T>]`=Some(T) tag; per-item
/// sparse diffs (`ObjVertexDiff`, `ObjTexCoordDiff`, `ObjNormalDiff`, `ObjFaceDiff`,
/// `ObjGroupDiff`) print as single-uppercase-letter `TAG:value` pairs inside their own `[...]`,
/// same convention as gif89a's `GifFrameDiff`.
//#region 🔖️Primitives













//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs




















//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs










//#endregion 🔖️DiffValueCodecs

//#region 🔖️CollectionCodecs


















//#endregion 🔖️CollectionCodecs

//#region 🔖️BinaryPrimitives














//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueBinaryCodecs





















//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️DiffValueBinaryCodecs










//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️CollectionBinaryCodecs

















//#endregion 🔖️CollectionBinaryCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️MutationDiffBuilders
// 🧮 Item-level field-change wrappers, exposed to `🧬️mutations` so `SetVertex`/`SetTexcoord`/
// `SetNormal`/`SetFace`'s `diff()` can compute a sparse per-field patch without the private
// `ObjIndexElem` trait itself leaving this module.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn vertex_field_changes(a: &ObjVertex, b: &ObjVertex) -> ObjVertexDiff {
    <ObjVertex as ObjIndexElem>::field_changes(a, b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn texcoord_field_changes(a: &ObjTexCoord, b: &ObjTexCoord) -> ObjTexCoordDiff {
    <ObjTexCoord as ObjIndexElem>::field_changes(a, b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn normal_field_changes(a: &ObjNormal, b: &ObjNormal) -> ObjNormalDiff {
    <ObjNormal as ObjIndexElem>::field_changes(a, b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn face_field_changes(a: &ObjFace, b: &ObjFace) -> ObjFaceDiff {
    <ObjFace as ObjIndexElem>::field_changes(a, b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_vertex(index: usize, vertex: ObjVertex) -> ObjDiff {
    ObjDiff { vertices: Some(ObjVerticesDiff { removed: vec![], modified: vec![], added: vec![ObjVertexAdded { index, vertex }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_vertex(index: usize) -> ObjDiff {
    ObjDiff { vertices: Some(ObjVerticesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_vertex(index: usize, diff: ObjVertexDiff) -> ObjDiff {
    ObjDiff { vertices: Some(ObjVerticesDiff { removed: vec![], modified: vec![ObjVertexModified { index, diff }], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_texcoord(index: usize, texcoord: ObjTexCoord) -> ObjDiff {
    ObjDiff { texcoords: Some(ObjTexCoordsDiff { removed: vec![], modified: vec![], added: vec![ObjTexCoordAdded { index, texcoord }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_texcoord(index: usize) -> ObjDiff {
    ObjDiff { texcoords: Some(ObjTexCoordsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_texcoord(index: usize, diff: ObjTexCoordDiff) -> ObjDiff {
    ObjDiff { texcoords: Some(ObjTexCoordsDiff { removed: vec![], modified: vec![ObjTexCoordModified { index, diff }], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_normal(index: usize, normal: ObjNormal) -> ObjDiff {
    ObjDiff { normals: Some(ObjNormalsDiff { removed: vec![], modified: vec![], added: vec![ObjNormalAdded { index, normal }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_normal(index: usize) -> ObjDiff {
    ObjDiff { normals: Some(ObjNormalsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_normal(index: usize, diff: ObjNormalDiff) -> ObjDiff {
    ObjDiff { normals: Some(ObjNormalsDiff { removed: vec![], modified: vec![ObjNormalModified { index, diff }], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_face(index: usize, face: ObjFace) -> ObjDiff {
    ObjDiff { faces: Some(ObjFacesDiff { removed: vec![], modified: vec![], added: vec![ObjFaceAdded { index, face }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_face(index: usize) -> ObjDiff {
    ObjDiff { faces: Some(ObjFacesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_face(index: usize, diff: ObjFaceDiff) -> ObjDiff {
    ObjDiff { faces: Some(ObjFacesDiff { removed: vec![], modified: vec![ObjFaceModified { index, diff }], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_group(index: usize, name: &str, faces: Vec<u64>, existed: bool) -> ObjDiff {
    if existed {
        ObjDiff { groups: Some(ObjGroupsDiff { removed: vec![], modified: vec![ObjGroupModified { name: name.to_string(), diff: ObjGroupDiff { faces: Some(faces) } }], added: vec![] }), ..Default::default() }
    } else {
        ObjDiff { groups: Some(ObjGroupsDiff { removed: vec![], modified: vec![], added: vec![ObjGroupAdded { index, group: ObjGroup { name: name.to_string(), faces } }] }), ..Default::default() }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_group(name: &str) -> ObjDiff {
    ObjDiff { groups: Some(ObjGroupsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_object(index: usize, name: &str, faces: Vec<u64>, existed: bool) -> ObjDiff {
    if existed {
        ObjDiff { objects: Some(ObjObjectsDiff { removed: vec![], modified: vec![ObjGroupModified { name: name.to_string(), diff: ObjGroupDiff { faces: Some(faces) } }], added: vec![] }), ..Default::default() }
    } else {
        ObjDiff { objects: Some(ObjObjectsDiff { removed: vec![], modified: vec![], added: vec![ObjObjectAdded { index, object: ObjObject { name: name.to_string(), faces } }] }), ..Default::default() }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_object(name: &str) -> ObjDiff {
    ObjDiff { objects: Some(ObjObjectsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_mtllib(mtllib: Option<String>) -> ObjDiff {
    ObjDiff { mtllib: Some(mtllib), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_usemtl(usemtl: Vec<ObjUsemtlRange>) -> ObjDiff {
    ObjDiff { usemtl: Some(usemtl), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_smoothing_groups(smoothing_groups: Vec<ObjSmoothingRange>) -> ObjDiff {
    ObjDiff { smoothing_groups: Some(smoothing_groups), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_unknown_statements(unknown_statements: Vec<ObjUnknownStatement>) -> ObjDiff {
    ObjDiff { unknown_statements: Some(unknown_statements), ..Default::default() }
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️DemoCases
/// 🧬️ Canonical "differs in every mutable field" snapshot A — every index-keyed collection
/// has 2 items (a stable prefix item + one that will be modified); every name-keyed
/// collection has 2 named entries (one that will be removed, one that will be modified).
/// Mirrors `🧬️mutations`' own `sweep_a`/`sweep_b` fixtures (kept local to this file, same
/// per-file-independent-fixture convention `stdio.zip`'s own diff/mutations pair uses).
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

/// 🧬️ Sweep B: every index-keyed collection's index-0 item is UNCHANGED, its index-1 item is
/// MODIFIED in every field (incl. a tri-state `Some(None)` on `texcoords[1].w`), and gains a
/// brand-new item at index 2. Name-keyed `groups`/`objects` show removed+modified+added
/// simultaneously from ONE `between(a,b)` call. `mtllib` exercises `Some->None` tri-state.
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

/// 🧪️ P2-FG1: representative `ObjDiff` values built declaratively (empty, the scalar and tri-state lanes, and index-keyed row triples) —
/// single source of truth reused by `diff_codec_text_binary_roundtrip_law` below AND by `../../⚙️engine/🦀️.rs`'s
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests.
#[cfg(any(test, feature = "testkit"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_diff_cases() -> Vec<ObjDiff> {
    vec![
        ObjDiff::default(),
        ObjDiff { mtllib: Some(None), usemtl: Some(Vec::new()), smoothing_groups: Some(Vec::new()), unknown_statements: Some(Vec::new()), ..Default::default() },
        ObjDiff {
            mtllib: Some(Some("a.mtl".into())),
            vertices: Some(ObjVerticesDiff { removed: vec![0], ..Default::default() }),
            faces: Some(ObjFacesDiff { removed: vec![1], modified: vec![ObjFaceModified { index: 0, diff: ObjFaceDiff { vertices: Some(Vec::new()) } }], added: Vec::new() }),
            ..Default::default()
        },
    ]
}
//#endregion 🔖️DemoCases
