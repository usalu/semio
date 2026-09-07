//! 🖇️ Energy model mutation — `RenameSetpointManager`: Sets one setpoint manager's identity field.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🖇️ `rename-setpoint-manager` payload. Sets one setpoint manager's identity field.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-setpoint-manager")]
pub struct RenameSetpointManager {
    pub id: crate::model::EntityId,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_setpoint_manager(id: crate::model::EntityId, new_name: String) -> EnergyModelMutation {
    EnergyModelMutation::RenameSetpointManager(RenameSetpointManager { id, new_name })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RenameSetpointManager {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "setpoint-manager", kind: "rename-setpoint-manager", record: "RenamedSetpointManager" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change setpoint manager {} name to {:?}", self.id.0, self.new_name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
