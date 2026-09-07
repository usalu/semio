//! 🧽️ Energy model mutation — `DeleteInfiltration`: Removes one zone infiltration specification, leaving the zone airtight.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧽️ `delete-infiltration` payload. Removes one zone infiltration specification, leaving the zone airtight.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-infiltration")]
pub struct DeleteInfiltration {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_infiltration(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteInfiltration(DeleteInfiltration { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteInfiltration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "infiltration", kind: "delete-infiltration", record: "DeletedInfiltration" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete Infiltration {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
