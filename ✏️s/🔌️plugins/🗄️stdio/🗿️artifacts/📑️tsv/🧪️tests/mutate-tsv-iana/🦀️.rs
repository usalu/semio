//! 🦀️ TSV IANA exhaustive mutation round-trip case — Rust adapter.
//!
//! Every scenario copies the immutable real fixture into the case work directory first; the
//! committed file is never written to. `oracle` handlers drive the registered `csv` reference
//! implementation (reconfigured for IANA TSV via this subset's own `🧪️oracle/🦀️component.rs`),
//! `subject` handlers drive this repository's own decode/mutate/encode round trip, and both results
//! are read back by the SAME independent reader (`project_tsv_grid`) before the
//! `semantic-tabular-mutate-v1` profile compares them. The subject half is gated behind the
//! generated host's `sut` feature so the oracle-only run never compiles the local implementation —
//! see §5.3 of the fleet brief.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::tsv::standards::v_iana::subsets::any::{oracle_apply_mutation, project_tsv_grid, read_grid, write_grid};
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, mutation_is_observable, round_trip_preserves};

//#region 🔖️Kinds
/// 🧾️ Test-case-local mirror of the `tsv-iana-any` catalog. Duplicated, not imported, from
/// `../../🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs::KINDS` — that
/// module lives in the SUBJECT crate, and the oracle role must not link the subject crate at all
/// (fleet brief §5.3), while this loop registers handlers for both roles from one list. That other
/// `KINDS` carries its own test proving it matches the enum AND the catalog manifest; a mismatch
/// HERE against either one is caught structurally instead — the contract phase fails with
/// `mutation-kind-uncovered`/`mutation-kind-undeclared` if this list omits or invents a kind, and the
/// runner fails every unregistered scenario id outright (`adapter has no {role} registration`).
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-trailing-newline", "set-line-ending", "insert-row", "remove-row", "set-cell"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://reuse-marketplaces.tsv";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.tsv"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️SpecHelpers
fn json_object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn kind_spec(kind: &str, params: Json) -> Json {
    json_object(vec![("kind", Json::String(kind.to_string())), ("params", params)])
}

fn rows_json(rows: &[Vec<String>]) -> Json {
    Json::Array(rows.iter().map(|row| Json::Array(row.iter().cloned().map(Json::String).collect())).collect())
}

