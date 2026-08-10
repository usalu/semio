//! 🧬️ PlySnapshot schema — persistent fields + real codecs.

use crate::artifacts::ply::STDIO_PLY_DOCUMENT_SCHEMA;
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
/// 📸️ Persisted `stdio.ply` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ply")]
pub struct PlySnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<MeshVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub faces: Vec<MeshTriangle>,
}

impl Default for PlySnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
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



pub fn parse_ply_text(text: &str) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    let mut lines = text.lines().peekable();
    if lines.next().map(|l| l.trim()) != Some("ply") {
        return Err("expected ply header".into());
    }
    let mut fmt = String::new();
    let mut vertex_count = 0usize;
    let mut face_count = 0usize;
    loop {
        let line = lines.next().ok_or("unexpected eof in header")?.trim();
        if line == "end_header" {
            break;
        }
        if let Some(rest) = line.strip_prefix("format ") {
            fmt = rest.to_string();
        } else if let Some(rest) = line.strip_prefix("element vertex ") {
            vertex_count = rest.parse::<usize>().map_err(|e| e.to_string())?;
        } else if let Some(rest) = line.strip_prefix("element face ") {
            face_count = rest.parse::<usize>().map_err(|e| e.to_string())?;
        }
    }
    if !fmt.starts_with("ascii") {
        return Err("only ascii ply supported".into());
    }
    let mut vertices = Vec::with_capacity(vertex_count);
    for _ in 0..vertex_count {
        let line = lines.next().ok_or("vertex eof")?;
        let mut p = line.split_whitespace();
        let x: f32 = p.next().ok_or("x")?.parse::<f32>().map_err(|e| e.to_string())?;
        let y: f32 = p.next().ok_or("y")?.parse::<f32>().map_err(|e| e.to_string())?;
        let z: f32 = p.next().ok_or("z")?.parse::<f32>().map_err(|e| e.to_string())?;
        vertices.push(MeshVertex { x, y, z });
    }
    let mut faces = Vec::new();
    for _ in 0..face_count {
        let line = lines.next().ok_or("face eof")?;
        let mut p = line.split_whitespace();
        let n: usize = p.next().ok_or("n")?.parse::<usize>().map_err(|e| e.to_string())?;
        let mut idxs = Vec::with_capacity(n);
        for _ in 0..n {
            idxs.push(p.next().ok_or("idx")?.parse::<u32>().map_err(|e| e.to_string())?);
        }
        faces.extend(triangulate_face(&idxs));
    }
    Ok((vertices, faces))
}

pub fn write_ply_text(vertices: &[MeshVertex], faces: &[MeshTriangle]) -> String {
    let mut out = String::new();
    out.push_str("ply\nformat ascii 1.0\n");
    out.push_str(&format!("element vertex {}\n", vertices.len()));
    out.push_str("property float x\nproperty float y\nproperty float z\n");
    out.push_str(&format!("element face {}\n", faces.len()));
    out.push_str("property list uchar int vertex_indices\nend_header\n");
    for v in vertices {
        out.push_str(&format!("{} {} {}\n", v.x, v.y, v.z));
    }
    for f in faces {
        out.push_str(&format!("3 {} {} {}\n", f.i0, f.i1, f.i2));
    }
    out
}

//#endregion 🔖️FormatCodec

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for PlySnapshot {
    const EXTENSION: &'static str = "ply";
    fn envelope_id() -> &'static str { "stdio.ply" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let (vertices, faces) = parse_ply_text(body).map_err(|e| {
            store::TextError::new(format!("ply parse: {e}"), dsl::TextSpan::at(1, 1))
        })?;
        Ok(Self { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), vertices, faces })
    }
    fn print_dsl(&self) -> String {
        let body = write_ply_text(&self.vertices, &self.faces);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PlySnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_ply_text(&self.vertices, &self.faces).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let (vertices, faces) = parse_ply_text(&text).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), vertices, faces })
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
