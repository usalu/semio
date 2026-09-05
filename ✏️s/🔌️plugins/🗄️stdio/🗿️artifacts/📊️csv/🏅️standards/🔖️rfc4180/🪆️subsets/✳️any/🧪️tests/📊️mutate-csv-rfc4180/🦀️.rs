//! 🦀️ CSV RFC 4180 exhaustive mutation round-trip case — Rust adapter.
//!
//! Every scenario copies the immutable real fixture into the case work directory first; the
//! committed file is never written to. `oracle` handlers drive the registered `csv` reference
//! implementation (via this subset's own `🦀️oracle.rs`), `subject` handlers drive this
//! repository's own decode/mutate/encode round trip, and both results are read back by the SAME
//! independent reader (`project_csv_grid`) before the `semantic-tabular-v1` profile compares them.
//! The subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! compiles the local implementation — see §5.3 of the fleet brief.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::csv::standards::v_rfc4180::subsets::any::{oracle_apply_mutation, project_csv_grid, read_grid, write_grid};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, mutation_is_observable, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Kinds
/// 🧾️ Test-case-local mirror of the `csv-rfc4180-any` catalog. Duplicated, not imported, from
/// `../../🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs::KINDS` — that
/// module lives in the SUBJECT crate, and the oracle role must not link the subject crate at all
/// (fleet brief §5.3), while this loop registers handlers for both roles from one list. That other
/// `KINDS` carries its own test proving it matches the enum AND the catalog manifest; a mismatch
/// HERE against either one is caught structurally instead — the contract phase fails with
/// `mutation-kind-uncovered`/`mutation-kind-undeclared` if this list omits or invents a kind, and the
/// runner fails every unregistered scenario id outright (`adapter has no {role} registration`).
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-has-header", "insert-record", "remove-record", "set-field"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🧪️reuse-marketplaces/📊️.csv";
/// 📑️ The real fixture's own baseline reading: record 0 is its header row.
const BASELINE_HAS_HEADER: bool = true;

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.csv"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️SpecHelpers
/// 📑️ RFC 4180 carries no header/data distinction on the wire (see `../../🏅️standards/🔖️rfc4180/
/// 🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`'s own doc comment) — `has_header` is metadata
/// this case tracks alongside the bytes, not something a projection can recover from them alone.
fn resulting_has_header(spec: &Json, baseline: bool) -> bool {
    match spec.str("kind").as_str() {
        "set-has-header" | "set-snapshot" => match spec.get("params").and_then(|params| params.get("hasHeader")) {
            Some(Json::Bool(flag)) => *flag,
            _ => baseline,
        },
        _ => baseline,
    }
}

fn json_object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn kind_spec(kind: &str, params: Json) -> Json {
    json_object(vec![("kind", Json::String(kind.to_string())), ("params", params)])
}

