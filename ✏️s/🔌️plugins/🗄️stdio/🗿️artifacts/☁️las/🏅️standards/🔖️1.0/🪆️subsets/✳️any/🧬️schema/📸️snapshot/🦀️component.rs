//! 🧬️ LasSnapshot schema — persistent fields + real codecs.

use crate::artifacts::las::STDIO_LAS_DOCUMENT_SCHEMA;
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
#[artifact_schema(id = "s.stdio.las")]
pub struct LasSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<MeshVertex>,
}

impl Default for LasSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_LAS_DOCUMENT_SCHEMA.into(),
            vertices: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️LasBinaryCodec

pub fn las_vertices_from_bytes(bytes: &[u8]) -> Result<Vec<MeshVertex>, String> {
    if bytes.len() < 227 {
        return Err("las header too short".into());
    }
    if &bytes[0..4] != b"LASF" {
        return Err("las signature missing".into());
    }
    let point_offset = u32::from_le_bytes(bytes[96..100].try_into().map_err(|_| "offset")?) as usize;
    let point_count = u32::from_le_bytes(bytes[107..111].try_into().map_err(|_| "count")?) as usize;
    let point_format = bytes[104];
    let record_len = u16::from_le_bytes(bytes[105..107].try_into().map_err(|_| "rlen")?) as usize;
    if record_len == 0 {
        return Err("las record length zero".into());
    }
    let x_scale = f64::from_le_bytes(bytes[131..139].try_into().map_err(|_| "xs")?);
    let y_scale = f64::from_le_bytes(bytes[139..147].try_into().map_err(|_| "ys")?);
    let z_scale = f64::from_le_bytes(bytes[147..155].try_into().map_err(|_| "zs")?);
    let x_off = f64::from_le_bytes(bytes[155..163].try_into().map_err(|_| "xo")?);
    let y_off = f64::from_le_bytes(bytes[163..171].try_into().map_err(|_| "yo")?);
    let z_off = f64::from_le_bytes(bytes[171..179].try_into().map_err(|_| "zo")?);
    let data_start = if point_offset >= 227 { point_offset } else { 227 };
    if point_format != 0 {
        return Err(format!("unsupported las point format {point_format}"));
    }
    let mut verts = Vec::with_capacity(point_count.min(1_000_000));
    let mut pos = data_start;
    for _ in 0..point_count {
        if pos + 20 > bytes.len() {
            break;
        }
        let xi = i32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap());
        let yi = i32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap());
        let zi = i32::from_le_bytes(bytes[pos + 8..pos + 12].try_into().unwrap());
        verts.push(MeshVertex {
            x: xi as f64 * x_scale + x_off,
            y: yi as f64 * y_scale + y_off,
            z: zi as f64 * z_scale + z_off,
        });
        pos += record_len;
    }
    Ok(verts)
}

pub fn las_bytes_from_vertices(verts: &[MeshVertex]) -> Vec<u8> {
    let header_size = 227usize;
    let record_len = 20u16;
    let count = verts.len() as u32;
    let mut out = vec![0u8; header_size + verts.len() * record_len as usize];
    out[0..4].copy_from_slice(b"LASF");
    out[24..26].copy_from_slice(&1u16.to_le_bytes());
    out[104] = 0;
    out[105..107].copy_from_slice(&record_len.to_le_bytes());
    out[107..111].copy_from_slice(&count.to_le_bytes());
    let x_scale = 0.01f64;
    let y_scale = 0.01f64;
    let z_scale = 0.01f64;
    out[131..139].copy_from_slice(&x_scale.to_le_bytes());
    out[139..147].copy_from_slice(&y_scale.to_le_bytes());
    out[147..155].copy_from_slice(&z_scale.to_le_bytes());
    out[96..100].copy_from_slice(&(header_size as u32).to_le_bytes());
    let mut pos = header_size;
    for v in verts {
        let xi = ((v.x) / x_scale).round() as i32;
        let yi = ((v.y) / y_scale).round() as i32;
        let zi = ((v.z) / z_scale).round() as i32;
        out[pos..pos + 4].copy_from_slice(&xi.to_le_bytes());
        out[pos + 4..pos + 8].copy_from_slice(&yi.to_le_bytes());
        out[pos + 8..pos + 12].copy_from_slice(&zi.to_le_bytes());
        pos += record_len as usize;
    }
    out
}


impl store::ArtifactDsl for LasSnapshot {
    const EXTENSION: &'static str = "las";
    fn envelope_id() -> &'static str { "stdio.las" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1))
            })?;
            bytes.push(byte);
            i += 2;
        }
        let vertices = las_vertices_from_bytes(&bytes).map_err(|e| {
            store::TextError::new(e, dsl::TextSpan::at(1, 1))
        })?;
        Ok(Self { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), vertices })
    }
    fn print_dsl(&self) -> String {
        let body: String = las_bytes_from_vertices(&self.vertices).iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for LasSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = las_bytes_from_vertices(&self.vertices);
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
        let vertices = las_vertices_from_bytes(&inner).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), vertices })
    }
}
//#endregion 🔖️LasBinaryCodec
