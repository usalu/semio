//! 🌬️ Energy model mutation — `ChangeZoneConditioned`: Sets whether the zone is served by heating or cooling equipment at all.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌬️ `change-zone-conditioned` payload. Sets whether the zone is served by heating or cooling equipment at all.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-conditioned")]
pub struct ChangeZoneConditioned {
    pub id: crate::model::EntityId,
    pub new_conditioned: bool,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_zone_conditioned<Op: From<ChangeZoneConditioned>>(id: crate::model::EntityId, new_conditioned: bool) -> Op {
    ChangeZoneConditioned { id, new_conditioned }.into()
}
//#endregion 🔖️Mutation
