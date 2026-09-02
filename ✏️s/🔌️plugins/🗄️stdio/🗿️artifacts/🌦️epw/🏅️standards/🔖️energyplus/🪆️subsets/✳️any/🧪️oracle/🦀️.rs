//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! independently of this repository's own codec so the subject has something real to be compared
//! against instead of being checked against its own reading.
//!
//! Reference: csv, exactly as the sibling `csv-rfc4180-mutate` entry uses it — genuinely
//! independent for EPW's 8,760 comma-separated hourly RECORDS (35 columns each), authoritative for
//! record structure and field values. `no-mutation`/`insert-record`/`remove-record`/
//! `set-record-field` go through it both as producer and reader and are typed `@mode-differential`
//! in this case's feature file.
//!
//! The 8 EPW header lines (LOCATION, DESIGN CONDITIONS, TYPICAL/EXTREME PERIODS, GROUND
//! TEMPERATURES, HOLIDAYS/DAYLIGHT SAVINGS, COMMENTS 1, COMMENTS 2, DATA PERIODS) are
//! EnergyPlus-specific grammar `csv` knows nothing about — it splits comma-separated cells, not
//! EPW field MEANING, and no third-party crate validates that meaning here (`epw-rs` is alpha and
//! read-only, rejected per the fleet brief's §6). `set-snapshot`/`set-location`/
//! `set-design-conditions`/`set-typical-extreme-periods`/`set-ground-temperatures`/
//! `set-holidays-dst`/`set-comments-1`/`set-comments-2`/`set-data-periods` are performed by this
//! module writing the header bytes itself (hand-rolled, independent of the subject crate — this
//! oracle role must never link it) and are typed `@mode-property`: no independent second PRODUCER
//! exists for header semantics, only this module's own self-consistent construction, read back with
//! `csv`'s generic comma-grid reader for structural evidence.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️LineSplit
/// ✂️ Splits on the file's own line-ending convention (CRLF if present anywhere, else bare LF),
/// dropping a single trailing empty segment produced by a final line terminator. Hand-rolled here,
/// independently of the subject crate's own `io::split_lines` — this is generic text mechanics, not
/// EPW-specific grammar, so duplicating it costs nothing and keeps the oracle role from linking the
/// subject.
#[cfg(feature = "oracles")]
fn split_lines(text: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = if text.contains("\r\n") { text.split("\r\n").collect() } else { text.split('\n').collect() };
    if lines.last() == Some(&"") {
        lines.pop();
    }
    lines
}
//#endregion 🔖️LineSplit

//#region 🔖️RecordGrid
/// 📥️ Independent read of the RECORD half only (everything after the 8 fixed header lines): every
/// row as a flat string grid via `csv`, `flexible(true)` because EPW's 35-column width is real
/// per-record information this crate is trusted for, not an assumption it bakes in.
#[cfg(feature = "oracles")]
pub fn read_record_grid(record_text: &str) -> Result<Vec<Vec<String>>, String> {
    let mut reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_reader(record_text.as_bytes());
    reader.records().map(|result| result.map(|record| record.iter().map(|cell| cell.to_string()).collect())).collect::<Result<Vec<_>, _>>().map_err(|error| format!("independent reader could not read an EPW record: {error}"))
}

/// 📤️ Independent write of the RECORD half: the reference writer decides its own minimal quoting,
/// and the terminator is set EXPLICITLY to CRLF because that is EPW's own real convention — the
/// `csv` crate's writer default is a bare `\n` (only its READER treats CRLF permissively), so
/// leaving it unset silently emitted an LF-terminated weather file whose records no longer matched
/// the format the header half above is written in. Caught by the `identity-round-trip` scenario's
/// carrier-law assertion, which reported a 6100-byte output against a 6124-byte input — exactly one
/// lost byte per data record.
#[cfg(feature = "oracles")]
pub fn write_record_grid(grid: &[Vec<String>]) -> Result<String, String> {
    let mut writer = csv::WriterBuilder::new().flexible(true).terminator(csv::Terminator::CRLF).from_writer(Vec::new());
    for record in grid {
        writer.write_record(record).map_err(|error| format!("epw record: {error}"))?;
    }
    let bytes = writer.into_inner().map_err(|error| format!("epw record grid finish: {error}"))?;
    String::from_utf8(bytes).map_err(|error| format!("epw record grid is not UTF-8: {error}"))
}
//#endregion 🔖️RecordGrid

//#region 🔖️Document
/// 🧬️ One EPW document split at the fixed 8-header-line boundary. The 8 header lines are kept RAW
/// (no CSV involvement — see module doc) and the records are the `csv`-read grid.
#[cfg(feature = "oracles")]
struct EpwDoc {
    header: [String; 8],
    records: Vec<Vec<String>>,
}

