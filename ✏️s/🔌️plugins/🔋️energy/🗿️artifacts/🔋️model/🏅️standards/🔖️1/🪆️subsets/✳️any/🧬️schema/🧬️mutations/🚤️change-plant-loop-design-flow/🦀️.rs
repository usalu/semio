//! 🚤️ Energy model mutation — `ChangePlantLoopDesignFlow`: Sets the mass flow the loop is designed around, in kg/s.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚤️ `change-plant-loop-design-flow` payload. Sets the mass flow the loop is designed around, in kg/s.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-plant-loop-design-flow")]
pub struct ChangePlantLoopDesignFlow {
    pub id: crate::model::EntityId,
    pub new_design_flow_kg_s: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_plant_loop_design_flow(id: crate::model::EntityId, new_design_flow_kg_s: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangePlantLoopDesignFlow(ChangePlantLoopDesignFlow { id, new_design_flow_kg_s })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangePlantLoopDesignFlow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "plant-loop", kind: "change-plant-loop-design-flow", record: "ChangedPlantLoopDesignFlow" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change plant loop {} design mass flow to {:?}", self.id.0, self.new_design_flow_kg_s)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
