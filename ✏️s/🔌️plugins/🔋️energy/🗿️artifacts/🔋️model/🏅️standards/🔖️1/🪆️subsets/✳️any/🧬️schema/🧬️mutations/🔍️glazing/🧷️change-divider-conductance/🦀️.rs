//! 🧷️ Energy model mutation — `ChangeFenestrationDividerConductance`: Sets the divider bars' whole-assembly thermal conductance in W/K, the frame term's sibling for the muntins inside the glazed area.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧷️ `change-fenestration-divider-conductance` payload. Sets the divider bars' whole-assembly thermal conductance in W/K, the frame term's sibling for the muntins inside the glazed area.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-divider-conductance")]
pub struct ChangeFenestrationDividerConductance {
    pub id: crate::model::EntityId,
    pub new_divider_conductance_w_k: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_divider_conductance<Op: From<ChangeFenestrationDividerConductance>>(id: crate::model::EntityId, new_divider_conductance_w_k: f64) -> Op {
    ChangeFenestrationDividerConductance { id, new_divider_conductance_w_k }.into()
}
//#endregion 🔖️Mutation
