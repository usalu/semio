//! ↩️ Inverse for `SetTextBoxBlocks`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SlideShape;
#[allow(unused_imports)]
use crate::schema::mutations::set_textbox_blocks;
#[allow(unused_imports)]
use crate::schema::mutations::support::shape_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetTextBoxBlocks, base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_textbox_blocks::SetTextBoxBlocks>,
{
    let super::SetTextBoxBlocks { slide_index, shape_index, .. } = payload;
    Ok(match shape_at(base, *slide_index, *shape_index) {
        Some(SlideShape::TextBox { blocks, .. }) => {
            vec![Op::from(set_textbox_blocks::SetTextBoxBlocks { slide_index: *slide_index, shape_index: *shape_index, blocks: blocks.clone() })]
        }
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
