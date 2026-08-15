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
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 64 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ZipSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let raw = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
}

#[cfg(test)]
mod shadow_tests {
    use super::*;

    #[test]
    fn logical_snapshot_and_facets_have_no_shadow_state() {
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
