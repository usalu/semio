//! 🧊️ Energy model mutation — `BindFenestrationGlazingConstruction`: Points the fenestration's optional glazing slot at an existing layered construction, which then supersedes `uValueWM2k`/`shgc`/`vlt`. This is the schema seam the ticket's oracle comparison needed: with only the three scalars a semio→EnergyPlus translation can emit nothing richer than `WindowMaterial:SimpleGlazingSystem`, worth +5.7 to +8.1 % of annual cooling on ANSI/ASHRAE 140 cases 600/900.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧊️ `bind-fenestration-glazing-construction` payload. Points the fenestration's optional glazing slot at an existing layered construction, which then supersedes `uValueWM2k`/`shgc`/`vlt`. This is the schema seam the ticket's oracle comparison needed: with only the three scalars a semio→EnergyPlus translation can emit nothing richer than `WindowMaterial:SimpleGlazingSystem`, worth +5.7 to +8.1 % of annual cooling on ANSI/ASHRAE 140 cases 600/900.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "bind-fenestration-glazing-construction")]
pub struct BindFenestrationGlazingConstruction {
    pub id: crate::model::EntityId,
    pub construction_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn bind_fenestration_glazing_construction<Op: From<BindFenestrationGlazingConstruction>>(id: crate::model::EntityId, construction_id: crate::model::EntityId) -> Op {
    BindFenestrationGlazingConstruction { id, construction_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
