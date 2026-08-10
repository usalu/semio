//! 🧬️ GltfSnapshot schema — persistent fields + real codecs.

use crate::artifacts::gltf::STDIO_GLTF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️MeshModel
/// 📍 Point or mesh vertex.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeshVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
//#endregion 🔖️MeshModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf")]
pub struct GltfSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<MeshVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub document: serde_json::Value,
}

impl Default for GltfSnapshot {
    fn default() -> Self {
        let vertices = Vec::new();
        let document = gltf_value_from_vertices(&vertices);
        Self {
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            vertices,
            document,
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️GltfJsonCodec

fn b64_decode(data: &str) -> Result<Vec<u8>, String> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = Vec::new();
    let mut buf = 0u32;
    let mut bits = 0u32;
    for ch in data.bytes().filter(|&b| b != b'=' && !b.is_ascii_whitespace()) {
        let val = TABLE.iter().position(|&t| t == ch).ok_or("invalid base64")? as u32;
        buf = (buf << 6) | val;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
            buf &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

fn b64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((n >> 18) & 63) as usize] as char);
        out.push(TABLE[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { TABLE[((n >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[(n & 63) as usize] as char } else { '=' });
    }
    out
}

fn gltf_decode_buffer_uri(uri: &str) -> Result<Vec<u8>, String> {
    let Some(data) = uri.strip_prefix("data:application/octet-stream;base64,") else {
        return Err("gltf buffer uri must be embedded base64".into());
    };
    b64_decode(data)
}

pub fn gltf_vertices_from_value(value: &serde_json::Value) -> Result<Vec<MeshVertex>, String> {
    let accessors = value.get("accessors").and_then(|v| v.as_array()).ok_or("missing accessors")?;
    let buffer_views = value.get("bufferViews").and_then(|v| v.as_array()).ok_or("missing bufferViews")?;
    let buffers = value.get("buffers").and_then(|v| v.as_array()).ok_or("missing buffers")?;
    let meshes = value.get("meshes").and_then(|v| v.as_array()).ok_or("missing meshes")?;
    let mut pos_accessor: Option<usize> = None;
    'outer: for mesh in meshes {
        let prims = mesh.get("primitives").and_then(|v| v.as_array()).ok_or("missing primitives")?;
        for prim in prims {
            if let Some(idx) = prim.get("attributes").and_then(|a| a.get("POSITION")).and_then(|v| v.as_u64()) {
                pos_accessor = Some(idx as usize);
                break 'outer;
            }
        }
    }
    let acc_idx = pos_accessor.ok_or("no POSITION accessor")?;
    let acc = accessors.get(acc_idx).ok_or("accessor index")?;
    if acc.get("type").and_then(|v| v.as_str()) != Some("VEC3") {
        return Err("POSITION must be VEC3".into());
    }
    if acc.get("componentType").and_then(|v| v.as_u64()) != Some(5126) {
        return Err("POSITION must be FLOAT".into());
    }
    let bv_idx = acc.get("bufferView").and_then(|v| v.as_u64()).ok_or("bufferView")? as usize;
    let bv = buffer_views.get(bv_idx).ok_or("bufferView idx")?;
    let buf_idx = bv.get("buffer").and_then(|v| v.as_u64()).ok_or("buffer")? as usize;
    let buf = buffers.get(buf_idx).ok_or("buffer idx")?;
    let uri = buf.get("uri").and_then(|v| v.as_str()).ok_or("buffer uri")?;
    let bytes = gltf_decode_buffer_uri(uri)?;
    let byte_offset = bv.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize
        + acc.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let count = acc.get("count").and_then(|v| v.as_u64()).ok_or("count")? as usize;
    let mut verts = Vec::with_capacity(count);
    let mut pos = byte_offset;
    for _ in 0..count {
        if pos + 12 > bytes.len() {
            break;
        }
        let x = f32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as f64;
        let y = f32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap()) as f64;
        let z = f32::from_le_bytes(bytes[pos + 8..pos + 12].try_into().unwrap()) as f64;
        verts.push(MeshVertex { x, y, z });
        pos += 12;
    }
    Ok(verts)
}

pub fn gltf_value_from_vertices(verts: &[MeshVertex]) -> serde_json::Value {
    let mut bin = Vec::with_capacity(verts.len() * 12);
    for v in verts {
        bin.extend_from_slice(&(v.x as f32).to_le_bytes());
        bin.extend_from_slice(&(v.y as f32).to_le_bytes());
        bin.extend_from_slice(&(v.z as f32).to_le_bytes());
    }
    let b64 = b64_encode(&bin);
    let uri = format!("data:application/octet-stream;base64,{b64}");
    serde_json::json!({
        "asset": { "version": "2.0" },
        "buffers": [{ "byteLength": bin.len(), "uri": uri }],
        "bufferViews": [{ "buffer": 0, "byteOffset": 0, "byteLength": bin.len() }],
        "accessors": [{
            "bufferView": 0,
            "componentType": 5126,
            "count": verts.len(),
            "type": "VEC3",
            "max": [1.0, 1.0, 1.0],
            "min": [0.0, 0.0, 0.0]
        }],
        "meshes": [{ "primitives": [{ "attributes": { "POSITION": 0 } }] }]
    })
}


impl store::DocumentDsl for GltfSnapshot {
    const EXTENSION: &'static str = "gltf";
    fn envelope_id() -> &'static str { "stdio.gltf" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let document = serde_json::from_str(body.trim()).map_err(|e| {
            store::TextError::new(format!("gltf json: {e}"), dsl::TextSpan::at(1, 1))
        })?;
        let vertices = gltf_vertices_from_value(&document).map_err(|e| {
            store::TextError::new(e, dsl::TextSpan::at(1, 1))
        })?;
        Ok(Self { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), vertices, document })
    }
    fn print_dsl(&self) -> String {
        let doc = if self.document.is_null() {
            gltf_value_from_vertices(&self.vertices)
        } else {
            self.document.clone()
        };
        let body = serde_json::to_string_pretty(&doc).unwrap_or_else(|_| "{}".into());
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for GltfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let doc = if self.document.is_null() {
            gltf_value_from_vertices(&self.vertices)
        } else {
            self.document.clone()
        };
        let raw = serde_json::to_vec(&doc).map_err(|e| store::PackError::Schema(e.to_string()))?;
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
        let document = serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let vertices = gltf_vertices_from_value(&document).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), vertices, document })
    }
}
//#endregion 🔖️GltfJsonCodec
