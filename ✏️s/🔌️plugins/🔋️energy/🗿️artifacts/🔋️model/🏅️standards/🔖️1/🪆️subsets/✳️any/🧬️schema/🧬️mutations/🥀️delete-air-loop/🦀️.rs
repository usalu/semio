//! 🥀️ Energy model mutation — `DeleteAirLoop`: Drops one air loop. Refused while an outdoor air system still names it, rather than cascading — the outdoor air system is a document of its own and its owner decides its fate.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥀️ `delete-air-loop` payload. Drops one air loop. Refused while an outdoor air system still names it, rather than cascading — the outdoor air system is a document of its own and its owner decides its fate.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-air-loop")]
pub struct DeleteAirLoop {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_air_loop(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteAirLoop(DeleteAirLoop { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteAirLoop {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "air-loop", kind: "delete-air-loop", record: "DeletedAirLoop" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete air loop {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
