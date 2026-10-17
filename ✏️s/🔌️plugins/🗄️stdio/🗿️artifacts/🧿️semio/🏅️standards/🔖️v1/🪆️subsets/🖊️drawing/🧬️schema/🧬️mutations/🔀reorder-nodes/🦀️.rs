//! 🔀️ `reorder-nodes` — repositions one child within the `Group` addressed by `parent` (never
//! spatial -- `DrawNode` carries no position of its own inside `children`, only sequence order).

use crate::schema::diff::NodePath;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReorderNodes {
    pub parent: NodePath,
    pub from: usize,
    pub to: usize,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for ReorderNodes
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<ReorderNodes>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "nodes", kind: "reorder-nodes", record: "ReorderedNodes" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reorder node #{} to #{}", self.from, self.to), &format!("Knoten #{} nach #{} verschieben", self.from, self.to))
    }
    fn target(&self) -> Vec<String> {
        vec![self.parent.layer.to_string(), self.from.to_string()]
    }
}
//#endregion 🔖️Payload
