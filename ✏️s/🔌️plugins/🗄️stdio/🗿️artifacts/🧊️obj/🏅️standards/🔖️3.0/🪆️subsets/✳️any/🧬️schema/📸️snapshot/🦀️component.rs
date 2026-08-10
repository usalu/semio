//! 🧬️ ObjSnapshot schema — persistent fields; real byte codec lives in `⚙️engine`.

use crate::artifacts::obj::STDIO_OBJ_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️MeshModel
/// 📍 A `v` position line.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// 🧵 A `vt` texture-coordinate line.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjTexCoord {
    pub u: f32,
    pub v: f32,
}

/// 📐 A `vn` normal line.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjNormal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// 🔗 One `v[/vt][/vn]` reference inside an `f` line (0-based, negative indices already resolved).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjFaceVertex {
    pub vertex: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texcoord: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal: Option<u32>,
}

/// 🧩 A `f` line (kept as its original n-gon, not eagerly triangulated), tagged with the
/// `o`/`g`/`usemtl`/`s` state active when it was parsed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjFace {
    pub vertices: Vec<ObjFaceVertex>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smoothing_group: Option<u32>,
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
    pub vertices: Vec<ObjVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub texcoords: Vec<ObjTexCoord>,
    #[state(persistent)]
    #[serde(default)]
    pub normals: Vec<ObjNormal>,
    #[state(persistent)]
    #[serde(default)]
    pub faces: Vec<ObjFace>,
}

impl Default for ObjSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(),
            vertices: Vec::new(),
            texcoords: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🔗 Real grammar lives in `⚙️engine::encode_obj`/`decode_obj` — see
// https://www.fileformat.info/format/wavefrontobj/egff.htm for the grammar this mirrors.
impl store::ArtifactDsl for ObjSnapshot {
    const EXTENSION: &'static str = "obj";
    fn envelope_id() -> &'static str { "stdio.obj" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        crate::artifacts::obj::engine::decode_obj(body)
            .map_err(|e| store::TextError::new(format!("obj parse: {e}"), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = crate::artifacts::obj::engine::encode_obj(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ObjSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::obj::engine::encode_obj(self).into_bytes();
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
        crate::artifacts::obj::engine::decode_obj(&text).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
