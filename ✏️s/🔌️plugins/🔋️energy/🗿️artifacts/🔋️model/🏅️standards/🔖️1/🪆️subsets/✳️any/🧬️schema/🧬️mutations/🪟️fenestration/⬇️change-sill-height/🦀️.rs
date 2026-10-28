//! ⬇️ Energy model mutation — `ChangeFenestrationSillHeight`: Sets the height of the glazing's sill above the host surface's base in metres.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⬇️ `change-fenestration-sill-height` payload. Sets the height of the glazing's sill above the host surface's base in metres.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-sill-height")]
pub struct ChangeFenestrationSillHeight {
    pub id: crate::model::EntityId,
    pub new_sill_height_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_sill_height<Op: From<ChangeFenestrationSillHeight>>(id: crate::model::EntityId, new_sill_height_m: f64) -> Op {
    ChangeFenestrationSillHeight { id, new_sill_height_m }.into()
}
//#endregion 🔖️Mutation
