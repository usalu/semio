//! 🧬️ SemioTableSnapshot — the neutral tabular interchange shape: named/typed columns plus rows
//! of scalar cells (what csv/tsv/xlsx eventually map onto). LEAF subset (no child slots, no link
//! slots) per the master plan's stdio target vocabulary (ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM,
//! W2b/table).
//!
//! Modeled directly on `✳️text`'s hand-rolled `ArtifactDsl`/`ArtifactPack` convention (real
//! hex/bracket text codec + real varint-length-prefixed binary codec, both wrapped in the shared
//! `store::semio_format` envelope). Cell VALUES reuse `✳️value`'s `SemioValue` scalar vocabulary
//! directly (`SemioTableRow.cells: Vec<SemioValue>`) rather than re-deriving a second scalar type —
//! `SemioTableCellKind` only mirrors `SemioValue`'s scalar variant NAMES as a declared column-type
//! tag (`Null`/`Bool`/`Int`/`Float`/`Str`/`Bytes` — no `List`/`Map`/`Ref`, a column's cells are
//! meant to stay scalar).
//!
//! CRITICAL INVARIANT: `rows[i].cells` is POSITIONALLY ALIGNED with `columns` (`cells[j]` belongs
//! to `columns[j]`). Every column insert/remove/reorder mutation applies the IDENTICAL
//! insert/remove/reorder, at the IDENTICAL index, to every row's `cells` — see
//! `🧬️mutations/🏗️create-column`/`🗑️delete-column`/`🔀reorder-columns`.

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::{dec_semio_value, enc_semio_value, enc_str, dec_str, write_str_lp, read_str_lp};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id AND `ArtifactSchema` descriptor id — same literal for
/// both, per the master plan's "Schema descriptor ids `s.stdio.semio` + `s.stdio.semio.<subset>`"
/// convention, one per subset.
pub const STDIO_SEMIOTABLE_DOCUMENT_SCHEMA: &str = "s.stdio.semio.table";
//#endregion 🔖️Ids

//#region 🔖️CellKind
/// 🏷️ The declared column-type tag — mirrors `SemioValue`'s SCALAR variant names only (`Null`/
/// `Bool`/`Int`/`Float`/`Str`/`Bytes`; no `List`/`Map`/`Ref` — a column's cells are meant to be
/// scalar). Text tags: `n`/`b`/`i`/`f`/`s`/`y`. Binary tags: `0`-`5` in declaration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SemioTableCellKind {
    #[default]
    Null,
    Bool,
    Int,
    Float,
    Str,
    Bytes,
}
//#endregion 🔖️CellKind

//#region 🔖️Column
/// 🏛️ One declared column: `name` is the NATIVE KEY (name-keyed collection — like cad layers/xlsx
/// sheets, `📓️taxonomy.md`'s addressing rule #2) plus its declared `kind` tag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemioTableColumn {
    pub name: String,
    pub kind: SemioTableCellKind,
}
//#endregion 🔖️Column

//#region 🔖️Row
/// 🧾️ One row: `cells` is POSITIONALLY ALIGNED with `columns` (see this module's own doc comment
/// for the invariant). Rows themselves are index-addressed (no stable id — an intrinsically
/// ordered, anonymous collection, `📓️taxonomy.md` addressing rule #3), the same shape
/// `insert-row`/`remove-row`/`reorder-rows` operate on. `SemioValue` (from `✳️value`) is reused
/// verbatim for cell data — real reuse, not reinvention.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemioTableRow {
    #[serde(default)]
    pub cells: Vec<SemioValue>,
}
//#endregion 🔖️Row

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.table")]
pub struct SemioTableSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub columns: Vec<SemioTableColumn>,
    #[state(persistent)]
    #[serde(default)]
    pub rows: Vec<SemioTableRow>,
}

impl Default for SemioTableSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(), columns: Vec::new(), rows: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TablePrimitives
/// 🧪️ Table-specific hex/bracket value primitives backing the hand-rolled `ArtifactDsl` below —
/// the general-purpose hex/string primitives (`enc_str`/`dec_str`) and the scalar `SemioValue`
/// codec (`enc_semio_value`/`dec_semio_value`) are IMPORTED from `✳️value`'s own `🔺️diff` module
/// (never re-derived, per this ticket's binding reuse mandate) — only the column/row shapes that
/// are genuinely local to `table` are defined here.
fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}

