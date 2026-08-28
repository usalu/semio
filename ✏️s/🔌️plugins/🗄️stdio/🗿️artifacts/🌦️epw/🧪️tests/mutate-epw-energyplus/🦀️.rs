//! 🦀️ EPW EnergyPlus exhaustive mutation round-trip case — Rust adapter.
//!
//! Every scenario copies the immutable real (synthetic-stub, see the feature's own honesty
//! caveat) fixture into the case work directory first; the committed file is never written to.
//! `oracle` handlers drive the registered `csv` reference implementation (via this subset's own
//! `🧪️oracle/🦀️component.rs`), `subject` handlers drive this repository's own decode/mutate/encode
//! round trip, and both results are read back by the SAME independent reader (`project_epw`)
//! before the `semantic-epw-v1` profile compares them. The subject half is gated behind the
//! generated host's `sut` feature so the oracle-only run never compiles the local implementation —
//! see §5.3 of the fleet brief.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::epw::standards::v_energyplus::subsets::any::{oracle_apply_mutation, project_epw, round_trip_epw};
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, mutation_is_observable, round_trip_preserves};

//#region 🔖️Kinds
/// 🧾️ Test-case-local mirror of the `epw-energyplus-any` catalog. Duplicated, not imported, from
/// `../../🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs::KINDS` — that
/// module lives in the SUBJECT crate, and the oracle role must not link the subject crate at all
/// (fleet brief §5.3), while this loop registers handlers for both roles from one list. That other
/// `KINDS` carries its own test proving it matches the enum AND the catalog manifest; a mismatch
/// HERE against either one is caught structurally instead — the contract phase fails with
/// `mutation-kind-uncovered`/`mutation-kind-undeclared` if this list omits or invents a kind, and the
/// runner fails every unregistered scenario id outright (`adapter has no {role} registration`).
const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "set-location",
    "set-design-conditions",
    "set-typical-extreme-periods",
    "set-ground-temperatures",
    "set-holidays-dst",
    "set-comments-1",
    "set-comments-2",
    "set-data-periods",
    "insert-record",
    "remove-record",
    "set-record-field",
];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "asset://🏅️standards/🔖️energyplus/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🌦️example.epw";

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
/// (`project_epw`) — never by calling this repository's own `EpwMutation::inverse`, which would
/// defeat the point of an independently-computed oracle. Mirrors that method's documented rule
/// exactly (index-aware, reading the pre-state it needs from the ORIGINAL document), just derived
/// from real bytes instead of a typed snapshot.
fn inverse_spec(original: &[u8], forward: &Json) -> Result<Json, String> {
    let params = forward.get("params").cloned().unwrap_or(Json::Null);
    let number = |key: &str| match params.get(key) {
        Some(Json::Number(value)) => Some(*value),
        _ => None,
    };
    match forward.str("kind").as_str() {
        "no-mutation" => Ok(kind_spec("no-mutation", json_object(vec![]))),
        "set-snapshot" => {
            let projection = project_epw(original)?;
            Ok(kind_spec("set-snapshot", json_object(vec![("snapshot", projection)])))
        }
        "set-location" => {
            let projection = project_epw(original)?;
            Ok(kind_spec("set-location", json_object(vec![("location", projection.get("location").cloned().unwrap_or(Json::Null))])))
        }
        "set-design-conditions" => {
            let projection = project_epw(original)?;
            Ok(kind_spec("set-design-conditions", json_object(vec![("value", projection.get("designConditions").cloned().unwrap_or(Json::String(String::new())))])))
        }
        "set-typical-extreme-periods" => {
            let projection = project_epw(original)?;
            Ok(kind_spec("set-typical-extreme-periods", json_object(vec![("value", projection.get("typicalExtremePeriods").cloned().unwrap_or(Json::String(String::new())))])))
        }
        "set-ground-temperatures" => {
            let projection = project_epw(original)?;
            Ok(kind_spec("set-ground-temperatures", json_object(vec![("value", projection.get("groundTemperatures").cloned().unwrap_or(Json::String(String::new())))])))
        }
        "set-holidays-dst" => {
            let projection = project_epw(original)?;
            Ok(kind_spec("set-holidays-dst", json_object(vec![("value", projection.get("holidaysDst").cloned().unwrap_or(Json::String(String::new())))])))
        }
        "set-comments-1" => {
            let projection = project_epw(original)?;
            Ok(kind_spec("set-comments-1", json_object(vec![("value", projection.get("comments1").cloned().unwrap_or(Json::String(String::new())))])))
        }
        "set-comments-2" => {
            let projection = project_epw(original)?;
            Ok(kind_spec("set-comments-2", json_object(vec![("value", projection.get("comments2").cloned().unwrap_or(Json::String(String::new())))])))
        }
        "set-data-periods" => {
            let projection = project_epw(original)?;
            Ok(kind_spec("set-data-periods", json_object(vec![("dataPeriods", projection.get("dataPeriods").cloned().unwrap_or(Json::Null))])))
        }
        "insert-record" => {
            let index = number("index").ok_or("insert-record inverse: missing `index`")?;
            Ok(kind_spec("remove-record", json_object(vec![("index", Json::Number(index))])))
        }
        "remove-record" => {
            let index = number("index").ok_or("remove-record inverse: missing `index`")? as usize;
            let projection = project_epw(original)?;
            let records = projection.array("records");
            let record = records.get(index).ok_or_else(|| format!("remove-record inverse: index {index} out of bounds ({} record(s))", records.len()))?;
            Ok(kind_spec("insert-record", json_object(vec![("index", Json::Number(index as f64)), ("fields", record.clone())])))
        }
        "set-record-field" => {
            let record_index = number("recordIndex").ok_or("set-record-field inverse: missing `recordIndex`")? as usize;
            let field_index = number("fieldIndex").ok_or("set-record-field inverse: missing `fieldIndex`")? as usize;
            let projection = project_epw(original)?;
            let records = projection.array("records");
            let value = records.get(record_index).and_then(|row| if let Json::Array(cells) = row { cells.get(field_index).cloned() } else { None }).unwrap_or(Json::String(String::new()));
            Ok(kind_spec("set-record-field", json_object(vec![("recordIndex", Json::Number(record_index as f64)), ("fieldIndex", Json::Number(field_index as f64)), ("value", value)])))
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
    use super::{inverse_spec, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::epw::standards::energyplus::subsets::any::io::{decode_epw, encode_epw};
    use semio_s_plugin_stdio::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::apply_epw_mutation;
    use semio_s_plugin_stdio::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriod, EpwDataPeriods, EpwLocation, EpwRecord, EPW_RECORD_FIELD_COUNT};
    use semio_s_plugin_stdio::artifacts::epw::{EpwMutation, EpwSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::epw::standards::v_energyplus::subsets::any::project_epw;

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

    fn location_from(value: &Json) -> EpwLocation {
        EpwLocation {
            city: value.str("city"),
            state_province: value.str("stateProvince"),
            country: value.str("country"),
            source: value.str("source"),
            wmo: value.str("wmo"),
            latitude: value.str("latitude"),
            longitude: value.str("longitude"),
            time_zone: value.str("timeZone"),
            elevation: value.str("elevation"),
        }
    }

    fn data_periods_from(value: &Json) -> EpwDataPeriods {
        let records_per_hour = match value.get("recordsPerHour") {
            Some(Json::Number(n)) => *n as u32,
            _ => 0,
        };
        let periods = value.array("periods").iter().map(|period| EpwDataPeriod { name: period.str("name"), start_day_of_week: period.str("startDayOfWeek"), start_date: period.str("startDate"), end_date: period.str("endDate") }).collect();
        EpwDataPeriods { records_per_hour, periods }
    }

    fn record_from(fields: &[String]) -> EpwRecord {
        let array: [String; EPW_RECORD_FIELD_COUNT] = std::array::from_fn(|i| fields.get(i).cloned().unwrap_or_default());
        EpwRecord::from_fields(array)
    }

    /// 🔀️ The same JSON mutation spec the oracle reads, turned into this repository's own typed
    /// `EpwMutation` — the only channel between the feature's parameters and the subject's codec.
    fn mutation_from_spec(spec: &Json) -> Result<EpwMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        let number = |key: &str| match params.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        };
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => EpwMutation::NoMutation,
            "set-snapshot" => {
                let snapshot = params.get("snapshot").cloned().unwrap_or(Json::Null);
                let records = snapshot.array("records").iter().map(|row| if let Json::Array(cells) = row { record_from(&cells.iter().map(|c| if let Json::String(s) = c { s.clone() } else { String::new() }).collect::<Vec<_>>()) } else { EpwRecord::default() }).collect();
                EpwMutation::SetSnapshot {
                    snapshot: EpwSnapshot {
                        location: location_from(&snapshot.get("location").cloned().unwrap_or(Json::Null)),
                        design_conditions: snapshot.str("designConditions"),
                        typical_extreme_periods: snapshot.str("typicalExtremePeriods"),
                        ground_temperatures: snapshot.str("groundTemperatures"),
                        holidays_dst: snapshot.str("holidaysDst"),
                        comments_1: snapshot.str("comments1"),
                        comments_2: snapshot.str("comments2"),
                        data_periods: data_periods_from(&snapshot.get("dataPeriods").cloned().unwrap_or(Json::Null)),
                        records,
                        ..EpwSnapshot::default()
                    },
                }
            }
            "set-location" => EpwMutation::SetLocation { location: location_from(&params.get("location").cloned().unwrap_or(Json::Null)) },
            "set-design-conditions" => EpwMutation::SetDesignConditions { value: params.str("value") },
            "set-typical-extreme-periods" => EpwMutation::SetTypicalExtremePeriods { value: params.str("value") },
            "set-ground-temperatures" => EpwMutation::SetGroundTemperatures { value: params.str("value") },
            "set-holidays-dst" => EpwMutation::SetHolidaysDst { value: params.str("value") },
            "set-comments-1" => EpwMutation::SetComments1 { value: params.str("value") },
            "set-comments-2" => EpwMutation::SetComments2 { value: params.str("value") },
            "set-data-periods" => EpwMutation::SetDataPeriods { data_periods: data_periods_from(&params.get("dataPeriods").cloned().unwrap_or(Json::Null)) },
            "insert-record" => EpwMutation::InsertRecord { index: number("index").ok_or("insert-record: missing `index`")? as usize, record: record_from(&strings(&params, "fields")) },
            "remove-record" => EpwMutation::RemoveRecord { index: number("index").ok_or("remove-record: missing `index`")? as usize },
            "set-record-field" => EpwMutation::SetRecordField { record_index: number("recordIndex").ok_or("set-record-field: missing `recordIndex`")? as usize, field_index: number("fieldIndex").ok_or("set-record-field: missing `fieldIndex`")? as usize, value: params.str("value") },
            other => return Err(format!("no subject rule for kind {other:?}")),
        })
    }

    fn decode(bytes: &[u8]) -> Result<EpwSnapshot, String> {
        let text = String::from_utf8(bytes.to_vec()).map_err(|error| format!("input is not UTF-8: {error}"))?;
        decode_epw(&text)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        apply_epw_mutation(&mut snapshot, &mutation);
        let output = encode_epw(&snapshot).into_bytes();
        let projection = project_epw(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&input)?;
        apply_epw_mutation(&mut snapshot, &mutation_from_spec(&spec)?);
        apply_epw_mutation(&mut snapshot, &mutation_from_spec(&inverse_spec(&input, &spec)?)?);
        let output = encode_epw(&snapshot).into_bytes();
        let projection = project_epw(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating (fleet brief, "the point of this wave") and this tripwire catches
    /// it. The real stub fixture is committed with CRLF line endings and this subset's own encoder
    /// also always writes CRLF (`codec_retention_law` in `../../🏅️standards/🔖️energyplus/🪆️subsets/
    /// ✳️any/🚪️io/🦀️component.rs` proves decode→encode is byte-preserving on it), so this scenario's
    /// non-triviality rests on genuinely mutating nothing and still routing through the typed model
    /// — see that Feature's own scenario text for the exact assertion this performs.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(&input)?;
        let output = encode_epw(&snapshot).into_bytes();
        let projection = project_epw(&output)?;
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
