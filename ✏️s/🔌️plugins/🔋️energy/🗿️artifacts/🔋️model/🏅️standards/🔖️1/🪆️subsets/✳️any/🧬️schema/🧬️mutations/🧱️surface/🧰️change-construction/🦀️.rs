//! 🧰️ Energy model mutation — `ChangeSurfaceConstruction`: Points the surface at another existing layered construction — the layer stack the conduction transfer functions are derived from.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧰️ `change-surface-construction` payload. Points the surface at another existing layered construction — the layer stack the conduction transfer functions are derived from.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-construction")]
pub struct ChangeSurfaceConstruction {
    pub id: crate::model::EntityId,
    pub new_construction_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_surface_construction<Op: From<ChangeSurfaceConstruction>>(id: crate::model::EntityId, new_construction_id: crate::model::EntityId) -> Op {
    ChangeSurfaceConstruction { id, new_construction_id }.into()
}
//#endregion 🔖️Mutation
