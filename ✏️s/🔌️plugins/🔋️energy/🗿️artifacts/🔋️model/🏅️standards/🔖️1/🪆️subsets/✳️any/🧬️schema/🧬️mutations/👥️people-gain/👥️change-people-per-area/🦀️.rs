//! 👥️ Energy model mutation — `ChangePeopleGainPeoplePerArea`: Sets occupant density (people/m²) on one people gain, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 👥️ `change-people-gain-people-per-area` payload. Sets occupant density (people/m²) on one people gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-people-gain-people-per-area")]
pub struct ChangePeopleGainPeoplePerArea {
    pub id: crate::model::EntityId,
    pub new_people_per_area: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_people_gain_people_per_area<Op: From<ChangePeopleGainPeoplePerArea>>(id: crate::model::EntityId, new_people_per_area: f64) -> Op {
    ChangePeopleGainPeoplePerArea { id, new_people_per_area }.into()
}
//#endregion 🔖️Mutation
