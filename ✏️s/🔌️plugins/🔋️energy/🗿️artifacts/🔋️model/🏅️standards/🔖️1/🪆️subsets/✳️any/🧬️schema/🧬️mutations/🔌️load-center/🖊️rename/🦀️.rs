//! 🖊️ Energy model mutation — `RenameElectricalLoadCenter`: Sets one electrical load center's identity field; a name a sibling already holds is refused.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🖊️ `rename-electrical-load-center` payload. Sets one electrical load center's identity field; a name a sibling already holds is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-electrical-load-center")]
pub struct RenameElectricalLoadCenter {
    pub id: crate::model::EntityId,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn rename_electrical_load_center<Op: From<RenameElectricalLoadCenter>>(id: crate::model::EntityId, new_name: String) -> Op {
    RenameElectricalLoadCenter { id, new_name }.into()
}
//#endregion 🔖️Mutation
