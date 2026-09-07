//! 🍃️ Energy model mutation — `ChangeSurfaceWindExposed`: Sets whether the outside face uses the wind-driven exterior convection correlation or the sheltered one.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🍃️ `change-surface-wind-exposed` payload. Sets whether the outside face uses the wind-driven exterior convection correlation or the sheltered one.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-wind-exposed")]
pub struct ChangeSurfaceWindExposed {
    pub id: crate::model::EntityId,
    pub new_wind_exposed: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_surface_wind_exposed(id: crate::model::EntityId, new_wind_exposed: bool) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSurfaceWindExposed(ChangeSurfaceWindExposed { id, new_wind_exposed })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSurfaceWindExposed {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "surface", kind: "change-surface-wind-exposed", record: "ChangedSurfaceWindExposed" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change surface {} wind exposure to {}", self.id.0, self.new_wind_exposed)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
