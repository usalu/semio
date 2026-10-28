//! 💦️ Energy model mutation — `ChangeEquipmentGainLatentFraction`: Sets latent fraction on one equipment gain, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💦️ `change-equipment-gain-latent-fraction` payload. Sets latent fraction on one equipment gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-equipment-gain-latent-fraction")]
pub struct ChangeEquipmentGainLatentFraction {
    pub id: crate::model::EntityId,
    pub new_latent_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_equipment_gain_latent_fraction<Op: From<ChangeEquipmentGainLatentFraction>>(id: crate::model::EntityId, new_latent_fraction: f64) -> Op {
    ChangeEquipmentGainLatentFraction { id, new_latent_fraction }.into()
}
//#endregion 🔖️Mutation
