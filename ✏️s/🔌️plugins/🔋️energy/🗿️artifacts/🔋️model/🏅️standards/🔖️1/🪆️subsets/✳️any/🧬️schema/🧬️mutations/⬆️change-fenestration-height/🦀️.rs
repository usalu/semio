//! ⬆️ Energy model mutation — `ChangeFenestrationHeight`: Sets the glazing's head-to-sill height in metres — the length the overhang and fin shadow geometry is projected against.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⬆️ `change-fenestration-height` payload. Sets the glazing's head-to-sill height in metres — the length the overhang and fin shadow geometry is projected against.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-height")]
pub struct ChangeFenestrationHeight {
    pub id: crate::model::EntityId,
    pub new_height_m: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_height(id: crate::model::EntityId, new_height_m: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationHeight(ChangeFenestrationHeight { id, new_height_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationHeight {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-height", record: "ChangedFenestrationHeight" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} height to {} m", self.id.0, self.new_height_m)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
