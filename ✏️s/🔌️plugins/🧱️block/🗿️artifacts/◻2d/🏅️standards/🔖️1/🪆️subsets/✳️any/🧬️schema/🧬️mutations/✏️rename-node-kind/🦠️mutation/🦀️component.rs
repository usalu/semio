//! ✏️ Block2d mutation — `RenameNodeKind`: the node kind's identity `name` field.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✏️ `rename-node-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-node-kind")]
pub struct RenameNodeKind {
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_node_kind(new_name: String) -> Block2dMutation {
    Block2dMutation::RenameNodeKind(RenameNodeKind { new_name })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for RenameNodeKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "node-kind", kind: "rename-node-kind", record: "RenamedNodeKind" };

    fn diff(&self, base: &Block2dSnapshot) -> Block2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename node kind to \"{}\"", self.new_name)
    }
}
//#endregion 🔖️Mutation
