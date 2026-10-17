//! ↩️ Inverse for `ChangeBlockFontSize`.
use super::ChangeBlockFontSize;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ChangeBlockFontSize, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::change_block_font_size::ChangeBlockFontSize>,
{
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Text { font_size, .. }) => vec![Op::from(ChangeBlockFontSize { id: payload.id.clone(), new_font_size: *font_size })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
