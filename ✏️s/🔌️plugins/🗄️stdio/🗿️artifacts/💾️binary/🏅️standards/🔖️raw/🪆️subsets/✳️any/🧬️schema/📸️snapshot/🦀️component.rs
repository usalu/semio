//! 🧬️ BinarySnapshot schema — persistent fields + real codecs.

use crate::artifacts::binary::STDIO_BINARY_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.binary` snapshot.
///
/// 🧪️ F6-PILOT: `dsl::DslRecord` added alongside the existing hand-rolled `store::ArtifactDsl`/
/// `store::ArtifactPack` below — NOT a replacement. `DslRecord` only gives this type `DslField`
/// (so it can be embedded as a variant payload, e.g. `BinaryMutation::SetSnapshot{snapshot}`),
/// it does not touch the artifact's own honest hex-text/raw-binary envelope format.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.binary")]
pub struct BinarySnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    #[dsl(base64)]
    pub bytes: Vec<u8>,
}

impl Default for BinarySnapshot {
    async fn default() -> Self {
        Self { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for BinarySnapshot {
    const EXTENSION: &'static str = "bin";
    async fn envelope_id() -> &'static str {
        "stdio.binary"
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
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
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        Ok(Self { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
    }
    async fn print_dsl(&self) -> String {
        let body: String = self.bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🧬️ CARRIER LAW (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3):
/// `s.stdio.binary@raw/*` is `CARRIER_BINARY` — its native `Binary` `IoPayload` IS the raw
/// external file content, byte-for-byte. The previous impl wrapped `self.bytes` in a
/// `SemioEnvelope` (`BINARY_MAGIC` header + token), which made every exported `.bin` file an
/// unopenable `.semio` pack container instead of the honest raw bytes — exactly the
/// `registry_export_media` class of bug the ticket exists to remove. Fixed here (the codec, not
/// the test): `encode_pack_with`/`decode_pack_with` are now the identity function on `bytes`.
/// Proven by `carrier_native_is_raw` in `🚪️io/🦀️component.rs`.
impl store::ArtifactPack for BinarySnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        Ok(self.bytes.clone())
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let _ = options;
        Ok(Self { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: bytes.to_vec() })
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