#[cfg(feature = "oracles")]
fn parse_doc(input: &[u8]) -> Result<EpwDoc, String> {
    let text = std::str::from_utf8(input).map_err(|error| format!("epw input is not UTF-8: {error}"))?;
    let lines = split_lines(text);
    if lines.len() < 8 {
        return Err(format!("epw: expected at least 8 header lines, got {}", lines.len()));
    }
    let header: [String; 8] = std::array::from_fn(|i| lines[i].to_string());
    let record_text = lines[8..].join("\n");
    let records = if record_text.is_empty() { Vec::new() } else { read_record_grid(&record_text)? };
    Ok(EpwDoc { header, records })
}

#[cfg(feature = "oracles")]
fn encode_doc(doc: &EpwDoc) -> Result<Vec<u8>, String> {
    let mut out = String::new();
    for line in &doc.header {
        out.push_str(line);
        out.push_str("\r\n");
    }
    out.push_str(&write_record_grid(&doc.records)?);
    Ok(out.into_bytes())
}

/// 🔁️ Independent decode/re-encode: the 8 header lines copied raw, the record grid genuinely
/// round-tripped through `csv`'s reader then writer. Used by the case's own `identity-round-trip`
/// oracle role — distinct from the `no-mutation` KIND, which is a true byte identity by design.
#[cfg(feature = "oracles")]
pub fn round_trip_epw(bytes: &[u8]) -> Result<Vec<u8>, String> {
    encode_doc(&parse_doc(bytes)?)
}

