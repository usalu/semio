//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `csv` reference implementation (reconfigured for IANA TSV: tab delimiter, no
//! quoting at all) so the subject's own mutation has an independent result to be compared against
//! instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it. This subset does not — IANA TSV has no
//! quoting/escaping mechanism at all (the shared stdio manifest's `csv` entry already covers RFC
//! 4180 CSV under a different configuration for a different capability), so its independent
//! reading/writing lives here rather than in the shared `tabular` module.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.
//! @see ../🧬️schema/📸️snapshot/🦀️.rs — the subject's own byte-exact split/rejoin codec
//! this module deliberately does NOT import (the oracle role must not link the subject crate).

use semio_repo_test_host::Json;

//#region 🔖️LineEnding
/// ↩️ Independent line-ending tag — NOT the subject's own `LineEnding`
/// (`../🧬️schema/📸️snapshot/🦀️.rs`): the oracle role must not link the subject crate at
/// all (fleet brief §5.3), so this is a standalone copy of the same two-value concept.
#[cfg(feature = "oracles")]
#[derive(Clone, Copy, PartialEq)]
enum TsvLineEnding {
    Lf,
    Crlf,
}

#[cfg(feature = "oracles")]
impl TsvLineEnding {
    fn as_str(self) -> &'static str {
        match self {
            TsvLineEnding::Lf => "\n",
            TsvLineEnding::Crlf => "\r\n",
        }
    }
    fn as_json(self) -> &'static str {
        match self {
            TsvLineEnding::Lf => "lf",
            TsvLineEnding::Crlf => "crlf",
        }
    }
    fn from_json(value: &str) -> Result<Self, String> {
        match value {
            "lf" => Ok(TsvLineEnding::Lf),
            "crlf" => Ok(TsvLineEnding::Crlf),
            other => Err(format!("unknown lineEnding {other:?}, expected \"lf\" or \"crlf\"")),
        }
    }
    /// 🔍️ Same detection rule as the subject's own `decode_tsv`: a literal `\r\n` anywhere marks
    /// the file CRLF, otherwise LF.
    fn sniff(text: &str) -> Self {
        if text.contains("\r\n") {
            TsvLineEnding::Crlf
        } else {
            TsvLineEnding::Lf
        }
    }
}
//#endregion 🔖️LineEnding

//#region 🔖️Body
/// 📊️ The whole-file state a byte-exact TSV split/rejoin needs: the cell grid plus the two
/// retention concerns `TsvMutation::SetTrailingNewline`/`SetLineEnding` mutate — REAL bytes here,
/// not cosmetic metadata: IANA TSV draws no header/data distinction for that role to fall to
/// instead, so this subset's own serialization choices are what its vocabulary makes substantive.
#[cfg(feature = "oracles")]
struct TsvBody {
    records: Vec<Vec<String>>,
    trailing_newline: bool,
    line_ending: TsvLineEnding,
}

/// 📥️ Independent IANA TSV read: tab-delimited, quoting OFF (`quoting(false)` — a `"` byte is
/// ordinary text, exactly as the format's own lack of an escaping mechanism requires), the
/// reader's default PERMISSIVE terminator so either a bare `\n` or a `\r\n` source splits into
/// records correctly regardless of which this file happens to use. `flexible(true)` because row
/// width is real per-row information, not a rectangular grid the format guarantees.
#[cfg(feature = "oracles")]
fn read_tsv(input: &[u8]) -> Result<TsvBody, String> {
    let text = std::str::from_utf8(input).map_err(|error| format!("independent reader: input is not UTF-8: {error}"))?;
    let line_ending = TsvLineEnding::sniff(text);
    let trailing_newline = !text.is_empty() && text.ends_with(line_ending.as_str());
    let mut reader = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(false).flexible(true).quoting(false).from_reader(input);
    let records =
        reader.records().map(|result| result.map(|record| record.iter().map(|cell| cell.to_string()).collect())).collect::<Result<Vec<Vec<String>>, _>>().map_err(|error| format!("independent reader could not read a TSV record: {error}"))?;
    Ok(TsvBody { records, trailing_newline, line_ending })
}

