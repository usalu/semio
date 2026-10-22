//! 🔴️ `remove-node` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioFlowDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_node;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveNode {
    pub id: String,
}

impl<Op> protocol::MutationKind<SemioFlowSnapshot, Op> for RemoveNode
where
    Op: protocol::Mutation<SemioFlowSnapshot, Diff = crate::schema::diff::SemioFlowDiff> + From<RemoveNode> + From<crate::schema::mutations::insert_node::InsertNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "node", kind: "remove-node", record: "RemoveNode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove node", "Knoten entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioFlowSnapshot, Op> for RemoveNode
where
    Op: protocol::ColdMutation<SemioFlowSnapshot, ReferenceDiff = crate::schema::diff::SemioFlowDiff> + From<RemoveNode> + From<crate::schema::mutations::insert_node::InsertNode>,
{
    fn diff(&self, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioFlowSnapshot>>::ReferenceDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
mod diff;
#[path = "↩️inverse/🦀️.rs"]
mod inverse;
//#endregion 🪢️TaxonomyMounts
