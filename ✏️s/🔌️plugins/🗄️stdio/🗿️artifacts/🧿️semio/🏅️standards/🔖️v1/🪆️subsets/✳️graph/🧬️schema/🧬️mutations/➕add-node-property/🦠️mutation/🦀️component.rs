//! ➕ `add-node-property` — attaches one property entry to a node at a FINAL-state index within
//! that node's `properties` (an intrinsically ordered, anonymous collection nested one level
//! inside `nodes` — exactly mirrors `add-node-port` but operating on `properties` instead of
//! `ports`; REUSES `✳️value`'s `SemioValueEntry`).

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddNodeProperty {
    pub node_id: GraphNodeId,
    pub index: usize,
    pub property: SemioValueEntry,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for AddNodeProperty {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "node-property", kind: "add-node-property", record: "AddedNodeProperty" };

    fn diff(&self, base: &SemioGraphSnapshot) -> <SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add property to node \"{}\" at #{}", self.node_id.value, self.index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.value.clone(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
