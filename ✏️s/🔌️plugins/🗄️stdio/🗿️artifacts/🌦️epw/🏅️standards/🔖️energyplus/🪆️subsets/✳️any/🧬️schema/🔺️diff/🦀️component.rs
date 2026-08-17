//! 🔺️ EpwDiff — handcrafted sparse structural diff. `records` is an index-keyed
//! removed/modified/added triple (EPW rows have no stable identity beyond position, same as
//! csv's own records); `location`/`data_periods` are rarely-mutated sub-documents so they use a
//! whole-substruct replace-in-place `Option<T>` slot (the same pattern csv's own top-level
//! `has_header: Option<bool>` uses, one level up — NOT the banned `snapshot: Option<EpwSnapshot>`
//! full-replace escape hatch, which never appears anywhere in this file); each modified record's
//! own 35 columns get a genuinely sparse per-field patch via [`EpwRecordDiff`].

use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot, EPW_RECORD_FIELD_COUNT};
use protocol::command::DiffAlgebra;
use protocol::DiffCodec;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️RecordDiff
/// 🔺️ Sparse per-field diff over [`EpwRecord`]'s 35 columns. Every field is independently
/// patchable (`field_sweep` exercises all 35 changing at once); `set_at`/`get_at` give the
/// numeric-index access `🧬️mutations::EpwMutation::SetRecordField` needs.
macro_rules! epw_record_diff {
    ($($field:ident => $index:expr),+ $(,)?) => {
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct EpwRecordDiff {
            $(
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub $field: Option<String>,
            )+
        }

        impl EpwRecordDiff {
            /// 🕳️ Whether this patch changes nothing.
            pub fn is_empty(&self) -> bool {
                $( self.$field.is_none() && )+ true
            }
            /// ▶️ Applies this patch to a record.
            pub fn apply(&self, base: &EpwRecord) -> EpwRecord {
                EpwRecord {
                    $( $field: self.$field.clone().unwrap_or_else(|| base.$field.clone()), )+
                }
            }
            /// 🧭️ State delta between two records (every differing column becomes `Some`).
            pub fn between(base: &EpwRecord, other: &EpwRecord) -> Self {
                Self {
                    $( $field: (base.$field != other.$field).then(|| other.$field.clone()), )+
                }
            }
            /// ➕️ LWW per-field absorb: `other`'s populated columns win.
            fn absorb(&mut self, other: Self) {
                $( if other.$field.is_some() { self.$field = other.$field; } )+
            }
            /// 📥️ Sets exactly one column by its canonical wire index (see [`EpwRecord::field_at`]).
            pub fn set_at(&mut self, index: usize, value: Option<String>) {
                match index {
                    $( $index => self.$field = value, )+
                    _ => {}
                }
            }
            /// 📤️ Reads one column's patch value by its canonical wire index.
            pub fn get_at(&self, index: usize) -> Option<&Option<String>> {
                match index {
                    $( $index => Some(&self.$field), )+
                    _ => None,
                }
            }
        }
    };
}

epw_record_diff! {
    year => 0, month => 1, day => 2, hour => 3, minute => 4, data_source_uncertainty => 5,
    dry_bulb_temp => 6, dew_point_temp => 7, relative_humidity => 8, atmospheric_pressure => 9,
    extraterrestrial_horizontal_radiation => 10, extraterrestrial_direct_normal_radiation => 11,
    horizontal_infrared_radiation => 12, global_horizontal_radiation => 13, direct_normal_radiation => 14,
    diffuse_horizontal_radiation => 15, global_horizontal_illuminance => 16, direct_normal_illuminance => 17,
    diffuse_horizontal_illuminance => 18, zenith_luminance => 19, wind_direction => 20, wind_speed => 21,
    total_sky_cover => 22, opaque_sky_cover => 23, visibility => 24, ceiling_height => 25,
    present_weather_observation => 26, present_weather_codes => 27, precipitable_water => 28,
    aerosol_optical_depth => 29, snow_depth => 30, days_since_last_snowfall => 31, albedo => 32,
    liquid_precip_depth => 33, liquid_precip_quantity => 34,
}

