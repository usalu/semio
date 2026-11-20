//! 🧱️ PDF page editor model: window and body ids, the page action table, the object kinds and the page edit value.

use super::*;

//#region 🧱️Model
pub const WINDOW_KIND_ID: &str = "pdf.page";
pub const BODY_KEY: &str = "pdf.page";
pub const INSPECTOR_WINDOW_KIND_ID: &str = "pdf.object";
pub const INSPECTOR_BODY_KEY: &str = "pdf.object";

pub const OBJECT_DOMAIN: &str = "objects";

const PAGE_ACTIONS: [&str; 67] = [
    "set-text",
    "move",
    "resize",
    "delete",
    "set-fill",
    "set-stroke",
    "insert-text",
    "insert-rectangle",
    "insert-line",
    "insert-image",
    "insert-page",
    "remove-page",
    "move-page",
    "set-page-size",
    "set-info",
    "set-annotation",
    "set-image",
    "set-font",
    "set-outline",
    "set-page-rotation",
    "set-page-box",
    "set-page-user-unit",
    "set-language",
    "set-page-layout",
    "set-page-mode",
    "set-optional-content",
    "set-embedded-file",
    "remove-embedded-file",
    "set-named-destination",
    "remove-named-destination",
    "set-page-label",
    "set-mark-info",
    "set-metadata",
    "set-viewer-preferences",
    "set-encryption",
    "set-output-intent",
    "set-form-field",
    "set-open-action",
    "set-document-id",
    "set-font-reference",
    "set-graphics-state",
    "set-pattern",
    "set-color-space",
    "set-properties",
    "set-font-metrics",
    "set-image-mask",
    "set-form-content",
    "set-page-transition",
    "set-catalog-entry",
    "set-trailer-entry",
    "set-annotation-appearance",
    "set-glyph",
    "set-indirect-object",
    "set-mesh-reference",
    "set-info-field",
    "set-page-extra",
    "set-annotation-style",
    "set-annotation-border",
    "set-annotation-markup",
    "set-form-settings",
    "set-extra-entry",
    "set-annotation-kind",
    "set-field-data",
    "set-resource-detail",
    "canvasPointerDown",
    "canvasPointerMove",
    "canvasPointerUp",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectKind {
    Text,
    Vector,
    Image,
    Form,
    Shading,
    Annotation,
}

impl ObjectKind {
    pub(super) fn letter(self) -> char {
        match self {
            Self::Text => 't',
            Self::Vector => 'v',
            Self::Image => 'i',
            Self::Form => 'f',
            Self::Shading => 's',
            Self::Annotation => 'a',
        }
    }
}

/// 🎯 One paintable, addressable detail on a page.
#[derive(Clone, Debug, PartialEq)]
pub struct PageObject {
    pub id: String,
    pub page: usize,
    pub kind: ObjectKind,
    pub start: usize,
    pub end: usize,
    pub ctm: PdfMatrix,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: String,
    pub font: String,
    pub font_size: f64,
    pub(super) text_placement: Option<Box<TextPlacement>>,
    pub fill: [f64; 3],
    pub stroke: [f64; 3],
    pub line_width: f64,
    pub(super) vector_stroke: VectorStroke,
    pub(super) path: Vec<VectorSegment>,
    pub(super) clip_quads: std::sync::Arc<Vec<ClipQuad>>,
    pub(super) clip_paths: std::sync::Arc<Vec<PathClip>>,
    pub(super) text_transform: PdfMatrix,
    pub(super) form_graphics: Option<Graphics>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct TextPlacement {
    pub matrix: PdfMatrix,
    pub line: PdfMatrix,
    pub next_origin: (f64, f64),
    pub rise: f64,
    pub render_mode: u32,
    pub horizontal_scale: f64,
    pub char_spacing: f64,
    pub word_spacing: f64,
}

pub(super) use semio_s_artifact_stdio_pdf_page_coordinates::{ClipQuad, VectorSegment, VectorStroke};

/// ✏️ One end-user edit, already checked against the action's required arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct PdfPageEdit {
    pub action: String,
    pub payload: String,
}

/// 🏷️ Stable action id for the command log.
pub fn static_action(action: &str) -> &'static str {
    PAGE_ACTIONS.into_iter().find(|candidate| *candidate == action).unwrap_or("page-edit")
}

/// 🧭 Whether `action` is a page-editor command rather than snapshot details or `set-page`.
pub fn is_page_action(action: &str) -> bool {
    PAGE_ACTIONS.contains(&action)
}
//#endregion 🧱️Model
