//! ↩️ Inverse for `SetTextBoxBlocks`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetTextBoxBlocks, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::SetTextBoxBlocks { slide_index, shape_index, .. } = payload;
    Ok(match shape_at(base, *slide_index, *shape_index) {
        Some(SlideShape::TextBox { blocks, .. }) => {
            vec![SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks { slide_index: *slide_index, shape_index: *shape_index, blocks: blocks.clone() })]
        }
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