const _: () = assert!(EPW_RECORD_FIELD_COUNT == 35, "EpwRecordDiff field-index table must match EPW_RECORD_FIELD_COUNT");
//#endregion 🔖️RecordDiff

//#region 🔖️RecordsDiff
/// 🧩 One record patched-in-place at a BASE index.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpwRecordModified {
    pub index: usize,
    pub diff: EpwRecordDiff,
}

/// 🧩 One record inserted at a FINAL index.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpwRecordAdded {
    pub index: usize,
    pub record: EpwRecord,
}

/// 🔺️ Index-keyed removed/modified/added triple over `EpwSnapshot::records`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpwRecordsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<EpwRecordModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<EpwRecordAdded>,
}

impl EpwRecordsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️RecordsDiff

//#region 🔖️IndexTransport
// 🧮 Base-free index transport for absorb — identical in shape to csv's own
// `simulate_slots`/`base_len_hint`/`absorb_records` (see that file's doc comments for the
// full rationale); renamed here to avoid symbol collisions across artifacts.

/// 🎰 One slot of a simulated post-removal/insertion array.
#[derive(Clone, Copy, Debug)]
enum Slot {
    Base(usize),
    Added(usize),
}

fn simulate_slots(len: usize, removed: &[usize], added_indices: &[usize]) -> Vec<Slot> {
    let mut slots: Vec<Slot> = (0..len).map(Slot::Base).collect();
    let mut removed_desc = removed.to_vec();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    removed_desc.dedup();
    for r in removed_desc {
        if r < slots.len() {
            slots.remove(r);
        }
    }
    let mut order: Vec<usize> = (0..added_indices.len()).collect();
    order.sort_by_key(|&i| added_indices[i]);
    for i in order {
        let at = added_indices[i].min(slots.len());
        slots.insert(at, Slot::Added(i));
    }
    slots
}

fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map(|m| m + 1).unwrap_or(0)
}
//#endregion 🔖️IndexTransport

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.epw`. No `snapshot: Option<EpwSnapshot>` full-replace slot — even
/// `SetSnapshot`'s diff is `EpwDiff::between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.epw.diff")]
pub struct EpwDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<EpwLocation>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub design_conditions: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub typical_extreme_periods: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ground_temperatures: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holidays_dst: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments_1: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments_2: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_periods: Option<EpwDataPeriods>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub records: Option<EpwRecordsDiff>,
}

impl MutationDiff<EpwSnapshot> for EpwDiff {
    fn apply(&self, base: &EpwSnapshot) -> MutationApplyResult<EpwSnapshot> {
        validate_epw_diff(self, base)?;
        Ok(apply_epw_diff_unchecked(self, base))
    }

    fn absorb(&mut self, other: Self) {
        if other.location.is_some() {
            self.location = other.location;
        }
        if other.design_conditions.is_some() {
            self.design_conditions = other.design_conditions;
        }
        if other.typical_extreme_periods.is_some() {
            self.typical_extreme_periods = other.typical_extreme_periods;
        }
        if other.ground_temperatures.is_some() {
            self.ground_temperatures = other.ground_temperatures;
        }
        if other.holidays_dst.is_some() {
            self.holidays_dst = other.holidays_dst;
        }
        if other.comments_1.is_some() {
            self.comments_1 = other.comments_1;
        }
        if other.comments_2.is_some() {
            self.comments_2 = other.comments_2;
        }
        if other.data_periods.is_some() {
            self.data_periods = other.data_periods;
        }
        let d2 = match other.records {
            None => return,
            Some(d2) => d2,
        };
        let d1 = match self.records.take() {
            None => {
                self.records = Some(d2);
                return;
            }
            Some(d1) => d1,
        };
        self.records = Some(absorb_records(d1, d2));
    }
}

