//! 🌇️ Energy model mutation — `ChangeFenestrationShgc`: Sets the solar heat gain coefficient — the fraction of incident solar the glazing passes to the zone, directly and by re-radiation.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌇️ `change-fenestration-shgc` payload. Sets the solar heat gain coefficient — the fraction of incident solar the glazing passes to the zone, directly and by re-radiation.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-shgc")]
pub struct ChangeFenestrationShgc {
    pub id: crate::model::EntityId,
    pub new_shgc: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_shgc<Op: From<ChangeFenestrationShgc>>(id: crate::model::EntityId, new_shgc: f64) -> Op {
    ChangeFenestrationShgc { id, new_shgc }.into()
}
//#endregion 🔖️Mutation
