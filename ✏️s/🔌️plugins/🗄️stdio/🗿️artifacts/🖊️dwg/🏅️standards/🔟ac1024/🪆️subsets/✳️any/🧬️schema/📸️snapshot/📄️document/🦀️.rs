//! 📄️ File-level sections: classes, dependencies, auxiliary header, revision history, preview, summary and application info.

#[path = "🔧️header/🦀️.rs"]
pub mod header;

//#region 🔖️DocumentModel
/// 🔧️ The AC1024 maintenance release (preamble byte 0x12) new documents carry — the one the committed
/// AutoCAD drawing and this codec's AC1024 header layout were verified against.
pub const DWG_NEW_DOCUMENT_MAINTENANCE_VERSION: u8 = 2;
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgClass {
    pub number: u16,
    pub proxy_flags: u32,
    pub application_name: String,
    pub cpp_class_name: String,
    pub dxf_name: String,
    #[value(default)]
    pub was_zombie: bool,
    #[value(default)]
    pub item_class_id: u16,
    #[value(default)]
    pub object_count: u32,
    #[value(default)]
    pub dwg_version: u32,
    #[value(default)]
    pub maintenance_version: u32,
    #[value(default)]
    pub reserved_values: Vec<u32>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDependency {
    pub feature: String,
    pub full_path: String,
    pub relative_path: String,
    pub fingerprint: String,
    pub version: String,
    #[value(default)]
    pub timestamp: u32,
    #[value(default)]
    pub file_size: u32,
    #[value(default)]
    pub affects_graphics: bool,
    #[value(default)]
    pub reference_count: u32,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgJulianDate {
    pub days: u32,
    pub milliseconds: u32,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgVersionStamp {
    pub version: u16,
    pub maintenance: u16,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgCompatibilityProfile {
    #[default]
    Autocad2009,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAuxiliaryHeader {
    pub total_saves: u32,
    pub save_partition_one: u16,
    pub save_partition_two: u16,
    pub save_generation: u32,
    pub legacy_stamp_one: DwgVersionStamp,
    pub legacy_stamp_two: DwgVersionStamp,
    pub compatibility_profile: DwgCompatibilityProfile,
    pub created_at: DwgJulianDate,
    pub updated_at: DwgJulianDate,
    pub handle_seed: u64,
    pub terminal_save_generation: u16,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgRevisionHistory {
    pub format_major: u32,
    pub format_minor: u32,
    #[value(default)]
    pub revisions: Vec<u32>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgPreviewOrigin {
    #[default]
    BottomUp,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgRgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgIndexedPreview {
    pub width: u32,
    pub height: u32,
    pub origin: DwgPreviewOrigin,
    #[value(default)]
    pub palette: Vec<DwgRgba>,
    #[value(default)]
    pub pixel_indices: Vec<u8>,
    pub background_palette_index: u8,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgApplicationPropertyKind {
    #[default]
    String,
    DateTime,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgApplicationProperty {
    pub id: u32,
    pub kind: DwgApplicationPropertyKind,
    pub value: String,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgProductInformation {
    pub name: String,
    pub build_version: String,
    pub registry_version: String,
    pub install_id: String,
    pub locale_id: String,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgApplicationHistory {
    pub history_identifier_one: String,
    pub history_identifier_two: String,
    pub class_version: u32,
    pub application_version_digest: String,
    pub application_version: String,
    pub trust_comment_digest: String,
    pub trust_comment: String,
    pub property_set_digest: String,
    pub property_format_identifier: String,
    #[value(default)]
    pub properties: Vec<DwgApplicationProperty>,
    pub product_digest: String,
    pub product: DwgProductInformation,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgCustomProperty {
    pub key: String,
    pub value: String,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgSummaryInfo {
    pub title: String,
    pub subject: String,
    pub author: String,
    pub keywords: String,
    pub comments: String,
    pub last_saved_by: String,
    pub revision_number: String,
    pub hyperlink_base: String,
    #[value(default)]
    pub total_editing_time: u64,
    #[value(default)]
    pub created_at: DwgJulianDate,
    #[value(default)]
    pub modified_at: DwgJulianDate,
    #[value(default)]
    pub custom_properties: Vec<DwgCustomProperty>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgApplicationInfo {
    pub name: String,
    pub version_checksum: String,
    pub version: String,
    pub comment_checksum: String,
    pub comment: String,
    pub product_checksum: String,
    pub product: String,
    pub application_version: String,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMeasurement {
    #[default]
    English,
    Metric,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgTemplate {
    pub description: String,
    pub measurement: DwgMeasurement,
}
//#endregion 🔖️DocumentModel
