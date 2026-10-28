//! 🚷️ Energy model mutation — `DeletePeopleGain`: Removes one occupancy gain. Nothing references a gain, so this never has to refuse for use.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚷️ `delete-people-gain` payload. Removes one occupancy gain. Nothing references a gain, so this never has to refuse for use.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-people-gain")]
pub struct DeletePeopleGain {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_people_gain<Op: From<DeletePeopleGain>>(id: crate::model::EntityId) -> Op {
    DeletePeopleGain { id }.into()
}

//#endregion 🔖️Mutation
