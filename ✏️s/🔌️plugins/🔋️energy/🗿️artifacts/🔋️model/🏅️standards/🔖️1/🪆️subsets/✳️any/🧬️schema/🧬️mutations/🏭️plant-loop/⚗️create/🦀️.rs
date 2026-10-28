//! ⚗️ Energy model mutation — `CreatePlantLoop`: Creates one plant loop. ⚠️ `equipment_ids` is the ONE reference this group cannot check: `Model` carries no chiller/boiler/pump collection at all (vocabulary §5.4), so the ids are opaque here — the list is only held to being ascending, free of duplicates and free of the unset id zero. When a plant-equipment collection lands, add a `g3_chk_reference` over it; the hole is recorded in this ticket's `📓️w7-g3-hvac.md` rather than papered over.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚗️ `create-plant-loop` payload. Creates one plant loop. ⚠️ `equipment_ids` is the ONE reference this group cannot check: `Model` carries no chiller/boiler/pump collection at all (vocabulary §5.4), so the ids are opaque here — the list is only held to being ascending, free of duplicates and free of the unset id zero. When a plant-equipment collection lands, add a `g3_chk_reference` over it; the hole is recorded in this ticket's `📓️w7-g3-hvac.md` rather than papered over.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-plant-loop")]
pub struct CreatePlantLoop {
    pub id: crate::model::EntityId,
    pub name: String,
    pub loop_type: crate::model::PlantLoopType,
    pub supply_temperature_c: f64,
    pub return_temperature_c: f64,
    pub design_flow_kg_s: f64,
    pub equipment_ids: Vec<crate::model::EntityId>,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_plant_loop<Op: From<CreatePlantLoop>>(
    id: crate::model::EntityId,
    name: String,
    loop_type: crate::model::PlantLoopType,
    supply_temperature_c: f64,
    return_temperature_c: f64,
    design_flow_kg_s: f64,
    equipment_ids: Vec<crate::model::EntityId>,
    index: Option<u32>,
) -> Op {
    CreatePlantLoop { id, name, loop_type, supply_temperature_c, return_temperature_c, design_flow_kg_s, equipment_ids, index }.into()
}
//#endregion 🔖️Mutation
