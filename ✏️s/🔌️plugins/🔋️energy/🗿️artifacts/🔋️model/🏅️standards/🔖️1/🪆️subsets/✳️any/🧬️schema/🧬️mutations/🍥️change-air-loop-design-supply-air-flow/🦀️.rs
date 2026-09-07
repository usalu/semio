//! 🍥️ Energy model mutation — `ChangeAirLoopDesignSupplyAirFlow`: Sets the volumetric air flow the loop is designed around, in m³/s.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🍥️ `change-air-loop-design-supply-air-flow` payload. Sets the volumetric air flow the loop is designed around, in m³/s.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-air-loop-design-supply-air-flow")]
pub struct ChangeAirLoopDesignSupplyAirFlow {
    pub id: crate::model::EntityId,
    pub new_design_supply_air_flow_m3_s: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_air_loop_design_supply_air_flow(id: crate::model::EntityId, new_design_supply_air_flow_m3_s: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeAirLoopDesignSupplyAirFlow(ChangeAirLoopDesignSupplyAirFlow { id, new_design_supply_air_flow_m3_s })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeAirLoopDesignSupplyAirFlow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "air-loop", kind: "change-air-loop-design-supply-air-flow", record: "ChangedAirLoopDesignSupplyAirFlow" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change air loop {} design supply air flow to {:?}", self.id.0, self.new_design_supply_air_flow_m3_s)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
