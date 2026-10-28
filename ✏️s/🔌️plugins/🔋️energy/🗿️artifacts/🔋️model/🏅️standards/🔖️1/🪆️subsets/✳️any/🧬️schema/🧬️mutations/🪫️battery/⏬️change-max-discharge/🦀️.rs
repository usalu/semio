//! ⏬️ Energy model mutation — `ChangeBatteryMaxDischarge`: Sets maximum discharge power (W) on one battery, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⏬️ `change-battery-max-discharge` payload. Sets maximum discharge power (W) on one battery, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-battery-max-discharge")]
pub struct ChangeBatteryMaxDischarge {
    pub id: crate::model::EntityId,
    pub new_max_discharge_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_battery_max_discharge<Op: From<ChangeBatteryMaxDischarge>>(id: crate::model::EntityId, new_max_discharge_w: f64) -> Op {
    ChangeBatteryMaxDischarge { id, new_max_discharge_w }.into()
}
//#endregion 🔖️Mutation