/// ↩️ The inverse mutation's OWN spec, computed by reading whatever pre-mutation state it needs
/// straight out of `original` with the same independent reader the oracle mutates with — never by
/// calling this repository's own `TsvMutation::inverse`, which would defeat the point of an
/// independently-computed oracle. Mirrors that method's documented rule exactly (index-aware,
/// reading the pre-state it needs from the ORIGINAL document), just derived from real bytes instead
/// of a typed snapshot.
fn inverse_spec(original: &[u8], forward: &Json) -> Result<Json, String> {
    let params = forward.get("params").cloned().unwrap_or(Json::Null);
    let number = |key: &str| match params.get(key) {
        Some(Json::Number(value)) => Some(*value),
        _ => None,
    };
    match forward.str("kind").as_str() {
        "no-mutation" => Ok(kind_spec("no-mutation", json_object(vec![]))),
        "set-snapshot" => {
            let grid = read_grid(original)?;
            Ok(kind_spec("set-snapshot", json_object(vec![("records", rows_json(&grid.records)), ("trailingNewline", Json::Bool(grid.trailing_newline)), ("lineEnding", Json::String(grid.line_ending))])))
        }
        "set-trailing-newline" => {
            let grid = read_grid(original)?;
            Ok(kind_spec("set-trailing-newline", json_object(vec![("trailingNewline", Json::Bool(grid.trailing_newline))])))
        }
        "set-line-ending" => {
            let grid = read_grid(original)?;
            Ok(kind_spec("set-line-ending", json_object(vec![("lineEnding", Json::String(grid.line_ending))])))
        }
        "insert-row" => {
            let index = number("index").ok_or("insert-row inverse: missing `index`")?;
            Ok(kind_spec("remove-row", json_object(vec![("index", Json::Number(index))])))
        }
        "remove-row" => {
            let index = number("index").ok_or("remove-row inverse: missing `index`")? as usize;
            let grid = read_grid(original)?;
            let row = grid.records.get(index).ok_or_else(|| format!("remove-row inverse: index {index} out of bounds ({} row(s))", grid.records.len()))?;
            Ok(kind_spec("insert-row", json_object(vec![("index", Json::Number(index as f64)), ("row", Json::Array(row.iter().cloned().map(Json::String).collect()))])))
        }
        "set-cell" => {
            let row_index = number("rowIndex").ok_or("set-cell inverse: missing `rowIndex`")? as usize;
            let field_index = number("fieldIndex").ok_or("set-cell inverse: missing `fieldIndex`")? as usize;
            let grid = read_grid(original)?;
            let value = grid.records.get(row_index).and_then(|row| row.get(field_index)).cloned().unwrap_or_default();
            Ok(kind_spec("set-cell", json_object(vec![("rowIndex", Json::Number(row_index as f64)), ("fieldIndex", Json::Number(field_index as f64)), ("value", Json::String(value))])))
        }
        other => Err(format!("no inverse rule for kind {other:?}")),
    }
}
//#endregion 🔖️SpecHelpers

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let output = oracle_apply_mutation(&input, &spec)?;
    let projection = project_tsv_grid(&output)?;
    mutation_is_observable(&spec.str("kind"), &projection, &project_tsv_grid(&input)?, &[])?;
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ The inverse law, asserted HERE by the reference against its own pre-mutation reading rather
/// than deferred to the parity phase: `apply(m)` followed by `apply(inverse(m))` has to land back
/// on the ORIGINAL table's semantic projection — including `trailingNewline` and `lineTerminator`,
/// which `semantic-tabular-mutate-v1` deliberately keeps live because this subset's own vocabulary
/// mutates them.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let undo = inverse_spec(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let projection = project_tsv_grid(&restored)?;
    inverse_restores(&spec.str("kind"), &projection, &project_tsv_grid(&input)?)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The oracle's own decode/re-encode, through the SAME independent `csv` reader/writer this
