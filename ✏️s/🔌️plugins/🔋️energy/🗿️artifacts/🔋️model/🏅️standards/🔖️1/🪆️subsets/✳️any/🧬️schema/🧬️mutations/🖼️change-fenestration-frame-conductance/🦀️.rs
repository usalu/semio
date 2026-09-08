//! 🖼️ Energy model mutation — `ChangeFenestrationFrameConductance`: Sets the frame's whole-assembly thermal conductance in W/K, added to the glazing term rather than distributed over the area.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🖼️ `change-fenestration-frame-conductance` payload. Sets the frame's whole-assembly thermal conductance in W/K, added to the glazing term rather than distributed over the area.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-frame-conductance")]
pub struct ChangeFenestrationFrameConductance {
    pub id: crate::model::EntityId,
    pub new_frame_conductance_w_k: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fenestration_frame_conductance(id: crate::model::EntityId, new_frame_conductance_w_k: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFenestrationFrameConductance(ChangeFenestrationFrameConductance { id, new_frame_conductance_w_k })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFenestrationFrameConductance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-frame-conductance", record: "ChangedFenestrationFrameConductance" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fenestration {} frame conductance to {} W/K", self.id.0, self.new_frame_conductance_w_k)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
