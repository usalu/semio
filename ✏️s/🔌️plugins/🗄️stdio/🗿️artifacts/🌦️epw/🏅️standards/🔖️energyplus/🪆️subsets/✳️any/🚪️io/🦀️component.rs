//! 🚪️ IO — real EnergyPlus Weather (EPW) text codec (LOSSLESS: all 8 header lines + all 35
//! per-record columns, https://bigladdersoftware.com/epx/docs/9-6/auxiliary-programs/energyplus-
//! weather-file-epw-data-dictionary.html — see `…/schema/snapshot` module doc for the full
//! rationale) plus composition/registration. 🦑 Codec + registration dissolved out of the former
//! `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriod, EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot, EPW_RECORD_FIELD_COUNT, STDIO_EPW_DOCUMENT_SCHEMA};

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
    use crate::artifacts::epw::standards::energyplus::subsets::any::schema::EpwAnalyzer;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct EpwComposerComposition;

    impl ArtifactComposition for EpwComposerComposition {
        type Snapshot = EpwSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "EpwComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = EpwAnalyzer::analyze(&native).await;
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "EpwComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, inferences, document codec. Called from
    /// this artifact's root-level `register()` (former standard-level `engine::register()`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::epw::standards::energyplus::subsets::any::schema::epw_artifact_schema_descriptor());
        register_artifact_inferences();
        let _ = store::register_document_codec(store::ArtifactCodec::of::<EpwSnapshot, crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::EpwMutation>(
            crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::STDIO_EPW_DOCUMENT_SCHEMA,
        ));
    }

    /// 💡️ Registers `s.stdio.epw.inference`'s facet leaves into the OS-wide inference catalog —
    /// sibling to the artifact schema descriptor above (separate registry, ticket
    /// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::epw::standards::energyplus::subsets::any::schema::inferences::epw_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️Sniff
/// 🔍️ Real magic: an EPW file's first line always starts with the `LOCATION` keyword
/// (https://bigladdersoftware.com/epx/docs/9-6/auxiliary-programs/energyplus-weather-file-epw-data-dictionary.html#location).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    let text = String::from_utf8_lossy(bytes);
    text.trim_start().starts_with("LOCATION,")
}
//#endregion 🔖️Sniff

//#region 🔖️LineSplit
/// ✂️ Splits on the file's own line-ending convention (CRLF if present anywhere, else bare LF —
/// real EPW files are CRLF, but decode stays lenient for hand-edited/foreign input); drops a
/// single trailing empty segment produced by a final line terminator.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_lines(text: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = if text.contains("\r\n") { text.split("\r\n").collect() } else { text.split('\n').collect() };
    if lines.last() == Some(&"") {
        lines.pop();
    }
    lines
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn require_prefix<'a>(line: &'a str, prefix: &str) -> Result<&'a str, String> {
    if line.starts_with(prefix) {
        Ok(line)
    } else {
        Err(format!("epw: expected a line starting with {prefix:?}, got {line:?}"))
    }
}
//#endregion 🔖️LineSplit

