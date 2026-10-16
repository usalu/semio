//! 🖌️ PDF page editor graphics state tracked while the content stream is interpreted.

use super::*;

//#region 🖌️Graphics
#[derive(Clone, Debug)]
pub(super) struct Graphics {
    pub(super) ctm: PdfMatrix,
    pub(super) parent_ctm: PdfMatrix,
    pub(super) fill: [f64; 3],
    pub(super) stroke: [f64; 3],
    pub(super) line_width: f64,
    pub(super) font: String,
    pub(super) font_size: f64,
    pub(super) leading: f64,
    pub(super) char_spacing: f64,
    pub(super) word_spacing: f64,
    pub(super) horizontal_scale: f64,
    pub(super) text_matrix: PdfMatrix,
    pub(super) line_matrix: PdfMatrix,
    pub(super) position_op: Option<usize>,
    pub(super) transform_op: Option<usize>,
}

impl Default for Graphics {
    fn default() -> Self {
        Self {
            ctm: PDF_IDENTITY_MATRIX,
            parent_ctm: PDF_IDENTITY_MATRIX,
            fill: [0.0, 0.0, 0.0],
            stroke: [0.0, 0.0, 0.0],
            line_width: 1.0,
            font: String::new(),
            font_size: 12.0,
            leading: 0.0,
            char_spacing: 0.0,
            word_spacing: 0.0,
            horizontal_scale: 100.0,
            text_matrix: PDF_IDENTITY_MATRIX,
            line_matrix: PDF_IDENTITY_MATRIX,
            position_op: None,
            transform_op: None,
        }
    }
}
//#endregion 🖌️Graphics
