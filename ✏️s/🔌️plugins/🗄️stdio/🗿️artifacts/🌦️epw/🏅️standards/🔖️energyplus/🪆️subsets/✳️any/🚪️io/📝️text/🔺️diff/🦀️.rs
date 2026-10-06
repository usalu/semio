//! 📝️ Text representation codec surface for `stdio.epw` (diff).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type EpwDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::energyplus::subsets::any::schema::diff::*;
use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot, EPW_RECORD_FIELD_COUNT};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, HashMap};



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_record(r: &EpwRecord) -> String {
    format!("[{}]", r.fields().iter().map(|f| enc_str(f)).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_record(s: &str) -> Result<EpwRecord, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    if parts.len() != EPW_RECORD_FIELD_COUNT {
        return Err(format!("record: expected {EPW_RECORD_FIELD_COUNT} fields, got {}", parts.len()));
    }
    let mut values: Vec<String> = Vec::with_capacity(EPW_RECORD_FIELD_COUNT);
    for p in parts {
        values.push(dec_str(p)?);
    }
    let arr: [String; EPW_RECORD_FIELD_COUNT] = values.try_into().map_err(|_| "record: field count mismatch".to_string())?;
    Ok(EpwRecord::from_fields(arr))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_location(l: &EpwLocation) -> String {
    format!("[{},{},{},{},{},{},{},{},{}]", enc_str(&l.city), enc_str(&l.state_province), enc_str(&l.country), enc_str(&l.source), enc_str(&l.wmo), enc_str(&l.latitude), enc_str(&l.longitude), enc_str(&l.time_zone), enc_str(&l.elevation),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_location(s: &str) -> Result<EpwLocation, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [city, state_province, country, source, wmo, latitude, longitude, time_zone, elevation] = parts.as_slice() else {
        return Err(format!("location: expected 9 fields, got {}", parts.len()));
    };
    Ok(EpwLocation {
        city: dec_str(city)?,
        state_province: dec_str(state_province)?,
        country: dec_str(country)?,
        source: dec_str(source)?,
        wmo: dec_str(wmo)?,
        latitude: dec_str(latitude)?,
        longitude: dec_str(longitude)?,
        time_zone: dec_str(time_zone)?,
        elevation: dec_str(elevation)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_data_periods(d: &EpwDataPeriods) -> String {
    let periods = d.periods.iter().map(|p| format!("[{},{},{},{}]", enc_str(&p.name), enc_str(&p.start_day_of_week), enc_str(&p.start_date), enc_str(&p.end_date))).collect::<Vec<_>>().join(",");
    format!("[{},[{}]]", d.records_per_hour, periods)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_data_periods(s: &str) -> Result<EpwDataPeriods, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [records_per_hour, periods] = parts.as_slice() else {
        return Err(format!("data_periods: expected 2 fields, got {}", parts.len()));
    };
    let periods = split_top_level(strip_brackets(periods)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|p| {
            let fields = split_top_level(strip_brackets(p)?, ',');
            let [name, start_day_of_week, start_date, end_date] = fields.as_slice() else {
                return Err(format!("data_period: expected 4 fields, got {}", fields.len()));
            };
            Ok(crate::standards::energyplus::subsets::any::schema::snapshot::EpwDataPeriod { name: dec_str(name)?, start_day_of_week: dec_str(start_day_of_week)?, start_date: dec_str(start_date)?, end_date: dec_str(end_date)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(EpwDataPeriods { records_per_hour: parse_usize(records_per_hour)? as u32, periods })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_record_diff(d: &EpwRecordDiff) -> String {
    let mut parts = Vec::with_capacity(EPW_RECORD_FIELD_COUNT);
    for i in 0..EPW_RECORD_FIELD_COUNT {
        let slot = d.get_at(i).expect("index within range");
        parts.push(encode_option(slot, |v| enc_str(v)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_record_diff(s: &str) -> Result<EpwRecordDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    if parts.len() != EPW_RECORD_FIELD_COUNT {
        return Err(format!("record diff: expected {EPW_RECORD_FIELD_COUNT} fields, got {}", parts.len()));
    }
    let mut d = EpwRecordDiff::default();
    for (i, p) in parts.into_iter().enumerate() {
        let v = decode_option(p, dec_str)?;
        d.set_at(i, v);
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_records_diff(d: &EpwRecordsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_record_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_record(&a.record))).collect::<Vec<_>>().join(",");
    format!("records{{[{removed}];[{modified}];[{added}]}}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_records_diff(body: &str) -> Result<EpwRecordsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("records: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("records modified: bad entry {entry:?}"))?;
            Ok(EpwRecordModified { index: parse_usize(idx)?, diff: dec_record_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("records added: bad entry {entry:?}"))?;
            Ok(EpwRecordAdded { index: parse_usize(idx)?, record: dec_record(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(EpwRecordsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_epw_diff(d: &EpwDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.location {
        tokens.push(format!("location={}", enc_location(v)));
    }
    if let Some(v) = &d.design_conditions {
        tokens.push(format!("design-conditions={}", enc_str(v)));
    }
    if let Some(v) = &d.typical_extreme_periods {
        tokens.push(format!("typical-extreme-periods={}", enc_str(v)));
    }
    if let Some(v) = &d.ground_temperatures {
        tokens.push(format!("ground-temperatures={}", enc_str(v)));
    }
    if let Some(v) = &d.holidays_dst {
        tokens.push(format!("holidays-dst={}", enc_str(v)));
    }
    if let Some(v) = &d.comments_1 {
        tokens.push(format!("comments-1={}", enc_str(v)));
    }
    if let Some(v) = &d.comments_2 {
        tokens.push(format!("comments-2={}", enc_str(v)));
    }
    if let Some(v) = &d.data_periods {
        tokens.push(format!("data-periods={}", enc_data_periods(v)));
    }
    if let Some(v) = &d.records {
        tokens.push(enc_records_diff(v));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_epw_diff(line: &str) -> Result<EpwDiff, String> {
    let mut d = EpwDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("location=") {
            d.location = Some(dec_location(rest)?);
        } else if let Some(rest) = token.strip_prefix("design-conditions=") {
            d.design_conditions = Some(dec_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("typical-extreme-periods=") {
            d.typical_extreme_periods = Some(dec_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("ground-temperatures=") {
            d.ground_temperatures = Some(dec_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("holidays-dst=") {
            d.holidays_dst = Some(dec_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("comments-1=") {
            d.comments_1 = Some(dec_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("comments-2=") {
            d.comments_2 = Some(dec_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("data-periods=") {
            d.data_periods = Some(dec_data_periods(rest)?);
        } else if let Some(rest) = token.strip_prefix("records{") {
            d.records = Some(dec_records_diff(rest.strip_suffix('}').ok_or_else(|| "records: missing closing brace".to_string())?)?);
        } else {
            return Err(format!("epw diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for EpwDiff {
fn print_diff(&self) -> String {
    print_epw_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_epw_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
