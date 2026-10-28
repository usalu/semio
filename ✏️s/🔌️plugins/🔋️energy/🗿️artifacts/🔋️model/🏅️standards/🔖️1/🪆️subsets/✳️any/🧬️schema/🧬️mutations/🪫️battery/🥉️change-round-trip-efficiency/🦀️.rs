//! 🥉️ Energy model mutation — `ChangeBatteryRoundTripEfficiency`: Sets round-trip efficiency on one battery, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥉️ `change-battery-round-trip-efficiency` payload. Sets round-trip efficiency on one battery, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-battery-round-trip-efficiency")]
pub struct ChangeBatteryRoundTripEfficiency {
    pub id: crate::model::EntityId,
    pub new_round_trip_efficiency: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_battery_round_trip_efficiency<Op: From<ChangeBatteryRoundTripEfficiency>>(id: crate::model::EntityId, new_round_trip_efficiency: f64) -> Op {
    ChangeBatteryRoundTripEfficiency { id, new_round_trip_efficiency }.into()
}
//#endregion 🔖️Mutation