//#region 🔖️Location
/// 📐️ EPW LOCATION line: `LOCATION,City,StateProvince,Country,Source,WMO,Latitude,Longitude,
/// TimeZone,Elevation` — 10 comma-separated tokens (`LOCATION` keyword + 9 data fields).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_location_line(line: &str) -> Result<EpwLocation, String> {
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() != 10 || fields[0] != "LOCATION" {
        return Err(format!("epw: LOCATION line must have exactly 10 fields, got {}: {line:?}", fields.len()));
    }
    Ok(EpwLocation {
        city: fields[1].to_string(),
        state_province: fields[2].to_string(),
        country: fields[3].to_string(),
        source: fields[4].to_string(),
        wmo: fields[5].to_string(),
        latitude: fields[6].to_string(),
        longitude: fields[7].to_string(),
        time_zone: fields[8].to_string(),
        elevation: fields[9].to_string(),
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_location_line(l: &EpwLocation) -> String {
    format!("LOCATION,{},{},{},{},{},{},{},{},{}", l.city, l.state_province, l.country, l.source, l.wmo, l.latitude, l.longitude, l.time_zone, l.elevation)
}
//#endregion 🔖️Location

//#region 🔖️DataPeriods
/// 📐️ DATA PERIODS line: `DATA PERIODS,N,RecordsPerHour,(Name,StartDayOfWeek,StartDate,EndDate)×N`.
/// The leading `N` is re-derived from `periods.len()` on encode (redundant, not lossy).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_data_periods_line(line: &str) -> Result<EpwDataPeriods, String> {
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() < 3 || fields[0] != "DATA PERIODS" {
        return Err(format!("epw: DATA PERIODS line malformed: {line:?}"));
    }
    let n_periods: usize = fields[1].parse().map_err(|_| format!("epw: bad DATA PERIODS count {:?}", fields[1]))?;
    let records_per_hour: u32 = fields[2].parse().map_err(|_| format!("epw: bad DATA PERIODS records-per-hour {:?}", fields[2]))?;
    let rest = &fields[3..];
    if rest.len() != n_periods * 4 {
        return Err(format!("epw: DATA PERIODS expected {} period fields for {n_periods} period(s), got {}", n_periods * 4, rest.len()));
    }
    let periods = rest.chunks(4).map(|c| EpwDataPeriod { name: c[0].to_string(), start_day_of_week: c[1].to_string(), start_date: c[2].to_string(), end_date: c[3].to_string() }).collect();
    Ok(EpwDataPeriods { records_per_hour, periods })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_data_periods_line(d: &EpwDataPeriods) -> String {
    let mut out = format!("DATA PERIODS,{},{}", d.periods.len(), d.records_per_hour);
    for p in &d.periods {
        out.push_str(&format!(",{},{},{},{}", p.name, p.start_day_of_week, p.start_date, p.end_date));
    }
    out
}
//#endregion 🔖️DataPeriods

//#region 🔖️Record
/// 📐️ One data record: exactly 35 comma-separated columns, spec order — no defaults, no
/// coercion; a wrong column count is a hard decode error (contrast with energy's plugin-side
/// `EpwWeather::parse`, which silently defaults short/malformed rows).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_record_line(line: &str) -> Result<EpwRecord, String> {
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() != EPW_RECORD_FIELD_COUNT {
        return Err(format!("expected {EPW_RECORD_FIELD_COUNT} columns, got {}: {line:?}", fields.len()));
    }
    let values: Vec<String> = fields.iter().map(|s| s.to_string()).collect();
    let arr: [String; EPW_RECORD_FIELD_COUNT] = values.try_into().map_err(|_| "record: field count mismatch".to_string())?;
    Ok(EpwRecord::from_fields(arr))
}
//#endregion 🔖️Record

//#region 🔖️SnapshotCodec
/// 📥️ Decodes a full EPW text document: 8 typed/retained header lines + N fully-typed 35-column
/// data records.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_epw(text: &str) -> Result<EpwSnapshot, String> {
    let lines = split_lines(text);
    if lines.len() < 8 {
        return Err(format!("epw: expected at least 8 header lines, got {}", lines.len()));
    }
    let location = parse_location_line(lines[0])?;
    let design_conditions = require_prefix(lines[1], "DESIGN CONDITIONS")?.to_string();
    let typical_extreme_periods = require_prefix(lines[2], "TYPICAL/EXTREME PERIODS")?.to_string();
    let ground_temperatures = require_prefix(lines[3], "GROUND TEMPERATURES")?.to_string();
    let holidays_dst = require_prefix(lines[4], "HOLIDAYS/DAYLIGHT SAVINGS")?.to_string();
    let comments_1 = require_prefix(lines[5], "COMMENTS 1")?.to_string();
    let comments_2 = require_prefix(lines[6], "COMMENTS 2")?.to_string();
    let data_periods = parse_data_periods_line(lines[7])?;

    let mut records = Vec::with_capacity(lines.len().saturating_sub(8));
    for (i, line) in lines[8..].iter().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        records.push(parse_record_line(line).map_err(|e| format!("epw: record {i}: {e}"))?);
    }
    if records.is_empty() {
        return Err("epw: no data records".into());
    }

    Ok(EpwSnapshot { schema: STDIO_EPW_DOCUMENT_SCHEMA.into(), location, design_conditions, typical_extreme_periods, ground_temperatures, holidays_dst, comments_1, comments_2, data_periods, records })
}

