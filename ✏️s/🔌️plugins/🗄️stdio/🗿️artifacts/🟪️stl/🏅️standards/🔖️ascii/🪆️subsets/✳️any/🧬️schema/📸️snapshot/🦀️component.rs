//! 🧬️ StlSnapshot schema — persistent fields + real codecs.

use crate::artifacts::stl::STDIO_STL_DOCUMENT_SCHEMA;
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
/// 📸️ Persisted `stdio.stl` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.stl")]
pub struct StlSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<MeshVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub faces: Vec<MeshTriangle>,
}

impl Default for StlSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_STL_DOCUMENT_SCHEMA.into(),
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



pub fn parse_stl_ascii(text: &str) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut tri: [Option<MeshVertex>; 3] = [None, None, None];
    let mut slot = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("vertex ") {
            let coords: Vec<f32> = line
                .trim_start_matches("vertex")
                .split_whitespace()
                .map(|s| s.parse::<f32>().map_err(|e: std::num::ParseFloatError| e.to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            if coords.len() < 3 {
                return Err("vertex coords".into());
            }
            let v = MeshVertex { x: coords[0], y: coords[1], z: coords[2] };
            tri[slot] = Some(v.clone());
            slot += 1;
            if slot == 3 {
                let i0 = vertices.len() as u32;
                for v in tri.iter().flatten() {
                    vertices.push(v.clone());
                }
                faces.push(MeshTriangle { i0, i1: i0 + 1, i2: i0 + 2 });
                tri = [None, None, None];
                slot = 0;
            }
        }
    }
    Ok((vertices, faces))
}

pub fn write_stl_ascii(vertices: &[MeshVertex], faces: &[MeshTriangle]) -> String {
    let mut out = String::from("solid mesh\n");
    for f in faces {
        let a = &vertices[f.i0 as usize];
        let b = &vertices[f.i1 as usize];
        let c = &vertices[f.i2 as usize];
        let ux = b.x - a.x;
        let uy = b.y - a.y;
        let uz = b.z - a.z;
        let vx = c.x - a.x;
        let vy = c.y - a.y;
        let vz = c.z - a.z;
        let nx = uy * vz - uz * vy;
        let ny = uz * vx - ux * vz;
        let nz = ux * vy - uy * vx;
        out.push_str(&format!("  facet normal {} {} {}\n", nx, ny, nz));
        out.push_str("    outer loop\n");
        out.push_str(&format!("      vertex {} {} {}\n", a.x, a.y, a.z));
        out.push_str(&format!("      vertex {} {} {}\n", b.x, b.y, b.z));
        out.push_str(&format!("      vertex {} {} {}\n", c.x, c.y, c.z));
        out.push_str("    endloop\n  endfacet\n");
    }
    out.push_str("endsolid mesh\n");
    out
}

pub fn parse_stl_binary(bytes: &[u8]) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    if bytes.len() < 84 {
        return Err("stl binary too short".into());
    }
    let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut off = 84usize;
    for _ in 0..count {
        if off + 50 > bytes.len() {
            return Err("stl binary truncated".into());
        }
        off += 12;
        let mut tri_verts = Vec::new();
        for _ in 0..3 {
            let x = f32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
            let y = f32::from_le_bytes(bytes[off + 4..off + 8].try_into().unwrap());
            let z = f32::from_le_bytes(bytes[off + 8..off + 12].try_into().unwrap());
            off += 12;
            let i = vertices.len() as u32;
            vertices.push(MeshVertex { x, y, z });
            tri_verts.push(i);
        }
        off += 2;
        faces.push(MeshTriangle { i0: tri_verts[0], i1: tri_verts[1], i2: tri_verts[2] });
    }
    Ok((vertices, faces))
}

pub fn write_stl_binary(vertices: &[MeshVertex], faces: &[MeshTriangle]) -> Vec<u8> {
    let mut out = vec![0u8; 84];
    out.extend_from_slice(&(faces.len() as u32).to_le_bytes());
    for f in faces {
        let a = &vertices[f.i0 as usize];
        let b = &vertices[f.i1 as usize];
        let c = &vertices[f.i2 as usize];
        let ux = b.x - a.x;
        let uy = b.y - a.y;
        let uz = b.z - a.z;
        let vx = c.x - a.x;
        let vy = c.y - a.y;
        let vz = c.z - a.z;
        let nx = uy * vz - uz * vy;
        let ny = uz * vx - ux * vz;
        let nz = ux * vy - uy * vx;
        out.extend_from_slice(&nx.to_le_bytes());
        out.extend_from_slice(&ny.to_le_bytes());
        out.extend_from_slice(&nz.to_le_bytes());
        for v in [a, b, c] {
            out.extend_from_slice(&v.x.to_le_bytes());
            out.extend_from_slice(&v.y.to_le_bytes());
            out.extend_from_slice(&v.z.to_le_bytes());
        }
        out.extend_from_slice(&0u16.to_le_bytes());
    }
    out
}

pub fn parse_stl_text(text: &str) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    parse_stl_ascii(text)
}

pub fn write_stl_text(vertices: &[MeshVertex], faces: &[MeshTriangle]) -> String {
    write_stl_ascii(vertices, faces)
}



pub fn parse_stl_bytes(bytes: &[u8]) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    if bytes.len() >= 5 && &bytes[0..5] == b"solid".as_ref() {
        parse_stl_ascii(std::str::from_utf8(bytes).map_err(|e| e.to_string())?)
    } else {
        parse_stl_binary(bytes)
    }
}

//#endregion 🔖️FormatCodec

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for StlSnapshot {
    const EXTENSION: &'static str = "stl";
    fn envelope_id() -> &'static str { "stdio.stl" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let (vertices, faces) = parse_stl_text(body).map_err(|e| {
            store::TextError::new(format!("stl parse: {e}"), dsl::TextSpan::at(1, 1))
        })?;
        Ok(Self { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), vertices, faces })
    }
    fn print_dsl(&self) -> String {
        let body = write_stl_text(&self.vertices, &self.faces);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for StlSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_stl_text(&self.vertices, &self.faces).into_bytes();
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
        let (vertices, faces) = parse_stl_text(&text).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), vertices, faces })
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