/// subset's mutations use.
///
/// The no-byte-pass-through tripwire every OTHER container in this wave asserts does NOT apply to
/// IANA TSV, and asserting it here would be a fabricated law: the format has no quoting, no
/// escaping and no other writer freedom, and the only two choices it does leave — the line
/// terminator and whether the last record is terminated — are carried in `TsvBody` and reproduced
/// exactly (`write_tsv`, and that module's own `no_mutation_is_a_true_byte_identity` test). A
/// byte-exact result is therefore the CORRECT outcome, so the honest form of the law is asserted
/// instead: the output must equal the input exactly, and the projection must be preserved. Both
/// are still real failures if the reference's reader or writer ever drifts.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = write_grid(&read_grid(&input)?)?;
    carrier_is_exact(&output, &input)?;
    let projection = project_tsv_grid(&output)?;
    round_trip_preserves(&projection, &project_tsv_grid(&input)?)?;
    Ok(Outcome::with_raw(output, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_spec, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::ArtifactDsl;
    use semio_s_plugin_stdio::artifacts::tsv::standards::iana::subsets::any::schema::mutations::{apply_tsv_mutation, insert_row, remove_row, set_cell, set_line_ending, set_snapshot, set_trailing_newline};
    use semio_s_plugin_stdio::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::{decode_tsv, encode_tsv, LineEnding};
    use semio_s_plugin_stdio::artifacts::tsv::{TsvMutation, TsvSnapshot, STDIO_TSV_DOCUMENT_SCHEMA};
    use semio_s_plugin_stdio_test_oracle::artifacts::tsv::standards::v_iana::subsets::any::project_tsv_grid;

    fn parse_line_ending(value: &str) -> Result<LineEnding, String> {
        match value {
            "lf" => Ok(LineEnding::Lf),
            "crlf" => Ok(LineEnding::Crlf),
            other => Err(format!("unknown lineEnding {other:?}, expected \"lf\" or \"crlf\"")),
        }
    }

    /// 🔀️ The same JSON mutation spec the oracle reads, turned into this repository's own typed
    /// `TsvMutation` — the only channel between the feature's parameters and the subject's codec.
    fn mutation_from_spec(spec: &Json) -> Result<TsvMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        let number = |key: &str| match params.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        };
        let boolean = |key: &str| match params.get(key) {
            Some(Json::Bool(value)) => Some(*value),
            _ => None,
        };
        let strings = |key: &str| -> Vec<String> {
            params
                .array(key)
                .iter()
                .map(|entry| match entry {
                    Json::String(text) => text.clone(),
                    _ => String::new(),
                })
                .collect()
        };
        Ok(match spec.str("kind").as_str() {
            "set-trailing-newline" => TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline: boolean("trailingNewline").ok_or("set-trailing-newline: missing `trailingNewline`")? }),
            "set-line-ending" => TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending: parse_line_ending(&params.str("lineEnding"))? }),
            "set-snapshot" => {
                let records = params
                    .array("records")
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
                    .collect();
                TsvMutation::SetSnapshot(set_snapshot::SetSnapshot {
                    snapshot: TsvSnapshot { schema: STDIO_TSV_DOCUMENT_SCHEMA.into(), records, trailing_newline: boolean("trailingNewline").unwrap_or(true), line_ending: params.get("lineEnding").and_then(|v| if let Json::String(s) = v { parse_line_ending(s).ok() } else { None }).unwrap_or(LineEnding::Lf) },
                })
            }
            "insert-row" => TsvMutation::InsertRow(insert_row::InsertRow { index: number("index").ok_or("insert-row: missing `index`")? as usize, row: strings("row") }),
            "remove-row" => TsvMutation::RemoveRow(remove_row::RemoveRow { index: number("index").ok_or("remove-row: missing `index`")? as usize }),
            "set-cell" => TsvMutation::SetCell(set_cell::SetCell { row_index: number("rowIndex").ok_or("set-cell: missing `rowIndex`")? as usize, field_index: number("fieldIndex").ok_or("set-cell: missing `fieldIndex`")? as usize, value: params.str("value") }),
            other => return Err(format!("no subject rule for kind {other:?}")),
        })
    }

    fn decode(bytes: &[u8]) -> Result<TsvSnapshot, String> {
        let text = String::from_utf8(bytes.to_vec()).map_err(|error| format!("input is not UTF-8: {error}"))?;
        Ok(decode_tsv(&text))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        apply_tsv_mutation(&mut snapshot, &mutation);
        let output = encode_tsv(&snapshot).into_bytes();
        let projection = project_tsv_grid(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&input)?;
        apply_tsv_mutation(&mut snapshot, &mutation_from_spec(&spec)?);
        apply_tsv_mutation(&mut snapshot, &mutation_from_spec(&inverse_spec(&input, &spec)?)?);
        let output = encode_tsv(&snapshot).into_bytes();
        let projection = project_tsv_grid(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating (fleet brief, "the point of this wave") and this tripwire catches
    /// it. IANA TSV's own bare grid codec (`decode_tsv`/`encode_tsv`) is a byte-exact split/rejoin
    /// BY DESIGN (no quoting mechanism exists for it to invent, and every retention field —
    /// trailing newline, line ending — is captured and replayed verbatim), so a raw decode/encode
    /// round trip through those two functions alone can never diverge from a well-formed input:
    /// that would make the tripwire untestable, not satisfied by accident. This case instead goes
    /// through the artifact's REAL document codec (`ArtifactDsl::parse_dsl`/`print_dsl`, re-exported
    /// by `semio_s_plugin_stdio` — the generated subject host links that crate, never `store` directly,
    /// the same pair `register_document_codec` wires into production) — `print_dsl` always prepends
    /// the `semio iana.tsv.dsl v1` envelope line the real committed fixture does not carry, which is
    /// a genuine writer choice this artifact's persisted form makes, not a fabricated difference.
    /// The projection compared against the oracle is computed from the BODY alone
    /// (`encode_tsv(&snapshot)`, envelope-free) so the independent reader is comparing the same
    /// grid shape on both sides.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let snapshot = <TsvSnapshot as ArtifactDsl>::parse_dsl(&text).map_err(|error| error.to_string())?;
        let enveloped = <TsvSnapshot as ArtifactDsl>::print_dsl(&snapshot).into_bytes();
        if enveloped == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let body = encode_tsv(&snapshot).into_bytes();
        let projection = project_tsv_grid(&body)?;
        Ok(Outcome::with_raw(enveloped, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
