//! 🧮️ Energy model mutation — `ChangeSpaceFloorArea`: Sets one space's floor area in square metres — the denominator every per-area gain in that space is expanded against.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧮️ `change-space-floor-area` payload. Sets one space's floor area in square metres — the denominator every per-area gain in that space is expanded against.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-space-floor-area")]
pub struct ChangeSpaceFloorArea {
    pub id: crate::model::EntityId,
    pub new_floor_area_m2: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_space_floor_area<Op: From<ChangeSpaceFloorArea>>(id: crate::model::EntityId, new_floor_area_m2: f64) -> Op {
    ChangeSpaceFloorArea { id, new_floor_area_m2 }.into()
}
//#endregion 🔖️Mutation
