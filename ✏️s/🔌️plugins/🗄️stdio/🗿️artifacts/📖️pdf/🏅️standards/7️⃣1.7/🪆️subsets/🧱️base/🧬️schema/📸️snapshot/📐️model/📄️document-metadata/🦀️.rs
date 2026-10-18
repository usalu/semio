//! 📄️ Document metadata of the PDF document model: the optional content, dates, information dictionary, embedded files, output intents, viewer preferences, open action, encryption and mark info. See [`super`].

use super::navigation::{PdfAction, PdfDestination};
use super::object_model::{PdfDictEntry, PdfObject};

//#region 🔖️OptionalContent
/// 👁️ An optional content group (§8.11.2).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfOptionalContentGroup {
    pub id: String,
    pub name: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub intent: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub usage: Vec<PdfDictEntry>,
}

/// 👁️ The optional content properties dictionary (§8.11.4): groups plus the default configuration.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfOptionalContent {
    #[value(default)]
    pub groups: Vec<PdfOptionalContentGroup>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default)]
    pub base_state_off: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub on: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub off: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub order: Vec<PdfObject>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}
//#endregion 🔖️OptionalContent

//#region 🔖️Document
/// 📅 A PDF date (§7.9.4) with an optional UTC offset in minutes.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfDate {
    pub year: i32,
    #[value(default = "PdfDate::one", default_controlled="PdfDate::one_controlled")]
    pub month: u32,
    #[value(default = "PdfDate::one", default_controlled="PdfDate::one_controlled")]
    pub day: u32,
    #[value(default)]
    pub hour: u32,
    #[value(default)]
    pub minute: u32,
    #[value(default)]
    pub second: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset_minutes: Option<i32>,
}

impl PdfDate {
    fn one_controlled(control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<u32, semio_framework_value::ValueError> {
        control.checkpoint()?;
        Ok(1)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn one() -> u32 {
        1
    }
}

/// 📇️ Document `/Info` dictionary (§14.3.3, Table 317).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfInfo {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<PdfDate>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<PdfDate>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trapped: Option<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfInfo {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self == &PdfInfo::default()
    }
}

/// 📎 An embedded file (§7.11.4) reachable through the `/EmbeddedFiles` name tree or a file
/// attachment annotation, keyed by `id`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
pub struct PdfEmbeddedFile {
    pub id: String,
    pub file_name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[value(with="pack::value::bytes", serialize_controlled_with="pack::value::bytes::to_value_controlled", deserialize_controlled_with="pack::value::bytes::from_value_controlled", retire_with="std::mem::drop")]
    pub data: Vec<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<PdfDate>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<PdfDate>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    #[value(default)]
    pub listed: bool,
}

/// 🏳️ An output intent (§14.11.5) — what PDF/A and PDF/X conformance is declared against.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfOutputIntent {
    pub subtype: String,
    pub condition_identifier: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<semio_framework_artifact_reference::ArtifactRef>,
}

/// 📖 Page layout to use when the document is opened (§12.2, Table 28).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum PdfPageLayout {
    SinglePage,
    OneColumn,
    TwoColumnLeft,
    TwoColumnRight,
    TwoPageLeft,
    TwoPageRight,
}

/// 📖 How the document is displayed when opened (§12.2, Table 28).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum PdfPageMode {
    UseNone,
    UseOutlines,
    UseThumbs,
    FullScreen,
    UseOc,
    UseAttachments,
}

/// 🖥️ Viewer preferences (§12.2, Table 150).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfViewerPreferences {
    #[value(default)]
    pub hide_toolbar: bool,
    #[value(default)]
    pub hide_menubar: bool,
    #[value(default)]
    pub hide_window_ui: bool,
    #[value(default)]
    pub fit_window: bool,
    #[value(default)]
    pub center_window: bool,
    #[value(default)]
    pub display_doc_title: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub non_full_screen_page_mode: Option<PdfPageMode>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub view_area: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub view_clip: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub print_area: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub print_clip: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub print_scaling: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub duplex: Option<String>,
    #[value(default)]
    pub pick_tray_by_pdf_size: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub print_page_range: Vec<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub num_copies: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

/// 🚪 What happens when the document opens (§7.7.2 `/OpenAction`).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfOpenAction {
    Destination { destination: PdfDestination },
    Action { action: PdfAction },
}

/// 🔐 The standard security handler's algorithm (§7.6.3).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum PdfEncryptionAlgorithm {
    Rc4_40,
    Rc4_128,
    Aes128,
    Aes256,
}

/// 🔐 Standard security handler parameters (§7.6.3, Table 21). Passwords are in-memory only:
/// the persistent form of this snapshot is the encrypted file itself, whose `/O`/`/U` entries
/// are derived from them on write. A decoded document records the algorithm, permissions and
/// the user password it was opened with (`""` for the empty user password).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfEncryption {
    pub algorithm: PdfEncryptionAlgorithm,
    #[value(default = "PdfEncryption::all_permissions")]
    pub permissions: i32,
    #[value(default)]
    pub user_password: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub owner_password: Option<String>,
    #[value(default = "PdfEncryption::yes")]
    pub encrypt_metadata: bool,
}

impl PdfEncryption {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn all_permissions() -> i32 {
        -1
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn yes() -> bool {
        true
    }
}

/// 🏷️ `/MarkInfo` (§14.7.1).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfMarkInfo {
    #[value(default)]
    pub marked: bool,
    #[value(default)]
    pub user_properties: bool,
    #[value(default)]
    pub suspects: bool,
}
//#endregion 🔖️Document
