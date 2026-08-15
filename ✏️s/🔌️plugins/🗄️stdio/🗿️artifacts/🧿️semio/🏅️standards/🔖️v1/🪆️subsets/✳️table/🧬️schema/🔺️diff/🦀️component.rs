//! 🔺️ SemioTableDiff — sparse per-field diff over `SemioTableSnapshot`. `table` has TWO mutable
//! fields (`columns`, `rows` — both intrinsically ordered, anonymous collections per
//! `📓️taxonomy.md`'s addressing rule #3 for `rows`, and rule #2 name-keyed for `columns`), so the
//! diff carries two `Option<…>` slots: whole-list wrappers rebuilt POSITIONALLY from `base` by
//! each mutation triad's own `🔺️diff` leaf (never a generic `between()` re-derivation) — same
//! shape `✳️text`'s own `SemioTextDiff` uses for its single `runs` field. No
//! `snapshot: Option<SemioTableSnapshot>` full-replace slot anywhere — whole-document replace is
//! `ArtifactStore::reset`, outside history.

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::split_top_level;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableRow, SemioTableSnapshot};
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️ColumnList
/// 📋 Whole-list wrapper for the `columns` field diff — every mutation triad rebuilds the full
/// ordered `values` vec from `base` and wraps it here (`SemioTextRunList`'s own shape).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SemioTableColumnList {
    pub values: Vec<SemioTableColumn>,
}
//#endregion 🔖️ColumnList

//#region 🔖️RowList
/// 📋 Whole-list wrapper for the `rows` field diff — same shape as [`SemioTableColumnList`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SemioTableRowList {
    pub values: Vec<SemioTableRow>,
}
//#endregion 🔖️RowList

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.table.diff")]
pub struct SemioTableDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<SemioTableColumnList>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<SemioTableRowList>,
}

impl SemioTableDiff {
    pub fn is_empty_diff(&self) -> bool {
        self.columns.is_none() && self.rows.is_none()
    }
}

