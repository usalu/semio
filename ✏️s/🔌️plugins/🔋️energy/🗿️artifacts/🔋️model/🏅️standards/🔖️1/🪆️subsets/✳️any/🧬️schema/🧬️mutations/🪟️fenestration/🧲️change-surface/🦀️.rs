//! 🧲️ Energy model mutation — `ChangeFenestrationSurface`: Rehosts the fenestration on another existing surface — the surface whose orientation, tilt and zone the window then inherits.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧲️ `change-fenestration-surface` payload. Rehosts the fenestration on another existing surface — the surface whose orientation, tilt and zone the window then inherits.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-surface")]
pub struct ChangeFenestrationSurface {
    pub id: crate::model::EntityId,
    pub new_surface_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_surface<Op: From<ChangeFenestrationSurface>>(id: crate::model::EntityId, new_surface_id: crate::model::EntityId) -> Op {
    ChangeFenestrationSurface { id, new_surface_id }.into()
}
//#endregion 🔖️Mutation