fn validate_epw_diff(diff: &EpwDiff, base: &EpwSnapshot) -> MutationApplyResult<()> {
    let Some(records) = &diff.records else { return Ok(()) };
    let mut removed = std::collections::HashSet::new();
    for &index in &records.removed {
        if index >= base.records.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "record removal target does not exist"));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "record removal target is repeated"));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &records.modified {
        if entry.index >= base.records.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "record modification target does not exist"));
        }
        if removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "record modification targets a removed item"));
        }
        if !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "record modification target is repeated"));
        }
    }
    let final_len = base.records.len() - removed.len() + records.added.len();
    let mut added = std::collections::HashSet::new();
    for entry in &records.added {
        if entry.index > final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "record addition is outside the final collection"));
        }
        if !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "record addition occupies a repeated final position"));
        }
    }
    Ok(())
}

fn apply_epw_diff_unchecked(diff: &EpwDiff, base: &EpwSnapshot) -> EpwSnapshot {
    let mut next = base.clone();
    if let Some(v) = &diff.location {
        next.location = v.clone();
    }
    if let Some(v) = &diff.design_conditions {
        next.design_conditions = v.clone();
    }
    if let Some(v) = &diff.typical_extreme_periods {
        next.typical_extreme_periods = v.clone();
    }
    if let Some(v) = &diff.ground_temperatures {
        next.ground_temperatures = v.clone();
    }
    if let Some(v) = &diff.holidays_dst {
        next.holidays_dst = v.clone();
    }
    if let Some(v) = &diff.comments_1 {
        next.comments_1 = v.clone();
    }
    if let Some(v) = &diff.comments_2 {
        next.comments_2 = v.clone();
    }
    if let Some(v) = &diff.data_periods {
        next.data_periods = v.clone();
    }
    if let Some(rdiff) = &diff.records {
        // 🥇 modified refers to BASE indices — apply before any removal shifts them.
        for m in &rdiff.modified {
            if let Some(rec) = next.records.get_mut(m.index) {
                *rec = m.diff.apply(rec);
            }
        }
        // 🥈 removed refers to BASE indices — process descending.
        let mut removed_desc = rdiff.removed.clone();
        removed_desc.sort_unstable_by(|a, b| b.cmp(a));
        removed_desc.dedup();
        for idx in removed_desc {
            if idx < next.records.len() {
                next.records.remove(idx);
            }
        }
        // 🥉 added refers to FINAL indices — process ascending, clamped.
        let mut added_asc = rdiff.added.clone();
        added_asc.sort_by_key(|a| a.index);
        for a in added_asc {
            let at = a.index.min(next.records.len());
            next.records.insert(at, a.record);
        }
    }
    next
}

/// ➕️ Structural, total, base-free absorb of two `records` triples (same algorithm as csv's).
fn absorb_records(d1: EpwRecordsDiff, d2: EpwRecordsDiff) -> EpwRecordsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = {
        let mut r = d1.removed.clone();
        r.sort_unstable();
        r.dedup();
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0);
    let base_len = base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied()).max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = simulate_slots(base_len, &d1.removed, &d1_added_indices);

    let mut final_removed: Vec<usize> = d1.removed.clone();
    let mut modified_map: BTreeMap<usize, EpwRecordDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<EpwRecordAdded>> = d1.added.into_iter().map(Some).collect();

    for mid_idx in &d2.removed {
        match mid_slots.get(*mid_idx) {
            Some(Slot::Base(b)) => {
                final_removed.push(*b);
                modified_map.remove(b);
            }
            Some(Slot::Added(ai)) => {
                added_alive[*ai] = None;
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid_slots.get(m2.index) {
            Some(Slot::Base(b)) => {
                modified_map.entry(*b).or_default().absorb(m2.diff.clone());
            }
            Some(Slot::Added(ai)) => {
                if let Some(added) = added_alive[*ai].as_mut() {
                    added.record = m2.diff.apply(&added.record);
                }
            }
            None => {}
        }
    }

    final_removed.sort_unstable();
    final_removed.dedup();
    for r in &final_removed {
        modified_map.remove(r);
    }
    let mut final_modified: Vec<EpwRecordModified> = modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| EpwRecordModified { index, diff }).collect();
    final_modified.sort_by_key(|m| m.index);

    let alive_mid_positions: Vec<usize> = mid_slots
        .iter()
        .enumerate()
        .filter_map(|(pos, slot)| match slot {
            Slot::Added(ai) if added_alive[*ai].is_some() => Some(pos),
            _ => None,
        })
        .collect();
    let d2_added_indices: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map(|m| m + 1).unwrap_or(0);
    let after_slots = simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let Slot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }

    let mut final_added: Vec<EpwRecordAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push(EpwRecordAdded { index: *after_pos, record: added.record });
            }
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);

    EpwRecordsDiff { removed: final_removed, modified: final_modified, added: final_added }
}