#[cfg(not(feature = "oracles"))]
pub fn round_trip_epw(_bytes: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Document

//#region 🔖️Header
/// 📍️ LOCATION line: `LOCATION,City,StateProvince,Country,Source,WMO,Latitude,Longitude,TimeZone,
/// Elevation` — plain comma join, no third-party involvement (see module doc: `csv` gives no more
/// assurance here than a hand join would, since it carries no knowledge of the 10-field shape).
#[cfg(feature = "oracles")]
fn location_line(location: &Json) -> String {
    let f = |key: &str| location.str(key);
    format!("LOCATION,{},{},{},{},{},{},{},{},{}", f("city"), f("stateProvince"), f("country"), f("source"), f("wmo"), f("latitude"), f("longitude"), f("timeZone"), f("elevation"))
}

#[cfg(feature = "oracles")]
fn parse_location_line(line: &str) -> Result<Json, String> {
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() != 10 || fields[0] != "LOCATION" {
        return Err(format!("epw: LOCATION line must have exactly 10 fields, got {}: {line:?}", fields.len()));
    }
    Ok(json_object(vec![
        ("city", Json::String(fields[1].to_string())),
        ("stateProvince", Json::String(fields[2].to_string())),
        ("country", Json::String(fields[3].to_string())),
        ("source", Json::String(fields[4].to_string())),
        ("wmo", Json::String(fields[5].to_string())),
        ("latitude", Json::String(fields[6].to_string())),
        ("longitude", Json::String(fields[7].to_string())),
        ("timeZone", Json::String(fields[8].to_string())),
        ("elevation", Json::String(fields[9].to_string())),
    ]))
}

/// 📅️ DATA PERIODS line: `DATA PERIODS,N,RecordsPerHour,(Name,StartDayOfWeek,StartDate,EndDate)×N`.
#[cfg(feature = "oracles")]
fn data_periods_line(data_periods: &Json) -> String {
    let records_per_hour = data_periods.get("recordsPerHour").and_then(|v| if let Json::Number(n) = v { Some(*n as i64) } else { None }).unwrap_or(1);
    let periods = data_periods.array("periods");
    let mut out = format!("DATA PERIODS,{},{}", periods.len(), records_per_hour);
    for period in periods {
        out.push_str(&format!(",{},{},{},{}", period.str("name"), period.str("startDayOfWeek"), period.str("startDate"), period.str("endDate")));
    }
    out
}

#[cfg(feature = "oracles")]
fn parse_data_periods_line(line: &str) -> Result<Json, String> {
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() < 3 || fields[0] != "DATA PERIODS" {
        return Err(format!("epw: DATA PERIODS line malformed: {line:?}"));
    }
    let n_periods: usize = fields[1].parse().map_err(|_| format!("epw: bad DATA PERIODS count {:?}", fields[1]))?;
    let records_per_hour: f64 = fields[2].parse().map_err(|_| format!("epw: bad DATA PERIODS records-per-hour {:?}", fields[2]))?;
    let rest = &fields[3..];
    if rest.len() != n_periods * 4 {
        return Err(format!("epw: DATA PERIODS expected {} period fields for {n_periods} period(s), got {}", n_periods * 4, rest.len()));
    }
    let periods: Vec<Json> =
        rest.chunks(4).map(|c| json_object(vec![("name", Json::String(c[0].to_string())), ("startDayOfWeek", Json::String(c[1].to_string())), ("startDate", Json::String(c[2].to_string())), ("endDate", Json::String(c[3].to_string()))])).collect();
    Ok(json_object(vec![("recordsPerHour", Json::Number(records_per_hour)), ("periods", Json::Array(periods))]))
}
//#endregion 🔖️Header

//#region 🔖️JsonHelpers
#[cfg(feature = "oracles")]
fn json_object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}
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
//#endregion 🔖️JsonHelpers

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
        "set-snapshot" => {
            let snapshot = params.get("snapshot").cloned().unwrap_or(Json::Null);
            let header: [String; 8] = [
                location_line(&snapshot.get("location").cloned().unwrap_or(Json::Null)),
                snapshot.str("designConditions"),
                snapshot.str("typicalExtremePeriods"),
                snapshot.str("groundTemperatures"),
                snapshot.str("holidaysDst"),
                snapshot.str("comments1"),
                snapshot.str("comments2"),
                data_periods_line(&snapshot.get("dataPeriods").cloned().unwrap_or(Json::Null)),
            ];
            let records: Vec<Vec<String>> = snapshot
                .array("records")
                .iter()
                .map(|row| match row {
                    Json::Array(cells) => cells.iter().map(|c| if let Json::String(s) = c { s.clone() } else { String::new() }).collect(),
                    _ => Vec::new(),
                })
                .collect();
            encode_doc(&EpwDoc { header, records })
        }
        "set-location" => {
            let mut doc = parse_doc(input)?;
            doc.header[0] = location_line(&params.get("location").cloned().unwrap_or(Json::Null));
            encode_doc(&doc)
        }
        "set-design-conditions" => {
            let mut doc = parse_doc(input)?;
            doc.header[1] = params.str("value");
            encode_doc(&doc)
        }
        "set-typical-extreme-periods" => {
            let mut doc = parse_doc(input)?;
            doc.header[2] = params.str("value");
            encode_doc(&doc)
        }
        "set-ground-temperatures" => {
            let mut doc = parse_doc(input)?;
            doc.header[3] = params.str("value");
            encode_doc(&doc)
        }
        "set-holidays-dst" => {
            let mut doc = parse_doc(input)?;
            doc.header[4] = params.str("value");
            encode_doc(&doc)
        }
        "set-comments-1" => {
            let mut doc = parse_doc(input)?;
            doc.header[5] = params.str("value");
            encode_doc(&doc)
        }
        "set-comments-2" => {
            let mut doc = parse_doc(input)?;
            doc.header[6] = params.str("value");
            encode_doc(&doc)
        }
        "set-data-periods" => {
            let mut doc = parse_doc(input)?;
            doc.header[7] = data_periods_line(&params.get("dataPeriods").cloned().unwrap_or(Json::Null));
            encode_doc(&doc)
        }
        "insert-record" => {
            let mut doc = parse_doc(input)?;
            let index = number(&params, "index").ok_or("insert-record: missing `index`")? as usize;
            let record = strings(&params, "fields");
            doc.records.insert(index.min(doc.records.len()), record);
            encode_doc(&doc)
        }
        "remove-record" => {
            let mut doc = parse_doc(input)?;
            let index = number(&params, "index").ok_or("remove-record: missing `index`")? as usize;
            if index >= doc.records.len() {
                return Err(format!("remove-record: index {index} out of bounds ({} record(s))", doc.records.len()));
            }
            doc.records.remove(index);
            encode_doc(&doc)
        }
        "set-record-field" => {
            let mut doc = parse_doc(input)?;
            let record_index = number(&params, "recordIndex").ok_or("set-record-field: missing `recordIndex`")? as usize;
            let field_index = number(&params, "fieldIndex").ok_or("set-record-field: missing `fieldIndex`")? as usize;
            let value = params.str("value");
            let record_count = doc.records.len();
            let record = doc.records.get_mut(record_index).ok_or_else(|| format!("set-record-field: record index {record_index} out of bounds ({record_count} record(s))"))?;
            if field_index >= record.len() {
                record.resize(field_index + 1, String::new());
            }
            record[field_index] = value;
            encode_doc(&doc)
        }
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementations are not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Projection
/// 👁️ Projects EPW bytes onto the `semantic-epw-v1` shape this case's oracle and subject are both
/// compared through: `location`/`dataPeriods` typed, the four retained-verbatim header blocks plus
/// both comment lines as plain strings, and the full ordered record grid.
#[cfg(feature = "oracles")]
pub fn project_epw(bytes: &[u8]) -> Result<Json, String> {
    let doc = parse_doc(bytes)?;
    let location = parse_location_line(&doc.header[0])?;
    let data_periods = parse_data_periods_line(&doc.header[7])?;
    let records: Vec<Json> = doc.records.iter().map(|row| Json::Array(row.iter().cloned().map(Json::String).collect())).collect();
    Ok(json_object(vec![
        ("format", Json::String("epw".to_string())),
        ("location", location),
        ("designConditions", Json::String(doc.header[1].clone())),
        ("typicalExtremePeriods", Json::String(doc.header[2].clone())),
        ("groundTemperatures", Json::String(doc.header[3].clone())),
        ("holidaysDst", Json::String(doc.header[4].clone())),
        ("comments1", Json::String(doc.header[5].clone())),
        ("comments2", Json::String(doc.header[6].clone())),
        ("dataPeriods", data_periods),
        ("recordCount", Json::Number(doc.records.len() as f64)),
        ("records", Json::Array(records)),
    ]))
}

