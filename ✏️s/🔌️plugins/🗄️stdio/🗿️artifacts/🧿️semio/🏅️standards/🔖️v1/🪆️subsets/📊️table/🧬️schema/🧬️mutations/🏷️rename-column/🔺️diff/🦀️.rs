//! 🔺️ Diff for `RenameColumn`.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableColumnDiff;
use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
/// 🧮️ Renames one column: a sparse `modified` row naming only the new name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RenameColumn, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    let Some(at) = base.columns.iter().position(|c| c.name == payload.name) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Column \"{}\" does not exist.", payload.name), [payload.name.clone()]);
    };
    if payload.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Column \"{}\" is already named \"{}\".", payload.name, payload.new_name));
    }
    if base.columns.iter().any(|c| c.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A column named \"{}\" already exists.", payload.new_name), [payload.new_name.clone()]);
    }
    protocol::MutationOutcome::new(SemioTableDiff { columns: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff: SemioTableColumnDiff { name: Some(payload.new_name.clone()), kind: None } }], ..Default::default() }), rows: None })
}
//#endregion 🔖️Diff
