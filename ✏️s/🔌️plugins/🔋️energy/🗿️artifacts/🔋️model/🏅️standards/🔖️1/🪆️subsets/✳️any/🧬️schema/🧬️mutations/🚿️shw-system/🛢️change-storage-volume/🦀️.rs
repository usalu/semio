//! 🛢️ Energy model mutation — `ChangeShwSystemStorageVolume`: Sets storage volume (m³) on one service hot water system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛢️ `change-shw-system-storage-volume` payload. Sets storage volume (m³) on one service hot water system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-shw-system-storage-volume")]
pub struct ChangeShwSystemStorageVolume {
    pub id: crate::model::EntityId,
    pub new_storage_volume_m3: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_shw_system_storage_volume<Op: From<ChangeShwSystemStorageVolume>>(id: crate::model::EntityId, new_storage_volume_m3: f64) -> Op {
    ChangeShwSystemStorageVolume { id, new_storage_volume_m3 }.into()
}
//#endregion 🔖️Mutation
