//! 🔀️ `reorder-nodes` — repositions one child within the `Group` addressed by `parent` (never
//! spatial -- `DrawNode` carries no position of its own inside `children`, only sequence order).

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderNodes {
    pub parent: NodePath,
    pub from: usize,
    pub to: usize,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for ReorderNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "nodes", kind: "reorder-nodes", record: "ReorderedNodes" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Reorder node #{} to #{}", self.from, self.to)
    }
    fn target(&self) -> Vec<String> {
        vec![self.parent.layer.to_string(), self.from.to_string()]
    }
}
//#endregion 🔖️Payload
