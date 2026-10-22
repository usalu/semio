//! 🗑️ `delete-edge` — removes an id-keyed edge. Deliberately does NOT cascade into `loop.edges` membership: `Loop` carries no `PersistentLabel` (SMO's ruling excludes `create-loop`/`delete-loop` for exactly this reason) and no modify-verb exists for a loop's edge list, so severing that membership here would produce a diff with no corresponding inverse in the approved vocabulary — flagged as an honest limitation, not silently invented. Absent `id` is `mutation.target-missing` (Error, empty diff) — see the sibling `🔺️diff` leaf, which is the authority; this line used to claim a no-op and did not match it.

use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteEdge {
    pub id: String,
}

impl<Op> protocol::MutationKind<SemioBrepSnapshot, Op> for DeleteEdge
where
    Op: protocol::Mutation<SemioBrepSnapshot, Diff = crate::schema::diff::SemioBrepDiff> + From<DeleteEdge> + From<crate::schema::mutations::create_edge::CreateEdge>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "edge", kind: "delete-edge", record: "DeletedEdge" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete edge \"{}\"", self.id), &format!("Kante \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioBrepSnapshot, Op> for DeleteEdge
where
    Op: protocol::ColdMutation<SemioBrepSnapshot, ReferenceDiff = crate::schema::diff::SemioBrepDiff> + From<DeleteEdge> + From<crate::schema::mutations::create_edge::CreateEdge>,
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
