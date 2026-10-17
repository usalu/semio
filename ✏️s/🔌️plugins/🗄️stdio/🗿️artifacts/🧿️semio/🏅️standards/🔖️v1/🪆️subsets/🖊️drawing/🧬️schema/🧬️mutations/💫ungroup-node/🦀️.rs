//! 💫️ `ungroup` — dissolves the `Group` node addressed by `at`, splicing its children back into
//! its parent's `children` at its own position, in their existing relative order.

use crate::schema::diff::NodePath;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct UngroupNode {
    pub at: NodePath,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for UngroupNode
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<UngroupNode> + From<crate::schema::mutations::group_nodes::GroupNodes>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "ungroup", entity: "node", kind: "ungroup-node", record: "UngroupedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Ungroup node in layer #{}", self.at.layer), &format!("Gruppierung von Knoten in Ebene #{} aufheben", self.at.layer))
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
