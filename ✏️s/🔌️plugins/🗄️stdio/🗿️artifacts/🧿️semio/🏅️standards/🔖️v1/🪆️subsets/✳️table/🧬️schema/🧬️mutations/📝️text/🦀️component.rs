//! ⚡️ Semio table artifact — hand-rolled `OpText` for `SemioTableMutation`.
//! `#[derive(dsl::Mutations)]` only generates `Mutation`/`SemanticMutation` (see
//! `../🦀️component.rs`'s `🔖️Mutations` region) — the wire-text codec stays handcrafted here, one
//! keyword per semantic verb, grammar `keyword:arg1,arg2,...` (`✳️text`'s own hex/bracket-encoded
//! value convention, reused so this facet's grammar can lean on the shared `hex` macro instead of
//! a quoted-string production). `CreateColumn.index: Option<usize>` encodes as an EMPTY token for
//! `None`, the digits for `Some(n)` — a comma-separated positional field can represent absence as
//! an empty slot without ambiguity.

pub use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::split_top_level;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::{
    create_column::mutation::CreateColumn, delete_column::mutation::DeleteColumn, edit_cell::mutation::EditCell, insert_row::mutation::InsertRow, remove_row::mutation::RemoveRow, rename_column::mutation::RenameColumn,
    reorder_columns::mutation::ReorderColumns, reorder_rows::mutation::ReorderRows,
};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{dec_cell_kind, dec_row, enc_cell_kind, enc_row};
use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::{dec_semio_value, dec_str, enc_semio_value, enc_str};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_opt_usize(v: Option<usize>) -> String {
    match v {
        Some(n) => n.to_string(),
        None => String::new(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_opt_usize(s: &str) -> Result<Option<usize>, String> {
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(parse_usize(s)?))
    }
}
//#endregion 🔖️Primitives

//#region 🔖️OpText
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_table_mutation(m: &SemioTableMutation) -> String {
    match m {
        SemioTableMutation::CreateColumn(p) => format!("createColumn:{},{},{}", enc_str(&p.name), enc_cell_kind(p.kind), enc_opt_usize(p.index)),
        SemioTableMutation::DeleteColumn(p) => format!("deleteColumn:{}", enc_str(&p.name)),
        SemioTableMutation::RenameColumn(p) => format!("renameColumn:{},{}", enc_str(&p.name), enc_str(&p.new_name)),
        SemioTableMutation::ReorderColumns(p) => format!("reorderColumns:{},{}", enc_str(&p.name), p.to_index),
        SemioTableMutation::InsertRow(p) => format!("insertRow:{},{}", p.index, enc_row(&p.row)),
        SemioTableMutation::RemoveRow(p) => format!("removeRow:{}", p.index),
        SemioTableMutation::ReorderRows(p) => format!("reorderRows:{},{}", p.from, p.to),
        SemioTableMutation::EditCell(p) => format!("editCell:{},{},{}", p.row_index, enc_str(&p.column_name), enc_semio_value(&p.new_value)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_table_mutation(line: &str) -> Result<SemioTableMutation, String> {
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("table mutation: missing ':' in {line:?}"))?;
    match tag {
        "createColumn" => {
            let parts = split_top_level(rest, ',');
            let [name, kind, index] = parts.as_slice() else { return Err(format!("createColumn: expected 3 fields, got {}", parts.len())) };
            Ok(SemioTableMutation::CreateColumn(CreateColumn { name: dec_str(name)?, kind: dec_cell_kind(kind)?, index: dec_opt_usize(index)? }))
        }
        "deleteColumn" => Ok(SemioTableMutation::DeleteColumn(DeleteColumn { name: dec_str(rest)? })),
        "renameColumn" => {
            let parts = split_top_level(rest, ',');
            let [name, new_name] = parts.as_slice() else { return Err(format!("renameColumn: expected 2 fields, got {}", parts.len())) };
            Ok(SemioTableMutation::RenameColumn(RenameColumn { name: dec_str(name)?, new_name: dec_str(new_name)? }))
        }
        "reorderColumns" => {
            let parts = split_top_level(rest, ',');
            let [name, to_index] = parts.as_slice() else { return Err(format!("reorderColumns: expected 2 fields, got {}", parts.len())) };
            Ok(SemioTableMutation::ReorderColumns(ReorderColumns { name: dec_str(name)?, to_index: parse_usize(to_index)? }))
        }
        "insertRow" => {
            let (idx, row) = rest.split_once(',').ok_or_else(|| "insertRow: missing comma".to_string())?;
            Ok(SemioTableMutation::InsertRow(InsertRow { index: parse_usize(idx)?, row: dec_row(row)? }))
        }
        "removeRow" => Ok(SemioTableMutation::RemoveRow(RemoveRow { index: parse_usize(rest)? })),
        "reorderRows" => {
            let parts = split_top_level(rest, ',');
            let [from, to] = parts.as_slice() else { return Err(format!("reorderRows: expected 2 fields, got {}", parts.len())) };
            Ok(SemioTableMutation::ReorderRows(ReorderRows { from: parse_usize(from)?, to: parse_usize(to)? }))
        }
        "editCell" => {
            let parts = split_top_level(rest, ',');
            let [row_index, column_name, new_value] = parts.as_slice() else { return Err(format!("editCell: expected 3 fields, got {}", parts.len())) };
            Ok(SemioTableMutation::EditCell(EditCell { row_index: parse_usize(row_index)?, column_name: dec_str(column_name)?, new_value: dec_semio_value(new_value)? }))
        }
        other => Err(format!("table mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioTableMutation {
    async fn print_op(&self) -> String {
        print_table_mutation(self)
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_table_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️DemoCases
/// 🌱 One representative value per variant — single source of truth for `ops_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️component.rs` and this file's own round-trip test.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioTableMutation> {
    use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableRow};
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    vec![
        SemioTableMutation::CreateColumn(CreateColumn { name: "notes".into(), kind: SemioTableCellKind::Str, index: Some(1) }),
        SemioTableMutation::CreateColumn(CreateColumn { name: "extra".into(), kind: SemioTableCellKind::Int, index: None }),
        SemioTableMutation::DeleteColumn(DeleteColumn { name: "label".into() }),
        SemioTableMutation::RenameColumn(RenameColumn { name: "label".into(), new_name: "title".into() }),
        SemioTableMutation::ReorderColumns(ReorderColumns { name: "score".into(), to_index: 0 }),
        SemioTableMutation::InsertRow(InsertRow { index: 1, row: SemioTableRow { cells: vec![SemioValue::Str { value: "x".into() }, SemioValue::Null] } }),
        SemioTableMutation::RemoveRow(RemoveRow { index: 0 }),
        SemioTableMutation::ReorderRows(ReorderRows { from: 0, to: 1 }),
        SemioTableMutation::EditCell(EditCell { row_index: 0, column_name: "score".into(), new_value: SemioValue::Float { lexeme: "9.000".into() } }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::OpText;

    #[semio_framework_async_macros::async_test]
    async fn op_text_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.await.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <SemioTableMutation as OpText>::parse_op(&printed).await.unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");
        }
    }
}
//#endregion 🧪️Tests
