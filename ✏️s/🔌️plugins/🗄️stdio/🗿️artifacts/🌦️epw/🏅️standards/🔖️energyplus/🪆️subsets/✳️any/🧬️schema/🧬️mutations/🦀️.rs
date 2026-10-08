//! 🧬️ EpwMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `EpwDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index/field-aware, reading the pre-state it needs from `base`.

use crate::standards::energyplus::subsets::any::schema::diff::{EpwDiff, EpwRecordAdded, EpwRecordDiff, EpwRecordModified, EpwRecordsDiff};










use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot};

use protocol::{Mutation};

//#region 🔖️Mutations
#[path = "📥insert-record/🦀️.rs"]
pub mod insert_record;
#[path = "📤remove-record/🦀️.rs"]
pub mod remove_record;
#[path = "💬set-comments1/🦀️.rs"]
pub mod set_comments1;
#[path = "🗨️set-comments2/🦀️.rs"]
pub mod set_comments2;
#[path = "📅set-data-periods/🦀️.rs"]
pub mod set_data_periods;
#[path = "🌡️set-design-conditions/🦀️.rs"]
pub mod set_design_conditions;
#[path = "🌍set-ground-temperatures/🦀️.rs"]
pub mod set_ground_temperatures;
#[path = "🎉set-holidays-dst/🦀️.rs"]
pub mod set_holidays_dst;
#[path = "📍set-location/🦀️.rs"]
pub mod set_location;
#[path = "🎚️set-record-field/🦀️.rs"]
pub mod set_record_field;
/// 📐️ Typed content mutation for `stdio.epw`.
/// 🧪️ F6: hand-rolled — `#[derive(dsl::DslOps)]` is not attempted here (the enum embeds
/// `EpwSnapshot`/`EpwLocation`/`EpwDataPeriods`, none of which implement `dsl::DslField`; wiring
/// that up is out of this ticket's scope, matching csv's/gif's own documented hand-roll rationale).
//#region 🔖️Leaves
#[path = "📆set-typical-extreme-periods/🦀️.rs"]
pub mod set_typical_extreme_periods;
//#endregion 🔖️Leaves

/// 🧭️ `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly
/// one leaf payload (a unit variant wraps none) and asserts `is_approved_verb(SEMANTICS.verb)`,
/// and `no` is not an approved verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = EpwSnapshot, diff = EpwDiff, schema = "EpwMutation")]
pub enum EpwMutation {
    /// 📍️ Replaces the LOCATION header line.
    SetLocation(set_location::SetLocation),
    /// 🌡️ Replaces the DESIGN CONDITIONS header line (retained verbatim).
    SetDesignConditions(set_design_conditions::SetDesignConditions),
    /// 📆️ Replaces the TYPICAL/EXTREME PERIODS header line (retained verbatim).
    SetTypicalExtremePeriods(set_typical_extreme_periods::SetTypicalExtremePeriods),
    /// 🌍️ Replaces the GROUND TEMPERATURES header line (retained verbatim).
    SetGroundTemperatures(set_ground_temperatures::SetGroundTemperatures),
    /// 🎉️ Replaces the HOLIDAYS/DAYLIGHT SAVINGS header line (retained verbatim).
    SetHolidaysDst(set_holidays_dst::SetHolidaysDst),
    /// 💬️ Replaces the COMMENTS 1 header line (retained verbatim).
    SetComments1(set_comments1::SetComments1),
    /// 🗨️ Replaces the COMMENTS 2 header line (retained verbatim).
    SetComments2(set_comments2::SetComments2),
    /// 📅️ Replaces the DATA PERIODS header line.
    SetDataPeriods(set_data_periods::SetDataPeriods),
    /// 📥️ Inserts a whole record at `index` (clamped to the end on apply).
    InsertRecord(insert_record::InsertRecord),
    /// 📤️ Removes the record at `index`.
    RemoveRecord(remove_record::RemoveRecord),
    /// 🎚️ Patches one of a record's 35 columns in place, addressed by its canonical wire index
    /// (see `EpwRecord::field_at`).
    SetRecordField(set_record_field::SetRecordField),
}

