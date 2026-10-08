//! 🧭️ The table editor's edit rules: which snapshot pointer raises which ONE concrete table mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{build, ent, event_path, field, ins, item, keyed, list_move, named, positions, rem, resolve, spread_snake, take, uint, unsupported, Entries, Reshape, ITEM};
use crate::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use semio_framework_plugin::Fault;
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{EditRules, RowKey, Selector, SnapshotEditEvent};

const CELL: &str = "$cell";

/// 📚 Every pointer a table editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[ent("/columns/*/name", "rename-column", &[named("name", "name")], "new_name"), ent("/rows/*/cells/*", "edit-cell", &[Selector::Index("row_index"), Selector::Index(CELL)], "new_value")],
    inserts: &[ins("/columns", "create-column", &[], Some("index"), ITEM), ins("/rows", "insert-row", &[], Some("index"), "row")],
    removes: &[rem("/columns", "delete-column", &[], keyed("name", "name")), rem("/rows", "remove-row", &[], RowKey::Index("index"))],
};

const RESHAPES: &[(&str, Reshape)] = &[("create-column", spread_snake), ("edit-cell", edit_cell)];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edit_cell(tree: &DslValue, entries: &mut Entries) -> Result<(), String> {
    let cell = uint(&take(entries, CELL)?).ok_or("the cell position is not a number")?;
    let name = field(tree, "columns").and_then(|columns| item(columns, cell)).and_then(|column| field(column, "name")).ok_or_else(|| format!("the table has no column {cell}"))?;
    entries.push(("column_name".to_string(), name.clone()));
    Ok(())
}

/// 🔀️ Moving a column is `reorder-columns`, moving a row `reorder-rows`; every other edit goes through [`EDIT_RULES`] and completes its payload.
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioTableSnapshot) -> Result<Option<Vec<SemioTableMutation>>, Fault> {
    if let Some((from, to)) = list_move(event, &["rows"])? {
        return positions(snapshot, event, "reorder-rows", &[("from", from), ("to", to)]).map(Some);
    }
    if let Some((from, to)) = list_move(event, &["columns"])? {
        let tree = snapshot.to_value();
        let name = field(&tree, "columns").and_then(|columns| item(columns, from)).and_then(|column| field(column, "name")).cloned().ok_or_else(|| unsupported(event_path(event), format!("the table has no column {from}")))?;
        return build::<SemioTableSnapshot, SemioTableMutation>(&tree, &[], "reorder-columns", vec![("name".to_string(), name), ("to_index".to_string(), DslValue::uint(to as u64))], event_path(event)).map(Some);
    }
    resolve::<SemioTableSnapshot, SemioTableMutation>(&EDIT_RULES, RESHAPES, snapshot, event).map(Some)
}
