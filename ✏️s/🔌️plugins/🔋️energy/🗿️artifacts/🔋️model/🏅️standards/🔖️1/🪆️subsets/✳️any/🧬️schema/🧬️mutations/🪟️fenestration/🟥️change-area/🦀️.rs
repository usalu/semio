//! 🟥️ Energy model mutation — `ChangeFenestrationArea`: Sets the glazed area in square metres — the area every window heat-balance and solar term is proportional to.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟥️ `change-fenestration-area` payload. Sets the glazed area in square metres — the area every window heat-balance and solar term is proportional to.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-area")]
pub struct ChangeFenestrationArea {
    pub id: crate::model::EntityId,
    pub new_area_m2: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_area<Op: From<ChangeFenestrationArea>>(id: crate::model::EntityId, new_area_m2: f64) -> Op {
    ChangeFenestrationArea { id, new_area_m2 }.into()
}
//#endregion 🔖️Mutation
