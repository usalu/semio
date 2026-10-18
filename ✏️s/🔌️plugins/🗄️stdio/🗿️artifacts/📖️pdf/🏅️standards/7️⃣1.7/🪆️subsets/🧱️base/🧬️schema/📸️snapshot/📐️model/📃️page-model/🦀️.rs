//! 📃️ Page of the PDF document model: the page record with its defaults and accessors. See [`super`].

use super::annotations_and_forms::PdfAnnotation;
use super::content_operators::{PdfOp, PdfTextArrayItem, PdfTextString};
use super::object_model::{PdfDictEntry, PdfRect};
use super::xobjects::PdfTransparencyGroup;

//#region 🔖️PageModel
/// 📄️ One page (§7.7.3.3, Table 30) -- inherited `/MediaBox`/`/CropBox`/`/Rotate` already
/// applied (§7.7.3.4). `content` is the concatenation of the page's content streams as typed
/// operators whose resource names are the document-level ids ([`PdfSnapshot::fonts`], images,
/// forms, …); the page's own `/Resources` dictionary is regenerated from them on write.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfPage {
    pub media_box: PdfRect,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub crop_box: Option<PdfRect>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bleed_box: Option<PdfRect>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trim_box: Option<PdfRect>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub art_box: Option<PdfRect>,
    #[value(default)]
    pub rotate: i32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_unit: Option<f64>,
    #[value(default)]
    pub content: Vec<PdfOp>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<PdfAnnotation>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<PdfTransparencyGroup>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub struct_parents: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<Vec<PdfDictEntry>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_actions: Vec<PdfDictEntry>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl Default for PdfPage {
    fn default() -> Self {
        Self::new(612.0, 792.0)
    }
}

impl PdfPage {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            media_box: [0.0, 0.0, width, height],
            crop_box: None,
            bleed_box: None,
            trim_box: None,
            art_box: None,
            rotate: 0,
            user_unit: None,
            content: Vec::new(),
            annotations: Vec::new(),
            group: None,
            thumbnail: None,
            struct_parents: None,
            transition: None,
            duration: None,
            metadata: None,
            additional_actions: Vec::new(),
            extra: Vec::new(),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn width(&self) -> f64 {
        self.media_box[2] - self.media_box[0]
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn height(&self) -> f64 {
        self.media_box[3] - self.media_box[1]
    }
    /// 🔤️ The Unicode text this page shows, in painting order — every `Text` operand of every
    /// text-showing operator, lines separated where the stream starts a new line.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn text(&self) -> String {
        fn push(out: &mut String, text: &PdfTextString) {
            if let PdfTextString::Text { text } = text {
                out.push_str(text);
            }
        }
        fn newline(out: &mut String) {
            if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
        }
        let mut out = String::new();
        for op in &self.content {
            match op {
                PdfOp::ShowText { text } => push(&mut out, text),
                PdfOp::NextLineShowText { text } | PdfOp::NextLineShowTextSpaced { text, .. } => {
                    newline(&mut out);
                    push(&mut out, text);
                }
                PdfOp::ShowTextArray { items } => {
                    for item in items {
                        if let PdfTextArrayItem::Text { text } = item {
                            out.push_str(text);
                        }
                    }
                }
                PdfOp::NextLine | PdfOp::MoveText { .. } | PdfOp::MoveTextSetLeading { .. } | PdfOp::SetTextMatrix { .. } => newline(&mut out),
                _ => {}
            }
        }
        while out.ends_with('\n') {
            out.pop();
        }
        out
    }
}
//#endregion 🔖️PageModel
