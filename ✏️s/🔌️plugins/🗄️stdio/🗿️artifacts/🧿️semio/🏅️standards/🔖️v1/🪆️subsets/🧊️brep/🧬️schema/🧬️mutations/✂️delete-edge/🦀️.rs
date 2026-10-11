//! 🗑️ `delete-edge` — removes an id-keyed edge. Deliberately does NOT cascade into `loop.edges` membership: `Loop` carries no `PersistentLabel` (SMO's ruling excludes `create-loop`/`delete-loop` for exactly this reason) and no modify-verb exists for a loop's edge list, so severing that membership here would produce a diff with no corresponding inverse in the approved vocabulary — flagged as an honest limitation, not silently invented. Absent `id` is `mutation.target-missing` (Error, empty diff) — see the sibling `🔺️diff` leaf, which is the authority; this line used to claim a no-op and did not match it.

use crate::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteEdge {
    pub id: String,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for DeleteEdge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "edge", kind: "delete-edge", record: "DeletedEdge" };

    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete edge \"{}\"", self.id), &format!("Kante \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
