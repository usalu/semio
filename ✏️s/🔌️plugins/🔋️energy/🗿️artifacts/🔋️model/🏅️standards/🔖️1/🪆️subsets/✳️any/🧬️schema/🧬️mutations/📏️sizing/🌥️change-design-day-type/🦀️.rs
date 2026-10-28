//! 🌥️ Energy model mutation — `ChangeSizingObjectDesignDayType`: Swaps which design day the sizing run reads.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌥️ `change-sizing-object-design-day-type` payload. Swaps which design day the sizing run reads.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-sizing-object-design-day-type")]
pub struct ChangeSizingObjectDesignDayType {
    pub id: crate::model::EntityId,
    pub new_design_day_type: crate::model::DesignDayType,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_sizing_object_design_day_type<Op: From<ChangeSizingObjectDesignDayType>>(id: crate::model::EntityId, new_design_day_type: crate::model::DesignDayType) -> Op {
    ChangeSizingObjectDesignDayType { id, new_design_day_type }.into()
}
//#endregion 🔖️Mutation
