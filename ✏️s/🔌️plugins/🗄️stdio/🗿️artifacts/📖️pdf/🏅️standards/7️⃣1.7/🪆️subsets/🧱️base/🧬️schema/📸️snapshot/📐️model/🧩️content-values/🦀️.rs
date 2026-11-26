//! 🧩️ Independent operands of PDF content operators.

use semio_s_artifact_stdio_pdf_model_object::PdfDictEntry;

/// 🔤️ A string operand of a text-showing operator. `Text` is Unicode recovered through the
/// selected font's encoding (and written back through it); `Codes` are logical unsigned character codes
/// whenever the font cannot map them to Unicode and back (§9.4.3). Native byte grouping belongs to I/O and its application font binding.
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfTextString {
    Text { text: String },
    Codes { codes: Vec<u32> },
}

impl PdfTextString {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }
}

/// 🧵 One element of a `TJ` array: a string or a horizontal adjustment in thousandths of text
/// space (§9.4.3, Table 109).
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfTextArrayItem {
    Text { text: String },
    Codes { codes: Vec<u32> },
    Adjust { amount: f64 },
}

/// 🏷️ The property-list operand of `DP`/`BDC` (§14.6): a named resource or an inline dictionary.
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfPropertyList {
    Named { name: String },
    Inline { entries: Vec<PdfDictEntry> },
}

/// ✂️ Line cap style (§8.4.3.3).
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum PdfLineCap {
    #[default]
    Butt,
    Round,
    Square,
}

/// 🔗️ Line join style (§8.4.3.4).
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum PdfLineJoin {
    #[default]
    Miter,
    Round,
    Bevel,
}