impl DiffAlgebra<EpwSnapshot> for EpwDiff {
    fn inverse(&self, base: &EpwSnapshot) -> Self {
        let applied = apply_epw_diff_unchecked(self, base);
        Self::between(&applied, base)
    }

    fn between(base: &EpwSnapshot, other: &EpwSnapshot) -> Self {
        let location = (base.location != other.location).then(|| other.location.clone());
        let design_conditions = (base.design_conditions != other.design_conditions).then(|| other.design_conditions.clone());
        let typical_extreme_periods = (base.typical_extreme_periods != other.typical_extreme_periods).then(|| other.typical_extreme_periods.clone());
        let ground_temperatures = (base.ground_temperatures != other.ground_temperatures).then(|| other.ground_temperatures.clone());
        let holidays_dst = (base.holidays_dst != other.holidays_dst).then(|| other.holidays_dst.clone());
        let comments_1 = (base.comments_1 != other.comments_1).then(|| other.comments_1.clone());
        let comments_2 = (base.comments_2 != other.comments_2).then(|| other.comments_2.clone());
        let data_periods = (base.data_periods != other.data_periods).then(|| other.data_periods.clone());

        let mut removed = Vec::new();
        let mut modified = Vec::new();
        let mut added = Vec::new();
        let min_len = base.records.len().min(other.records.len());
        for i in 0..min_len {
            let b = &base.records[i];
            let o = &other.records[i];
            if b == o {
                continue;
            }
            let d = EpwRecordDiff::between(b, o);
            if !d.is_empty() {
                modified.push(EpwRecordModified { index: i, diff: d });
            }
        }
        for i in min_len..base.records.len() {
            removed.push(i);
        }
        for i in min_len..other.records.len() {
            added.push(EpwRecordAdded { index: i, record: other.records[i].clone() });
        }

        let records = if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(EpwRecordsDiff { removed, modified, added }) };
        Self { location, design_conditions, typical_extreme_periods, ground_temperatures, holidays_dst, comments_1, comments_2, data_periods, records }
    }

    fn is_empty(&self) -> bool {
        self.location.is_none()
            && self.design_conditions.is_none()
            && self.typical_extreme_periods.is_none()
            && self.ground_temperatures.is_none()
            && self.holidays_dst.is_none()
            && self.comments_1.is_none()
            && self.comments_2.is_none()
            && self.data_periods.is_none()
            && self.records.as_ref().map_or(true, EpwRecordsDiff::is_empty)
    }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
