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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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


//#endregion 🔖️Apply


//#endregion 🔖️MutationTrait


//#region OpCodecs











//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests



#[cfg(test)]
use protocol::{OpBinary,OpText};
