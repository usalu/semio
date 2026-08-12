//! 💫️ `ungroup` — dissolves the `Group` node addressed by `at`, splicing its children back into
//! its parent's `children` at its own position, in their existing relative order.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UngroupNode {
    pub at: NodePath,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for UngroupNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "ungroup", entity: "node", kind: "ungroup", record: "UngroupedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> <SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Ungroup node in layer #{}", self.at.layer)
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
