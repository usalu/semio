//! 🔗️ `insert-edge` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use crate::schema::snapshot::FlowEdge;
#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioFlowDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_edge;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertEdge {
    pub edge: FlowEdge,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl InsertEdge {
    /// 🔗️ Transfers one typed edge into its insertion command without a wire round trip.
    pub fn new(edge: FlowEdge) -> Self {
        Self { edge, at: None }
    }
}

impl<Op> protocol::MutationKind<SemioFlowSnapshot, Op> for InsertEdge
where
    Op: protocol::Mutation<SemioFlowSnapshot, Diff = crate::schema::diff::SemioFlowDiff> + From<InsertEdge> + From<crate::schema::mutations::remove_edge::RemoveEdge>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "edge", kind: "insert-edge", record: "InsertEdge" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert edge", "Kante einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioFlowSnapshot, Op> for InsertEdge
where
    Op: protocol::ColdMutation<SemioFlowSnapshot, ReferenceDiff = crate::schema::diff::SemioFlowDiff> + From<InsertEdge> + From<crate::schema::mutations::remove_edge::RemoveEdge>,
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
