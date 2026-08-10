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
// 📌 The real byte-level ply codec (header grammar, ascii + binary_little_endian +
// binary_big_endian body decode/encode) lives in `engine::{encode_ply, decode_ply,
// encode_ply_with_format}` per the png/jpg precedent — these two helpers stay here only
// because `🚪️io/{import,export}/…/txt/utf-8` still calls them by name for the ascii-text
// serializer pair; both simply delegate to the engine's canonical ascii codec.
pub fn parse_ply_text(text: &str) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    let snap = crate::artifacts::ply::engine::decode_ply(text.as_bytes())?;
    Ok((snap.vertices, snap.faces))
}

pub fn write_ply_text(vertices: &[MeshVertex], faces: &[MeshTriangle]) -> String {
    let snap = PlySnapshot { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), vertices: vertices.to_vec(), faces: faces.to_vec() };
    let bytes = crate::artifacts::ply::engine::encode_ply(&snap).unwrap_or_default();
    String::from_utf8(bytes).unwrap_or_default()
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
        crate::artifacts::ply::engine::decode_ply(body.as_bytes())
            .map_err(|e| store::TextError::new(format!("ply parse: {e}"), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::ply::engine::encode_ply(self).unwrap_or_default();
        let body = String::from_utf8(bytes).unwrap_or_default();
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
        let raw = crate::artifacts::ply::engine::encode_ply(self).map_err(|e| store::PackError::Schema(e))?;
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
        crate::artifacts::ply::engine::decode_ply(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
