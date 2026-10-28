//! 📉️ Energy model mutation — `RemoveOutputVariable`: Detaches one reporting registration addressed by its `(name, key)` natural composite key.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📉️ `remove-output-variable` payload. Detaches one reporting registration addressed by its `(name, key)` natural composite key.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-output-variable")]
pub struct RemoveOutputVariable {
    pub name: String,
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_output_variable<Op: From<RemoveOutputVariable>>(name: String, key: String) -> Op {
    RemoveOutputVariable { name, key }.into()
}

//#endregion 🔖️Mutation
