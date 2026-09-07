//! 🐬️ Energy model mutation — `ChangeFenestrationFinDepth`: Sets how far the two vertical projections beside the window jambs reach out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 630/930 are exactly this field.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🐬️ `change-fenestration-fin-depth` payload. Sets how far the two vertical projections beside the window jambs reach out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 630/930 are exactly this field.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-fin-depth")]
pub struct ChangeFenestrationFinDepth {
    pub id: crate::model::EntityId,
    pub new_fin_depth_m: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_fin_depth(id: crate::model::EntityId, new_fin_depth_m: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationFinDepth(ChangeFenestrationFinDepth { id, new_fin_depth_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationFinDepth {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-fin-depth", record: "ChangedFenestrationFinDepth" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} fin depth to {} m", self.id.0, self.new_fin_depth_m)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
