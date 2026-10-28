//! 🏦️ Energy model mutation — `CreateElectricalLoadCenter`: Adds one electrical load centre — the node that sums on-site generation and storage against the building's electrical demand. `pvIds` and `batteryIds` are checked against the document; `generatorIds` is carried verbatim and NOT checked, because `Model` has no generator collection for it to reference (vocabulary §5.4), which is also why this group ships no `add-electrical-load-center-generator`.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏦️ `create-electrical-load-center` payload. Adds one electrical load centre — the node that sums on-site generation and storage against the building's electrical demand. `pvIds` and `batteryIds` are checked against the document; `generatorIds` is carried verbatim and NOT checked, because `Model` has no generator collection for it to reference (vocabulary §5.4), which is also why this group ships no `add-electrical-load-center-generator`.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-electrical-load-center")]
pub struct CreateElectricalLoadCenter {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub name: String,
    pub generator_ids: Vec<crate::model::EntityId>,
    pub pv_ids: Vec<crate::model::EntityId>,
    pub battery_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_electrical_load_center<Op: From<CreateElectricalLoadCenter>>(index: u32, id: crate::model::EntityId, name: String, generator_ids: Vec<crate::model::EntityId>, pv_ids: Vec<crate::model::EntityId>, battery_ids: Vec<crate::model::EntityId>) -> Op {
    CreateElectricalLoadCenter { index, id, name, generator_ids, pv_ids, battery_ids }.into()
}
//#endregion 🔖️Mutation
