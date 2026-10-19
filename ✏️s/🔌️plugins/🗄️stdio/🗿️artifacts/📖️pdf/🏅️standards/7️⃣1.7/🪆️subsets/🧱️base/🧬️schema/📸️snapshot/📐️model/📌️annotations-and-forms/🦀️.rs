//! 📌️ Annotations and forms of the PDF document model: the appearances, border styles, markup annotations, annotation kinds, form fields and the interactive form. See [`super`].

use super::document_metadata::PdfDate;
use super::navigation::{PdfAction, PdfDestination, PdfFileSpecification};
use super::object_model::{PdfDictEntry, PdfRect};

//#region 🔖️Annotations
/// 🖼️ One appearance sub-dictionary entry: the form XObject `form` shown in appearance `state`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfAppearanceState {
    pub state: String,
    pub form: String,
}

/// 🖼️ The appearance of one state: a single form XObject id, or a form per appearance state.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfAppearanceEntry {
    Single { form: String },
    States { states: Vec<PdfAppearanceState> },
}

/// 🖼️ An appearance dictionary (§12.5.5).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfAppearance {
    pub normal: PdfAppearanceEntry,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rollover: Option<PdfAppearanceEntry>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub down: Option<PdfAppearanceEntry>,
}

/// 🟦 An annotation border (§12.5.4 `/Border` or the `/BS` border style dictionary).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfBorderStyle {
    pub width: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dash: Option<Vec<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radii: Option<[f64; 2]>,
}

/// 💬 Fields shared by markup annotations (§12.5.6.2, Table 170).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfMarkupAnnotation {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub popup: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rich_contents: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<PdfDate>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reply_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
}

/// 🏷️ Subtype-specific annotation data (§12.5.6, Table 169 — every subtype).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfAnnotationKind {
    Text { open: bool, icon: Option<String>, state: Option<String>, state_model: Option<String> },
    Link { action: Option<PdfAction>, destination: Option<PdfDestination>, highlight: Option<String>, quad_points: Vec<f64> },
    FreeText { default_appearance: String, quadding: u32, callout: Option<Vec<f64>>, line_ending: Option<String>, rich_text: Option<String> },
    Line { points: [f64; 4], line_endings: Option<[String; 2]>, interior_color: Option<Vec<f64>>, leader_length: Option<f64>, caption: bool },
    Square { interior_color: Option<Vec<f64>>, rect_differences: Option<PdfRect> },
    Circle { interior_color: Option<Vec<f64>>, rect_differences: Option<PdfRect> },
    Polygon { vertices: Vec<f64>, interior_color: Option<Vec<f64>> },
    PolyLine { vertices: Vec<f64>, line_endings: Option<[String; 2]>, interior_color: Option<Vec<f64>> },
    Highlight { quad_points: Vec<f64> },
    Underline { quad_points: Vec<f64> },
    Squiggly { quad_points: Vec<f64> },
    StrikeOut { quad_points: Vec<f64> },
    Stamp { icon: Option<String> },
    Caret { rect_differences: Option<PdfRect>, symbol: Option<String> },
    Ink { paths: Vec<Vec<f64>> },
    Popup { parent: Option<u64>, open: bool },
    FileAttachment { file: PdfFileSpecification, icon: Option<String> },
    Sound { sound: Vec<PdfDictEntry>, icon: Option<String> },
    Movie { title: Option<String>, movie: Vec<PdfDictEntry>, activation: Option<Vec<PdfDictEntry>> },
    Widget { field: Option<String>, highlight: Option<String>, characteristics: Vec<PdfDictEntry>, action: Option<PdfAction>, additional_actions: Vec<PdfDictEntry> },
    Screen { title: Option<String>, characteristics: Vec<PdfDictEntry>, action: Option<PdfAction>, additional_actions: Vec<PdfDictEntry> },
    PrinterMark { mark_style: Option<String>, colorants: Vec<PdfDictEntry> },
    TrapNet { entries: Vec<PdfDictEntry> },
    Watermark { fixed_print: Option<Vec<PdfDictEntry>> },
    ThreeD { entries: Vec<PdfDictEntry> },
    Redact { quad_points: Vec<f64>, interior_color: Option<Vec<f64>>, overlay_text: Option<String>, repeat: bool, default_appearance: Option<String>, quadding: u32 },
    Unknown { subtype: String, entries: Vec<PdfDictEntry> },
}

