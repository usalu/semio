//! ➕️ `create-node` — brings a new `DrawNode` into existence as a child of the `Group` addressed
//! by `parent`, at a FINAL-state index within that group's `children`. `DrawNode` carries no
//! stable id of its own (recursive, anonymous scene-graph collection), so the address is
//! `NodePath` — the same structural substitute the sibling `🔺️diff` facet's own `diff_at_path`/
//! `node_at` already establish for every node-addressed mutation in this facet.

use crate::schema::diff::NodePath;
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateNode {
    pub parent: NodePath,
    pub index: usize,
    pub node: DrawNode,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for CreateNode
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<CreateNode> + From<crate::schema::mutations::delete_node::DeleteNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create node in layer #{} at #{}", self.parent.layer, self.index), &format!("Knoten in Ebene #{} an #{} erstellen", self.parent.layer, self.index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.parent.layer.to_string(), self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioDrawingSnapshot, Op> for CreateNode
where
    Op: protocol::ColdMutation<SemioDrawingSnapshot, ReferenceDiff = crate::schema::diff::SemioDrawingDiff> + From<CreateNode> + From<crate::schema::mutations::delete_node::DeleteNode>,
{
    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioDrawingSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
