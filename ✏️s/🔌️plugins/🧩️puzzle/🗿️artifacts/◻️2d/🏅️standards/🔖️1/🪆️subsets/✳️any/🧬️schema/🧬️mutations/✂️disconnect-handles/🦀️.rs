//! ✂️ Puzzle2d mutation — `DisconnectHandles`: removes a directed link between two handles.

use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// ✂️ `disconnect-handles` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "disconnect-handles")]
pub struct DisconnectHandles {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_handles(id: String) -> Puzzle2dMutation {
    Puzzle2dMutation::DisconnectHandles(DisconnectHandles { id })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for DisconnectHandles {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "handles", kind: "disconnect-handles", record: "DisconnectedHandles" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Disconnect \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
