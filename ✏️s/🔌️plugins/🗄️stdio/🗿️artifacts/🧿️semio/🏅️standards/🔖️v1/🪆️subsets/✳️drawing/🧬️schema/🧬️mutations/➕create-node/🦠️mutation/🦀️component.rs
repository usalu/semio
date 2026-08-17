//! ➕️ `create-node` — brings a new `DrawNode` into existence as a child of the `Group` addressed
//! by `parent`, at a FINAL-state index within that group's `children`. `DrawNode` carries no
//! stable id of its own (recursive, anonymous scene-graph collection), so the address is
//! `NodePath` — the same structural substitute the sibling `🔺️diff` facet's own `diff_at_path`/
//! `node_at` already establish for every node-addressed mutation in this facet.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNode {
    pub parent: NodePath,
    pub index: usize,
    pub node: DrawNode,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create node in layer #{} at #{}", self.parent.layer, self.index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.parent.layer.to_string(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
