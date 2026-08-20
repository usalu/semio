//! 🧷️ `group` — introduces a new `Group` node wrapping a CONTIGUOUS run of siblings (`indices`,
//! ascending, substituting for the taxonomy's "member ids" -- `DrawNode` has no stable id, the
//! same structural substitute `NodePath` already establishes throughout this facet) under the
//! `Group` addressed by `parent`. Restricted to contiguous runs so `ungroup` can restore the
//! EXACT original membership/positions losslessly (a non-contiguous grouping would interleave
//! with untouched siblings in a way `ungroup` could not reconstruct from `base` alone).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupNodes {
    pub parent: NodePath,
    pub indices: Vec<usize>,
    pub transform: SemioTransform,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for GroupNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "group", entity: "nodes", kind: "group", record: "GroupedNodes" };

    async fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Group {} node(s) in layer #{}", self.indices.len(), self.parent.layer)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.parent.layer.to_string()]
    }
}
//#endregion 🔖️Payload
