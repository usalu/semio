//! 📝️ Text representation codec surface for `stdio.semio.table` (diff) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::table::schema::diff::*;
use crate::standards::v_rfc8259::subsets::base::io::text::diff::split_top_level;
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableRow, SemioTableSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
/// 🧪️ Hand-rolled `protocol::DiffCodec`. Unlike `🔤️text` (one mutable field), `table` has TWO —
/// `print_diff` MUST stay ONE PHYSICAL LINE: present fields are joined with `;` (empty string when
/// neither present, `columns=[...]` alone, `rows=[...]` alone, or `columns=[...];rows=[...]` when
/// both present). `split_top_level(line, ';')` parses back (bracket-nesting aware, so a `;` can
/// never appear inside an encoded column/row's own hex/bracket payload — there is none — this is
/// purely a top-level field separator).
use crate::document::io::text::diff::{dec_row};
use crate::document::io::text::diff::{enc_row};
use crate::standards::v1::subsets::table::io::text::snapshot::{dec_column};
use crate::standards::v1::subsets::table::io::text::snapshot::{enc_column};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_columns(list: &SemioTableColumnList) -> String {
    format!("[{}]", list.values.iter().map(enc_column).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_columns(s: &str) -> Result<SemioTableColumnList, String> {
    use crate::standards::v_rfc8259::subsets::base::io::text::diff::strip_brackets;
    let values = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_column).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioTableColumnList { values })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rows(list: &SemioTableRowList) -> String {
    format!("[{}]", list.values.iter().map(enc_row).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rows(s: &str) -> Result<SemioTableRowList, String> {
    use crate::standards::v_rfc8259::subsets::base::io::text::diff::strip_brackets;
    let values = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_row).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioTableRowList { values })
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
