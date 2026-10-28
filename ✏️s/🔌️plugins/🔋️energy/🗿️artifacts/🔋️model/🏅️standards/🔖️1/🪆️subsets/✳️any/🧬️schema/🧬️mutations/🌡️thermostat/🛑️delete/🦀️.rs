//! 🛑️ Energy model mutation — `DeleteThermostat`: Drops one zone's setpoint control, leaving the zone free-floating. Refused when no thermostat carries the id, so an undo chain can never invent a deletion that had no partner.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛑️ `delete-thermostat` payload. Drops one zone's setpoint control, leaving the zone free-floating. Refused when no thermostat carries the id, so an undo chain can never invent a deletion that had no partner.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-thermostat")]
pub struct DeleteThermostat {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_thermostat<Op: From<DeleteThermostat>>(id: crate::model::EntityId) -> Op {
    DeleteThermostat { id }.into()
}

//#endregion 🔖️Mutation