pub fn diff_set_snapshot(base: &EpwSnapshot, next: &EpwSnapshot) -> EpwDiff {
    EpwDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` (dsl-derive rejects `Vec<Option<T>>`-shaped diffs —
/// see csv's own `CsvRecordDiff` doc comment for the confirmed root cause; `EpwRecordDiff` has no
/// such field, but its 35-field breadth makes a derive-based approach equally impractical here).
/// **Grammar**: one space-separated `name=value` token per changed top-level field; `records`
/// prints as `records{[removed];[modified];[added]}`. Strings are lowercase hex (EPW column
/// values may contain `,`/`?`/spaces, which this grammar's own separators are built from — hex
/// sidesteps escaping entirely, same convention as csv's/gif89a's hand-rolled diff codecs).
//#region 🔖️Primitives
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only).
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
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) fn enc_record(r: &EpwRecord) -> String {
    format!("[{}]", r.fields().iter().map(|f| enc_str(f)).collect::<Vec<_>>().join(","))
}
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
pub(crate) fn enc_location(l: &EpwLocation) -> String {
    format!("[{},{},{},{},{},{},{},{},{}]", enc_str(&l.city), enc_str(&l.state_province), enc_str(&l.country), enc_str(&l.source), enc_str(&l.wmo), enc_str(&l.latitude), enc_str(&l.longitude), enc_str(&l.time_zone), enc_str(&l.elevation),)
}
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
pub(crate) fn enc_data_periods(d: &EpwDataPeriods) -> String {
    let periods = d.periods.iter().map(|p| format!("[{},{},{},{}]", enc_str(&p.name), enc_str(&p.start_day_of_week), enc_str(&p.start_date), enc_str(&p.end_date))).collect::<Vec<_>>().join(",");
    format!("[{},[{}]]", d.records_per_hour, periods)
}
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
            Ok(crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwDataPeriod { name: dec_str(name)?, start_day_of_week: dec_str(start_day_of_week)?, start_date: dec_str(start_date)?, end_date: dec_str(end_date)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(EpwDataPeriods { records_per_hour: parse_usize(records_per_hour)? as u32, periods })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
fn enc_record_diff(d: &EpwRecordDiff) -> String {
    let mut parts = Vec::with_capacity(EPW_RECORD_FIELD_COUNT);
    for i in 0..EPW_RECORD_FIELD_COUNT {
        let slot = d.get_at(i).expect("index within range");
        parts.push(encode_option(slot, |v| enc_str(v)));
    }
    format!("[{}]", parts.join(","))
}
fn dec_record_diff(s: &str) -> Result<EpwRecordDiff, String> {
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

fn enc_records_diff(d: &EpwRecordsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_record_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_record(&a.record))).collect::<Vec<_>>().join(",");
    format!("records{{[{removed}];[{modified}];[{added}]}}")
}
fn dec_records_diff(body: &str) -> Result<EpwRecordsDiff, String> {
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
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
fn print_epw_diff(d: &EpwDiff) -> String {
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
fn parse_epw_diff(line: &str) -> Result<EpwDiff, String> {
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

impl DiffCodec for EpwDiff {
    fn print_diff(&self) -> String {
        print_epw_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_epw_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim, same simplification csv's/gif89a's hand-rolled
    /// `DiffCodec`s use.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;

    fn location(city: &str) -> EpwLocation {
        EpwLocation { city: city.into(), state_province: "NI".into(), country: "DEU".into(), source: "SRC".into(), wmo: "10238".into(), latitude: "52.37".into(), longitude: "9.74".into(), time_zone: "1.0".into(), elevation: "55.0".into() }
    }
    fn record(seed: &str) -> EpwRecord {
        let mut r = EpwRecord::default();
        r.year = "2026".into();
        r.month = "1".into();
        r.day = "15".into();
        r.hour = seed.into();
        r.dry_bulb_temp = format!("-{seed}.0");
        r
    }
    fn snapshot(city: &str) -> EpwSnapshot {
        EpwSnapshot { location: location(city), records: vec![record("1"), record("2"), record("3")], ..EpwSnapshot::default() }
    }

    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = snapshot("Hannover");
        let mut b = snapshot("Berlin");
        b.records[1] = record("2-modified, tricky [value]");
        let cases = vec![EpwDiff::default(), EpwDiff::between(&a, &b), EpwDiff::between(&b, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = EpwDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = EpwDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
