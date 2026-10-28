//! ⏫️ Energy model mutation — `ChangeBatteryMaxCharge`: Sets maximum charge power (W) on one battery, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⏫️ `change-battery-max-charge` payload. Sets maximum charge power (W) on one battery, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-battery-max-charge")]
pub struct ChangeBatteryMaxCharge {
    pub id: crate::model::EntityId,
    pub new_max_charge_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_battery_max_charge<Op: From<ChangeBatteryMaxCharge>>(id: crate::model::EntityId, new_max_charge_w: f64) -> Op {
    ChangeBatteryMaxCharge { id, new_max_charge_w }.into()
}
//#endregion 🔖️Mutation