/// 📌 An annotation (§12.5.2, Table 164) on a page. `contents`/`name`/`modified`/`flags`/
/// `border`/`color`/`appearance` are the common entries; `markup` the markup-annotation ones.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfAnnotation {
    pub rect: PdfRect,
    pub kind: PdfAnnotationKind,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[value(default)]
    pub flags: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub border: Option<PdfBorderStyle>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub color: Vec<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub appearance: Option<PdfAppearance>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub appearance_state: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub markup: Option<PdfMarkupAnnotation>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub optional_content: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub struct_parent: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfAnnotation {
    pub fn new(rect: PdfRect, kind: PdfAnnotationKind) -> Self {
        Self { rect, kind, contents: None, name: None, modified: None, flags: 4, border: None, color: Vec::new(), appearance: None, appearance_state: None, markup: None, optional_content: None, struct_parent: None, extra: Vec::new() }
    }
    /// 🔗 A URI link over `rect` with no visible border.
    pub fn link(rect: PdfRect, uri: impl Into<String>) -> Self {
        let mut annotation = Self::new(rect, PdfAnnotationKind::Link { action: Some(PdfAction::uri(uri)), destination: None, highlight: None, quad_points: Vec::new() });
        annotation.border = Some(PdfBorderStyle { width: 0.0, style: None, dash: None, radii: None });
        annotation
    }
}
//#endregion 🔖️Annotations

//#region 🔖️Forms
/// 📝 Interactive form field data by field type (§12.7.4).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfFormFieldKind {
    Button { value: Option<String>, default_value: Option<String>, options: Vec<String> },
    Text { value: Option<String>, default_value: Option<String>, max_length: Option<u32>, rich_value: Option<String> },
    Choice { values: Vec<String>, default_values: Vec<String>, options: Vec<(String, String)>, top_index: Option<u32> },
    Signature { value: Option<Vec<PdfDictEntry>> },
    Container,
}

/// 📝 One field of the interactive form tree (§12.7.3). `widgets` are `(page index, annotation
/// index)` pairs of the Widget annotations presenting this field.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", retire_with="retire_pdf_form_field")]
pub struct PdfFormField {
    pub name: String,
    pub kind: PdfFormFieldKind,
    #[value(default)]
    pub flags: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alternate_name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mapping_name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_appearance: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub quadding: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub widgets: Vec<[u32; 2]>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<PdfFormField>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_actions: Vec<PdfDictEntry>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

/// ♻️ Retires field forests and delegates signature dictionaries to their owner.
fn retire_pdf_form_field(value: PdfFormField) {
    let mut pending = vec![value];
    while let Some(PdfFormField { kind, children, additional_actions, extra, .. }) = pending.pop() {
        pending.extend(children);
        <PdfFormFieldKind as semio_framework_value::FromValue>::retire_decoded(kind);
        <Vec<PdfDictEntry> as semio_framework_value::FromValue>::retire_decoded(additional_actions);
        <Vec<PdfDictEntry> as semio_framework_value::FromValue>::retire_decoded(extra);
    }
}

/// 📝 The document's interactive form dictionary (§12.7.2).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfAcroForm {
    #[value(default)]
    pub fields: Vec<PdfFormField>,
    #[value(default)]
    pub need_appearances: bool,
    #[value(default)]
    pub signature_flags: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_appearance: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub quadding: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub default_fonts: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}
//#endregion 🔖️Forms
