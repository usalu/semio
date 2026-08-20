//! 🚪️ IO stdio.obj (3.0/✳️any) — registration now flows through the `s.stdio.obj`
//! `ArtifactDeclaration` (`crate::artifacts::obj::declaration`), not per-leaf register().
//!
//! 📐 Documented normal form (codec_retention_law): `encode_obj` always emits, in this fixed
//! order, `mtllib` (if any), the `v`/`vt`/`vn` blocks, then one `f` line per face preceded by
//! whichever of `o`/`g`/`usemtl`/`s` changed since the previous face (transition-only, exactly
//! like the pre-migration encoder), then a trailer of every retained `unknown_statements` entry
//! (comments and any line the codec doesn't otherwise model) in their original relative order.
//! `unknown_statements[].line_index` is informational (the line number AT THE TIME OF THAT
//! DECODE) and is NOT preserved byte-for-byte across a decode→encode cycle — re-encoding always
//! relocates retained comments/unknown lines into the trailer, so a fresh decode of the
//! re-encoded text renumbers them to their new trailer position. Every other field (geometry,
//! `groups`/`objects`/`usemtl`/`smoothing_groups` membership, `mtllib`, and the unknown
//! statements' own text/relative order) is fully retained, and from the SECOND generation
//! onward decode/encode is a true fixed point (see `codec_retention_law` below). A second,
//! narrower limitation: `groups`/`objects`/`usemtl`/`smoothing_groups` reconstruction assumes
//! the "sticky range" shape real parsing always produces (once a name/material/group becomes
//! active it stays active until explicitly changed) — every real `.obj` file has this shape by
//! construction; a hand-built snapshot with genuinely disjoint/non-contiguous membership for the
//! SAME name is a synthetic case this text codec doesn't attempt to round-trip (diff/mutation
//! semantics are unaffected either way, since those operate on the snapshot directly, never
//! through the text codec).
//#region 🔖️Codec
//#region 🔖️IndexResolution
use crate::artifacts::obj::schema::snapshot::{ObjFace, ObjFaceVertex, ObjGroup, ObjNormal, ObjObject, ObjSmoothingRange, ObjTexCoord, ObjUnknownStatement, ObjUsemtlRange, ObjVertex};
use crate::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};
use std::collections::HashMap;

/// 🔢 Resolves a raw OBJ index (1-based positive, or negative = relative to the
/// current end of the list at parse time) against `current_len` — the OBJ spec's
/// own negative-index rule.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_index(current_len: usize, raw: i64) -> Result<u32, String> {
    if raw > 0 {
        Ok((raw - 1) as u32)
    } else if raw < 0 {
        let idx = current_len as i64 + raw;
        if idx < 0 {
            return Err(format!("negative index {raw} out of range (list has {current_len} entries)"));
        }
        Ok(idx as u32)
    } else {
        Err("obj index 0 is not valid (indices are 1-based)".into())
    }
}

/// 🧩 Parses one `f` face-vertex token (`v`, `v/vt`, `v//vn`, `v/vt/vn`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_face_vertex(token: &str, vertex_count: usize, texcoord_count: usize, normal_count: usize) -> Result<ObjFaceVertex, String> {
    let mut parts = token.split('/');
    let v_raw: i64 = parts.next().ok_or("empty face token")?.parse().map_err(|e| format!("face vertex index: {e}"))?;
    let vertex = resolve_index(vertex_count, v_raw)?;
    let vt_raw = parts.next().unwrap_or("");
    let texcoord = if vt_raw.is_empty() { None } else { Some(resolve_index(texcoord_count, vt_raw.parse().map_err(|e| format!("face texcoord index: {e}"))?)?) };
    let vn_raw = parts.next().unwrap_or("");
    let normal = if vn_raw.is_empty() { None } else { Some(resolve_index(normal_count, vn_raw.parse().map_err(|e| format!("face normal index: {e}"))?)?) };
    Ok(ObjFaceVertex { vertex, texcoord, normal })
}
//#endregion 🔖️IndexResolution

