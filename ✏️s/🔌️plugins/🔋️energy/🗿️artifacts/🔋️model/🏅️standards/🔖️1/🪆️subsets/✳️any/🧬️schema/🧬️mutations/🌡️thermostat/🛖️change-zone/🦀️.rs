//! 🛖️ Energy model mutation — `ChangeThermostatZone`: Moves one thermostat to another zone. A foreign key, not a nesting field — zones do not nest, so this is a plain reassignment under the general axis and not a `move-to-<container>`.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛖️ `change-thermostat-zone` payload. Moves one thermostat to another zone. A foreign key, not a nesting field — zones do not nest, so this is a plain reassignment under the general axis and not a `move-to-<container>`.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-thermostat-zone")]
pub struct ChangeThermostatZone {
    pub id: crate::model::EntityId,
    pub new_zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_thermostat_zone<Op: From<ChangeThermostatZone>>(id: crate::model::EntityId, new_zone_id: crate::model::EntityId) -> Op {
    ChangeThermostatZone { id, new_zone_id }.into()
}
//#endregion 🔖️Mutation