/// ↩️ The inverse mutation's OWN spec, computed by reading whatever pre-mutation state it needs
/// straight out of `original` with the same independent reader the oracle mutates with — never by
/// calling this repository's own `CsvMutation::inverse`, which would defeat the point of an
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
        "set-has-header" => Ok(kind_spec("set-has-header", json_object(vec![("hasHeader", Json::Bool(BASELINE_HAS_HEADER))]))),
        "set-snapshot" => {
            let grid = read_grid(original)?;
            let rows = Json::Array(grid.into_iter().map(|record| Json::Array(record.into_iter().map(Json::String).collect())).collect());
            Ok(kind_spec("set-snapshot", json_object(vec![("hasHeader", Json::Bool(BASELINE_HAS_HEADER)), ("rows", rows)])))
        }
        "insert-record" => {
            let index = number("index").ok_or("insert-record inverse: missing `index`")?;
            Ok(kind_spec("remove-record", json_object(vec![("index", Json::Number(index))])))
        }
        "remove-record" => {
            let index = number("index").ok_or("remove-record inverse: missing `index`")? as usize;
            let grid = read_grid(original)?;
            let record = grid.get(index).ok_or_else(|| format!("remove-record inverse: index {index} out of bounds ({} record(s))", grid.len()))?;
            let fields = Json::Array(record.iter().cloned().map(Json::String).collect());
            Ok(kind_spec("insert-record", json_object(vec![("index", Json::Number(index as f64)), ("fields", fields)])))
        }
        "set-field" => {
            let record_index = number("recordIndex").ok_or("set-field inverse: missing `recordIndex`")? as usize;
            let field_index = number("fieldIndex").ok_or("set-field inverse: missing `fieldIndex`")? as usize;
            let grid = read_grid(original)?;
            let value = grid.get(record_index).and_then(|record| record.get(field_index)).cloned().unwrap_or_default();
            Ok(kind_spec("set-field", json_object(vec![("recordIndex", Json::Number(record_index as f64)), ("fieldIndex", Json::Number(field_index as f64)), ("value", Json::String(value))])))
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
    let projection = project_csv_grid(&output, resulting_has_header(&spec, BASELINE_HAS_HEADER))?;
    mutation_is_observable(&spec.str("kind"), &projection, &project_csv_grid(&input, BASELINE_HAS_HEADER)?, &[])?;
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ The inverse law, asserted HERE by the reference against its own pre-mutation reading rather
/// than deferred to the parity phase: `apply(m)` followed by `apply(inverse(m))` has to land back
/// on the ORIGINAL document's semantic projection. Without the check the scenario passes for any
/// inverse the `csv` crate merely tolerated, which is not what `@mode-property` claims.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let undo = inverse_spec(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let projection = project_csv_grid(&restored, BASELINE_HAS_HEADER)?;
    inverse_restores(&spec.str("kind"), &projection, &project_csv_grid(&input, BASELINE_HAS_HEADER)?)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The oracle's own decode/re-encode, through the SAME independent `csv` reader/writer this
/// subset's mutations use — proves the reference library itself is stable on the real fixture before
/// the subject's own codec is asked to be. Both halves of the identity law are asserted in role:
/// the record grid must survive unchanged, and the output must not be the input bytes back again.
/// The second half is genuinely checkable here rather than contrived — the committed fixture is
/// CRLF-terminated and the `csv` writer terminates with its own default LF, so a byte-identical
/// result could only come from a copy that never parsed anything.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = write_grid(&read_grid(&input)?)?;
    reparsed_not_copied(&output, &input)?;
    let projection = project_csv_grid(&output, BASELINE_HAS_HEADER)?;
    round_trip_preserves(&projection, &project_csv_grid(&input, BASELINE_HAS_HEADER)?)?;
    Ok(Outcome::with_raw(output, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_spec, mutable_input, BASELINE_HAS_HEADER};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::csv::standards::v_rfc4180::subsets::any::schema::mutations::apply_csv_mutation;
    use semio_s_plugin_stdio::artifacts::csv::standards::v_rfc4180::subsets::any::schema::snapshot::{decode_csv, encode_csv};
    use semio_s_plugin_stdio::artifacts::csv::{CsvField, CsvMutation, CsvRecord, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};
    use semio_s_plugin_stdio_test_oracle::artifacts::csv::standards::v_rfc4180::subsets::any::project_csv_grid;

    /// 🔀️ The same JSON mutation spec the oracle reads, turned into this repository's own typed
    /// `CsvMutation` — the only channel between the feature's parameters and the subject's codec.
    fn mutation_from_spec(spec: &Json) -> Result<CsvMutation, String> {
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
            "set-has-header" => CsvMutation::SetHasHeader(crate::artifacts::csv::schema::mutations::set_has_header::SetHasHeader { has_header: boolean("hasHeader").ok_or("set-has-header: missing `hasHeader`")? }),
            "set-snapshot" => {
                let records = params
                    .array("rows")
                    .iter()
                    .map(|row| match row {
                        Json::Array(cells) => CsvRecord {
                            fields: cells
                                .iter()
                                .map(|cell| CsvField {
                                    value: match cell {
                                        Json::String(text) => text.clone(),
                                        _ => String::new(),
                                    },
                                    quoted: false,
                                })
                                .collect(),
                        },
                        _ => CsvRecord::default(),
                    })
                    .collect();
                CsvMutation::SetSnapshot(crate::artifacts::csv::schema::mutations::set_snapshot::SetSnapshot { snapshot: CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: boolean("hasHeader").unwrap_or(BASELINE_HAS_HEADER), records } })
            }
            "insert-record" => CsvMutation::InsertRecord(crate::artifacts::csv::schema::mutations::insert_record::InsertRecord { index: number("index").ok_or("insert-record: missing `index`")? as usize, record: CsvRecord { fields: strings("fields").into_iter().map(|value| CsvField { value, quoted: false }).collect() } }),
            "remove-record" => CsvMutation::RemoveRecord(crate::artifacts::csv::schema::mutations::remove_record::RemoveRecord { index: number("index").ok_or("remove-record: missing `index`")? as usize }),
            "set-field" => CsvMutation::SetField(crate::artifacts::csv::schema::mutations::set_field::SetField { record_index: number("recordIndex").ok_or("set-field: missing `recordIndex`")? as usize, field_index: number("fieldIndex").ok_or("set-field: missing `fieldIndex`")? as usize, value: params.str("value"), quoted: false }),
            other => return Err(format!("no subject rule for kind {other:?}")),
        })
    }

    fn decode(bytes: &[u8]) -> Result<CsvSnapshot, String> {
        let text = String::from_utf8(bytes.to_vec()).map_err(|error| format!("input is not UTF-8: {error}"))?;
        decode_csv(&text)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        apply_csv_mutation(&mut snapshot, &mutation);
        let output = encode_csv(&snapshot).into_bytes();
        let projection = project_csv_grid(&output, snapshot.has_header)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&input)?;
        apply_csv_mutation(&mut snapshot, &mutation_from_spec(&spec)?);
        apply_csv_mutation(&mut snapshot, &mutation_from_spec(&inverse_spec(&input, &spec)?)?);
        let output = encode_csv(&snapshot).into_bytes();
        let projection = project_csv_grid(&output, snapshot.has_header)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating (fleet brief, "the point of this wave") and this tripwire catches it:
    /// the real fixture is committed with CRLF line endings (RFC 4180's own §2 rule 1) while this
    /// repository's encoder always writes LF, so a genuine re-encode can never coincidentally
    /// reproduce the input bytes.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(&input)?;
        let output = encode_csv(&snapshot).into_bytes();
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_csv_grid(&output, snapshot.has_header)?;
        Ok(Outcome::with_raw(output, projection))
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
