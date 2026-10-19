//! 🖋️ Content operators of the PDF document model: the text strings and arrays, property lists, inline images, line styles and the operator enum. See [`super`].

use super::colour_and_graphics_state::PdfColorSpace;
use super::object_model::{PdfDictEntry, PdfMatrix, PdfObject};
use super::xobjects::PdfImageBody;

//#region 🔖️Content
/// 🔤️ A string operand of a text-showing operator. `Text` is Unicode recovered through the
/// selected font's encoding (and written back through it); `Codes` are logical unsigned character codes
/// whenever the font cannot map them to Unicode and back (§9.4.3). Native byte grouping belongs to I/O and its application font binding.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfTextArrayItem {
    Text { text: String },
    Codes { codes: Vec<u32> },
    Adjust { amount: f64 },
}

/// 🏷️ The property-list operand of `DP`/`BDC` (§14.6): a named resource or an inline dictionary.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfPropertyList {
    Named { name: String },
    Inline { entries: Vec<PdfDictEntry> },
}

/// 🖼️ An inline image (`BI … ID … EI`, §8.9.7). Abbreviated keys are expanded on read; `data` is
/// decoded except for image codecs, exactly as for [`PdfObject::Stream`].
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfInlineImage {
    pub width: u32,
    pub height: u32,
    #[value(default)]
    pub bits_per_component: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color_space: Option<PdfColorSpace>,
    #[value(default)]
    pub image_mask: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub decode: Vec<f64>,
    #[value(default)]
    pub interpolate: bool,
    pub body:PdfImageBody,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

/// ✂️ Line cap style (§8.4.3.3).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum PdfLineCap {
    #[default]
    Butt,
    Round,
    Square,
}

/// 🔗️ Line join style (§8.4.3.4).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum PdfLineJoin {
    #[default]
    Miter,
    Round,
    Bevel,
}

/// 🖋️ One content-stream operator with typed operands — every operator of ISO 32000-1 Table 51.
/// Painting order is the `Vec<PdfOp>` order; nothing is inferred or re-ordered on either side.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "op", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfOp {
    SetLineWidth { width: f64 },
    SetLineCap { cap: PdfLineCap },
    SetLineJoin { join: PdfLineJoin },
    SetMiterLimit { limit: f64 },
    SetDash { array: Vec<f64>, phase: f64 },
    SetRenderingIntent { intent: String },
    SetFlatness { flatness: f64 },
    SetExtGState { name: String },
    Save,
    Restore,
    Transform { matrix: PdfMatrix },
    MoveTo { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    CurveTo { x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64 },
    CurveToInitial { x2: f64, y2: f64, x3: f64, y3: f64 },
    CurveToFinal { x1: f64, y1: f64, x3: f64, y3: f64 },
    ClosePath,
    Rectangle { x: f64, y: f64, width: f64, height: f64 },
    Stroke,
    CloseStroke,
    Fill,
    FillEvenOdd,
    FillStroke,
    FillStrokeEvenOdd,
    CloseFillStroke,
    CloseFillStrokeEvenOdd,
    EndPath,
    Clip,
    ClipEvenOdd,
    BeginText,
    EndText,
    SetCharSpacing { spacing: f64 },
    SetWordSpacing { spacing: f64 },
    SetHorizontalScale { scale: f64 },
    SetLeading { leading: f64 },
    SetFont { name: String, size: f64 },
    SetTextRenderingMode { mode: u32 },
    SetTextRise { rise: f64 },
    MoveText { tx: f64, ty: f64 },
    MoveTextSetLeading { tx: f64, ty: f64 },
    SetTextMatrix { matrix: PdfMatrix },
    NextLine,
    ShowText { text: PdfTextString },
    ShowTextArray { items: Vec<PdfTextArrayItem> },
    NextLineShowText { text: PdfTextString },
    NextLineShowTextSpaced { word_spacing: f64, char_spacing: f64, text: PdfTextString },
    SetGlyphWidth { wx: f64, wy: f64 },
    SetGlyphWidthAndBox { wx: f64, wy: f64, llx: f64, lly: f64, urx: f64, ury: f64 },
    SetStrokeColorSpace { name: String },
    SetFillColorSpace { name: String },
    SetStrokeColor { components: Vec<f64> },
    SetStrokeColorN { components: Vec<f64>, pattern: Option<String> },
    SetFillColor { components: Vec<f64> },
    SetFillColorN { components: Vec<f64>, pattern: Option<String> },
    SetStrokeGray { gray: f64 },
    SetFillGray { gray: f64 },
    SetStrokeRgb { r: f64, g: f64, b: f64 },
    SetFillRgb { r: f64, g: f64, b: f64 },
    SetStrokeCmyk { c: f64, m: f64, y: f64, k: f64 },
    SetFillCmyk { c: f64, m: f64, y: f64, k: f64 },
    PaintShading { name: String },
    PaintXObject { name: String },
    InlineImage { image: PdfInlineImage },
    MarkedContentPoint { tag: String },
    MarkedContentPointWithProperties { tag: String, properties: PdfPropertyList },
    BeginMarkedContent { tag: String },
    BeginMarkedContentWithProperties { tag: String, properties: PdfPropertyList },
    EndMarkedContent,
    BeginCompatibility,
    EndCompatibility,
    Unknown { operator: String, operands: Vec<PdfObject> },
}
//#endregion 🔖️Content
