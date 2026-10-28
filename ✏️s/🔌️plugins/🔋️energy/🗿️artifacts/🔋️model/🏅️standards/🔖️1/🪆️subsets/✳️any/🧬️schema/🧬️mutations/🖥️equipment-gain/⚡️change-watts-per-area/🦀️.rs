//! ⚡️ Energy model mutation — `ChangeEquipmentGainWattsPerArea`: Sets equipment power density (W/m²) on one equipment gain, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚡️ `change-equipment-gain-watts-per-area` payload. Sets equipment power density (W/m²) on one equipment gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-equipment-gain-watts-per-area")]
pub struct ChangeEquipmentGainWattsPerArea {
    pub id: crate::model::EntityId,
    pub new_watts_per_area: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_equipment_gain_watts_per_area<Op: From<ChangeEquipmentGainWattsPerArea>>(id: crate::model::EntityId, new_watts_per_area: f64) -> Op {
    ChangeEquipmentGainWattsPerArea { id, new_watts_per_area }.into()
}
//#endregion 🔖️Mutation
