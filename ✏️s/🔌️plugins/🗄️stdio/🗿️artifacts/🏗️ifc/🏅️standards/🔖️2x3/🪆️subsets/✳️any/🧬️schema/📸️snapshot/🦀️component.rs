//! 📸️ Ifc2x3Snapshot — the `2x3` standard's OWN typed snapshot (buildingSMART Coordination
//! View 2.0 era, IFC2X3 / ISO-PAS 16739:2005 schema, still ISO 10303-21 Part-21 syntax like
//! `📐️step`/`🔖️4`). Deliberately its own newtype (NOT a `pub use` of
//! `step::engine::part21::Part21Document`, and not the same Rust type as `4`'s `IfcSnapshot`) —
//! W1's own recon (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/STATUS.md`,
//! "shared-type violation" entry) flags reusing a cross-artifact type's IDENTITY as the exact
//! anti-pattern this repo bans ("copy-pasted shared types... die"). Reuse here is scoped to
//! PARSING CODE ONLY: this struct wraps a `Part21Document` as an internal field and the codec
//! below calls straight into `step::engine::part21::{parse_part21, write_part21}` — the tokenizer
//! itself is genuinely shared (IFC2X3 is STEP Part-21 syntax + a different EXPRESS schema), but
//! `Ifc2x3Snapshot` the TYPE is this standard's own.

use crate::artifacts::step::engine::part21::Part21Document;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id — distinct from `4`'s `"stdio.ifc"` so the two
/// standards' document codecs never collide in the shared `store::document_codec_registry`.
pub const STDIO_IFC2X3_DOCUMENT_SCHEMA: &str = "stdio.ifc.2x3";
/// 🧬️ Artifact schema descriptor id — distinct from `4`'s `"s.stdio.ifc"`.
pub const IFC2X3_ARTIFACT_SCHEMA_ID: &str = "s.stdio.ifc.2x3";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.ifc.2x3` snapshot — the full, lossless generic Part-21 graph, own type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3")]
pub struct Ifc2x3Snapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub document: Part21Document,
}

impl Default for Ifc2x3Snapshot {
    fn default() -> Self {
        Self { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document: Part21Document::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Codec
impl store::ArtifactDsl for Ifc2x3Snapshot {
    const EXTENSION: &'static str = "ifc";
    fn envelope_id() -> &'static str { STDIO_IFC2X3_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        crate::artifacts::ifc::standards::v2x3::engine::decode_ifc2x3(body.as_bytes())
            .map_err(|e| store::TextError::new(format!("ifc2x3 parse: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(self).unwrap_or_default();
        let body = String::from_utf8(bytes).unwrap_or_default();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Ifc2x3Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(self).map_err(store::PackError::Schema)?;
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
        crate::artifacts::ifc::standards::v2x3::engine::decode_ifc2x3(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️Codec
