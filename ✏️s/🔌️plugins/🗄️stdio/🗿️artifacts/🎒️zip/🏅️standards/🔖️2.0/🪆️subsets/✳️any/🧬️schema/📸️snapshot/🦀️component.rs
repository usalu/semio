//! 🧬️ ZipSnapshot schema — persistent fields + real ZIP codecs.

use crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Entry
/// 🎒️ One ZIP archive member (uncompressed payload).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntry {
    pub name: String,
    #[serde(default)]
    pub data: Vec<u8>,
}
//#endregion Entry

//#region Snapshot
/// 📸️ Persisted `stdio.zip` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip")]
pub struct ZipSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub entries: Vec<ZipEntry>,
}

impl Default for ZipSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: Vec::new(),
        }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for ZipSnapshot {
    const EXTENSION: &'static str = "zip";
    fn envelope_id() -> &'static str { "stdio.zip" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        // hex of zip bytes
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
        crate::artifacts::zip::engine::decode_zip(&bytes).map_err(|e| {
            store::TextError::new(e, dsl::TextSpan::at(1, 1))
        })
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::zip::engine::encode_zip(self, true).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ZipSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::zip::engine::encode_zip(self, true)
            .map_err(|e| store::PackError::Schema(e))?;
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
        crate::artifacts::zip::engine::decode_zip(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion HandcraftedArtifactCodecs
