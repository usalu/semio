//! 📝️ Text representation codec surface for `stdio.semio.table` (diff) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::table::schema::diff::*;
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, dec_opt, enc_indexed_triple, enc_opt, split_top_level, strip_brackets};
use crate::standards::v1::subsets::base::schema::triples::{IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::table::io::text::snapshot::{dec_cell_kind, enc_cell_kind};
use crate::standards::v1::subsets::value::io::text::diff::{dec_semio_value, dec_str, enc_semio_value, enc_str};
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableRow, SemioTableSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
/// 🧪️ Hand-rolled `protocol::DiffCodec`. Unlike `🔤️text` (one mutable field), `table` has TWO —
/// `print_diff` MUST stay ONE PHYSICAL LINE: present fields are joined with `;` (empty string when
/// neither present, `columns=[...]` alone, `rows=[...]` alone, or `columns=[...];rows=[...]` when
/// both present). `split_top_level(line, ';')` parses back (bracket-nesting aware, so a `;` can
/// never appear inside an encoded column/row's own hex/bracket payload — there is none — this is
/// purely a top-level field separator).
use crate::standards::v1::subsets::table::io::text::snapshot::{dec_row};
use crate::standards::v1::subsets::table::io::text::snapshot::{enc_row};
use crate::standards::v1::subsets::table::io::text::snapshot::{dec_column};
use crate::standards::v1::subsets::table::io::text::snapshot::{enc_column};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_column_diff(d: &SemioTableColumnDiff) -> String {
    format!("[{},{}]", enc_opt(d.name.as_ref(), |v| enc_str(v)), enc_opt(d.kind.as_ref(), |k| enc_cell_kind(*k).to_string()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_column_diff(s: &str) -> Result<SemioTableColumnDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, kind] = parts.as_slice() else { return Err(format!("column diff: expected 2 fields, got {}", parts.len())) };
    Ok(SemioTableColumnDiff { name: dec_opt(name, dec_str)?, kind: dec_opt(kind, dec_cell_kind)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_row_diff(d: &SemioTableRowDiff) -> String {
    format!("[{}]", enc_opt(d.cells.as_ref(), |cells| enc_indexed_triple(cells, |c| enc_semio_value(&c.value), enc_semio_value)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_row_diff(s: &str) -> Result<SemioTableRowDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [cells] = parts.as_slice() else { return Err(format!("row diff: expected 1 field, got {}", parts.len())) };
    Ok(SemioTableRowDiff { cells: dec_opt(cells, |triple| dec_indexed_triple(triple, |c| dec_semio_value(c).map(|value| Replace { value }), dec_semio_value))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_columns(columns: &IndexedTripleDiff<SemioTableColumnDiff, SemioTableColumn>) -> String {
    format!("[{}]", enc_indexed_triple(columns, enc_column_diff, enc_column))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_columns(s: &str) -> Result<IndexedTripleDiff<SemioTableColumnDiff, SemioTableColumn>, String> {
    dec_indexed_triple(strip_brackets(s)?, dec_column_diff, dec_column)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rows(rows: &IndexedTripleDiff<SemioTableRowDiff, SemioTableRow>) -> String {
    format!("[{}]", enc_indexed_triple(rows, enc_row_diff, enc_row))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rows(s: &str) -> Result<IndexedTripleDiff<SemioTableRowDiff, SemioTableRow>, String> {
    dec_indexed_triple(strip_brackets(s)?, dec_row_diff, dec_row)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_table_diff(d: &SemioTableDiff) -> String {
    let mut parts = Vec::new();
    if let Some(list) = &d.columns {
        parts.push(format!("columns={}", enc_columns(list)));
    }
    if let Some(list) = &d.rows {
        parts.push(format!("rows={}", enc_rows(list)));
    }
    parts.join(";")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_table_diff(line: &str) -> Result<SemioTableDiff, String> {
    if line.is_empty() {
        return Ok(SemioTableDiff::default());
    }
    let mut columns = None;
    let mut rows = None;
    for token in split_top_level(line, ';') {
        if let Some(rest) = token.strip_prefix("columns=") {
            columns = Some(dec_columns(rest)?);
        } else if let Some(rest) = token.strip_prefix("rows=") {
            rows = Some(dec_rows(rest)?);
        } else {
            return Err(format!("table diff: unknown token {token:?}"));
        }
    }
    Ok(SemioTableDiff { columns, rows })
}

impl protocol::DiffText for SemioTableDiff {
fn print_diff(&self) -> String {
    print_table_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_table_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
