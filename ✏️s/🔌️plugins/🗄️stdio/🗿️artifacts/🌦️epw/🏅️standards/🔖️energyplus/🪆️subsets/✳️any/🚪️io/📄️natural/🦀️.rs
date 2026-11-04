//! 📄️ The natural EnergyPlus Weather text codec: 8 header lines and 35-column records, shared by the editor's natural-file open/save and the IO registry.

use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriod, EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot, EPW_RECORD_FIELD_COUNT, STDIO_EPW_DOCUMENT_SCHEMA};

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
pub fn parse_record_line(line: &str) -> Result<EpwRecord, String> {
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() != EPW_RECORD_FIELD_COUNT {
        return Err(format!("expected {EPW_RECORD_FIELD_COUNT} columns, got {}: {line:?}", fields.len()));
    }
    let values: Vec<String> = fields.iter().map(|s| s.to_string()).collect();
    let arr: [String; EPW_RECORD_FIELD_COUNT] = values.try_into().map_err(|_| "record: field count mismatch".to_string())?;
    Ok(EpwRecord::from_fields(arr))
}
//#endregion 🔖️Record

//#region 🔖️DecodeError
/// 🚧️ A refused EPW file with a stable code and the reason in both languages. English comes first and no language is the default.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpwDecodeError {
    pub code: &'static str,
    pub en: String,
    pub de: String,
}

impl EpwDecodeError {
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn new(code: &'static str, en: String, de: String) -> Self {
        Self { code, en, de }
    }

    /// 🔤️ The refusal for bytes that are not UTF-8 text.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn not_utf8(detail: &str) -> Self {
        Self::new("stdio.epw.not-utf8", format!("the file is not valid UTF-8: {detail}"), format!("die Datei ist kein gültiges UTF-8: {detail}"))
    }
}

impl From<String> for EpwDecodeError {
    fn from(detail: String) -> Self {
        Self::new("stdio.epw.malformed", detail.clone(), format!("Ungültige EPW-Zeile: {detail}"))
    }
}

impl std::fmt::Display for EpwDecodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[{}] EPW: {} / EPW: {}", self.code, self.en, self.de)
    }
}

impl std::error::Error for EpwDecodeError {}

impl From<EpwDecodeError> for String {
    fn from(error: EpwDecodeError) -> Self {
        error.to_string()
    }
}
//#endregion 🔖️DecodeError

//#region 🔖️SnapshotCodec
/// 📥️ Decodes a full EPW text document: 8 typed/retained header lines + N fully-typed 35-column
/// data records.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_epw(text: &str) -> Result<EpwSnapshot, EpwDecodeError> {
    let lines = split_lines(text);
    if lines.len() < 8 {
        return Err(EpwDecodeError::new("stdio.epw.header-lines", format!("expected at least 8 header lines, got {}", lines.len()), format!("mindestens 8 Kopfzeilen erwartet, {} gefunden", lines.len())));
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
        records.push(parse_record_line(line).map_err(|e| EpwDecodeError::new("stdio.epw.record", format!("record {i}: {e}"), format!("Datensatz {i}: {e}")))?);
    }
    if records.is_empty() {
        return Err(EpwDecodeError::new("stdio.epw.no-records", "the file has no data records".into(), "die Datei enthält keine Datensätze".into()));
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
