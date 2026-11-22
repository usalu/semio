//! 🖌️ PDF page editor graphics state tracked while the content stream is interpreted.

use super::*;

//#region 🖌️Graphics
#[derive(Clone, Debug, PartialEq)]
pub struct PathClip {
    pub path: std::sync::Arc<[VectorSegment]>,
    pub even_odd: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Graphics {
    pub ctm: PdfMatrix,
    pub clip_quads: std::sync::Arc<Vec<ClipQuad>>,
    pub clip_paths: std::sync::Arc<Vec<PathClip>>,
    pub fill: [f64; 3],
    pub stroke: [f64; 3],
    pub line_width: f64,
    pub vector_stroke: VectorStroke,
    pub font: String,
    pub font_size: f64,
    pub leading: f64,
    pub text_rise: f64,
    pub text_render_mode: u32,
    pub char_spacing: f64,
    pub word_spacing: f64,
    pub horizontal_scale: f64,
    pub text_matrix: PdfMatrix,
    pub line_matrix: PdfMatrix,
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
