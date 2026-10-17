//! ↩️ Inverse for `InsertTableRow`.
use super::InsertTableRow;
use crate::standards::v1::subsets::table::schema::mutations::remove_table_row::RemoveTableRow;
use crate::{NoteBlockNode, NoteSnapshot};

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &InsertTableRow, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::table::schema::mutations::remove_table_row::RemoveTableRow>,
{
    Ok(match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(NoteBlockNode::Table { columns, .. }) if payload.cells.as_ref().is_none_or(|cells| cells.len() == columns.len()) => vec![Op::from(RemoveTableRow { id: payload.id.clone() })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
