//! 🔺️ EpwDiff — handcrafted sparse structural diff. `records` is an index-keyed
//! removed/modified/added triple (EPW rows have no stable identity beyond position, same as
//! csv's own records); `location`/`data_periods` are rarely-mutated sub-documents so they use a
//! whole-substruct replace-in-place `Option<T>` slot (the same pattern csv's own top-level
//! `has_header: Option<bool>` uses, one level up — NOT the banned `snapshot: Option<EpwSnapshot>`
//! full-replace escape hatch, which never appears anywhere in this file); each modified record's
//! own 35 columns get a genuinely sparse per-field patch via [`EpwRecordDiff`].

use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot, EPW_RECORD_FIELD_COUNT};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️RecordDiff
/// 🔺️ Sparse per-field diff over [`EpwRecord`]'s 35 columns. Every field is independently
/// patchable (`field_sweep` exercises all 35 changing at once); `set_at`/`get_at` give the
/// numeric-index access `🧬️mutations::EpwMutation::SetRecordField` needs.
macro_rules! epw_record_diff {
    ($($field:ident => $index:expr),+ $(,)?) => {
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        pub struct EpwRecordDiff {
            $(
                #[value(default, skip_serializing_if = "Option::is_none")]
                pub $field: Option<String>,
            )+
        }

        impl EpwRecordDiff {
            /// 🕳️ Whether this patch changes nothing.
            // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
            pub fn is_empty(&self) -> bool {
                $( self.$field.is_none() && )+ true
            }
            /// ▶️ Applies this patch to a record.
            // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
            pub fn apply(&self, base: &EpwRecord) -> EpwRecord {
                EpwRecord {
                    $( $field: self.$field.clone().unwrap_or_else(|| base.$field.clone()), )+
                }
            }
            /// 🧭️ State delta between two records (every differing column becomes `Some`).
            // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
            pub fn between(base: &EpwRecord, other: &EpwRecord) -> Self {
                Self {
                    $( $field: (base.$field != other.$field).then(|| other.$field.clone()), )+
                }
            }
            /// ➕️ LWW per-field absorb: `other`'s populated columns win.
            // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
            fn absorb(&mut self, other: Self) {
                $( if other.$field.is_some() { self.$field = other.$field; } )+
            }
            /// 📥️ Sets exactly one column by its canonical wire index (see [`EpwRecord::field_at`]).
            // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
            pub fn set_at(&mut self, index: usize, value: Option<String>) {
                match index {
                    $( $index => self.$field = value, )+
                    _ => {}
                }
            }
            /// 📤️ Reads one column's patch value by its canonical wire index.
            // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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


//#endregion 🔖️RecordDiff

//#region 🔖️RecordsDiff
/// 🧩 One record patched-in-place at a BASE index.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct EpwRecordModified {
    pub index: usize,
    pub diff: EpwRecordDiff,
}

/// 🧩 One record inserted at a FINAL index.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct EpwRecordAdded {
    pub index: usize,
    pub record: EpwRecord,
}

/// 🔺️ Index-keyed removed/modified/added triple over `EpwSnapshot::records`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct EpwRecordsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<EpwRecordModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<EpwRecordAdded>,
}

impl EpwRecordsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map_or(0, |m| m + 1)
}
//#endregion 🔖️IndexTransport

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.epw`. No `snapshot: Option<EpwSnapshot>` full-replace slot — even
/// `SetSnapshot`'s diff is `EpwDiff::between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.epw.diff")]
pub struct EpwDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<EpwLocation>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub design_conditions: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub typical_extreme_periods: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ground_temperatures: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub holidays_dst: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comments_1: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comments_2: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data_periods: Option<EpwDataPeriods>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_records(d1: EpwRecordsDiff, d2: EpwRecordsDiff) -> EpwRecordsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = {
        let mut r = d1.removed.clone();
        r.sort_unstable();
        r.dedup();
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map_or(0, |m| m + 1);
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
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map_or(0, |m| m + 1);
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
            && self.records.as_ref().is_none_or(EpwRecordsDiff::is_empty)
    }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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








//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs








//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs





//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests

const _: () = assert!(EPW_RECORD_FIELD_COUNT == 35, "EpwRecordDiff field-index table must match EPW_RECORD_FIELD_COUNT");
