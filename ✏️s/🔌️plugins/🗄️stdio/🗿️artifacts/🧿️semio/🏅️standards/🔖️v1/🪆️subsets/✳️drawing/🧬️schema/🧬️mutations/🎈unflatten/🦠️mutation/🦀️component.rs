//! 🎈️️ `unflatten` — restores the node addressed by `at` wholesale to `original` (a captured
//! hierarchy, per `📓️taxonomy.md`'s `flatten`/`unflatten` row: "addr + captured hierarchy").

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnflattenNode {
    pub at: NodePath,
    pub original: DrawNode,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for UnflattenNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unflatten", entity: "node", kind: "unflatten", record: "UnflattenedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> <SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Unflatten node in layer #{}", self.at.layer)
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
