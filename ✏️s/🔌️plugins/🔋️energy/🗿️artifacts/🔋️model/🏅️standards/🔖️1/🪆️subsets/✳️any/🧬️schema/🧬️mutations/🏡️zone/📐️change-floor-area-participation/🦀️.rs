//! 📐️ Energy model mutation — `ChangeZoneFloorAreaParticipation`: Sets whether the zone's floor area counts toward the building total the normalized reports divide by.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📐️ `change-zone-floor-area-participation` payload. Sets whether the zone's floor area counts toward the building total the normalized reports divide by.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-floor-area-participation")]
pub struct ChangeZoneFloorAreaParticipation {
    pub id: crate::model::EntityId,
    pub new_part_of_total_floor_area: bool,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_zone_floor_area_participation<Op: From<ChangeZoneFloorAreaParticipation>>(id: crate::model::EntityId, new_part_of_total_floor_area: bool) -> Op {
    ChangeZoneFloorAreaParticipation { id, new_part_of_total_floor_area }.into()
}
//#endregion 🔖️Mutation
