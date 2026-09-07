//! 🧢️ Energy model mutation — `ChangeFenestrationOverhangDepth`: Sets how far the horizontal projection above the window head reaches out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 610/910 are exactly this field.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧢️ `change-fenestration-overhang-depth` payload. Sets how far the horizontal projection above the window head reaches out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 610/910 are exactly this field.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-overhang-depth")]
pub struct ChangeFenestrationOverhangDepth {
    pub id: crate::model::EntityId,
    pub new_overhang_depth_m: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_overhang_depth(id: crate::model::EntityId, new_overhang_depth_m: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationOverhangDepth(ChangeFenestrationOverhangDepth { id, new_overhang_depth_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationOverhangDepth {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-overhang-depth", record: "ChangedFenestrationOverhangDepth" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} overhang depth to {} m", self.id.0, self.new_overhang_depth_m)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
