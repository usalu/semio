//! 🧬️ ObjSnapshot schema — persistent fields + real codecs.

use crate::artifacts::obj::STDIO_OBJ_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️MeshModel

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeshVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeshTriangle {
    pub i0: u32,
    pub i1: u32,
    pub i2: u32,
}

//#endregion 🔖️MeshModel

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.obj` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.obj")]
pub struct ObjSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<MeshVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub faces: Vec<MeshTriangle>,
}

impl Default for ObjSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(),
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️FormatCodec

fn parse_face_indices(token: &str) -> Result<Vec<u32>, String> {
    let mut out = Vec::new();
    for part in token.split('/') {
        let idx = part.trim();
        if idx.is_empty() {
            continue;
        }
        let n: i32 = idx.parse::<i32>().map_err(|e| e.to_string())?;
        let u = if n > 0 { n as u32 - 1 } else { 0 };
        out.push(u);
        break;
    }
    Ok(out)
}

fn triangulate_face(indices: &[u32]) -> Vec<MeshTriangle> {
    if indices.len() < 3 {
        return Vec::new();
    }
    let mut tris = Vec::new();
    for i in 1..indices.len() - 1 {
        tris.push(MeshTriangle { i0: indices[0], i1: indices[i], i2: indices[i + 1] });
    }
    tris
}



pub fn parse_obj_text(text: &str) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x: f32 = parts.next().ok_or("v x")?.parse::<f32>().map_err(|e| e.to_string())?;
                let y: f32 = parts.next().ok_or("v y")?.parse::<f32>().map_err(|e| e.to_string())?;
                let z: f32 = parts.next().ok_or("v z")?.parse::<f32>().map_err(|e| e.to_string())?;
                vertices.push(MeshVertex { x, y, z });
            }
            Some("f") => {
                let tokens: Vec<&str> = parts.collect();
                let mut idxs = Vec::new();
                for t in tokens {
                    let mut got = parse_face_indices(t)?;
                    idxs.append(&mut got);
                }
                faces.extend(triangulate_face(&idxs));
            }
            _ => {}
        }
    }
    Ok((vertices, faces))
}

pub fn write_obj_text(vertices: &[MeshVertex], faces: &[MeshTriangle]) -> String {
    let mut out = String::from("# Wavefront OBJ\n");
    for v in vertices {
        out.push_str(&format!("v {} {} {}\n", v.x, v.y, v.z));
    }
    for f in faces {
        out.push_str(&format!("f {} {} {}\n", f.i0 + 1, f.i1 + 1, f.i2 + 1));
    }
    out
}

//#endregion 🔖️FormatCodec

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for ObjSnapshot {
    const EXTENSION: &'static str = "obj";
    fn envelope_id() -> &'static str { "stdio.obj" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let (vertices, faces) = parse_obj_text(body).map_err(|e| {
            store::TextError::new(format!("obj parse: {e}"), dsl::TextSpan::at(1, 1))
        })?;
        Ok(Self { schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(), vertices, faces })
    }
    fn print_dsl(&self) -> String {
        let body = write_obj_text(&self.vertices, &self.faces);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for ObjSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_obj_text(&self.vertices, &self.faces).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let (vertices, faces) = parse_obj_text(&text).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self { schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(), vertices, faces })
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
