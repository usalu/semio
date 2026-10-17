//! ➖️ `delete-node` — removes the `DrawNode` addressed BY `at` (whose last path segment is its
//! own index within its parent's `children`) from the recursive scene graph, capturing it for
//! `inverse`. `at.path` must be non-empty — a layer's own root cannot be deleted this way (that is
//! `delete-layer`'s job).

use crate::schema::diff::NodePath;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteNode {
    pub at: NodePath,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for DeleteNode
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<DeleteNode> + From<crate::schema::mutations::create_node::CreateNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete node in layer #{}", self.at.layer), &format!("Knoten in Ebene #{} löschen", self.at.layer))
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload

//#region 🔖️ParentSplit
/// ✂️️ Splits `at` into (parent address, own index within the parent's `children`) -- `None` for
/// the layer root (empty `path`), which has no parent to remove a child from.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parent_and_index(at: &NodePath) -> Option<(NodePath, usize)> {
    let (index, parent_path) = at.path.split_last()?;
    Some((NodePath { layer: at.layer, path: parent_path.to_vec() }, *index))
}
//#endregion 🔖️ParentSplit
