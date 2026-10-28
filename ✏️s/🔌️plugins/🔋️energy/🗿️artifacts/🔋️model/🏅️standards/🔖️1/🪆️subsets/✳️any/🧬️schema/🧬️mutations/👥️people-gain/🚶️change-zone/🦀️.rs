//! 🚶️ Energy model mutation — `ChangePeopleGainZone`: Re-points one people gain's zone reference; a zone the document does not define is refused.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚶️ `change-people-gain-zone` payload. Re-points one people gain's zone reference; a zone the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-people-gain-zone")]
pub struct ChangePeopleGainZone {
    pub id: crate::model::EntityId,
    pub new_zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_people_gain_zone<Op: From<ChangePeopleGainZone>>(id: crate::model::EntityId, new_zone_id: crate::model::EntityId) -> Op {
    ChangePeopleGainZone { id, new_zone_id }.into()
}
//#endregion 🔖️Mutation
