//! 🖌️ PDF page editor graphics state tracked while the content stream is interpreted.

use super::*;

//#region 🖌️Graphics
#[derive(Clone, Debug, PartialEq)]
pub(super) struct PathClip {
    pub(super) path: std::sync::Arc<[VectorSegment]>,
    pub(super) even_odd: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Graphics {
    pub(super) ctm: PdfMatrix,
    pub(super) clip_quads: std::sync::Arc<Vec<ClipQuad>>,
    pub(super) clip_paths: std::sync::Arc<Vec<PathClip>>,
    pub(super) fill: [f64; 3],
    pub(super) stroke: [f64; 3],
    pub(super) line_width: f64,
    pub(super) vector_stroke: VectorStroke,
    pub(super) font: String,
    pub(super) font_size: f64,
    pub(super) leading: f64,
    pub(super) text_rise: f64,
    pub(super) text_render_mode: u32,
    pub(super) char_spacing: f64,
    pub(super) word_spacing: f64,
    pub(super) horizontal_scale: f64,
    pub(super) text_matrix: PdfMatrix,
    pub(super) line_matrix: PdfMatrix,
}

impl Default for Graphics {
    fn default() -> Self {
        Self {
            ctm: PDF_IDENTITY_MATRIX,
            clip_quads: Default::default(),
            clip_paths: Default::default(),
            fill: [0.0, 0.0, 0.0],
            stroke: [0.0, 0.0, 0.0],
            line_width: 1.0,
            vector_stroke: VectorStroke::default(),
            font: String::new(),
            font_size: 12.0,
            leading: 0.0,
            text_rise: 0.0,
            text_render_mode: 0,
            char_spacing: 0.0,
            word_spacing: 0.0,
            horizontal_scale: 100.0,
            text_matrix: PDF_IDENTITY_MATRIX,
            line_matrix: PDF_IDENTITY_MATRIX,
        }
    }
}
//#endregion 🖌️Graphics
