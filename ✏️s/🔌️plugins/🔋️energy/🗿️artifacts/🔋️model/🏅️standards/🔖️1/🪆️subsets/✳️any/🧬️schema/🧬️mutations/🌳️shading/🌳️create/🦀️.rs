//! 🌳️ Energy model mutation — `CreateShadingSurface`: Adds one free-standing solar obstruction — the site-level sibling of a window's own attached overhang and fins, which stay on the fenestration.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌳️ `create-shading-surface` payload. Adds one free-standing solar obstruction — the site-level sibling of a window's own attached overhang and fins, which stay on the fenestration.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-shading-surface")]
pub struct CreateShadingSurface {
    pub id: crate::model::EntityId,
    pub name: String,
    pub vertices_m: Vec<[f64; 3]>,
    pub transmittance_schedule_id: Option<crate::schedule::ScheduleId>,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_shading_surface<Op: From<CreateShadingSurface>>(id: crate::model::EntityId, name: String, vertices_m: Vec<[f64; 3]>, transmittance_schedule_id: Option<crate::schedule::ScheduleId>, index: Option<u32>) -> Op {
    CreateShadingSurface { id, name, vertices_m, transmittance_schedule_id, index }.into()
}
//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