//#region 🔖️Decode
/// 📥 Parses a real Wavefront OBJ text body: `v`/`vt`/`vn` (incl. optional `w`), `f` (v, v/vt,
/// v//vn, v/vt/vn, negative-relative indices, n-gons), `o`/`g` (multi-name)/`usemtl`/`mtllib`/`s`,
/// with every comment and unrecognized statement retained in `unknown_statements`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_obj(text: &str) -> Result<ObjSnapshot, String> {
    let mut vertices = Vec::new();
    let mut texcoords = Vec::new();
    let mut normals = Vec::new();
    let mut faces: Vec<ObjFace> = Vec::new();
    let mut groups: Vec<ObjGroup> = Vec::new();
    let mut group_index: HashMap<String, usize> = HashMap::new();
    let mut objects: Vec<ObjObject> = Vec::new();
    let mut object_index: HashMap<String, usize> = HashMap::new();
    let mut mtllib: Option<String> = None;
    let mut usemtl: Vec<ObjUsemtlRange> = Vec::new();
    let mut smoothing_groups: Vec<ObjSmoothingRange> = Vec::new();
    let mut unknown_statements: Vec<ObjUnknownStatement> = Vec::new();

    let mut cur_active_groups: Vec<String> = Vec::new();
    let mut cur_active_object: Option<String> = None;
    let mut cur_material: Option<String> = None;
    let mut cur_smoothing: Option<u32> = None;
    let mut have_smoothing = false;

    for (line_index, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            unknown_statements.push(ObjUnknownStatement { line_index, raw: line.to_string() });
            continue;
        }
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x: f64 = parts.next().ok_or("v x")?.parse().map_err(|e| format!("{e}"))?;
                let y: f64 = parts.next().ok_or("v y")?.parse().map_err(|e| format!("{e}"))?;
                let z: f64 = parts.next().ok_or("v z")?.parse().map_err(|e| format!("{e}"))?;
                let w = parts.next().map(str::parse::<f64>).transpose().map_err(|e| format!("{e}"))?;
                vertices.push(ObjVertex { x, y, z, w });
            }
            Some("vt") => {
                let u: f64 = parts.next().ok_or("vt u")?.parse().map_err(|e| format!("{e}"))?;
                let v: f64 = parts.next().unwrap_or("0").parse().map_err(|e| format!("{e}"))?;
                let w = parts.next().map(str::parse::<f64>).transpose().map_err(|e| format!("{e}"))?;
                texcoords.push(ObjTexCoord { u, v, w });
            }
            Some("vn") => {
                let x: f64 = parts.next().ok_or("vn x")?.parse().map_err(|e| format!("{e}"))?;
                let y: f64 = parts.next().ok_or("vn y")?.parse().map_err(|e| format!("{e}"))?;
                let z: f64 = parts.next().ok_or("vn z")?.parse().map_err(|e| format!("{e}"))?;
                normals.push(ObjNormal { x, y, z });
            }
            Some("f") => {
                let mut face_vertices = Vec::new();
                for token in parts {
                    face_vertices.push(parse_face_vertex(token, vertices.len(), texcoords.len(), normals.len())?);
                }
                if face_vertices.len() < 3 {
                    return Err(format!("face has fewer than 3 vertices: {line}"));
                }
                let face_index = faces.len();
                faces.push(ObjFace { vertices: face_vertices });
                for name in &cur_active_groups {
                    let gi = if let Some(&gi) = group_index.get(name) {
                        gi
                    } else {
                        groups.push(ObjGroup { name: name.clone(), faces: Vec::new() });
                        let gi = groups.len() - 1;
                        group_index.insert(name.clone(), gi);
                        gi
                    };
                    groups[gi].faces.push(face_index);
                }
                if let Some(name) = &cur_active_object {
                    let oi = if let Some(&oi) = object_index.get(name) {
                        oi
                    } else {
                        objects.push(ObjObject { name: name.clone(), faces: Vec::new() });
                        let oi = objects.len() - 1;
                        object_index.insert(name.clone(), oi);
                        oi
                    };
                    objects[oi].faces.push(face_index);
                }
            }
            Some("o") => {
                cur_active_object = parts.next().map(|s| s.to_string());
            }
            Some("g") => {
                cur_active_groups = parts.map(|s| s.to_string()).collect();
            }
            Some("usemtl") => {
                let name = parts.next().map(|s| s.to_string());
                if name != cur_material {
                    if let Some(m) = &name {
                        usemtl.push(ObjUsemtlRange { face_index_from: faces.len(), material: m.clone() });
                    }
                    cur_material = name;
                }
            }
            Some("mtllib") => {
                let rest: Vec<&str> = parts.collect();
                if !rest.is_empty() {
                    mtllib = Some(rest.join(" "));
                }
            }
            Some("s") => {
                let next = match parts.next() {
                    Some("off") | None => None,
                    Some(v) => v.parse::<u32>().ok(),
                };
                if !have_smoothing || next != cur_smoothing {
                    smoothing_groups.push(ObjSmoothingRange { face_index_from: faces.len(), group: next });
                    cur_smoothing = next;
                    have_smoothing = true;
                }
            }
            _ => {
                unknown_statements.push(ObjUnknownStatement { line_index, raw: line.to_string() });
            }
        }
    }

    Ok(ObjSnapshot { schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(), vertices, texcoords, normals, faces, groups, objects, mtllib, usemtl, smoothing_groups, unknown_statements })
}
//#endregion 🔖️Decode

