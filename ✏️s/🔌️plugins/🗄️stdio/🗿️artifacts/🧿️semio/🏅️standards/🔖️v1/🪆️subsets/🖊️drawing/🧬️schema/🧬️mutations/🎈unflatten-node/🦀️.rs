//! 🎈️️ `unflatten` — restores the node addressed by `at` wholesale to `original` (a captured
//! hierarchy, per `📓️taxonomy.md`'s `flatten`/`unflatten` row: "addr + captured hierarchy"). Its own
//! inverse captures the node it overwrites, so undoing it is exact for any node it replaces.

use crate::schema::diff::NodePath;
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct UnflattenNode {
    pub at: NodePath,
    pub original: DrawNode,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for UnflattenNode
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<UnflattenNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unflatten", entity: "node", kind: "unflatten-node", record: "UnflattenedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Unflatten node in layer #{}", self.at.layer), &format!("Reduzierung von Knoten in Ebene #{} aufheben", self.at.layer))
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
