//! ⚙️ ObjEngine — owns a real `ObjArtifact` + the real Wavefront OBJ 3.0 text codec.

use crate::artifacts::obj::schema::snapshot::{ObjFace, ObjFaceVertex, ObjNormal, ObjTexCoord, ObjVertex};
use crate::artifacts::obj::{ObjArtifact, ObjDiff, ObjMutation, ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_obj_snapshot() -> ObjSnapshot {
    ObjSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Codec
//#region 🔖️IndexResolution
/// 🔢 Resolves a raw OBJ index (1-based positive, or negative = relative to the
/// current end of the list at parse time) against `current_len` — the OBJ spec's
/// own negative-index rule.
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
fn parse_face_vertex(token: &str, vertex_count: usize, texcoord_count: usize, normal_count: usize) -> Result<ObjFaceVertex, String> {
    let mut parts = token.split('/');
    let v_raw: i64 = parts.next().ok_or("empty face token")?.parse().map_err(|e| format!("face vertex index: {e}"))?;
    let vertex = resolve_index(vertex_count, v_raw)?;
    let vt_raw = parts.next().unwrap_or("");
    let texcoord = if vt_raw.is_empty() {
        None
    } else {
        Some(resolve_index(texcoord_count, vt_raw.parse().map_err(|e| format!("face texcoord index: {e}"))?)?)
    };
    let vn_raw = parts.next().unwrap_or("");
    let normal = if vn_raw.is_empty() {
        None
    } else {
        Some(resolve_index(normal_count, vn_raw.parse().map_err(|e| format!("face normal index: {e}"))?)?)
    };
    Ok(ObjFaceVertex { vertex, texcoord, normal })
}
//#endregion 🔖️IndexResolution

//#region 🔖️Decode
/// 📥 Parses a real Wavefront OBJ text body: `v`/`vt`/`vn`, `f` (v, v/vt, v//vn, v/vt/vn,
/// negative-relative indices), `o`/`g`/`usemtl`/`s`.
pub fn decode_obj(text: &str) -> Result<ObjSnapshot, String> {
    let mut vertices = Vec::new();
    let mut texcoords = Vec::new();
    let mut normals = Vec::new();
    let mut faces = Vec::new();

    let mut cur_object: Option<String> = None;
    let mut cur_group: Option<String> = None;
    let mut cur_material: Option<String> = None;
    let mut cur_smoothing: Option<u32> = None;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x: f32 = parts.next().ok_or("v x")?.parse().map_err(|e| format!("{e}"))?;
                let y: f32 = parts.next().ok_or("v y")?.parse().map_err(|e| format!("{e}"))?;
                let z: f32 = parts.next().ok_or("v z")?.parse().map_err(|e| format!("{e}"))?;
                vertices.push(ObjVertex { x, y, z });
            }
            Some("vt") => {
                let u: f32 = parts.next().ok_or("vt u")?.parse().map_err(|e| format!("{e}"))?;
                let v: f32 = parts.next().unwrap_or("0").parse().map_err(|e| format!("{e}"))?;
                texcoords.push(ObjTexCoord { u, v });
            }
            Some("vn") => {
                let x: f32 = parts.next().ok_or("vn x")?.parse().map_err(|e| format!("{e}"))?;
                let y: f32 = parts.next().ok_or("vn y")?.parse().map_err(|e| format!("{e}"))?;
                let z: f32 = parts.next().ok_or("vn z")?.parse().map_err(|e| format!("{e}"))?;
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
                faces.push(ObjFace {
                    vertices: face_vertices,
                    object: cur_object.clone(),
                    group: cur_group.clone(),
                    material: cur_material.clone(),
                    smoothing_group: cur_smoothing,
                });
            }
            Some("o") => {
                cur_object = parts.next().map(|s| s.to_string());
            }
            Some("g") => {
                let rest: Vec<&str> = parts.collect();
                cur_group = if rest.is_empty() { None } else { Some(rest.join(" ")) };
            }
            Some("usemtl") => {
                cur_material = parts.next().map(|s| s.to_string());
            }
            Some("s") => {
                cur_smoothing = match parts.next() {
                    Some("off") | None => None,
                    Some(v) => v.parse::<u32>().ok(),
                };
            }
            _ => {}
        }
    }

    Ok(ObjSnapshot { schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(), vertices, texcoords, normals, faces })
}
//#endregion 🔖️Decode

//#region 🔖️Encode
fn write_face_vertex(out: &mut String, fv: &ObjFaceVertex) {
    match (fv.texcoord, fv.normal) {
        (Some(vt), Some(vn)) => out.push_str(&format!("{}/{}/{}", fv.vertex + 1, vt + 1, vn + 1)),
        (Some(vt), None) => out.push_str(&format!("{}/{}", fv.vertex + 1, vt + 1)),
        (None, Some(vn)) => out.push_str(&format!("{}//{}", fv.vertex + 1, vn + 1)),
        (None, None) => out.push_str(&format!("{}", fv.vertex + 1)),
    }
}

