//! ⬇️ Energy model mutation — `ChangeFenestrationSillHeight`: Sets the height of the glazing's sill above the host surface's base in metres.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⬇️ `change-fenestration-sill-height` payload. Sets the height of the glazing's sill above the host surface's base in metres.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-sill-height")]
pub struct ChangeFenestrationSillHeight {
    pub id: crate::model::EntityId,
    pub new_sill_height_m: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_sill_height(id: crate::model::EntityId, new_sill_height_m: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationSillHeight(ChangeFenestrationSillHeight { id, new_sill_height_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationSillHeight {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-sill-height", record: "ChangedFenestrationSillHeight" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} sill height to {} m", self.id.0, self.new_sill_height_m)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
