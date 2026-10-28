//! 🫗️ Energy model mutation — `ClearFenestrationGlazingConstruction`: Empties the fenestration's optional glazing slot, handing the optics back to `uValueWM2k`/`shgc`/`vlt`. Refused when the slot is already empty, so an undo chain can never invent a clear that had no partner.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🫗️ `clear-fenestration-glazing-construction` payload. Empties the fenestration's optional glazing slot, handing the optics back to `uValueWM2k`/`shgc`/`vlt`. Refused when the slot is already empty, so an undo chain can never invent a clear that had no partner.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "clear-fenestration-glazing-construction")]
pub struct ClearFenestrationGlazingConstruction {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn clear_fenestration_glazing_construction<Op: From<ClearFenestrationGlazingConstruction>>(id: crate::model::EntityId) -> Op {
    ClearFenestrationGlazingConstruction { id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
