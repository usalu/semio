//! 🏋️ Energy model mutation — `ChangeRefrigerationSystemDesignLoad`: Sets design load (W) on one refrigeration system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏋️ `change-refrigeration-system-design-load` payload. Sets design load (W) on one refrigeration system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-refrigeration-system-design-load")]
pub struct ChangeRefrigerationSystemDesignLoad {
    pub id: crate::model::EntityId,
    pub new_design_load_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_refrigeration_system_design_load<Op: From<ChangeRefrigerationSystemDesignLoad>>(id: crate::model::EntityId, new_design_load_w: f64) -> Op {
    ChangeRefrigerationSystemDesignLoad { id, new_design_load_w }.into()
}
//#endregion 🔖️Mutation
