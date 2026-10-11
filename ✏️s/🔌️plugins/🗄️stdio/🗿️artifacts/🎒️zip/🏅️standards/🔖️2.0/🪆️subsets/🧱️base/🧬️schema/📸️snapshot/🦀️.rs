//! 🧬️ ZIP snapshot retains typed member state independently of native archive encoding.

use crate::STDIO_ZIP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

//#region EntryMetadata
/// 🧩 An authored ZIP extra-field record. Derived fields use an empty payload marker: `0x0001`
/// is regenerated from sizes and offsets, while `0x7075` and `0x6375` are regenerated from the
/// current Unicode text and the matching explicit legacy bytes.
#[derive(Clone, Debug, semio_framework_value::RetireOwned, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, value_derive::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct ZipExtraField {
    pub id: u16,
    #[value(default)]
    #[dsl(base64)]
    pub data: Vec<u8>,
}

/// 📍 Local-file-header state whose values are not derived from payload size, CRC, or offset.
#[derive(Clone, Debug, semio_framework_value::RetireOwned, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, value_derive::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct ZipLocalHeaderMetadata {
    pub version_needed: u16,
    pub flags: u16,
    pub modified_time: u16,
    pub modified_date: u16,
    #[value(default)]
    pub extra_fields: Vec<ZipExtraField>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unicode_path_legacy_name: Option<Vec<u8>>,
}

impl Default for ZipLocalHeaderMetadata {
    fn default() -> Self {
        Self { version_needed: 20, flags: 0x0800, modified_time: 0, modified_date: 0x0021, extra_fields: Vec::new(), unicode_path_legacy_name: None }
    }
}

/// 📒 Central-directory-header state whose values are not derived from payload size, CRC, or offset.
#[derive(Clone, Debug, semio_framework_value::RetireOwned, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, value_derive::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct ZipCentralHeaderMetadata {
    pub version_made_by: u16,
    pub version_needed: u16,
    pub flags: u16,
    pub modified_time: u16,
    pub modified_date: u16,
    #[value(default)]
    pub extra_fields: Vec<ZipExtraField>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unicode_path_legacy_name: Option<Vec<u8>>,
    pub comment: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unicode_comment_legacy: Option<Vec<u8>>,
    pub internal_attributes: u16,
    pub external_attributes: u32,
}

impl Default for ZipCentralHeaderMetadata {
    fn default() -> Self {
        Self {
            version_made_by: 45,
            version_needed: 20,
            flags: 0x0800,
            modified_time: 0,
            modified_date: 0x0021,
            extra_fields: Vec::new(),
            unicode_path_legacy_name: None,
            comment: String::new(),
            unicode_comment_legacy: None,
            internal_attributes: 0,
            external_attributes: 0,
        }
    }
}

/// 🎛️ Complete persisted ZIP member serialization policy.
#[derive(Clone, Debug, semio_framework_value::RetireOwned, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, value_derive::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct ZipEntryMetadata {
    pub compression_method: u16,
    pub local: ZipLocalHeaderMetadata,
    pub central: ZipCentralHeaderMetadata,
    pub data_descriptor_signature: bool,
}

impl Default for ZipEntryMetadata {
    fn default() -> Self {
        Self { compression_method: 8, local: ZipLocalHeaderMetadata::default(), central: ZipCentralHeaderMetadata::default(), data_descriptor_signature: false }
    }
}
//#endregion EntryMetadata

//#region Entry
/// 🎒️ One logical ZIP archive member with complete, stable serialization metadata.
#[derive(Clone, Debug, semio_framework_value::RetireOwned, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, value_derive::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct ZipEntry {
    pub name: String,
    #[value(default)]
    #[dsl(base64)]
    pub data: Vec<u8>,
    #[value(default)]
    pub metadata: ZipEntryMetadata,
}
//#endregion Entry

//#region Snapshot
/// 📸️ Persisted `stdio.zip` snapshot.
#[derive(Clone, Debug, semio_framework_value::RetireOwned, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip")]
pub struct ZipSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub entries: Vec<ZipEntry>,
    /// 💬️ Archive-level comment (EOCD comment field).
    #[state(artifact)]
    #[value(default)]
    pub comment: String,
    /// 🔤 EOCD comments have no encoding flag, so their chosen wire encoding is persisted.
    #[state(artifact)]
    #[value(default = "default_true")]
    pub comment_utf8: bool,
}

fn default_true() -> bool {
    true
}

impl Default for ZipSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries: Vec::new(), comment: String::new(), comment_utf8: true }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs




#[cfg(test)]
#[path = "🧪️tests/🔬️shadow/🦀️.rs"]
mod shadow_tests;
//#endregion HandcraftedArtifactCodecs