//#region 🔖️Encode
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_face_vertex(out: &mut String, fv: &ObjFaceVertex) {
    match (fv.texcoord, fv.normal) {
        (Some(vt), Some(vn)) => out.push_str(&format!("{}/{}/{}", fv.vertex + 1, vt + 1, vn + 1)),
        (Some(vt), None) => out.push_str(&format!("{}/{}", fv.vertex + 1, vt + 1)),
        (None, Some(vn)) => out.push_str(&format!("{}//{}", fv.vertex + 1, vn + 1)),
        (None, None) => out.push_str(&format!("{}", fv.vertex + 1)),
    }
}

/// 📤 Writes a real Wavefront OBJ 3.0 text body per this module's documented normal form.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_obj(snap: &ObjSnapshot) -> String {
    let mut out = String::new();
    if let Some(lib) = &snap.mtllib {
        out.push_str(&format!("mtllib {lib}\n"));
    }
    for v in &snap.vertices {
        match v.w {
            Some(w) => out.push_str(&format!("v {} {} {} {}\n", v.x, v.y, v.z, w)),
            None => out.push_str(&format!("v {} {} {}\n", v.x, v.y, v.z)),
        }
    }
    for vt in &snap.texcoords {
        match vt.w {
            Some(w) => out.push_str(&format!("vt {} {} {}\n", vt.u, vt.v, w)),
            None => out.push_str(&format!("vt {} {}\n", vt.u, vt.v)),
        }
    }
    for vn in &snap.normals {
        out.push_str(&format!("vn {} {} {}\n", vn.x, vn.y, vn.z));
    }

    let object_at = |i: usize| -> Option<&str> { snap.objects.iter().find(|o| o.faces.contains(&i)).map(|o| o.name.as_str()) };
    let groups_at = |i: usize| -> Vec<&str> { snap.groups.iter().filter(|g| g.faces.contains(&i)).map(|g| g.name.as_str()).collect() };
    let material_at = |i: usize| -> Option<&str> { snap.usemtl.iter().rev().find(|r| r.face_index_from <= i).map(|r| r.material.as_str()) };
    let smoothing_at = |i: usize| -> Option<Option<u32>> { snap.smoothing_groups.iter().rev().find(|r| r.face_index_from <= i).map(|r| r.group) };

    let mut prev_object: Option<&str> = None;
    let mut prev_groups: Vec<&str> = Vec::new();
    let mut prev_material: Option<&str> = None;
    let mut prev_smoothing_emitted = false;
    let mut prev_smoothing: Option<u32> = None;
    let mut started = false;

    for (i, face) in snap.faces.iter().enumerate() {
        let object = object_at(i);
        if !started || object != prev_object {
            if let Some(name) = object {
                out.push_str(&format!("o {name}\n"));
            }
            prev_object = object;
        }
        let groups = groups_at(i);
        if !started || groups != prev_groups {
            if groups.is_empty() {
                if !prev_groups.is_empty() {
                    out.push_str("g\n");
                }
            } else {
                out.push_str(&format!("g {}\n", groups.join(" ")));
            }
            prev_groups = groups;
        }
        let material = material_at(i);
        if !started || material != prev_material {
            if let Some(m) = material {
                out.push_str(&format!("usemtl {m}\n"));
            }
            prev_material = material;
        }
        if let Some(s) = smoothing_at(i) {
            if !prev_smoothing_emitted || s != prev_smoothing {
                match s {
                    Some(n) => out.push_str(&format!("s {n}\n")),
                    None => out.push_str("s off\n"),
                }
                prev_smoothing = s;
                prev_smoothing_emitted = true;
            }
        }
        started = true;

        out.push('f');
        for fv in &face.vertices {
            out.push(' ');
            write_face_vertex(&mut out, fv);
        }
        out.push('\n');
    }

    for u in &snap.unknown_statements {
        out.push_str(&u.raw);
        out.push('\n');
    }
    out
}
//#endregion 🔖️Encode
//#endregion 🔖️Codec

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::obj::standards::v3_0::subsets::any::schema::ObjAnalyzer;
    use crate::artifacts::obj::ObjSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct ObjComposerComposition;

    impl ArtifactComposition for ObjComposerComposition {
        type Snapshot = ObjSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "ObjComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = ObjAnalyzer::analyze(&native).await;
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "ObjComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_matches_schema() {
        let snapshot = crate::artifacts::obj::engine::empty_obj_snapshot();
        assert_eq!(snapshot.schema, STDIO_OBJ_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn codec_round_trip() {
        let snap = crate::artifacts::obj::engine::empty_obj_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <ObjSnapshot as store::ArtifactDsl>::parse_dsl(&text).await.expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <ObjSnapshot as store::ArtifactPack>::decode_pack(&bytes).await.expect("decode");
        assert_eq!(decoded, snap);
    }

    #[semio_framework_async_macros::async_test]
    async fn negative_indices_resolve_correctly() {
        let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf -3 -2 -1\n";
        let snap = decode_obj(text).expect("parse");
        assert_eq!(snap.vertices.len(), 3);
        let idxs: Vec<u32> = snap.faces[0].vertices.iter().map(|fv| fv.vertex).collect();
        assert_eq!(idxs, vec![0, 1, 2]);
    }

    #[semio_framework_async_macros::async_test]
    async fn out_of_range_negative_index_is_error() {
        let text = "v 0 0 0\nf -2 1 1\n";
        let err = decode_obj(text).unwrap_err();
        assert!(err.contains("out of range"), "unexpected error: {err}");
    }

    #[semio_framework_async_macros::async_test]
    async fn face_index_forms_all_supported() {
        let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nvt 0 0\nvt 1 0\nvt 0 1\nvn 0 0 1\n\
                     f 1/1/1 2/2/1 3/3/1\nf 1//1 2//1 3//1\nf 1/1 2/2 3/3\nf 1 2 3\n";
        let snap = decode_obj(text).expect("parse");
        assert_eq!(snap.faces.len(), 4);
        assert_eq!(snap.faces[0].vertices[0].texcoord, Some(0));
        assert_eq!(snap.faces[0].vertices[0].normal, Some(0));
        assert_eq!(snap.faces[1].vertices[0].texcoord, None);
        assert_eq!(snap.faces[1].vertices[0].normal, Some(0));
        assert_eq!(snap.faces[2].vertices[0].texcoord, Some(0));
        assert_eq!(snap.faces[2].vertices[0].normal, None);
        assert_eq!(snap.faces[3].vertices[0].texcoord, None);
        assert_eq!(snap.faces[3].vertices[0].normal, None);
    }

    #[semio_framework_async_macros::async_test]
    async fn multi_group_multi_material_negative_index_round_trip() {
        let text = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nv 0 0 1\nv 1 0 1\n\
                     vt 0 0\nvt 1 0\nvt 1 1\nvn 0 0 1\nvn 0 0 -1\n\
                     o Cube\ng Front\nusemtl Red\ns 1\n\
                     f 1/1/1 2/2/1 3/3/1\n\
                     f -4/-3/-2 -3/-2/-2 1/1/2\n\
                     g Back\nusemtl Blue\ns off\n\
                     f 4 3 2\n\
                     f -6 -5 -4\n";
        let snap = decode_obj(text).expect("parse");
        assert_eq!(snap.vertices.len(), 6);
        assert_eq!(snap.texcoords.len(), 3);
        assert_eq!(snap.normals.len(), 2);
        assert_eq!(snap.faces.len(), 4);

        assert_eq!(snap.objects.len(), 1);
        assert_eq!(snap.objects[0].name, "Cube");
        assert_eq!(snap.objects[0].faces, vec![0, 1, 2, 3]);

        assert_eq!(snap.groups.len(), 2);
        assert_eq!(snap.groups[0], ObjGroup { name: "Front".into(), faces: vec![0, 1] });
        assert_eq!(snap.groups[1], ObjGroup { name: "Back".into(), faces: vec![2, 3] });

        assert_eq!(snap.usemtl, vec![ObjUsemtlRange { face_index_from: 0, material: "Red".into() }, ObjUsemtlRange { face_index_from: 2, material: "Blue".into() },]);
        assert_eq!(snap.smoothing_groups, vec![ObjSmoothingRange { face_index_from: 0, group: Some(1) }, ObjSmoothingRange { face_index_from: 2, group: None },]);

        assert_eq!(snap.faces[1].vertices[0].vertex, 2);
        assert_eq!(snap.faces[1].vertices[0].texcoord, Some(0));
        assert_eq!(snap.faces[1].vertices[0].normal, Some(0));
        assert_eq!(snap.faces[3].vertices[0].vertex, 0);

        let text2 = encode_obj(&snap);
        let snap2 = decode_obj(&text2).expect("re-parse");
        assert_eq!(snap2, snap, "round trip through encode/decode must be lossless");
    }

    #[semio_framework_async_macros::async_test]
    async fn optional_w_components_retained() {
        let text = "v 0 0 0 1.5\nv 1 0 0\nvt 0.1 0.2 0.3\nvt 0.5 0.5\nvn 0 0 1\nf 1/1/1 2/2/1 1/1/1\n";
        let snap = decode_obj(text).expect("parse");
        assert_eq!(snap.vertices[0].w, Some(1.5));
        assert_eq!(snap.vertices[1].w, None);
        assert_eq!(snap.texcoords[0].w, Some(0.3));
        assert_eq!(snap.texcoords[1].w, None);
    }

    #[semio_framework_async_macros::async_test]
    async fn mtllib_last_occurrence_wins() {
        let text = "mtllib a.mtl\nv 0 0 0\nv 1 0 0\nv 0 1 0\nmtllib b.mtl c.mtl\nf 1 2 3\n";
        let snap = decode_obj(text).expect("parse");
        assert_eq!(snap.mtllib.as_deref(), Some("b.mtl c.mtl"));
    }

    //#region 🔖️CodecRetentionLaw
    /// 🔁️ decode→encode retains every field (geometry, group/object/usemtl/smoothing
    /// membership, mtllib, and unknown-statement content+relative order); per this module's
    /// documented normal form, `unknown_statements[].line_index` is renumbered on re-encode
    /// (comments/unrecognized lines move into a trailer), so from the SECOND generation onward
    /// decode/encode is a true fixed point.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let fixture = "# leading comment\nmtllib materials.mtl\n\
                        v 0 0 0\nv 1 0 0\nv 0 1 0 1\n\
                        vt 0 0\nvt 1 0 0.5\nvn 0 0 1\n\
                        o Cube\ng Front\nusemtl Red\ns 1\n\
                        f 1/1/1 2/2/1 3/1/1\n\
                        # trailing comment\nweird_directive foo bar\n";
        let snap1 = decode_obj(fixture).expect("decode");
        assert_eq!(snap1.mtllib.as_deref(), Some("materials.mtl"));
        assert_eq!(snap1.vertices[2].w, Some(1.0));
        assert_eq!(snap1.texcoords[1].w, Some(0.5));
        assert_eq!(snap1.unknown_statements.len(), 3, "leading comment + trailing comment + weird_directive");

        let text2 = encode_obj(&snap1);
        let snap2 = decode_obj(&text2).expect("re-decode");

        assert_eq!(snap1.vertices, snap2.vertices);
        assert_eq!(snap1.texcoords, snap2.texcoords);
        assert_eq!(snap1.normals, snap2.normals);
        assert_eq!(snap1.faces, snap2.faces);
        assert_eq!(snap1.groups, snap2.groups);
        assert_eq!(snap1.objects, snap2.objects);
        assert_eq!(snap1.mtllib, snap2.mtllib);
        assert_eq!(snap1.usemtl, snap2.usemtl);
        assert_eq!(snap1.smoothing_groups, snap2.smoothing_groups);
        assert_eq!(snap1.unknown_statements.iter().map(|u| u.raw.clone()).collect::<Vec<_>>(), snap2.unknown_statements.iter().map(|u| u.raw.clone()).collect::<Vec<_>>(), "unknown-statement content and relative order must be retained");

        // 🔁 second-generation stability: a true fixed point from here on.
        let text3 = encode_obj(&snap2);
        let snap3 = decode_obj(&text3).expect("re-decode 2");
        assert_eq!(snap2, snap3, "decode/encode must be a fixed point from the second generation onward");
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG1: per-artifact conformance laws (recipe §4 item 6) — grammar/protocol
    /// parseability, `Recognizer` against real fixtures AND real `print_op`/`print_diff` output,
    /// `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, and the
    /// fixture-honesty round-trip. Dissolved out of `⚙️engine`'s own test region (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — same convention every prior pilot
    /// wave (json/csv/zip/png/txt/binary) established.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::obj::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
        /// message).
        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
        /// for the demo mesh — same preamble-stripped body reconstruction
        /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses, so this is a
        /// direct proof this artifact will pass that harness once graduated, not merely an
        /// analogue.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&crate::artifacts::obj::engine::demo_obj_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every representative `ObjMutation` variant (`mutations::demo_mutation_cases()`),
        /// including `SetSnapshot`'s whole nested `ObjSnapshot` tree, precisely field-by-field
        /// (this artifact's own leaf collections are all flat records, no `REST` fallback
        /// needed).
        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff`
        /// output for every representative `ObjDiff` (`diff::demo_diff_cases()`), incl. the empty
        /// diff and a two-directional `between()` result exercising every index-/name-keyed
        /// collection triple and both tri-states.
        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// snapshot pack (`encode_pack`, envelope-unwrapped, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff`. All three facets are
        /// plain `framing record` payloads (no `backward`/`jump`), so the ordinary
        /// `consumed == bytes.len()` law holds for all of them.
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&crate::artifacts::obj::engine::demo_obj_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().await.unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().await.unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_obj_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again.
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = crate::artifacts::obj::engine::demo_obj_snapshot();

            let parsed = <ObjSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).await.expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_obj_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_obj_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <ObjSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).await.expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_obj_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_obj_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::artifacts::obj::standards::v3_0::subsets::any::schema::ObjComposer as ObjRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<ObjRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
