//! 🧼️ Energy model mutation — `DeleteWaterSystem`: Removes one cold-water use system. Nothing in the document references one, so this never refuses for use.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧼️ `delete-water-system` payload. Removes one cold-water use system. Nothing in the document references one, so this never refuses for use.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-water-system")]
pub struct DeleteWaterSystem {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_water_system<Op: From<DeleteWaterSystem>>(id: crate::model::EntityId) -> Op {
    DeleteWaterSystem { id }.into()
}

//#endregion 🔖️Mutation
