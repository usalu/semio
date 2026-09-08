//! 🌡️ Energy model mutation — `UpdateGroundTemperature`: Sets the whole inseparable ground-temperature facet — twelve monthly building-surface values, twelve monthly shallow values and one deep value are read together by the ground heat transfer solve.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌡️ `update-ground-temperature` payload. Sets the whole inseparable ground-temperature facet — twelve monthly building-surface values, twelve monthly shallow values and one deep value are read together by the ground heat transfer solve.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "update-ground-temperature")]
pub struct UpdateGroundTemperature {
    pub building_surface_c: Vec<f64>,
    pub shallow_c: Vec<f64>,
    pub deep_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_ground_temperature(building_surface_c: Vec<f64>, shallow_c: Vec<f64>, deep_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::UpdateGroundTemperature(UpdateGroundTemperature { building_surface_c, shallow_c, deep_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for UpdateGroundTemperature {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "ground-temperature", kind: "update-ground-temperature", record: "UpdatedGroundTemperature" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Update ground temperatures".to_string()
    }
}
//#endregion 🔖️Mutation
