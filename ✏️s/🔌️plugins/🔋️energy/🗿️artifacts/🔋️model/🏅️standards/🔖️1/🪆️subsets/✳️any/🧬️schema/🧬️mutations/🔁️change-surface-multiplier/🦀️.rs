//! 🔁️ Energy model mutation — `ChangeSurfaceMultiplier`: Sets how many identical instances of the surface the heat balance is scaled by.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔁️ `change-surface-multiplier` payload. Sets how many identical instances of the surface the heat balance is scaled by.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-multiplier")]
pub struct ChangeSurfaceMultiplier {
    pub id: crate::model::EntityId,
    pub new_multiplier: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_surface_multiplier(id: crate::model::EntityId, new_multiplier: u32) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSurfaceMultiplier(ChangeSurfaceMultiplier { id, new_multiplier })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSurfaceMultiplier {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "surface", kind: "change-surface-multiplier", record: "ChangedSurfaceMultiplier" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change surface {} multiplier to {}", self.id.0, self.new_multiplier)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
