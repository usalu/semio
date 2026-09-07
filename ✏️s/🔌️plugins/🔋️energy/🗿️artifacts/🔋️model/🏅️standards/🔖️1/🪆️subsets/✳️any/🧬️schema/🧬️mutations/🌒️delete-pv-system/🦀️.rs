//! 🌒️ Energy model mutation — `DeletePvSystem`: Removes one photovoltaic array. Refused while a load centre still lists it, so the electrical bus never sums over an array the document no longer defines.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌒️ `delete-pv-system` payload. Removes one photovoltaic array. Refused while a load centre still lists it, so the electrical bus never sums over an array the document no longer defines.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-pv-system")]
pub struct DeletePvSystem {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_pv_system(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeletePvSystem(DeletePvSystem { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeletePvSystem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "pv-system", kind: "delete-pv-system", record: "DeletedPvSystem" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete Pv System {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
