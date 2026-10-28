//! 🗜️ Energy model mutation — `ChangeSurfaceZone`: Reassigns one surface to another existing zone — the zone whose air heat balance the surface's inside face exchanges with.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗜️ `change-surface-zone` payload. Reassigns one surface to another existing zone — the zone whose air heat balance the surface's inside face exchanges with.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-zone")]
pub struct ChangeSurfaceZone {
    pub id: crate::model::EntityId,
    pub new_zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_surface_zone<Op: From<ChangeSurfaceZone>>(id: crate::model::EntityId, new_zone_id: crate::model::EntityId) -> Op {
    ChangeSurfaceZone { id, new_zone_id }.into()
}
//#endregion 🔖️Mutation
