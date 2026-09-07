//! ⛓️ Energy model mutation — `ChangeOutdoorAirSystemAirLoop`: Moves the outdoor air system to another air loop.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛓️ `change-outdoor-air-system-air-loop` payload. Moves the outdoor air system to another air loop.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-outdoor-air-system-air-loop")]
pub struct ChangeOutdoorAirSystemAirLoop {
    pub id: crate::model::EntityId,
    pub new_air_loop_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_outdoor_air_system_air_loop(id: crate::model::EntityId, new_air_loop_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeOutdoorAirSystemAirLoop(ChangeOutdoorAirSystemAirLoop { id, new_air_loop_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeOutdoorAirSystemAirLoop {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "outdoor-air-system", kind: "change-outdoor-air-system-air-loop", record: "ChangedOutdoorAirSystemAirLoop" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change outdoor air system {} air loop to {}", self.id.0, self.new_air_loop_id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
