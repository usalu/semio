//! 🖋️ Content operators of the PDF document model: the text strings and arrays, property lists, inline images, line styles and the operator enum. See [`super`].

use super::colour_and_graphics_state::PdfColorSpace;
use semio_s_artifact_stdio_pdf_model_object::{PdfDictEntry, PdfMatrix, PdfObject};
use super::xobjects::{PdfImageBody, PdfPackedSamples};

//#region 🔖️Content
pub use semio_s_artifact_stdio_pdf_model_content_values::{PdfTextString, PdfTextArrayItem, PdfPropertyList, PdfLineCap, PdfLineJoin};

/// 🖼️ An inline image (`BI … ID … EI`, §8.9.7). Abbreviated keys are expanded on read; `data` is
/// decoded except for image codecs, exactly as for [`PdfObject::Stream`].
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
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

impl PdfInlineImage {
    /// 🖌️ The colour components of one inline pixel: one for a stencil mask, else its colour space's.
    pub fn components(&self)->u32 {if self.image_mask {1} else {self.color_space.as_ref().and_then(PdfColorSpace::components).unwrap_or(1)}}
    /// 🎛️ The inline sample depth: one bit for a stencil mask, else its `BitsPerComponent`.
    pub fn depth(&self)->u32 {if self.image_mask {1} else {self.bits_per_component}}
    /// 🔬️ Views the inline packed body as logical samples; `None` for a foreign artifact body.
    pub fn packed_samples(&self)->Option<Result<PdfPackedSamples<'_>,&'static str>> {
        match &self.body {PdfImageBody::Samples {packed}=>Some(PdfPackedSamples::new(packed,self.width,self.height,self.components(),self.depth())),PdfImageBody::Artifact {..}=>None}
    }
}

/// 🖋️ One content-stream operator with typed operands — every operator of ISO 32000-1 Table 51.
/// Painting order is the `Vec<PdfOp>` order; nothing is inferred or re-ordered on either side.
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
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
    InlineImage { image: Box<PdfInlineImage> },
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
