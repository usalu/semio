//! 📝️ Text representation codec surface for `stdio.semio.table` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::table::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use crate::value::io::text::diff::{dec_semio_value};
use crate::value::io::text::diff::{enc_semio_value};
use crate::standards::v1::subsets::drawing::io::binary::snapshot::{read_str_lp};
use crate::standards::v1::subsets::drawing::io::binary::snapshot::{write_str_lp};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;
use framework_schema::ArtifactSchema;

/// 🧪️ Table-specific hex/bracket value primitives backing the hand-rolled `ArtifactDsl` below —
/// the general-purpose hex/string primitives (`enc_str`/`dec_str`) and the scalar `SemioValue`
/// codec (`enc_semio_value`/`dec_semio_value`) are IMPORTED from `🔢️value`'s own `🔺️diff` module
/// (never re-derived, per this ticket's binding reuse mandate) — only the column/row shapes that
/// are genuinely local to `table` are defined here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_column(c: &SemioTableColumn) -> String {
    format!("[{},{}]", enc_str(&c.name), enc_cell_kind(c.kind))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_column(s: &str) -> Result<SemioTableColumn, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, kind] = parts.as_slice() else { return Err(format!("column: expected 2 fields, got {}", parts.len())) };
    Ok(SemioTableColumn { name: dec_str(name)?, kind: dec_cell_kind(kind)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_row(r: &SemioTableRow) -> String {
    enc_list(&r.cells, enc_semio_value)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_row(s: &str) -> Result<SemioTableRow, String> {
    Ok(SemioTableRow { cells: dec_list(s, dec_semio_value)? })
}

/// 📄️ The real structured text body: three lines — `schema=<hex>`, `columns=[<col>,...]`,
/// `rows=[<row>,...]` — matching the grammar's `document = artifact-mark schema-line columns-line
/// rows-line`. Newlines are pure lexer trivia in the shared dialect, so this is genuinely
/// recognizable by `dsl::Recognizer`, not merely readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_table_snapshot_body(s: &SemioTableSnapshot) -> String {
    format!("schema={}\ncolumns={}\nrows={}", enc_str(&s.schema), enc_list(&s.columns, enc_column), enc_list(&s.rows, enc_row))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_table_snapshot_body(body: &str) -> Result<SemioTableSnapshot, String> {
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

/// 🎁 Real structured text/binary codecs, wrapped in the repo-wide `store::semio_format` envelope.
impl store::ArtifactDsl for SemioTableSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOTABLE_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_table_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_table_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📤️ This subset's own `#[value(rename_all = "camelCase")]` structural JSON projection of
/// `s.stdio.semio.table` — the shape `📊️mutate-semio-table` compares under `ordered-json-v1`, derived from the
/// snapshot type itself rather than hand-written a second time in the adapter, where it could drift
/// away from the type it claims to project. Cells are a discriminated value union rather than plain scalars, and rows are positional, so
/// the projection has to preserve both the cell tagging and the row order the fixtures were authored
/// in.
/// A thin `pack::to_json_string` wrapper (first-party, over `ToValue`/`DslValue`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_table_snapshot_json(snapshot: &SemioTableSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_table_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`SemioTableSnapshot`] values, so `📊️mutate-semio-table`'s adapter reads the
/// committed fixture instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_table_snapshot_json(text: &str) -> Result<SemioTableSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📝️ Parses `s.stdio.semio.table` DSL text into a [`SemioTableSnapshot`] — a named pass-through of this snapshot's own
/// `store::ArtifactDsl` impl above, whose trait and error type are both unnameable outside this
/// crate, so `📊️mutate-semio-table`'s `identity-round-trip` scenario reaches the real committed
/// artifact (`../../🖼️assets/📃️sheet/🗣️.dsl.semio`) through this instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_table_dsl(text: &str) -> Result<SemioTableSnapshot, String> {
    <SemioTableSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📝️ Renders a [`SemioTableSnapshot`] back as `s.stdio.semio.table` DSL text — the inverse of
/// [`parse_semio_table_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_table_dsl(snapshot: &SemioTableSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::table::schema::snapshot::*;
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::value::io::text::diff::{dec_semio_value};
use crate::value::io::text::diff::{enc_semio_value};
use crate::standards::v1::subsets::drawing::io::binary::snapshot::{read_str_lp};
use crate::standards::v1::subsets::drawing::io::binary::snapshot::{write_str_lp};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