#[cfg(not(feature = "oracles"))]
pub fn project_epw(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Projection

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    const REAL_FIXTURE: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🌦️example.epw");

    fn spec(kind: &str, params: Json) -> Json {
        json_object(vec![("kind", Json::String(kind.to_string())), ("params", params)])
    }

    #[test]
    fn no_mutation_is_a_true_byte_identity() {
        let output = oracle_apply_mutation(REAL_FIXTURE, &spec("no-mutation", Json::Object(vec![]))).unwrap();
        assert_eq!(output, REAL_FIXTURE);
    }

    #[test]
    fn insert_and_remove_record_are_inverse_on_the_real_fixture() {
        let fields: Vec<Json> = vec!["2026", "1", "15", "99", "0"].into_iter().map(|s| Json::String(s.to_string())).chain((0..30).map(|_| Json::String(String::new()))).collect();
        let inserted = oracle_apply_mutation(REAL_FIXTURE, &spec("insert-record", json_object(vec![("index", Json::Number(1.0)), ("fields", Json::Array(fields))]))).unwrap();
        let inserted_doc = parse_doc(&inserted).unwrap();
        assert_eq!(inserted_doc.records.len(), 25, "24 real records + 1 inserted");
        assert_eq!(inserted_doc.records[1][3], "99", "the inserted record's hour column must land at index 1");

        let removed = oracle_apply_mutation(&inserted, &spec("remove-record", json_object(vec![("index", Json::Number(1.0))]))).unwrap();
        let removed_doc = parse_doc(&removed).unwrap();
        let original_doc = parse_doc(REAL_FIXTURE).unwrap();
        assert_eq!(removed_doc.records, original_doc.records, "insert then remove at the same index must restore the original record grid");
    }

    #[test]
    fn set_record_field_patches_a_single_cell() {
        let output = oracle_apply_mutation(REAL_FIXTURE, &spec("set-record-field", json_object(vec![("recordIndex", Json::Number(2.0)), ("fieldIndex", Json::Number(6.0)), ("value", Json::String("12.3".to_string()))]))).unwrap();
        let doc = parse_doc(&output).unwrap();
        assert_eq!(doc.records[2][6], "12.3");
    }

    #[test]
    fn set_location_replaces_only_the_location_line() {
        let location = json_object(vec![
            ("city", Json::String("Berlin".to_string())),
            ("stateProvince", Json::String("Berlin".to_string())),
            ("country", Json::String("DEU".to_string())),
            ("source", Json::String("semio-fixture".to_string())),
            ("wmo", Json::String("10382".to_string())),
            ("latitude", Json::String("52.52".to_string())),
            ("longitude", Json::String("13.405".to_string())),
            ("timeZone", Json::String("1.0".to_string())),
            ("elevation", Json::String("34.0".to_string())),
        ]);
        let output = oracle_apply_mutation(REAL_FIXTURE, &spec("set-location", json_object(vec![("location", location)]))).unwrap();
        let projection = project_epw(&output).unwrap();
        assert_eq!(projection.get("location").unwrap().str("city"), "Berlin");
        let original = project_epw(REAL_FIXTURE).unwrap();
        assert_eq!(projection.get("records"), original.get("records"), "set-location must not touch the records");
    }

    #[test]
    fn project_epw_round_trips_the_real_fixture_structurally() {
        let projection = project_epw(REAL_FIXTURE).unwrap();
        assert_eq!(projection.str("format"), "epw");
        assert_eq!(projection.get("location").unwrap().str("city"), "Hannover");
        assert_eq!(projection.get("recordCount").unwrap(), &Json::Number(24.0));
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let result = oracle_apply_mutation(REAL_FIXTURE, &spec("not-a-real-kind", Json::Object(vec![])));
        assert!(result.is_err(), "an unrecognised kind must fail loudly");
    }
}
//#endregion 🧪️Tests