/// 📤️ Independent IANA TSV write: tab delimiter, `QuoteStyle::Never` — the format has no quoting
/// mechanism, so the reference writer must never invent one — and the body's OWN line-ending
/// choice as the record terminator. The reference writer always terminates every record it writes,
/// including the last, so a body that wants no trailing terminator has that final separator
/// stripped back off afterward.
#[cfg(feature = "oracles")]
fn write_tsv(body: &TsvBody) -> Result<Vec<u8>, String> {
    let terminator = match body.line_ending {
        TsvLineEnding::Lf => csv::Terminator::Any(b'\n'),
        TsvLineEnding::Crlf => csv::Terminator::CRLF,
    };
    let mut writer = csv::WriterBuilder::new().delimiter(b'\t').quote_style(csv::QuoteStyle::Never).terminator(terminator).from_writer(Vec::new());
    for record in &body.records {
        writer.write_record(record).map_err(|error| format!("tsv row: {error}"))?;
    }
    let mut bytes = writer.into_inner().map_err(|error| format!("tsv finish: {error}"))?;
    if !body.trailing_newline {
        let sep = body.line_ending.as_str().as_bytes();
        if bytes.ends_with(sep) {
            bytes.truncate(bytes.len() - sep.len());
        }
    }
    Ok(bytes)
}
//#endregion 🔖️Body

//#region 🔖️Grid
/// 📊️ Public, JSON-friendly mirror of [`TsvBody`] — what the sibling test case's adapter reads
/// real pre-mutation state out of (to build an inverse mutation's own spec) without depending on
/// this module's private `TsvLineEnding` type.
#[cfg(feature = "oracles")]
pub struct TsvGrid {
    pub records: Vec<Vec<String>>,
    pub trailing_newline: bool,
    pub line_ending: String,
}

#[cfg(feature = "oracles")]
pub fn read_grid(input: &[u8]) -> Result<TsvGrid, String> {
    let body = read_tsv(input)?;
    Ok(TsvGrid { records: body.records, trailing_newline: body.trailing_newline, line_ending: body.line_ending.as_json().to_string() })
}

#[cfg(feature = "oracles")]
pub fn write_grid(grid: &TsvGrid) -> Result<Vec<u8>, String> {
    write_tsv(&TsvBody { records: grid.records.clone(), trailing_newline: grid.trailing_newline, line_ending: TsvLineEnding::from_json(&grid.line_ending)? })
}
//#endregion 🔖️Grid

