//! 🚪️ IO — real EnergyPlus Weather (EPW) text codec (LOSSLESS: all 8 header lines + all 35
//! per-record columns, https://bigladdersoftware.com/epx/docs/9-6/auxiliary-programs/energyplus-
//! weather-file-epw-data-dictionary.html — see `…/schema/snapshot` module doc for the full
//! rationale) plus composition/registration. 🦑 Codec + registration dissolved out of the former
//! `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
use crate::apply_mutation;
use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriod, EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot, EPW_RECORD_FIELD_COUNT, STDIO_EPW_DOCUMENT_SCHEMA};

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
    use crate::standards::energyplus::subsets::any::io::EpwAnalyzer;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct EpwComposerComposition;

    impl ArtifactComposition for EpwComposerComposition {
        type Snapshot = EpwSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
            let analysis = EpwAnalyzer::analyze(&native);
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
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::energyplus::subsets::any::schema::epw_artifact_schema_descriptor()).expect("schema descriptor publication");
        register_artifact_inferences();
        semio_framework_plugin::io::register_native_document_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.epw", standard: semio_framework_artifact_reference::StandardId("energyplus"), subset: semio_framework_artifact_reference::SubsetId("*") }, store::ArtifactCodec::bare::<EpwSnapshot, crate::standards::energyplus::subsets::any::schema::mutations::EpwMutation>(crate::standards::energyplus::subsets::any::schema::snapshot::STDIO_EPW_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
    }

    /// 💡️ Registers `s.stdio.epw.inference`'s facet leaves into the OS-wide inference catalog —
    /// sibling to the artifact schema descriptor above (separate registry, ticket
    /// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::energyplus::subsets::any::schema::inferences::epw_artifact_inference_descriptor()).expect("schema descriptor publication");
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
/// coercion; a wrong column count is a hard decode error.
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::energyplus::subsets::any::io::EpwComposer as EpwRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<EpwRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::standards::energyplus::subsets::any::schema::diff::EpwDiff;
    use crate::standards::energyplus::subsets::any::schema::mutations::{EpwMutation};

    use crate::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct EpwBuilderConstruction {
        snapshot: EpwSnapshot,
    }

    impl ArtifactBuilder for EpwBuilderConstruction {
        type Snapshot = EpwSnapshot;
        type Mutation = EpwMutation;
        type Diff = EpwDiff;
        fn empty() -> Self {
            Self { snapshot: EpwSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<EpwSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<EpwSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::energyplus::subsets::any::io;
    use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwSnapshot, STDIO_EPW_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct EpwParts {
        pub snapshot: Option<EpwSnapshot>,
    }

    pub struct EpwAnalyzerAnalysis;

    impl ArtifactAnalysis for EpwAnalyzerAnalysis {
        type Parts = EpwParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if io::sniff_real_bytes(bytes) {
                        return semio_framework_plugin::io::Confidence::High;
                    }
                    let marker = STDIO_EPW_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if io::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_EPW_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = EpwParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <EpwSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <EpwSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec EpwBuilderFacets {
        construction: EpwBuilderConstruction,
        analysis: EpwAnalyzerAnalysis,
        composition: crate::standards::energyplus::subsets::any::io::derived_composition::EpwComposerComposition,
    }
    builder: EpwBuilder,
    analyzer: EpwAnalyzer,
    composer: EpwComposer,
);