/// 🧾️ Kebab-case spelling of every `EpwMutation` variant, in declaration order — the exhaustive
/// mutation catalog `epw-energyplus-any` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &[
    "set-location",
    "set-design-conditions",
    "set-typical-extreme-periods",
    "set-ground-temperatures",
    "set-holidays-dst",
    "set-comments1",
    "set-comments2",
    "set-data-periods",
    "insert-record",
    "remove-record",
    "set-record-field",
];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_epw_mutation(snapshot: &mut EpwSnapshot, mutation: &EpwMutation) -> protocol::MutationOutcome<EpwDiff> {
    let outcome = <EpwMutation as Mutation<EpwSnapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply


//#endregion 🔖️MutationTrait

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to exactly `next`: each header line that moved, then every record row in place (one
/// `set-record-field` per differing column) and the diverging tail (surplus rows removed last first, missing rows inserted). The
/// snapshot `schema` is a constant of the artifact and never a leaf.
pub fn net_mutations(base: &EpwSnapshot, next: &EpwSnapshot) -> Vec<EpwMutation> {
    let mut leaves = Vec::new();
    if base.location != next.location {
        leaves.push(EpwMutation::SetLocation(set_location::SetLocation { location: next.location.clone() }));
    }
    if base.design_conditions != next.design_conditions {
        leaves.push(EpwMutation::SetDesignConditions(set_design_conditions::SetDesignConditions { value: next.design_conditions.clone() }));
    }
    if base.typical_extreme_periods != next.typical_extreme_periods {
        leaves.push(EpwMutation::SetTypicalExtremePeriods(set_typical_extreme_periods::SetTypicalExtremePeriods { value: next.typical_extreme_periods.clone() }));
    }
    if base.ground_temperatures != next.ground_temperatures {
        leaves.push(EpwMutation::SetGroundTemperatures(set_ground_temperatures::SetGroundTemperatures { value: next.ground_temperatures.clone() }));
    }
    if base.holidays_dst != next.holidays_dst {
        leaves.push(EpwMutation::SetHolidaysDst(set_holidays_dst::SetHolidaysDst { value: next.holidays_dst.clone() }));
    }
    if base.comments_1 != next.comments_1 {
        leaves.push(EpwMutation::SetComments1(set_comments1::SetComments1 { value: next.comments_1.clone() }));
    }
    if base.comments_2 != next.comments_2 {
        leaves.push(EpwMutation::SetComments2(set_comments2::SetComments2 { value: next.comments_2.clone() }));
    }
    if base.data_periods != next.data_periods {
        leaves.push(EpwMutation::SetDataPeriods(set_data_periods::SetDataPeriods { data_periods: next.data_periods.clone() }));
    }
    let paired = base.records.len().min(next.records.len());
    for (record_index, (before, after)) in base.records.iter().zip(&next.records).enumerate().filter(|(_, (before, after))| before != after) {
        for field_index in 0..crate::standards::energyplus::subsets::any::schema::snapshot::EPW_RECORD_FIELD_COUNT {
            if let (Some(old), Some(new)) = (before.field_at(field_index), after.field_at(field_index)) {
                if old != new {
                    leaves.push(EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index, field_index, value: new.to_string() }));
                }
            }
        }
    }
    leaves.extend((paired..base.records.len()).rev().map(|index| EpwMutation::RemoveRecord(remove_record::RemoveRecord { index })));
    leaves.extend(next.records.iter().enumerate().skip(paired).map(|(index, record)| EpwMutation::InsertRecord(insert_record::InsertRecord { index, record: Box::new(record.clone()) })));
    leaves
}
//#endregion 🔖️Net

//#region OpCodecs











//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests



#[cfg(test)]
use protocol::{OpBinary,OpText};
