//! 🦀️ EPW EnergyPlus exhaustive mutation round-trip case — Rust adapter.
//!
//! Every scenario copies the immutable real (synthetic-stub, see the feature's own honesty
//! caveat) fixture into the case work directory first; the committed file is never written to.
//! `oracle` handlers drive the registered `csv` reference implementation (via this subset's own
//! `🦀️oracle.rs`), `subject` handlers drive this repository's own decode/mutate/encode
//! round trip, and both results are read back by the SAME independent reader (`project_epw`)
//! before the `semantic-epw-v1` profile compares them. The subject half is gated behind the
//! generated host's `sut` feature so the oracle-only run never compiles the local implementation —
//! see §5.3 of the fleet brief.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::epw::standards::v_energyplus::subsets::any::{epw_snapshot_wire, oracle_apply_mutation, project_epw, round_trip_epw, EPW_RECORD_COLUMNS};
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, mutation_is_observable, round_trip_preserves};


//#region 🔖️Input
const INPUT: &str = "asset://🎬️demo/🧪️example/🌦️.epw";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.epw"))?;
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

/// ↩️ The inverse mutation's OWN spec, computed by reading whatever pre-mutation state it needs
/// straight out of `original` with the same independent reader the oracle mutates with
/// ([`epw_snapshot_wire`]) — never by calling this repository's own `EpwMutation::inverse`, which would
/// defeat the point of an independently-computed oracle. Mirrors that method's documented rule
/// exactly (index-aware, reading the pre-state it needs from the ORIGINAL document), and every spec it
/// returns carries the leaf wire payload, like the feature's own rows.
fn inverse_spec(original: &[u8], forward: &Json) -> Result<Json, String> {
    let params = forward.get("params").cloned().unwrap_or(Json::Null);
    let number = |key: &str| match params.get(key) {
        Some(Json::Number(value)) => Some(*value),
        _ => None,
    };
    let snapshot = epw_snapshot_wire(original)?;
    let header = |kind: &str, key: &str, member: &str| Ok(kind_spec(kind, json_object(vec![(key, snapshot.get(member).cloned().unwrap_or(Json::Null))])));
    match forward.str("kind").as_str() {
        "set-snapshot" => Ok(kind_spec("set-snapshot", json_object(vec![("snapshot", snapshot.clone())]))),
        "set-location" => header("set-location", "location", "location"),
        "set-design-conditions" => header("set-design-conditions", "value", "designConditions"),
        "set-typical-extreme-periods" => header("set-typical-extreme-periods", "value", "typicalExtremePeriods"),
        "set-ground-temperatures" => header("set-ground-temperatures", "value", "groundTemperatures"),
        "set-holidays-dst" => header("set-holidays-dst", "value", "holidaysDst"),
        "set-comments1" => header("set-comments1", "value", "comments1"),
        "set-comments2" => header("set-comments2", "value", "comments2"),
        "set-data-periods" => header("set-data-periods", "dataPeriods", "dataPeriods"),
        "insert-record" => {
            let index = number("index").ok_or("insert-record inverse: missing `index`")?;
            Ok(kind_spec("remove-record", json_object(vec![("index", Json::Number(index))])))
        }
        "remove-record" => {
            let index = number("index").ok_or("remove-record inverse: missing `index`")? as usize;
            let records = snapshot.array("records");
            let record = records.get(index).ok_or_else(|| format!("remove-record inverse: index {index} out of bounds ({} record(s))", records.len()))?;
            Ok(kind_spec("insert-record", json_object(vec![("index", Json::Number(index as f64)), ("record", record.clone())])))
        }
        "set-record-field" => {
            let record_index = number("recordIndex").ok_or("set-record-field inverse: missing `recordIndex`")? as usize;
            let field_index = number("fieldIndex").ok_or("set-record-field inverse: missing `fieldIndex`")? as usize;
            let column = EPW_RECORD_COLUMNS.get(field_index).ok_or_else(|| format!("set-record-field inverse: field index {field_index} names no EPW column"))?;
            let value = snapshot.array("records").get(record_index).map(|record| record.str(column)).unwrap_or_default();
            Ok(kind_spec("set-record-field", json_object(vec![("recordIndex", Json::Number(record_index as f64)), ("fieldIndex", Json::Number(field_index as f64)), ("value", Json::String(value))])))
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
    let projection = project_epw(&output)?;
    mutation_is_observable(&spec.str("kind"), &projection, &project_epw(&input)?, &[])?;
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ The inverse law, asserted HERE by the reference against its own pre-mutation reading rather
/// than deferred to the parity phase: `apply(m)` followed by `apply(inverse(m))` has to land back
/// on the ORIGINAL weather file's semantic projection — all eight header blocks and the full
/// ordered record grid.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let undo = inverse_spec(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let projection = project_epw(&restored)?;
    inverse_restores(&spec.str("kind"), &projection, &project_epw(&input)?)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The oracle's own decode/re-encode: the 8 header lines copied raw, the record grid through
/// the SAME independent `csv` reader/writer this subset's record-kind mutations use — proves the
/// reference library itself is stable on the real fixture before the subject's own codec is asked
/// to be.
///
/// The no-byte-pass-through tripwire most other containers in this wave assert does NOT apply to
/// EPW, and asserting it here would be a fabricated law: EPW is a fixed-column CSV-shaped text
/// format with no object layout, no whitespace freedom and one normative CRLF terminator, and this
/// subset's own schema stores every record column as a `String` precisely so nothing is ever
/// reformatted (see the feature file's own note, and `codec_retention_law` on the subject side).
/// So the honest form of the law is asserted instead: the output must reproduce the input exactly
/// AND carry the same semantic projection — both still real failures if the reference's split,
/// reader or writer ever drifts.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = round_trip_epw(&input)?;
    carrier_is_exact(&output, &input)?;
    let projection = project_epw(&output)?;
    round_trip_preserves(&projection, &project_epw(&input)?)?;
    Ok(Outcome::with_raw(output, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::io::{decode_epw, encode_epw};
    use semio_s_plugin_stdio_test_oracle::law::wire_operation;
    use semio_s_artifact_stdio_epw::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::schema::mutations::apply_epw_mutation;
    use semio_s_artifact_stdio_epw::{EpwMutation, EpwSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::epw::standards::v_energyplus::subsets::any::project_epw;

    /// 🔀️ The spec's wire payload, decoded by the aggregate's own generic payload constructor — the only
    /// channel between the feature's parameters and the subject's codec.
    fn mutation_of(spec: &Json) -> Result<EpwMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    fn decode(bytes: &[u8]) -> Result<EpwSnapshot, String> {
        let text = String::from_utf8(bytes.to_vec()).map_err(|error| format!("input is not UTF-8: {error}"))?;
        decode_epw(&text)
    }

    fn outcome(snapshot: &EpwSnapshot) -> Result<Outcome, String> {
        let output = encode_epw(snapshot).into_bytes();
        let projection = project_epw(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        apply_epw_mutation(&mut snapshot, &mutation_of(&ctx.doc_json()?)?);
        outcome(&snapshot)
    }

    /// ↩️ The forward op, then the production inverse computed against the pre-mutation snapshot.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let forward = mutation_of(&ctx.doc_json()?)?;
        let backward = mutation_inverse(&forward, &snapshot);
        apply_epw_mutation(&mut snapshot, &forward);
        for mutation in &backward {
            apply_epw_mutation(&mut snapshot, mutation);
        }
        outcome(&snapshot)
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating (fleet brief, "the point of this wave") and this tripwire catches
    /// it. The real stub fixture is committed with CRLF line endings and this subset's own encoder
    /// also always writes CRLF (`codec_retention_law` in `../../🏅️standards/🔖️energyplus/🪆️subsets/
    /// ✳️any/🚪️io/🦀️.rs` proves decode→encode is byte-preserving on it), so this scenario's
    /// non-triviality rests on genuinely mutating nothing and still routing through the typed model
    /// — see that Feature's own scenario text for the exact assertion this performs.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        outcome(&decode(&mutable_input(ctx)?)?)
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
