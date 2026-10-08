//! ↩️ Inverse for `SetParagraphStyle`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetParagraphStyle, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::SetParagraphStyle { path, .. } = payload;
    Ok(match block_at(base, path) {
        Some(DocBlock::Paragraph { style_id, .. }) => vec![SemioDocumentMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { path: path.clone(), style_id: style_id.clone() })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
