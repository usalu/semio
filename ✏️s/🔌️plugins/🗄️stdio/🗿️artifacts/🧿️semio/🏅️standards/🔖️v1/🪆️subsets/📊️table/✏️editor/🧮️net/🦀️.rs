//! 🧮️ Net of one snapshot edit as table domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `table` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::{net_ordered, NetStep};
use crate::standards::v1::subsets::table::schema::mutations::{create_column, delete_column, edit_cell, insert_row, remove_row, rename_column, SemioTableMutation};
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioTableSnapshot, next: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    let mut out = Vec::new();
    let mut columns = base.columns.clone();
    let mut rows = base.rows.clone();
    for step in net_ordered(&base.columns, &next.columns) {
        match step {
            NetStep::Modify { index, item } if columns[index].kind == item.kind => {
                out.push(SemioTableMutation::RenameColumn(rename_column::RenameColumn { name: columns[index].name.clone(), new_name: item.name.clone() }));
                columns[index].name = item.name.clone();
            }
            NetStep::Modify { index, item } => {
                out.push(SemioTableMutation::DeleteColumn(delete_column::DeleteColumn { name: columns[index].name.clone() }));
                out.push(SemioTableMutation::CreateColumn(create_column::CreateColumn { name: item.name.clone(), kind: item.kind, index: Some(index) }));
                columns[index] = item.clone();
                rows.iter_mut().filter(|row| index < row.cells.len()).for_each(|row| row.cells[index] = SemioValue::Null);
            }
            NetStep::Remove { index } => {
                out.push(SemioTableMutation::DeleteColumn(delete_column::DeleteColumn { name: columns[index].name.clone() }));
                columns.remove(index);
                rows.iter_mut().filter(|row| index < row.cells.len()).for_each(|row| {
                    row.cells.remove(index);
                });
            }
            NetStep::Insert { index, item } => {
                out.push(SemioTableMutation::CreateColumn(create_column::CreateColumn { name: item.name.clone(), kind: item.kind, index: Some(index) }));
                columns.insert(index, item.clone());
                rows.iter_mut().for_each(|row| row.cells.insert(index.min(row.cells.len()), SemioValue::Null));
            }
        }
    }
    for step in net_ordered(&rows, &next.rows) {
        match step {
            NetStep::Modify { index, item } if item.cells.len() == rows[index].cells.len() && item.cells.len() == next.columns.len() => {
                for (cell, (before, after)) in rows[index].cells.iter().zip(&item.cells).enumerate() {
                    if before != after {
                        out.push(SemioTableMutation::EditCell(edit_cell::EditCell { row_index: index, column_name: next.columns[cell].name.clone(), new_value: after.clone() }));
                    }
                }
            }
            NetStep::Modify { index, item } => {
                out.push(SemioTableMutation::RemoveRow(remove_row::RemoveRow { index }));
                out.push(SemioTableMutation::InsertRow(insert_row::InsertRow { index, row: item.clone() }));
            }
            NetStep::Remove { index } => out.push(SemioTableMutation::RemoveRow(remove_row::RemoveRow { index })),
            NetStep::Insert { index, item } => out.push(SemioTableMutation::InsertRow(insert_row::InsertRow { index, row: item.clone() })),
        }
    }
    out
}
