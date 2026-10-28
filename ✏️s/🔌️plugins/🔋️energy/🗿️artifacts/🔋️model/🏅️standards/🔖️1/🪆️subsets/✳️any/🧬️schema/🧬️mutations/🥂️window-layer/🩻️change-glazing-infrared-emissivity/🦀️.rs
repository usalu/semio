//! 🩻️ Energy model mutation — `ChangeGlazingMaterialInfraredEmissivity`: Sets both long-wave emissivities of one glazing pane in one step — front and back are one optical facet of the same sheet, and a coating that changes one almost always changes the other.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🩻️ `change-glazing-material-infrared-emissivity` payload. Sets both long-wave emissivities of one glazing pane in one step — front and back are one optical facet of the same sheet, and a coating that changes one almost always changes the other.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-glazing-material-infrared-emissivity")]
pub struct ChangeGlazingMaterialInfraredEmissivity {
    pub id: crate::model::EntityId,
    pub new_infrared_emissivity_front: f64,
    pub new_infrared_emissivity_back: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_glazing_material_infrared_emissivity<Op: From<ChangeGlazingMaterialInfraredEmissivity>>(id: crate::model::EntityId, new_infrared_emissivity_front: f64, new_infrared_emissivity_back: f64) -> Op {
    ChangeGlazingMaterialInfraredEmissivity { id, new_infrared_emissivity_front, new_infrared_emissivity_back }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
