//! 🗑️ `delete-node` — removes an id-keyed node, cascading to every edge severed by its removal
//! (captures the full removed payload + severed cascade for its inverse, per taxonomy's `delete`
//! canonical args).

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteNode {
    pub id: GraphNodeId,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for DeleteNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

    fn diff(&self, base: &SemioGraphSnapshot) -> <SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete node \"{}\"", self.id.value)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}
//#endregion 🔖️Payload
