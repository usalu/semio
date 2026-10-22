//! 🏗️ `create-edge` — brings a new id-keyed edge into existence between two vertices (referential integrity across `start_vertex`/`end_vertex` is the subset validator's job, not this diff constructor's), carrying its own curve. A duplicate `id` already present in `base` is a no-op.

use crate::schema::snapshot::{BrepCurve, SemioBrepSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateEdge {
    pub id: String,
    pub start_vertex: String,
    pub end_vertex: String,
    pub curve: BrepCurve,
    pub tol: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioBrepSnapshot, Op> for CreateEdge
where
    Op: protocol::Mutation<SemioBrepSnapshot, Diff = crate::schema::diff::SemioBrepDiff> + From<CreateEdge> + From<crate::schema::mutations::delete_edge::DeleteEdge>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "edge", kind: "create-edge", record: "CreatedEdge" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create edge \"{}\"", self.id), &format!("Kante \"{}\" erstellen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioBrepSnapshot, Op> for CreateEdge
where
    Op: protocol::ColdMutation<SemioBrepSnapshot, ReferenceDiff = crate::schema::diff::SemioBrepDiff> + From<CreateEdge> + From<crate::schema::mutations::delete_edge::DeleteEdge>,
{
    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioBrepSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