pub(crate) fn enc_cell_kind(k: SemioTableCellKind) -> char {
    match k {
        SemioTableCellKind::Null => 'n',
        SemioTableCellKind::Bool => 'b',
        SemioTableCellKind::Int => 'i',
        SemioTableCellKind::Float => 'f',
        SemioTableCellKind::Str => 's',
        SemioTableCellKind::Bytes => 'y',
    }
}
pub(crate) fn dec_cell_kind(s: &str) -> Result<SemioTableCellKind, String> {
    match s {
        "n" => Ok(SemioTableCellKind::Null),
        "b" => Ok(SemioTableCellKind::Bool),
        "i" => Ok(SemioTableCellKind::Int),
        "f" => Ok(SemioTableCellKind::Float),
        "s" => Ok(SemioTableCellKind::Str),
        "y" => Ok(SemioTableCellKind::Bytes),
        other => Err(format!("bad cell kind {other:?}")),
    }
}
pub(crate) fn enc_column(c: &SemioTableColumn) -> String {
    format!("[{},{}]", enc_str(&c.name), enc_cell_kind(c.kind))
}
pub(crate) fn dec_column(s: &str) -> Result<SemioTableColumn, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, kind] = parts.as_slice() else { return Err(format!("column: expected 2 fields, got {}", parts.len())) };
    Ok(SemioTableColumn { name: dec_str(name)?, kind: dec_cell_kind(kind)? })
}
pub(crate) fn enc_row(r: &SemioTableRow) -> String {
    enc_list(&r.cells, enc_semio_value)
}
pub(crate) fn dec_row(s: &str) -> Result<SemioTableRow, String> {
    Ok(SemioTableRow { cells: dec_list(s, dec_semio_value)? })
}

/// 📄️ The real structured text body: three lines — `schema=<hex>`, `columns=[<col>,...]`,
/// `rows=[<row>,...]` — matching the grammar's `document = artifact-mark schema-line columns-line
/// rows-line`. Newlines are pure lexer trivia in the shared dialect, so this is genuinely
/// recognizable by `dsl::Recognizer`, not merely readable.
fn print_table_snapshot_body(s: &SemioTableSnapshot) -> String {
    format!("schema={}\ncolumns={}\nrows={}", enc_str(&s.schema), enc_list(&s.columns, enc_column), enc_list(&s.rows, enc_row))
}
fn parse_table_snapshot_body(body: &str) -> Result<SemioTableSnapshot, String> {
    let mut schema = None;
    let mut columns = Vec::new();
    let mut rows = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("columns=") {
            columns = dec_list(rest, dec_column)?;
        } else if let Some(rest) = line.strip_prefix("rows=") {
            rows = dec_list(rest, dec_row)?;
        } else {
            return Err(format!("semio table snapshot: unknown line {line:?}"));
        }
    }
    Ok(SemioTableSnapshot { schema: schema.ok_or_else(|| "semio table snapshot: missing schema line".to_string())?, columns, rows })
}
//#endregion 🔖️TablePrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ `write_str_lp`/`read_str_lp` are IMPORTED from `✳️value`'s own `🔺️diff` module (real
/// LEB128-varint-length-prefixed binary primitives, `store::pack_rt::write_varint_u64`/
/// `store::ByteReader`-backed) — reused, not re-derived.
pub(crate) fn write_column(out: &mut Vec<u8>, c: &SemioTableColumn) {
    write_str_lp(out, &c.name);
    out.push(match c.kind {
        SemioTableCellKind::Null => 0,
        SemioTableCellKind::Bool => 1,
        SemioTableCellKind::Int => 2,
        SemioTableCellKind::Float => 3,
        SemioTableCellKind::Str => 4,
        SemioTableCellKind::Bytes => 5,
    });
}
pub(crate) fn read_column(reader: &mut store::ByteReader<'_>) -> Result<SemioTableColumn, String> {
    let name = read_str_lp(reader)?;
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    let kind = match tag {
        0 => SemioTableCellKind::Null,
        1 => SemioTableCellKind::Bool,
        2 => SemioTableCellKind::Int,
        3 => SemioTableCellKind::Float,
        4 => SemioTableCellKind::Str,
        5 => SemioTableCellKind::Bytes,
        other => return Err(format!("unsupported cell kind tag {other}")),
    };
    Ok(SemioTableColumn { name, kind })
}
pub(crate) fn write_row(out: &mut Vec<u8>, r: &SemioTableRow) {
    store::pack_rt::write_varint_u64(out, r.cells.len() as u64);
    for cell in &r.cells {
        crate::artifacts::semio::standards::v1::subsets::value::schema::diff::enc_semio_value_bin(cell, out);
    }
}
pub(crate) fn read_row(reader: &mut store::ByteReader<'_>) -> Result<SemioTableRow, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut cells = Vec::with_capacity(count as usize);
    for _ in 0..count {
        cells.push(crate::artifacts::semio::standards::v1::subsets::value::schema::diff::dec_semio_value_bin(reader)?);
    }
    Ok(SemioTableRow { cells })
}