/// 📤️ Encodes a full EPW text document. Always emits CRLF line endings (the real EnergyPlus
/// convention, matching every field's own retained W0 fixture) with a trailing CRLF after the
/// last record.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_epw(snap: &EpwSnapshot) -> String {
    let mut lines: Vec<String> = Vec::with_capacity(8 + snap.records.len());
    lines.push(encode_location_line(&snap.location));
    lines.push(snap.design_conditions.clone());
    lines.push(snap.typical_extreme_periods.clone());
    lines.push(snap.ground_temperatures.clone());
    lines.push(snap.holidays_dst.clone());
    lines.push(snap.comments_1.clone());
    lines.push(snap.comments_2.clone());
    lines.push(encode_data_periods_line(&snap.data_periods));
    for r in &snap.records {
        lines.push(r.fields().join(","));
    }
    let mut out = lines.join("\r\n");
    out.push_str("\r\n");
    out
}
//#endregion 🔖️SnapshotCodec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const REAL_FIXTURE: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🌦️example.epw");

    #[semio_framework_async_macros::async_test]
    async fn sniffs_and_parses_a_real_shaped_location_line() {
        let line = "LOCATION,Hannover,Niedersachsen,DEU,semio-fixture,10238,52.37,9.74,1.0,55.0";
        assert!(sniff_real_bytes(line.as_bytes()));
        let loc = parse_location_line(line).expect("parse");
        assert_eq!(loc.city, "Hannover");
        assert_eq!(loc.latitude, "52.37");
        assert_eq!(loc.elevation, "55.0");
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_a_short_location_line() {
        assert!(parse_location_line("LOCATION,Hannover").is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_a_record_with_the_wrong_column_count() {
        assert!(parse_record_line("2026,1,15,1,0").is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn decodes_the_real_fixture_with_all_24_records_and_35_columns() {
        let snap = decode_epw(REAL_FIXTURE).expect("decode real fixture");
        assert_eq!(snap.location.city, "Hannover");
        assert_eq!(snap.location.latitude, "52.37");
        assert_eq!(snap.location.time_zone, "1.0");
        assert_eq!(snap.location.elevation, "55.0");
        assert_eq!(snap.records.len(), 24, "fixture is one full day, hour-ending 1..24");
        for (i, r) in snap.records.iter().enumerate() {
            assert_eq!(r.hour, (i + 1).to_string(), "hour column must run 1..24 in order");
            assert_eq!(r.fields().len(), EPW_RECORD_FIELD_COUNT);
        }
        assert_eq!(snap.data_periods.records_per_hour, 1);
        assert_eq!(snap.data_periods.periods.len(), 1);
        assert_eq!(snap.data_periods.periods[0].name, "Data");
    }

    //#region 🔖️CodecRetentionLaw
    /// 🔁️ decode→encode is byte-preserving on the real W0 fixture
    /// (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📚️examples/🎬️demo/🖼️assets/🌦️example.epw`,
    /// verified upstream by `verify_epw.py`): all 24 records × 35 columns exact, all 8 header
    /// lines exact, byte-for-byte incl. CRLF.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = decode_epw(REAL_FIXTURE).expect("decode real fixture");
        let reencoded = encode_epw(&snap);
        assert_eq!(reencoded, REAL_FIXTURE, "decode->encode must be byte-preserving on the real W0 fixture");

        let reparsed = decode_epw(&reencoded).expect("re-decode");
        assert_eq!(reparsed, snap, "re-parsing the re-encoded text must yield the identical snapshot");
    }
    //#endregion 🔖️CodecRetentionLaw
}
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::epw::standards::energyplus::subsets::any::schema::EpwComposer as EpwRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<EpwRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