/// 📤 Writes a real Wavefront OBJ 3.0 text body, re-emitting `o`/`g`/`usemtl`/`s` whenever
/// they change between consecutive faces.
pub fn encode_obj(snap: &ObjSnapshot) -> String {
    let mut out = String::from("# Wavefront OBJ\n");
    for v in &snap.vertices {
        out.push_str(&format!("v {} {} {}\n", v.x, v.y, v.z));
    }
    for vt in &snap.texcoords {
        out.push_str(&format!("vt {} {}\n", vt.u, vt.v));
    }
    for vn in &snap.normals {
        out.push_str(&format!("vn {} {} {}\n", vn.x, vn.y, vn.z));
    }

    let mut cur_object: Option<&str> = None;
    let mut cur_group: Option<&str> = None;
    let mut cur_material: Option<&str> = None;
    let mut cur_smoothing: Option<u32> = None;
    let mut started = false;

    for f in &snap.faces {
        let object = f.object.as_deref();
        let group = f.group.as_deref();
        let material = f.material.as_deref();
        if !started || object != cur_object {
            if let Some(o) = object {
                out.push_str(&format!("o {o}\n"));
            }
            cur_object = object;
        }
        if !started || group != cur_group {
            if let Some(g) = group {
                out.push_str(&format!("g {g}\n"));
            }
            cur_group = group;
        }
        if !started || material != cur_material {
            if let Some(m) = material {
                out.push_str(&format!("usemtl {m}\n"));
            }
            cur_material = material;
        }
        if !started || f.smoothing_group != cur_smoothing {
            match f.smoothing_group {
                Some(s) => out.push_str(&format!("s {s}\n")),
                None => out.push_str("s off\n"),
            }
            cur_smoothing = f.smoothing_group;
        }
        started = true;

        out.push('f');
        for fv in &f.vertices {
            out.push(' ');
            write_face_vertex(&mut out, fv);
        }
        out.push('\n');
    }
    out
}
//#endregion 🔖️Encode
//#endregion 🔖️Codec

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::obj::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<ObjSnapshot, ObjMutation>(STDIO_OBJ_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.obj",
        extension: Some("obj"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::obj::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::obj::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::obj::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::obj::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.obj"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.obj`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::obj::schema::obj_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.obj` artifact engine.
pub struct ObjEngine {
    artifact_state: ObjArtifact,
    snapshot_state: ObjSnapshot,
}

impl ObjEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: ObjSnapshot) -> Self {
        let artifact_state = ObjArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}

impl protocol::ArtifactEngine for ObjEngine {
    type Artifact = ObjArtifact;
    type Snapshot = ObjSnapshot;
    type Mutation = ObjMutation;
    type Diff = ObjDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact_state
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot_state
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_obj_snapshot();
        assert_eq!(snapshot.schema, STDIO_OBJ_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_obj_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <ObjSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <ObjSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    #[test]
    fn negative_indices_resolve_correctly() {
        let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf -3 -2 -1\n";
        let snap = decode_obj(text).expect("parse");
        assert_eq!(snap.vertices.len(), 3);
        let idxs: Vec<u32> = snap.faces[0].vertices.iter().map(|fv| fv.vertex).collect();
        assert_eq!(idxs, vec![0, 1, 2]);
    }

    #[test]
    fn out_of_range_negative_index_is_error() {
        let text = "v 0 0 0\nf -2 1 1\n";
        let err = decode_obj(text).unwrap_err();
        assert!(err.contains("out of range"), "unexpected error: {err}");
    }

    #[test]
    fn face_index_forms_all_supported() {
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

    #[test]
    fn multi_group_multi_material_negative_index_round_trip() {
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
        assert_eq!(snap.faces[0].object.as_deref(), Some("Cube"));
        assert_eq!(snap.faces[0].group.as_deref(), Some("Front"));
        assert_eq!(snap.faces[0].material.as_deref(), Some("Red"));
        assert_eq!(snap.faces[0].smoothing_group, Some(1));
        assert_eq!(snap.faces[1].vertices[0].vertex, 2);
        assert_eq!(snap.faces[1].vertices[0].texcoord, Some(0));
        assert_eq!(snap.faces[1].vertices[0].normal, Some(0));
        assert_eq!(snap.faces[2].group.as_deref(), Some("Back"));
        assert_eq!(snap.faces[2].material.as_deref(), Some("Blue"));
        assert_eq!(snap.faces[2].smoothing_group, None);
        assert_eq!(snap.faces[3].vertices[0].vertex, 0);

        let text2 = encode_obj(&snap);
        let snap2 = decode_obj(&text2).expect("re-parse");
        assert_eq!(snap2, snap, "round trip through encode/decode must be lossless");
    }
}
//#endregion 🧪️Tests
