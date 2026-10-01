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
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    let carrier = match spec.str("kind").as_str() {
        "set-has-header" => params,
        "set-snapshot" => params.get("snapshot").cloned().unwrap_or(Json::Null),
        _ => Json::Null,
    };
    match carrier.get("hasHeader") {
        Some(Json::Bool(flag)) => *flag,
        _ => baseline,
    }
}

/// 📄️ A plain row as the `CsvRecord` wire (`{"fields": [{"value", "quoted": false}]}`); quoting is left to the writer.
fn record_wire(values: &[String]) -> Json {
    json_object(vec![("fields", Json::Array(values.iter().map(|value| json_object(vec![("value", Json::String(value.clone())), ("quoted", Json::Bool(false))])).collect()))])
}

fn json_object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn kind_spec(kind: &str, params: Json) -> Json {
    json_object(vec![("kind", Json::String(kind.to_string())), ("params", params)])
}

/// ↩️ The inverse mutation's OWN spec (leaf wire payloads, like every row), computed by reading whatever pre-mutation state it needs
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
        "set-has-header" => Ok(kind_spec("set-has-header", json_object(vec![("hasHeader", Json::Bool(BASELINE_HAS_HEADER))]))),
        "set-snapshot" => {
            let records = Json::Array(read_grid(original)?.iter().map(|record| record_wire(record)).collect());
            Ok(kind_spec("set-snapshot", json_object(vec![("snapshot", json_object(vec![("schema", Json::String("stdio.csv".to_string())), ("hasHeader", Json::Bool(BASELINE_HAS_HEADER)), ("records", records)]))])))
        }
        "insert-record" => {
            let index = number("index").ok_or("insert-record inverse: missing `index`")?;
            Ok(kind_spec("remove-record", json_object(vec![("index", Json::Number(index))])))
        }
        "remove-record" => {
            let index = number("index").ok_or("remove-record inverse: missing `index`")? as usize;
            let grid = read_grid(original)?;
            let record = grid.get(index).ok_or_else(|| format!("remove-record inverse: index {index} out of bounds ({} record(s))", grid.len()))?;
            Ok(kind_spec("insert-record", json_object(vec![("index", Json::Number(index as f64)), ("record", record_wire(record))])))
        }
        "set-field" => {
            let record_index = number("recordIndex").ok_or("set-field inverse: missing `recordIndex`")? as usize;
            let field_index = number("fieldIndex").ok_or("set-field inverse: missing `fieldIndex`")? as usize;
            let grid = read_grid(original)?;
            let value = grid.get(record_index).and_then(|record| record.get(field_index)).cloned().unwrap_or_default();
            Ok(kind_spec("set-field", json_object(vec![("recordIndex", Json::Number(record_index as f64)), ("fieldIndex", Json::Number(field_index as f64)), ("value", Json::String(value)), ("quoted", Json::Bool(false))])))
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
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_csv::standards::v_rfc4180::subsets::any::schema::mutations::apply_csv_mutation;
    use semio_s_plugin_stdio_test_oracle::law::wire_operation;
    use semio_s_artifact_stdio_csv::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_csv::standards::v_rfc4180::subsets::any::schema::snapshot::{decode_csv, encode_csv};
    use semio_s_artifact_stdio_csv::{CsvMutation, CsvSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::csv::standards::v_rfc4180::subsets::any::project_csv_grid;

    /// 🔀️ The scenario's `<id>`/`<params>` spec decoded as the leaf wire payload it is, through the aggregate's own
    /// derive-generated payload constructor — the only channel between the feature's parameters and the subject's codec.
    fn mutation_from_spec(spec: &Json) -> Result<CsvMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    fn decode(bytes: &[u8]) -> Result<CsvSnapshot, String> {
        let text = String::from_utf8(bytes.to_vec()).map_err(|error| format!("input is not UTF-8: {error}"))?;
        decode_csv(&text)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        apply_csv_mutation(&mut snapshot, &mutation_from_spec(&ctx.doc_json()?)?);
        let output = encode_csv(&snapshot).into_bytes();
        let projection = project_csv_grid(&output, snapshot.has_header)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// ↩️ The subset's OWN `Mutation::inverse` (`inverse_csv_mutation`) applied after the forward step — the
    /// implementation's algebra, compared against the oracle's independently derived undo in the parity phase.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let undo = mutation_inverse(&mutation, &snapshot);
        apply_csv_mutation(&mut snapshot, &mutation);
        for step in &undo {
            apply_csv_mutation(&mut snapshot, step);
        }
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
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
