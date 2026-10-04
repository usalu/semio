//! 🏷️ `rename-node` — a node's identity key changes and every edge endpoint naming it follows (taxonomy's `rename` verb:
//! the id; the display text is `change-node-label`). Edges are id-keyed entities whose `source`/`target` are data, so the
//! cascade rewrites those fields and never deletes or re-creates an edge.

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Payload
/// 🏷️ `rename-node` payload — the node and the id it takes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameNode {
    pub id: GraphNodeId,
    pub new_id: GraphNodeId,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for RenameNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "node", kind: "rename-node", record: "RenamedNode" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename node \"{}\" to \"{}\"", self.id.value, self.new_id.value), &format!("Knoten \"{}\" in \"{}\" umbenennen", self.id.value, self.new_id.value))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}
//#endregion 🔖️Payload
