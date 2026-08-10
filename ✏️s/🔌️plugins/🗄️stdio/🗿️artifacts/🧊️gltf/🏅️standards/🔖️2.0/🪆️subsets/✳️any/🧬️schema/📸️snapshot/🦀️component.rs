//! 🧬️ GltfSnapshot schema — persistent fields. Byte/container codecs (base64, accessor decode,
//! `.gltf`/`.glb` parse+serialize) live in `🏅️standards/🔖️2.0/⚙️engine` (ticket
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D2 gltf/glb merge step 1) — this
//! file only owns the persisted shape and its `ArtifactDsl`/`ArtifactPack` envelope glue.

use crate::artifacts::gltf::STDIO_GLTF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️SourceForm
/// 🧵 Which wire dialect a snapshot was last parsed from -- drives [`serialize_gltf_document`]'s
/// choice of whether a no-`uri` buffer needs re-embedding as a data uri (a `.glb`-sourced buffer
/// serialized back out as plain `.gltf` JSON text has no BIN chunk to lean on).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum GltfSourceForm {
    #[default]
    Json,
    Glb,
}
//#endregion 🔖️SourceForm

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.gltf` snapshot: the full glTF 2.0 JSON document verbatim (`asset`,
/// `scenes`, `nodes`, `meshes`, `accessors`, `bufferViews`, `buffers`, `materials`, `textures`,
/// `images`, `samplers`, `skins`, `animations`, `cameras`, `extensions`, `extras`, and any
/// unmodeled field -- nothing is dropped), plus `buffers`: the resolved raw bytes for each
/// `document.buffers[i]` (index-aligned), since a `.glb`-sourced buffer may have no `uri` at all
/// and its bytes must live somewhere other than the JSON.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf")]
pub struct GltfSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub document: serde_json::Value,
    #[state(persistent)]
    #[serde(default)]
    pub buffers: Vec<Vec<u8>>,
    #[state(persistent)]
    #[serde(default)]
    pub source_form: GltfSourceForm,
}

impl Default for GltfSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            document: serde_json::json!({ "asset": { "version": "2.0" } }),
            buffers: Vec::new(),
            source_form: GltfSourceForm::Json,
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for GltfSnapshot {
    const EXTENSION: &'static str = "gltf";
    fn envelope_id() -> &'static str { "stdio.gltf" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        crate::artifacts::gltf::engine::parse_gltf_document(body.trim().as_bytes())
            .map_err(|e| store::TextError::new(format!("gltf json: {e}"), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body_bytes = crate::artifacts::gltf::engine::serialize_gltf_document(self);
        let body = String::from_utf8(body_bytes).unwrap_or_else(|_| "{}".into());
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for GltfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::gltf::engine::serialize_gltf_document(self);
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
        crate::artifacts::gltf::engine::parse_gltf_document(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