/// 🎁 `format u8` + varint-length-prefixed `schema` UTF-8 — both genuinely, individually
/// protocol-walkable, matching `📡️component.protocol.semio`'s header/segment fields exactly —
/// then `columns`/`rows` (varint counts + per-item real recursive encodings) as the honest opaque
/// `payload` tail (`protocol-array-of-records` gap — homogeneous, variable-length repeated
/// records), same boundary `✳️text`'s own snapshot binary uses for `runs`.
fn encode_table_snapshot_binary(s: &SemioTableSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.columns.len() as u64);
    for c in &s.columns {
        write_column(&mut out, c);
    }
    store::pack_rt::write_varint_u64(&mut out, s.rows.len() as u64);
    for r in &s.rows {
        write_row(&mut out, r);
    }
    out
}
fn decode_table_snapshot_binary(bytes: &[u8]) -> Result<SemioTableSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let column_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut columns = Vec::with_capacity(column_count as usize);
    for _ in 0..column_count {
        columns.push(read_column(&mut reader)?);
    }
    let row_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut rows = Vec::with_capacity(row_count as usize);
    for _ in 0..row_count {
        rows.push(read_row(&mut reader)?);
    }
    Ok(SemioTableSnapshot { schema, columns, rows })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs, wrapped in the repo-wide `store::semio_format` envelope.
impl store::ArtifactDsl for SemioTableSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOTABLE_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_table_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_table_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioTableSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_table_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_table_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.table` document — three columns (`label: Str`, `score: Float`,
/// `active: Bool`) across three rows, exercising every `SemioTableCellKind`/`SemioValue` scalar
/// variant at least once (`Str`/`Float`/`Bool` via the declared column kinds; `Null`/`Int`/`Bytes`
/// via cell values — a cell's actual `SemioValue` kind is independent of its column's declared
/// tag, no runtime enforcement, matching a lenient real-world tabular format). Single source of
/// truth for `📚️examples/📃️sheet/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` and for the
/// conformance-law tests in `🚪️io/🦀️component.rs`.
#[cfg(test)]
pub(crate) fn demo_table_snapshot() -> SemioTableSnapshot {
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![
            SemioTableColumn { name: "label".into(), kind: SemioTableCellKind::Str },
            SemioTableColumn { name: "score".into(), kind: SemioTableCellKind::Float },
            SemioTableColumn { name: "active".into(), kind: SemioTableCellKind::Bool },
        ],
        rows: vec![
            SemioTableRow { cells: vec![SemioValue::Str { value: "widget".into() }, SemioValue::Float { lexeme: "3.500".into() }, SemioValue::Bool { value: true }] },
            SemioTableRow { cells: vec![SemioValue::Null, SemioValue::Int { lexeme: "42".into() }, SemioValue::Bytes { value: vec![0, 1, 2, 255] }] },
            SemioTableRow { cells: vec![SemioValue::Str { value: "gadget".into() }, SemioValue::Float { lexeme: "1.250".into() }, SemioValue::Bool { value: false }] },
        ],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn populated() -> SemioTableSnapshot {
        demo_table_snapshot()
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = SemioTableSnapshot::default();
        let bytes = <SemioTableSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioTableSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = SemioTableSnapshot::default();
        let text = <SemioTableSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioTableSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law: decode(encode(snapshot)) is byte-for-byte structurally identical
    /// on a fully-populated snapshot (columns/rows non-empty), not just the default. Also asserts
    /// the CRITICAL row/column alignment invariant survives a round trip untouched.
    #[test]
    fn codec_retention_law() {
        let snap = populated();
        let bytes = <SemioTableSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioTableSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
        for row in &back.rows {
            assert_eq!(row.cells.len(), back.columns.len(), "row/column alignment must survive a pack round trip");
        }
        let text = <SemioTableSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <SemioTableSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }
}
//#endregion 🔖️Tests