//#region 🔖️SpecReaders
#[cfg(feature = "oracles")]
fn mutation_params(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Null)
}
#[cfg(feature = "oracles")]
fn number(value: &Json, key: &str) -> Option<f64> {
    match value.get(key) {
        Some(Json::Number(number)) => Some(*number),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn boolean(value: &Json, key: &str) -> Option<bool> {
    match value.get(key) {
        Some(Json::Bool(flag)) => Some(*flag),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn strings(value: &Json, key: &str) -> Vec<String> {
    value
        .array(key)
        .iter()
        .map(|entry| match entry {
            Json::String(text) => text.clone(),
            _ => String::new(),
        })
        .collect()
}
#[cfg(feature = "oracles")]
fn rows(value: &Json, key: &str) -> Vec<Vec<String>> {
    value
        .array(key)
        .iter()
        .map(|row| match row {
            Json::Array(cells) => cells
                .iter()
                .map(|cell| match cell {
                    Json::String(text) => text.clone(),
                    _ => String::new(),
                })
                .collect(),
            _ => Vec::new(),
        })
        .collect()
}
#[cfg(feature = "oracles")]
fn line_ending_param(value: &Json, key: &str) -> Result<Option<TsvLineEnding>, String> {
    match value.get(key) {
        Some(Json::String(text)) => TsvLineEnding::from_json(text).map(Some),
        _ => Ok(None),
    }
}
//#endregion 🔖️SpecReaders

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = mutation_params(spec);
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => Ok(input.to_vec()),
        "set-snapshot" => write_tsv(&TsvBody { records: rows(&params, "records"), trailing_newline: boolean(&params, "trailingNewline").unwrap_or(true), line_ending: line_ending_param(&params, "lineEnding")?.unwrap_or(TsvLineEnding::Lf) }),
        "set-trailing-newline" => {
            let mut body = read_tsv(input)?;
            body.trailing_newline = boolean(&params, "trailingNewline").ok_or("set-trailing-newline: missing `trailingNewline`")?;
            write_tsv(&body)
        }
        "set-line-ending" => {
            let mut body = read_tsv(input)?;
            body.line_ending = line_ending_param(&params, "lineEnding")?.ok_or("set-line-ending: missing `lineEnding`")?;
            write_tsv(&body)
        }
        "insert-row" => {
            let mut body = read_tsv(input)?;
            let index = number(&params, "index").ok_or("insert-row: missing `index`")? as usize;
            body.records.insert(index.min(body.records.len()), strings(&params, "row"));
            write_tsv(&body)
        }
        "remove-row" => {
            let mut body = read_tsv(input)?;
            let index = number(&params, "index").ok_or("remove-row: missing `index`")? as usize;
            if index >= body.records.len() {
                return Err(format!("remove-row: index {index} out of bounds ({} row(s))", body.records.len()));
            }
            body.records.remove(index);
            write_tsv(&body)
        }
        "set-cell" => {
            let mut body = read_tsv(input)?;
            let row_index = number(&params, "rowIndex").ok_or("set-cell: missing `rowIndex`")? as usize;
            let field_index = number(&params, "fieldIndex").ok_or("set-cell: missing `fieldIndex`")? as usize;
            let value = params.str("value");
            let row_count = body.records.len();
            let row = body.records.get_mut(row_index).ok_or_else(|| format!("set-cell: row index {row_index} out of bounds ({row_count} row(s))"))?;
            if field_index >= row.len() {
                row.resize(field_index + 1, String::new());
            }
            row[field_index] = value;
            write_tsv(&body)
        }
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Projection
/// 👁️ Projects TSV bytes with the INDEPENDENT reader onto the `semantic-tabular-mutate-v1` shape
/// this case's oracle and subject are both compared through. Unlike RFC 4180 CSV's `has_header`
/// (caller-tracked metadata the bytes never carry), `trailingNewline`/`lineTerminator` here are
/// read straight OUT of the bytes — IANA TSV draws no header/data distinction, so this subset's
/// vocabulary makes its own real serialization concerns substantive instead, and the catalog's own
/// `semantic-tabular-mutate-v1` profile keeps both fields live in the comparison rather than
/// ignoring them the way the base `semantic-tabular-v1` profile does for RFC 4180's writer-freedom
/// equivalents.
#[cfg(feature = "oracles")]
pub fn project_tsv_grid(bytes: &[u8]) -> Result<Json, String> {
    let body = read_tsv(bytes)?;
    Ok(Json::Object(vec![
        ("format".to_string(), Json::String("tsv".to_string())),
        ("recordCount".to_string(), Json::Number(body.records.len() as f64)),
        ("records".to_string(), Json::Array(body.records.into_iter().map(|record| Json::Array(record.into_iter().map(Json::String).collect())).collect())),
        ("trailingNewline".to_string(), Json::Bool(body.trailing_newline)),
        ("lineTerminator".to_string(), Json::String(body.line_ending.as_json().to_string())),
    ]))
}

#[cfg(not(feature = "oracles"))]
pub fn project_tsv_grid(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Projection

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    fn spec(kind: &str, params: Json) -> Json {
        Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
    }

    #[test]
    fn no_mutation_is_a_true_byte_identity() {
        let input = b"a\tb\r\n1\t2\r\n";
        let output = oracle_apply_mutation(input, &spec("no-mutation", Json::Object(vec![]))).unwrap();
        assert_eq!(output, input);
    }

    #[test]
    fn insert_and_remove_row_are_inverse_on_a_real_shaped_grid() {
        let input = b"id\tname\n1\tAlpha\n2\tBeta\n";
        let inserted = oracle_apply_mutation(input, &spec("insert-row", Json::Object(vec![("index".to_string(), Json::Number(1.0)), ("row".to_string(), Json::Array(vec![Json::String("9".to_string()), Json::String("Neu".to_string())]))]))).unwrap();
        assert_eq!(read_tsv(&inserted).unwrap().records, vec![vec!["id".to_string(), "name".to_string()], vec!["9".to_string(), "Neu".to_string()], vec!["1".to_string(), "Alpha".to_string()], vec!["2".to_string(), "Beta".to_string()]]);

        let removed = oracle_apply_mutation(&inserted, &spec("remove-row", Json::Object(vec![("index".to_string(), Json::Number(1.0))]))).unwrap();
        assert_eq!(read_tsv(&removed).unwrap().records, read_tsv(input).unwrap().records);
    }

    #[test]
    fn set_cell_patches_a_single_cell_with_no_quoting_invented() {
        let input = b"id\tnote\n1\tplain\n";
        let output =
            oracle_apply_mutation(input, &spec("set-cell", Json::Object(vec![("rowIndex".to_string(), Json::Number(1.0)), ("fieldIndex".to_string(), Json::Number(1.0)), ("value".to_string(), Json::String("has, comma and \"quote\"".to_string()))])))
                .unwrap();
        let text = String::from_utf8(output.clone()).unwrap();
        assert!(!text.contains('"') || text.contains("\"quote\""), "the value must survive verbatim, no quoting invented, got {text:?}");
        assert_eq!(read_tsv(&output).unwrap().records[1][1], "has, comma and \"quote\"");
    }

    #[test]
    fn set_trailing_newline_genuinely_adds_and_removes_bytes() {
        let input = b"a\tb\n1\t2\n";
        let stripped = oracle_apply_mutation(input, &spec("set-trailing-newline", Json::Object(vec![("trailingNewline".to_string(), Json::Bool(false))]))).unwrap();
        assert_eq!(stripped, b"a\tb\n1\t2");

        let restored = oracle_apply_mutation(&stripped, &spec("set-trailing-newline", Json::Object(vec![("trailingNewline".to_string(), Json::Bool(true))]))).unwrap();
        assert_eq!(restored, input);
    }

    #[test]
    fn set_line_ending_genuinely_rewrites_every_terminator() {
        let input = b"a\tb\n1\t2\n";
        let output = oracle_apply_mutation(input, &spec("set-line-ending", Json::Object(vec![("lineEnding".to_string(), Json::String("crlf".to_string()))]))).unwrap();
        assert_eq!(output, b"a\tb\r\n1\t2\r\n");
    }

    #[test]
    fn project_tsv_grid_carries_trailing_newline_and_line_terminator_live() {
        let lf = project_tsv_grid(b"a\tb\n1\t2\n").unwrap();
        let crlf_no_trailing = project_tsv_grid(b"a\tb\r\n1\t2").unwrap();
        assert_eq!(lf.str("format"), "tsv");
        assert_ne!(lf, crlf_no_trailing, "trailingNewline/lineTerminator must actually participate in the projection");
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let input = b"a\tb\n1\t2\n";
        let result = oracle_apply_mutation(input, &spec("not-a-real-kind", Json::Object(vec![])));
        assert!(result.is_err(), "an unrecognised kind must fail loudly");
    }
}
//#endregion 🧪️Tests
