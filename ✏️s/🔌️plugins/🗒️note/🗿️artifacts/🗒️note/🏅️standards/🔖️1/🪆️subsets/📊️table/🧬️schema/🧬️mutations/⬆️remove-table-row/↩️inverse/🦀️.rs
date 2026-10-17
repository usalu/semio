//! ↩️ Inverse for `RemoveTableRow`.
use super::RemoveTableRow;
use crate::standards::v1::subsets::table::schema::mutations::insert_table_row::InsertTableRow;
use crate::{NoteBlockNode, NoteSnapshot};

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RemoveTableRow, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::table::schema::mutations::insert_table_row::InsertTableRow>,
{
    Ok(match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(NoteBlockNode::Table { rows, .. }) if rows.len() > 1 => vec![Op::from(InsertTableRow { id: payload.id.clone(), cells: rows.last().cloned() })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
