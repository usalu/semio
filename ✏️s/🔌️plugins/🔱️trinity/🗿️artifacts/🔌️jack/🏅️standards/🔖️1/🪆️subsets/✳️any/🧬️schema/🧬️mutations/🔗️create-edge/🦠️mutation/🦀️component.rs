//! 🔗️ TrinityGraph mutation — `CreateEdge`: brings a new id-keyed edge into existence.
use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use crate::artifacts::jack::{Edge, JackSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔗️ `create-edge` payload — full initial edge payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEdge {
    pub edge: Edge,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_edge(edge: Edge) -> TrinityGraphMutation {
    TrinityGraphMutation::CreateEdge(CreateEdge { edge })
}

impl protocol::MutationKind<JackSnapshot, TrinityGraphMutation> for CreateEdge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "edge", kind: "create-edge", record: "CreatedEdge" };

    fn diff(&self, base: &JackSnapshot) -> JackDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create edge \"{}\"", self.edge.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.edge.id.clone()]
    }
}
//#endregion 🔖️Mutation