impl MutationDiff<SemioTableSnapshot> for SemioTableDiff {
    fn apply(&self, base: &SemioTableSnapshot) -> SemioTableSnapshot {
        let mut next = base.clone();
        if let Some(list) = &self.columns {
            next.columns = list.values.clone();
        }
        if let Some(list) = &self.rows {
            next.rows = list.values.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.columns.is_some() {
            self.columns = other.columns;
        }
        if other.rows.is_some() {
            self.rows = other.rows;
        }
    }
}

/// 🧮️ `table`'s own `DiffAlgebra` — required by the `✳️any` envelope's own dispatch (`SemioDiff`
/// delegates `between`/`inverse`/`is_empty` straight through to every wrapped subset's own impl).
/// Whole-list `between`/`inverse` are honest here (not apply-then-capture): a change is fully
/// described by "the new/old `columns`/`rows` value", same shape every mutation triad's own
/// `🔺️diff` leaf already produces.
impl protocol::command::DiffAlgebra<SemioTableSnapshot> for SemioTableDiff {
    fn between(base: &SemioTableSnapshot, other: &SemioTableSnapshot) -> Self {
        SemioTableDiff { columns: (base.columns != other.columns).then(|| SemioTableColumnList { values: other.columns.clone() }), rows: (base.rows != other.rows).then(|| SemioTableRowList { values: other.rows.clone() }) }
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Self {
        SemioTableDiff { columns: self.columns.as_ref().map(|_| SemioTableColumnList { values: base.columns.clone() }), rows: self.rows.as_ref().map(|_| SemioTableRowList { values: base.rows.clone() }) }
    }
    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec`. Unlike `✳️text` (one mutable field), `table` has TWO —
/// `print_diff` MUST stay ONE PHYSICAL LINE: present fields are joined with `;` (empty string when
/// neither present, `columns=[...]` alone, `rows=[...]` alone, or `columns=[...];rows=[...]` when
/// both present). `split_top_level(line, ';')` parses back (bracket-nesting aware, so a `;` can
/// never appear inside an encoded column/row's own hex/bracket payload — there is none — this is
/// purely a top-level field separator).
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{dec_column, dec_row, enc_column, enc_row};

fn enc_columns(list: &SemioTableColumnList) -> String {
    format!("[{}]", list.values.iter().map(enc_column).collect::<Vec<_>>().join(","))
}
fn dec_columns(s: &str) -> Result<SemioTableColumnList, String> {
    use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::strip_brackets;
    let values = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_column).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioTableColumnList { values })
}
fn enc_rows(list: &SemioTableRowList) -> String {
    format!("[{}]", list.values.iter().map(enc_row).collect::<Vec<_>>().join(","))
}
fn dec_rows(s: &str) -> Result<SemioTableRowList, String> {
    use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::strip_brackets;
    let values = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_row).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioTableRowList { values })
}

fn print_table_diff(d: &SemioTableDiff) -> String {
    let mut parts = Vec::new();
    if let Some(list) = &d.columns {
        parts.push(format!("columns={}", enc_columns(list)));
    }
    if let Some(list) = &d.rows {
        parts.push(format!("rows={}", enc_rows(list)));
    }
    parts.join(";")
}
fn parse_table_diff(line: &str) -> Result<SemioTableDiff, String> {
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

impl protocol::DiffCodec for SemioTableDiff {
    fn print_diff(&self) -> String {
        print_table_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_table_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    /// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=`columns`, bit1=`rows`) are
    /// two REAL fixed fields; each present section follows as a real varint count + per-item
    /// binary encoding (reusing the snapshot facet's own `write_column`/`read_column` and the
    /// value subset's own `enc_semio_value_bin`/`dec_semio_value_bin` for row cells).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::write_column;
        use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::enc_semio_value_bin;
        let presence: u8 = (if self.columns.is_some() { 0b0000_0001 } else { 0 }) | (if self.rows.is_some() { 0b0000_0010 } else { 0 });
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(list) = &self.columns {
            store::pack_rt::write_varint_u64(&mut out, list.values.len() as u64);
            for c in &list.values {
                write_column(&mut out, c);
            }
        }
        if let Some(list) = &self.rows {
            store::pack_rt::write_varint_u64(&mut out, list.values.len() as u64);
            for r in &list.values {
                store::pack_rt::write_varint_u64(&mut out, r.cells.len() as u64);
                for cell in &r.cells {
                    enc_semio_value_bin(cell, &mut out);
                }
            }
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{read_column, SemioTableRow};
        use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::dec_semio_value_bin;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
        }
        if bytes[0] != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
        }
        let presence = bytes[1];
        let mut reader = store::ByteReader::new(&bytes[2..]);
        let columns = if presence & 0b0000_0001 != 0 {
            let count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff columns count", offset: 2, detail: e.to_string() })?;
            let mut values = Vec::with_capacity(count as usize);
            for _ in 0..count {
                values.push(read_column(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff column", offset: 2, detail: e })?);
            }
            Some(SemioTableColumnList { values })
        } else {
            None
        };
        let rows = if presence & 0b0000_0010 != 0 {
            let count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff rows count", offset: 2, detail: e.to_string() })?;
            let mut values = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let cell_count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff row cell count", offset: 2, detail: e.to_string() })?;
                let mut cells = Vec::with_capacity(cell_count as usize);
                for _ in 0..cell_count {
                    cells.push(dec_semio_value_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff cell", offset: 2, detail: e })?);
                }
                values.push(SemioTableRow { cells });
            }
            Some(SemioTableRowList { values })
        } else {
            None
        };
        Ok(SemioTableDiff { columns, rows })
    }
}
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioTableDiff` cases — single source of truth for `diff_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️component.rs`. Covers: empty, columns-only, rows-only, and
/// both-present (the two-optional-field `;`-joined `print_diff` shape).
#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<SemioTableDiff> {
    use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{demo_table_snapshot, SemioTableCellKind, SemioTableColumn, SemioTableRow};
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    vec![
        SemioTableDiff::default(),
        SemioTableDiff { columns: Some(SemioTableColumnList { values: demo_table_snapshot().columns }), rows: None },
        SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: demo_table_snapshot().rows }) },
        SemioTableDiff {
            columns: Some(SemioTableColumnList { values: vec![SemioTableColumn { name: "extra".into(), kind: SemioTableCellKind::Int }] }),
            rows: Some(SemioTableRowList { values: vec![SemioTableRow { cells: vec![SemioValue::Int { lexeme: "7".into() }] }] }),
        },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    use protocol::DiffCodec;

    fn one_col_row(name: &str, kind: SemioTableCellKind, value: SemioValue) -> SemioTableSnapshot {
        SemioTableSnapshot { schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(), columns: vec![SemioTableColumn { name: name.into(), kind }], rows: vec![SemioTableRow { cells: vec![value] }] }
    }

    #[test]
    fn apply_replaces_columns_and_rows_wholesale() {
        let base = one_col_row("a", SemioTableCellKind::Str, SemioValue::Str { value: "x".into() });
        let diff = SemioTableDiff {
            columns: Some(SemioTableColumnList { values: vec![SemioTableColumn { name: "b".into(), kind: SemioTableCellKind::Int }] }),
            rows: Some(SemioTableRowList { values: vec![SemioTableRow { cells: vec![SemioValue::Int { lexeme: "1".into() }] }] }),
        };
        let next = diff.apply(&base);
        assert_eq!(next.columns[0].name, "b");
        assert_eq!(next.rows[0].cells[0], SemioValue::Int { lexeme: "1".into() });
    }

    #[test]
    fn absorb_last_write_wins() {
        let mut d1 = SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: vec![SemioTableRow { cells: vec![SemioValue::Str { value: "a".into() }] }] }) };
        let d2 = SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: vec![SemioTableRow { cells: vec![SemioValue::Str { value: "b".into() }] }] }) };
        d1.absorb(d2.clone());
        assert_eq!(d1, d2);
    }

    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioTableDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioTableDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }

    #[test]
    fn print_diff_joins_both_fields_with_semicolon_on_one_line() {
        let d = &demo_diff_cases()[3];
        let printed = d.print_diff();
        assert!(printed.contains(';'), "expected both-present diff to join with ';', got {printed:?}");
        assert_eq!(printed.matches('\n').count(), 0);
    }
}
//#endregion 🔖️Tests
