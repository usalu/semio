//! 🚷️ Energy model mutation — `DeletePeopleGain`: Removes one occupancy gain. Nothing references a gain, so this never has to refuse for use.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚷️ `delete-people-gain` payload. Removes one occupancy gain. Nothing references a gain, so this never has to refuse for use.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-people-gain")]
pub struct DeletePeopleGain {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_people_gain(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeletePeopleGain(DeletePeopleGain { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeletePeopleGain {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "people-gain", kind: "delete-people-gain", record: "DeletedPeopleGain" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete People Gain {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
