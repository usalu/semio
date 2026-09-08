//! 🧷️ Energy model mutation — `ChangeFenestrationDividerConductance`: Sets the divider bars' whole-assembly thermal conductance in W/K, the frame term's sibling for the muntins inside the glazed area.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧷️ `change-fenestration-divider-conductance` payload. Sets the divider bars' whole-assembly thermal conductance in W/K, the frame term's sibling for the muntins inside the glazed area.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-divider-conductance")]
pub struct ChangeFenestrationDividerConductance {
    pub id: crate::model::EntityId,
    pub new_divider_conductance_w_k: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_divider_conductance(id: crate::model::EntityId, new_divider_conductance_w_k: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationDividerConductance(ChangeFenestrationDividerConductance { id, new_divider_conductance_w_k })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationDividerConductance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-divider-conductance", record: "ChangedFenestrationDividerConductance" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} divider conductance to {} W/K", self.id.0, self.new_divider_conductance_w_k)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
