//! 🎯️ Navigation of the PDF document model: the destinations, file specifications, actions, outline items, named destinations and page labels. See [`super`].

use super::object_model::{PdfDictEntry, PdfRect};

//#region 🔖️Navigation
/// 🎯 How a destination positions the page (§12.3.2.2, Table 151).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfDestinationFit {
    Xyz { left: Option<f64>, top: Option<f64>, zoom: Option<f64> },
    Fit,
    FitHorizontal { top: Option<f64> },
    FitVertical { left: Option<f64> },
    FitRectangle { rect: PdfRect },
    FitBoundingBox,
    FitBoundingBoxHorizontal { top: Option<f64> },
    FitBoundingBoxVertical { left: Option<f64> },
}

/// 🎯 A destination (§12.3.2): a page of this document by index, a page of a remote document by
/// number, or a named destination.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfDestination {
    Page { page: u32, fit: PdfDestinationFit },
    RemotePage { page: u32, fit: PdfDestinationFit },
    Named { name: String },
}

/// 📎 A file specification (§7.11.3): a path or an embedded file by id.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfFileSpecification {
    Path { path: String },
    Embedded { file: String },
}

/// 🎬 An action (§12.6.4). `next` chains follow-up actions.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfActionKind {
    GoTo { destination: PdfDestination },
    GoToRemote { file: PdfFileSpecification, destination: PdfDestination, new_window: Option<bool> },
    GoToEmbedded { destination: PdfDestination, new_window: Option<bool> },
    Launch { file: PdfFileSpecification, new_window: Option<bool> },
    Thread { file: Option<PdfFileSpecification>, thread: u32 },
    Uri { uri: String, is_map: bool },
    Sound { sound: String, volume: Option<f64>, synchronous: bool, repeat: bool, mix: bool },
    Movie { annotation: Option<String>, operation: Option<String> },
    Hide { annotations: Vec<String>, hide: bool },
    Named { name: String },
    SubmitForm { url: String, fields: Vec<String>, flags: u32 },
    ResetForm { fields: Vec<String>, flags: u32 },
    ImportData { file: PdfFileSpecification },
    JavaScript { script: String },
    SetOptionalContentState { states: Vec<PdfDictEntry>, preserve_radio_buttons: bool },
    Rendition { entries: Vec<PdfDictEntry> },
    Transition { entries: Vec<PdfDictEntry> },
    GoTo3dView { entries: Vec<PdfDictEntry> },
    Unknown { subtype: String, entries: Vec<PdfDictEntry> },
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", retire_with="retire_pdf_action")]
pub struct PdfAction {
    pub kind: PdfActionKind,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub next: Vec<PdfAction>,
}

/// ♻️ Retires an action forest through its typed kind owner.
fn retire_pdf_action(value: PdfAction) {
    let mut pending = vec![value];
    while let Some(PdfAction { kind, next }) = pending.pop() {
        pending.extend(next);
        <PdfActionKind as semio_framework_value::FromValue>::retire_decoded(kind);
    }
}

impl PdfAction {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn uri(uri: impl Into<String>) -> Self {
        Self { kind: PdfActionKind::Uri { uri: uri.into(), is_map: false }, next: Vec::new() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn go_to(page: u32) -> Self {
        Self { kind: PdfActionKind::GoTo { destination: PdfDestination::Page { page, fit: PdfDestinationFit::Fit } }, next: Vec::new() }
    }
}

/// 📑 One outline (bookmark) item (§12.3.3) with its nested children.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", retire_with="retire_pdf_outline_item")]
pub struct PdfOutlineItem {
    pub title: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<PdfDestination>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<PdfAction>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<[f64; 3]>,
    #[value(default)]
    pub italic: bool,
    #[value(default)]
    pub bold: bool,
    #[value(default)]
    pub open: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<PdfOutlineItem>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

/// ♻️ Retires outline children and their owned actions without recursive destruction.
fn retire_pdf_outline_item(value: PdfOutlineItem) {
    let mut pending = vec![value];
    while let Some(PdfOutlineItem { children, action, extra, .. }) = pending.pop() {
        pending.extend(children);
        <Option<PdfAction> as semio_framework_value::FromValue>::retire_decoded(action);
        <Vec<PdfDictEntry> as semio_framework_value::FromValue>::retire_decoded(extra);
    }
}

impl PdfOutlineItem {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_page(title: impl Into<String>, page: u32) -> Self {
        Self { title: title.into(), destination: Some(PdfDestination::Page { page, fit: PdfDestinationFit::Fit }), action: None, color: None, italic: false, bold: false, open: false, children: Vec::new(), extra: Vec::new() }
    }
}

/// 📛 One entry of the document's `/Dests` name tree (§12.3.2.3).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfNamedDestination {
    pub name: String,
    pub destination: PdfDestination,
}

/// 🔢 Page label numbering style (§12.4.2, Table 159).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum PdfPageLabelStyle {
    Decimal,
    RomanUpper,
    RomanLower,
    LettersUpper,
    LettersLower,
}

/// 🔢 One page-label range starting at page `start_index`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfPageLabelRange {
    pub start_index: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<PdfPageLabelStyle>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[value(default = "PdfPageLabelRange::one")]
    pub start: u32,
}

impl PdfPageLabelRange {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn one() -> u32 {
        1
    }
}
//#endregion 🔖️Navigation
