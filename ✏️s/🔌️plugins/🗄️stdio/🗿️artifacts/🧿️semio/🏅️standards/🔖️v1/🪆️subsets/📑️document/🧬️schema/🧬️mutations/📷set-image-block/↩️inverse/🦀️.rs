//! ↩️ Inverse for `SetImageBlock`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetImageBlock, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::SetImageBlock { path, .. } = payload;
    Ok(match block_at(base, path) {
        Some(DocBlock::Image { image_id, alt, width, height }) => {
            vec![SemioDocumentMutation::SetImageBlock(set_image_block::SetImageBlock { path: path.clone(), image_id: image_id.clone(), alt: alt.clone(), width: *width, height: *height })]
        }
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
