//! 🔩️ Energy model mutation — `AddPlantLoopEquipment`: Puts one central-plant equipment id on a plant loop. ⚠️ This is the ONE reference in the whole HVAC group that is NOT checked against a collection: `Model` has no chiller, boiler or pump type at all (vocabulary §5.4), so the id is opaque and only the list shape is enforced. Declared and documented rather than silently validated against nothing; when a plant-equipment collection lands this gains a `g3_chk_reference` and the refusal vector below becomes a referential one.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔩️ `add-plant-loop-equipment` payload. Puts one central-plant equipment id on a plant loop. ⚠️ This is the ONE reference in the whole HVAC group that is NOT checked against a collection: `Model` has no chiller, boiler or pump type at all (vocabulary §5.4), so the id is opaque and only the list shape is enforced. Declared and documented rather than silently validated against nothing; when a plant-equipment collection lands this gains a `g3_chk_reference` and the refusal vector below becomes a referential one.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-plant-loop-equipment")]
pub struct AddPlantLoopEquipment {
    pub id: crate::model::EntityId,
    pub equipment_id: crate::model::EntityId,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn add_plant_loop_equipment<Op: From<AddPlantLoopEquipment>>(id: crate::model::EntityId, equipment_id: crate::model::EntityId, index: Option<u32>) -> Op {
    AddPlantLoopEquipment { id, equipment_id, index }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
