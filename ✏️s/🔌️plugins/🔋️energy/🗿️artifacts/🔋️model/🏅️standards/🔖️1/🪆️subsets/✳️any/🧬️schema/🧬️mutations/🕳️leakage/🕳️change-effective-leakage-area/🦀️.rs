//! 🕳️ Energy model mutation — `ChangeInfiltrationEffectiveLeakageArea`: Sets effective leakage area (m²) on one infiltration, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕳️ `change-infiltration-effective-leakage-area` payload. Sets effective leakage area (m²) on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-effective-leakage-area")]
pub struct ChangeInfiltrationEffectiveLeakageArea {
    pub id: crate::model::EntityId,
    pub new_effective_leakage_area_m2: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_effective_leakage_area<Op: From<ChangeInfiltrationEffectiveLeakageArea>>(id: crate::model::EntityId, new_effective_leakage_area_m2: f64) -> Op {
    ChangeInfiltrationEffectiveLeakageArea { id, new_effective_leakage_area_m2 }.into()
}
//#endregion 🔖️Mutation
