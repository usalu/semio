//! 🧬️ ZipSnapshot schema — persistent fields + real ZIP codecs.

use crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Entry
/// 🎒️ One logical ZIP archive member: its path and decompressed semantic payload.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntry {
    pub name: String,
    #[serde(default)]
    #[dsl(base64)]
    pub data: Vec<u8>,
}
//#endregion Entry

//#region Snapshot
/// 📸️ Persisted `stdio.zip` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip")]
pub struct ZipSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub entries: Vec<ZipEntry>,
    /// 💬️ Archive-level comment (EOCD comment field).
    #[state(artifact)]
    #[serde(default)]
    pub comment: String,
}

impl Default for ZipSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries: Vec::new(), comment: String::new() }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for ZipSnapshot {
    const EXTENSION: &'static str = "zip";
    fn envelope_id() -> &'static str {
        "stdio.zip"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: Vec<char> = body.chars().filter(|character| !character.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for pair in hex.chunks_exact(2) {
            let high = pair[0].to_digit(16).ok_or_else(|| store::TextError::new("invalid hex digit", dsl::TextSpan::at(1, 1)))?;
            let low = pair[1].to_digit(16).ok_or_else(|| store::TextError::new("invalid hex digit", dsl::TextSpan::at(1, 1)))?;
            bytes.push(((high << 4) | low) as u8);
        }
        crate::artifacts::zip::standards::v2_0::subsets::any::io::decode_zip(&bytes).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::zip::standards::v2_0::subsets::any::io::encode_zip(self).expect("canonical ZIP encoding");
        let body: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ZipSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::zip::standards::v2_0::subsets::any::io::encode_zip(self).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        crate::artifacts::zip::standards::v2_0::subsets::any::io::decode_zip(&inner).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}

#[cfg(test)]
mod shadow_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn logical_snapshot_and_facets_have_no_shadow_state() {
        let json = format!("{:?}", ZipSnapshot::default());
        for forbidden in ["localExtra", "centralExtra", "physical", "sourceBytes", "nativeArchive", "method", "dosDate", "flags", "versionMadeBy", "internalAttrs", "externalAttrs"] {
            assert!(!json.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
        }
        for facet in [
            include_str!("🟦️component.ts"),
            include_str!("🔗️component.graphql"),
            include_str!("🔣️component.json"),
            include_str!("🛰️component.proto"),
            include_str!("../🔺️diff/🟦️component.ts"),
            include_str!("../🔺️diff/🔗️component.graphql"),
            include_str!("../🔺️diff/🔣️component.json"),
            include_str!("../🔺️diff/🛰️component.proto"),
            include_str!("../🧬️mutations/🟦️component.ts"),
            include_str!("../🧬️mutations/🔗️component.graphql"),
            include_str!("../🧬️mutations/🔣️component.json"),
            include_str!("../🧬️mutations/🛰️component.proto"),
        ] {
            for forbidden in [
                "ZipExtraField",
                "localExtra",
                "centralExtra",
                "local_extra",
                "central_extra",
                "ZipCompressionMethod",
                "SetEntryMethod",
                "SetEntryFlags",
                "setEntryMethod",
                "setEntryFlags",
                "set_entry_method",
                "set_entry_flags",
                "dosDate",
                "dos_date",
                "unixMtime",
                "unix_mtime",
                "versionMadeBy",
                "version_made_by",
                "internalAttrs",
                "internal_attrs",
                "externalAttrs",
                "external_attrs",
                "nativeArchive",
                "sourceBytes",
            ] {
                assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
            }
        }
    }
}
//#endregion HandcraftedArtifactCodecs
